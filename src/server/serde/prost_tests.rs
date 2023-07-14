#[cfg(test)]
mod prost_tests {
    use serde::{Deserialize, Serialize};

    use crate::server::{
        proto,
        serde::{prost::NatsMessageSerde, Deserializer, Serializer},
    };

    #[test]
    fn test_prost_serialization() {
        let serde = NatsMessageSerde::<proto::UserData>::default();

        let test_msg = proto::UserData {
            id: "1234".into(),
            attr: "test".into(),
            credit: 23.23,
        };

        let serialized_test_msg = serde.serialize(test_msg.clone());

        //assert_eq!(vec![1], serialized_test_msg);
        // [10, 4, 49, 50, 51, 52, 18, 4, 116, 101, 115, 116, 25, 123, 20, 174, 71, 225, 58, 55, 64]

        // JSON:
        // [123, 34, 105, 100, 34, 58, 34, 49, 50, 51, 52, 34, 44, 34, 110, 105, 99, 107, 34, 58, 34, 116, 101, 115, 116, 34, 44, 34, 99, 114, 101, 100, 105, 116, 34, 58, 50, 51, 46, 50, 51, 125]

        let deserized_test_msg = serde.deserialize(serialized_test_msg.into()).unwrap();

        assert_eq!(deserized_test_msg, test_msg);
    }

    #[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
    struct SerializableUser {
        pub id: String,
        pub attr: String,
        pub credit: f32,
    }
}
