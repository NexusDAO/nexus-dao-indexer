use crate::models::{Mapping, Operation, Ratify};
use async_graphql::futures_util::Stream;
use async_graphql::*;
use std::time::Duration;

/// The GraphQL top-level Query type
#[derive(MergedObject, Default)]
pub struct Query(RatificationsQuery, OperationsQuery);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, EmptyMutation, SubscriptionRoot>;

/// Create the GraphQL schema
pub fn create_schema() -> GraphQLSchema {
    Schema::build(Query::default(), EmptyMutation, SubscriptionRoot::default()).finish()
}

/// The Query segment for Operations
#[derive(Default)]
pub struct OperationsQuery;

/// The Query segment for Ratifications
#[derive(Default)]
pub struct RatificationsQuery;

/// Queries for the `Operation` model
#[Object]
impl OperationsQuery {
    async fn operation(&self, program_name: String) -> anyhow::Result<Vec<Operation>> {
        Operation::list_by_program_name(&program_name)
    }
}

/// Queries for the `Ratify` model
#[Object]
impl RatificationsQuery {
    async fn ratifications(&self, height: i64) -> anyhow::Result<Vec<Ratify>> {
        Ratify::list_by_height(height)
    }
}

/// The GraphQL top-level Subscription type
#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(MappingSubscription);

#[derive(Default)]
pub struct MappingSubscription;

// TODO: Using channel to get the mapping status
/// Subscription for the mapping status
#[Subscription]
impl MappingSubscription {
    async fn mappings(
        &self,
        program_name: String,
        mapping_name: String,
        key: String,
    ) -> impl Stream<Item = Result<String>> {
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                let mapping = Mapping::get_mapping_by_mapping_key(&program_name, &mapping_name, &key)?;
                yield Ok(mapping.value);
            }
        }
    }
}
