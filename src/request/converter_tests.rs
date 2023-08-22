#[cfg(test)]
mod converter_tests {
    use std::vec;

    use bytes::Bytes;
    use chat_proto::runtiva::chatgroups::v1 as chatgroups_proto;
    use tonic::metadata::MetadataValue;

    use crate::{
        request::{Converter, NatsEnvelope},
        server::serde::NatsMessageSerde,
    };

    #[test]
    fn converter_test() {
        let bytes = b"\n\x0e\n\x02te\x12\x08trailers\n \n\x0ccontent-type\x12\x10application/grpc\nE\n\nuser-agent\x127grpc-node/1.24.7 grpc-c/8.0.0 (windows; chttp2; ganges)\n-\n\x14grpc-accept-encoding\x12\x15identity,deflate,gzip\n \n\x0faccept-encoding\x12\ridentity,gzip\x12$\x08\x86\xc8\xb8\xd7\xa3\xe5\xb4\x0c\x12\x05Hello\x18\n\"\x10\x86\xc8\xb8\xd7\xa3\xe5\xb4\x0c\x9b\xae\xb8\xd7\xa3\xe5\xb4\x0c";

        let bytes: Bytes = bytes.to_vec().into();

        let serde = NatsMessageSerde::<chatgroups_proto::NatsChatGroupCreateRequest>::default();

        let conv = Converter::new(serde);
        let result: NatsEnvelope<super::CreateChatGroupRequest> = conv.convert(bytes).unwrap();

        assert_eq!(result.headers.0.len(), 5);
        assert_eq!(
            result.headers.0.get("te").unwrap(),
            MetadataValue::from_static("trailers")
        );

        assert_eq!(
            result.headers.0.get("content-type").unwrap(),
            MetadataValue::from_static("application/grpc")
        );

        assert_eq!(
            result.headers.0.get("user-agent").unwrap(),
            MetadataValue::from_static("grpc-node/1.24.7 grpc-c/8.0.0 (windows; chttp2; ganges)")
        );

        let values = result.headers.0.get_all("grpc-accept-encoding");
        let mut i = values.iter();
        assert_eq!(
            *i.next().unwrap(),
            MetadataValue::from_static("identity,deflate,gzip")
        );
        assert!(i.next().is_none());

        assert_eq!(result.data.owner_id, 6987577771828230);
        assert_eq!(result.data.title, "Hello".to_string());
        assert_eq!(result.data.ttl_period, 10);
        assert_eq!(result.data.user_ids.len(), 2);
        assert_eq!(
            result.data.user_ids,
            vec![6987577771828230, 6987577771824923]
        );
    }
}

use chat_proto::runtiva::chatgroups::v1 as chatgroups_proto;
use serde::{Deserialize, Serialize};

use crate::{
    request::nats_request::{RequestHeaders, TryFromNatsRequest},
    server::NatsTransportError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateChatGroupRequest {
    pub id: i64,
    pub public_id: String,
    pub owner_id: i64,
    pub title: String,
    pub ttl_period: i32,
    pub user_ids: Vec<i64>,
}

impl TryFromNatsRequest<chatgroups_proto::NatsChatGroupCreateRequest> for CreateChatGroupRequest {
    type Error = NatsTransportError;

    fn try_from(
        value: chatgroups_proto::NatsChatGroupCreateRequest,
    ) -> Result<(Self, RequestHeaders), Self::Error> {
        let id = 1000i64;
        let public_id = "random_val".to_string();

        let headers: RequestHeaders = value.headers.into();

        let value = value.data.unwrap();

        let request = CreateChatGroupRequest {
            id,
            public_id,
            owner_id: value.owner_id,
            title: value.title,
            ttl_period: value.ttl_period,
            user_ids: value.user_ids.into_iter().map(|id| id).collect(),
        };

        Ok((request, headers))
    }
}
