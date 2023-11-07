use crate::models::{Mapping, Ratify};
use anyhow::Error;
use async_graphql::futures_util::Stream;
use async_graphql::*;
use std::time::Duration;
use tokio_stream::StreamExt;

/// The GraphQL top-level Query type
#[derive(MergedObject, Default)]
pub struct Query(RatificationsQuery, MappingsQuery);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, EmptyMutation, SubscriptionRoot>;

/// Create the GraphQL schema
pub fn create_schema() -> GraphQLSchema {
    Schema::build(Query::default(), EmptyMutation, SubscriptionRoot::default()).finish()
}

/// The Query segment for Ratifications
#[derive(Default)]
pub struct RatificationsQuery;

/// The Query segment for Mappings
#[derive(Default)]
pub struct MappingsQuery;

/// Queries for the `Ratify` model
#[Object]
impl RatificationsQuery {
    async fn ratifications(&self, height: i64) -> Result<Vec<Ratify>, Error> {
        Ratify::list_by_height(height)
    }
}

/// Queries for the `Mapping` model
#[Object]
impl MappingsQuery {
    async fn mapping(
        &self,
        program_name: String,
        mapping_name: String,
        mapping_key: String,
    ) -> Result<Mapping, Error> {
        Mapping::find(&program_name, &mapping_name, &mapping_key)
    }
}

/// The GraphQL top-level Subscription type
#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(HeightSubscription);

#[derive(Default)]
pub struct HeightSubscription;

/// Subscription for the latest height
#[Subscription]
impl HeightSubscription {
    async fn latest_height(&self) -> impl Stream<Item = i64> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += 1;
                yield value;
            }
        }
    }
}
