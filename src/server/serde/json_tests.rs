#[cfg(test)]
mod json_tests {
    use serde::{Deserialize, Serialize};

    use crate::server::serde::{json::NatsJson, Deserializer, Serializer};

    #[test]
    fn test_json() {
        let serde = NatsJson::<SerializableUser>::default();

        let test_msg = SerializableUser {
            id: "1234".into(),
            attr: "test".into(),
            credit: 23.23,
        };

        let serialized_test_msg = serde.serialize(test_msg.clone());

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
