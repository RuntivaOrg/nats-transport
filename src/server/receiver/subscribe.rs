use std::sync::Arc;

use async_nats::Message;
use async_trait::async_trait;
use futures::{future::BoxFuture, Future};

use crate::server::NatsServer;

/// Trait for subscribing to a NATs subject.
/// examples of subject: `name.abc`, `name.abc.>`, `name.abc.*`, `name.abc.*.def`
/// F is the callback function that will be called when a message is received.
#[async_trait(?Send)]
pub trait Subscribe {
    async fn subscribe<F, Fut>(
        &self,
        nats_server: Arc<NatsServer>,
        subject: String,
        proc: F,
    ) -> Result<(), BoxFuture<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(Message) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync;
}
