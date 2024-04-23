/// Collection of graphql entities
mod entities;
use async_graphql::{
    ComplexObject, Context, EmptyMutation, EmptySubscription, Object, Schema, SchemaBuilder,
};
use entities::{RobotAction, Session};
use models::robot_action;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

/// The GraphQL schema exposed by the service
pub type RootSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// A schema builder for the service
pub fn root_schema_builder() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription).enable_federation()
}

/// The root query of the service
#[derive(Debug, Clone, Default)]
pub struct Query;

#[ComplexObject]
impl Session {
    /// Fetches all the data collected during a session
    async fn robot_actions(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<RobotAction>> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(robot_action::Entity::find()
            .filter(robot_action::Column::BlsessionId.eq(self.id))
            .all(database)
            .await?
            .into_iter()
            .map(RobotAction::from)
            .collect())
    }
}

#[Object]
impl Query {
    /// Reference Sessions resolver for the router
    #[graphql(entity)]
    async fn router_robot_operations(&self, id: i32) -> Session {
        Session { id }
    }
}
