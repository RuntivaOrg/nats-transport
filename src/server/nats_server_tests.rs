#[cfg(test)]
mod nats_server_tests {
    use serde::Serialize;

    use crate::proto_test as proto;
    use crate::server::{NatsServer, NatsTransportError};

    #[tokio::test]
    async fn test_publish_prost() -> Result<(), NatsTransportError> {
        let nats = NatsServer::initialize("".into()).await?;

        let test_msg = proto::UserData {
            id: "1234".into(),
            attr: "test".into(),
            credit: 23.23,
        };

        use crate::server::nats_server::PublishProst;
        nats.publish("chat.abc.test".into(), test_msg).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_publish_json() -> Result<(), NatsTransportError> {
        let nats = NatsServer::initialize("".into()).await?;

        let test_msg = SerializableUser {
            id: "1234".into(),
            attr: "test".into(),
            credit: 23.23,
        };

        use crate::server::nats_server::PublishJson;
        let subject = format!("chat.channel.command.create.[channel_id]");

        nats.publish(subject, test_msg).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_push_msg() -> Result<(), NatsTransportError> {
        let nats = NatsServer::initialize("".into()).await?;

        let msg = "My test message".to_string();

        nats.push_msg("chat.channel.test".into(), msg)
            .await
            .unwrap();

        Ok(())
    }

    #[derive(Debug, Serialize)]
    struct SerializableUser {
        pub id: String,
        pub attr: String,
        pub credit: f32,
    }
}
