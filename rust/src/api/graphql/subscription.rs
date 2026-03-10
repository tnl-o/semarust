//! GraphQL Subscription корень - заглушка

use async_graphql::{Context, Object, Subscription, Result};
use futures::{Stream, stream};

/// Корневой тип для Subscription
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    /// Подписка на создание задач (заглушка)
    async fn task_created(&self, _ctx: &Context<'_>) -> Result<impl Stream<Item = String>> {
        Ok(stream::empty())
    }
}
