use crate::models::{Mapping, Ratify};
use anyhow::Error;
use async_graphql::*;

/// The GraphQL top-level Query type
#[derive(MergedObject, Default)]
pub struct Query(RatificationsQuery, MappingsQuery);

/// The application's top-level merged GraphQL schema
pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> GraphQLSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish()
}

/// The Query segment for Ratifications
#[derive(Default)]
pub struct RatificationsQuery {}

/// The Query segment for Mappings
#[derive(Default)]
pub struct MappingsQuery {}

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
