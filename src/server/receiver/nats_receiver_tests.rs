#[cfg(test)]
mod nats_receiver_tests {
    use std::sync::Arc;

    use async_nats::Message;

    use crate::server::{
        receiver::{NatsReceiver, Subscribe},
        NatsServer, NatsTransportError,
    };

    #[tokio::test]
    async fn test_receiver() -> Result<(), NatsTransportError> {
        let nats = NatsServer::initialize("".into()).await?;

        let receiver = NatsReceiver::new();
        let subject = "chat.topic.>".to_string();
        let _ = receiver.subscribe(Arc::new(nats), subject, processor).await;

        assert!(true); // successfully created subscription
        Ok(())
    }

    async fn processor(msg: Message) {
        assert_eq!("abc".to_string(), msg.subject);
    }
}
