extern crate diesel;

use crate::database::insert_ratify;
use crate::models::NewRatify;
use crate::routes::router;
use anyhow::{format_err, Context, Error};
use clap::Parser;
use cli::{Cli, Commands};
use futures03::StreamExt;
use prost::Message;
use proto::{module_output::Data as ModuleOutputData, BlockScopedData, Ratifications};
use std::{env, net::SocketAddr, str::FromStr, sync::Arc};
use substreams::SubstreamsEndpoint;
use substreams_stream::{BlockResponse, SubstreamsStream};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

mod cli;
mod database;
mod handlers;
mod models;
mod proto;
mod routes;
mod schema;
mod substreams;
mod substreams_stream;

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

                    if let Some(Ratifications { ratifications }) =
                        extract_ratifications(data, &module_name).unwrap()
                    {
                        ratifications.into_iter().for_each(|ratify| {
                            let starting_round = if ratify.starting_round.is_some() {
                                Some(ratify.starting_round.unwrap().to_string())
                            } else {
                                None
                            };
                            let total_stake = if ratify.total_stake.is_some() {
                                Some(ratify.total_stake.unwrap().to_string())
                            } else {
                                None
                            };
                            let block_reward = if ratify.block_reward.is_some() {
                                Some(ratify.block_reward.unwrap().to_string())
                            } else {
                                None
                            };
                            let puzzle_reward = if ratify.puzzle_reward.is_some() {
                                Some(ratify.puzzle_reward.unwrap().to_string())
                            } else {
                                None
                            };

                            let new_ratify = NewRatify {
                                ratification_id: &ratify.id,
                                height: i64::from(ratify.height),
                                type_: &ratify.r#type,
                                starting_round: starting_round.as_deref(),
                                total_stake: total_stake.as_deref(),
                                block_reward: block_reward.as_deref(),
                                puzzle_reward: puzzle_reward.as_deref(),
                            };
                            insert_ratify(new_ratify)
                                .context("insertion in db failed")
                                .unwrap();
                        });
                    }
                }
                // FIXME: Handling of the cursor is missing here. It should be saved each time
                // a full block has been correctly inserted in the database. By saving it
                // in the database, we ensure that if we crash, on startup we are going to
                // read it back from database and start back our SubstreamsStream with it
                // ensuring we are continuously streaming without ever losing a single
                // element.
            },
        }
    }
}

async fn serve(host: &String, port: &u16) {
    let app = router().layer(
        CorsLayer::new()
            .allow_origin(AllowOrigin::any())
            .allow_methods(AllowMethods::any())
            .allow_headers(AllowHeaders::any()),
    );
    let addr = SocketAddr::from_str(&format!("{}:{}", host, port)).unwrap();
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn extract_ratifications(
    data: BlockScopedData,
    module_name: &String,
) -> Result<Option<Ratifications>, Error> {
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
            let result: Ratifications = Message::decode(data.value.as_slice())?;
            Ok(Some(result))
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
