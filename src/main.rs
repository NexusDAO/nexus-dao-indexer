extern crate diesel;

use crate::models::{Input, NewRecord, Output};
use anyhow::{format_err, Context, Error};
use axum::{extract::Query, response::Json, routing::get, Router};
use clap::{Parser, Subcommand};
use diesel::{
    r2d2::{ConnectionManager, PoolError},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use futures03::StreamExt;
use lazy_static::lazy_static;
use models::{Record, RespRecords};
use prost::Message;
use proto::{module_output::Data as ModuleOutputData, BlockScopedData, Records};
use r2d2::{Pool, PooledConnection};
use std::{collections::HashMap, env, net::SocketAddr, str::FromStr, sync::Arc};
use substreams::SubstreamsEndpoint;
use substreams_stream::{BlockResponse, SubstreamsStream};

mod models;
mod proto;
mod schema;
mod substreams;
mod substreams_stream;

type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool<ConnectionManager<PgConnection>> = create_pg_pool().unwrap();
}

/// Simple programvscode-file://vscode-app/Applications/Visual%20Studio%20Code.app/Contents/Resources/app/out/vs/code/electron-sandbox/workbench/workbench.html to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Synchronize data from endpoint and save it to a database.
    Sync {
        /// Substreams gRPC endpoint url
        #[arg(short, long, default_value_t = String::from("http://localhost:18015"))]
        endpoint_url: String,

        /// File path for ".spkg"
        #[arg(short, long)]
        package_file: String,

        /// Module name
        #[arg(short, long)]
        module_name: String,

        /// Start block to stream from
        #[arg(short, long)]
        start_block: i64,

        /// Start block to stream from
        #[arg(short = 't', long, default_value_t = u64::MAX)]
        end_block: u64,
    },
    /// Start query service.
    Serve {
        /// Port to listen on
        #[arg(short = 'P', long, default_value_t = 8080)]
        port: u16,

        /// Host ip to listen on
        #[arg(short = 'H', long, default_value_t = String::from("127.0.0.1"))]
        host: String,
    },

    /// Start both `sync` and `serve` services simultaneously
    All {
        /// Substreams gRPC endpoint url
        #[arg(short, long, default_value_t = String::from("http://localhost:18015"))]
        endpoint_url: String,

        /// File path for ".spkg"
        #[arg(short, long)]
        package_file: String,

        /// Module name
        #[arg(short, long)]
        module_name: String,

        /// Start block to stream from
        #[arg(short, long)]
        start_block: i64,

        /// Start block to stream from
        #[arg(short = 't', long, default_value_t = u64::MAX)]
        end_block: u64,

        /// Port to listen on
        #[arg(short = 'P', long, default_value_t = 8080)]
        port: u16,

        /// Host ip to listen on
        #[arg(short = 'H', long, default_value_t = String::from("127.0.0.1"))]
        host: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Sync {
            endpoint_url,
            package_file,
            module_name,
            start_block,
            end_block,
        }) => {
            sync(
                endpoint_url,
                package_file,
                module_name,
                start_block,
                end_block,
            )
            .await;
        }

        Some(Commands::Serve { port, host }) => {
            serve(host, port).await;
        }

        Some(Commands::All {
            endpoint_url,
            package_file,
            module_name,
            start_block,
            end_block,
            port,
            host,
        }) => {
            tokio::join!(
                sync(
                    endpoint_url,
                    package_file,
                    module_name,
                    start_block,
                    end_block,
                ),
                serve(host, port),
            );
        }

        None => {}
    }
}

async fn sync(
    endpoint_url: &String,
    package_file: &String,
    module_name: &String,
    start_block: &i64,
    end_block: &u64,
) {
    let token_env = env::var("SUBSTREAMS_API_TOKEN").unwrap_or("".to_string());
    let mut token: Option<String> = None;
    if token_env.len() > 0 {
        token = Some(token_env);
    }

    let package = read_package(&package_file).unwrap();
    let endpoint = Arc::new(SubstreamsEndpoint::new(&endpoint_url, token).await.unwrap());

    // FIXME: Handling of the cursor is missing here. It should be loaded from
    // the database and the SubstreamStream will correctly resume from the right
    // block.
    let cursor: Option<String> = None;

    let mut stream = SubstreamsStream::new(
        endpoint.clone(),
        cursor,
        package.modules.clone(),
        module_name.to_string(),
        *start_block,
        *end_block,
    );

    let mut conn = POOL.get().unwrap();

    loop {
        match stream.next().await {
            None => {
                println!("Stream consumed");
                break;
            }
            Some(event) => match event {
                Err(_) => {}
                Ok(BlockResponse::New(data)) => {
                    println!("Consuming module output (cursor {})", data.cursor);

                    match extract_records(data, &module_name).unwrap() {
                        Some(record) => {
                            batch_insert_records(&mut conn, &record)
                                .context("insertion in db failed")
                                .unwrap();
                        }
                        None => {}
                    }

                    // FIXME: Handling of the cursor is missing here. It should be saved each time
                    // a full block has been correctly inserted in the database. By saving it
                    // in the database, we ensure that if we crash, on startup we are going to
                    // read it back from database and start back our SubstreamsStream with it
                    // ensuring we are continuously streaming without ever losing a single
                    // element.
                }
            },
        }
    }
}

async fn serve(host: &String, port: &u16) {
    let app = Router::new().route("/records", get(records_handler));
    let addr = SocketAddr::from_str(&format!("{}:{}", host, port)).unwrap();
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn records_handler(Query(params): Query<HashMap<String, String>>) -> Json<Vec<RespRecords>> {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = POOL.get().unwrap();
    let default_start_block = "0".to_string();
    let default_end_block = i64::MAX.to_string();
    let start_block = params.get("start_block").unwrap_or(&default_start_block);
    let start_block = i64::from_str(start_block).unwrap_or(0);
    let end_block = params.get("end_block").unwrap_or(&default_end_block);
    let end_block = i64::from_str(&end_block).unwrap_or(i64::MAX);

    let records = get_records_by_height(&mut conn, start_block, end_block).unwrap();

    let results = records
        .iter()
        .map(|record| {
            let outputs: Vec<Output> = serde_json::from_str(&record.outputs).unwrap();
            let record_values = outputs
                .into_iter()
                .filter_map(|output| {
                    if output.r#type.eq("record") {
                        Some(output.value)
                    } else {
                        None
                    }
                })
                .collect();

            RespRecords {
                records: record_values,
                transaction_id: record.transaction_id.clone(),
                transition_id: record.transition_id.clone(),
                network: record.network,
                height: record.height,
                timestamp: record.timestamp,
            }
        })
        .collect::<Vec<RespRecords>>();

    Json(results)
}

fn batch_insert_records(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    records: &Records,
) -> Result<(), Error> {
    use schema::record;

    for record in records.records.iter() {
        let inputs = record
            .inputs
            .iter()
            .map(|input| Input {
                r#type: input.r#type.clone(),
                id: input.id.clone(),
                value: input.value.clone(),
            })
            .collect::<Vec<Input>>();

        let outputs = record
            .outputs
            .iter()
            .map(|output| Output {
                r#type: output.r#type.clone(),
                id: output.id.clone(),
                checksum: output.checksum.clone(),
                value: output.value.clone(),
            })
            .collect::<Vec<Output>>();

        let new_record = NewRecord {
            program: &record.program,
            function: &record.function,
            inputs: &serde_json::to_string(&inputs).unwrap(),
            outputs: &serde_json::to_string(&outputs).unwrap(),
            block_hash: &record.block_hash,
            previous_hash: &record.previous_hash,
            transaction_id: &record.transaction_id,
            transition_id: &record.transition_id,
            network: record.network as i64,
            height: record.height as i64,
            timestamp: record.timestamp as i64,
        };

        diesel::insert_into(record::table)
            .values(&new_record)
            .on_conflict(record::transition_id)
            .do_nothing()
            .execute(conn)?;
    }

    Ok(())
}

fn get_records_by_height(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    start_block: i64,
    end_block: i64,
) -> Result<Vec<Record>, Error> {
    use schema::record::dsl::*;

    let records = record
        .filter(height.between(start_block, end_block))
        .select(Record::as_select())
        .load(conn)
        .expect("Error loading records");

    Ok(records)
}

fn extract_records(data: BlockScopedData, module_name: &String) -> Result<Option<Records>, Error> {
    let output = data
        .outputs
        .first()
        .ok_or(format_err!("expecting one module output"))?;
    if &output.name != module_name {
        return Err(format_err!(
            "invalid module output name {}, expecting {}",
            output.name,
            module_name
        ));
    }
    match output.data.as_ref().unwrap() {
        ModuleOutputData::MapOutput(data) => {
            let records: Records = Message::decode(data.value.as_slice())?;
            Ok(Some(records))
        }
        ModuleOutputData::StoreDeltas(_) => Err(format_err!(
            "invalid module output StoreDeltas, expecting MapOutput"
        )),
    }
}

fn read_package(file: &str) -> Result<proto::Package, anyhow::Error> {
    let content = std::fs::read(file).context(format_err!("read package {}", file))?;
    proto::Package::decode(content.as_ref()).context("decode command")
}

pub fn create_pg_pool() -> Result<PgPool, PoolError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    PgPool::builder().build(manager)
}
