//#![feature(async_closure)]
use std::sync::Arc;

use async_nats::Message;
use async_trait::async_trait;
use futures::future::BoxFuture;
use futures::Future;
use futures::StreamExt;

use crate::server::NatsServer;

use super::Subscribe;

pub struct NatsReceiver {}

impl Default for NatsReceiver {
    fn default() -> Self {
        Self::new()
    }
}

impl NatsReceiver {
    pub fn new() -> NatsReceiver {
        NatsReceiver {}
    }
}

#[async_trait(?Send)]
impl Subscribe for NatsReceiver {
    async fn subscribe<F, Fut>(
        &self,
        nats_server: Arc<NatsServer>,
        subject: String,
        proc: F,
    ) -> Result<(), BoxFuture<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(Message) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync,
    {
        tokio::task::spawn(async move {
            let result = nats_server.client().subscribe(subject).await?;

            // TODO: How to properly handle this error?
            // if let Err(e) = result {
            //     panic!("Unable to initialize NATS subscription for : {}", e);
            // }

            // let mut subscription = result.unwrap();
            let mut subscription = result;
            while let Some(message) = subscription.next().await {
                proc(message).await;
            }

            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        Ok(())
    }
}

#[cfg(test)]
#[path = "./nats_receiver_tests.rs"]
mod nats_receiver_tests;
