//! GraphQL Mutation корень - заглушка

use async_graphql::{Context, Object, Result};

/// Корневой тип для Mutation
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Заглушка mutation
    async fn ping(&self) -> Result<String> {
        Ok("pong".to_string())
    }
}
