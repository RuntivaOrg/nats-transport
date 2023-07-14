#[cfg(test)]
mod nats_request_tests {

    use tonic::metadata::{MetadataMap, MetadataValue};

    use crate::request::nats_request::RequestHeaders;
    use chat_proto::chat::{self as proto};

    #[test]
    fn test_from_proto_metadata_map_and_back() {
        let headers = proto::MetadataMap {
            key: "key".to_string(),
            value: vec!["value1".to_string(), "value2".to_string()],
        };

        // Create object to test
        let vec_headers = vec![headers];

        // Convert to RequestHeaders
        let converted: RequestHeaders = RequestHeaders::from(vec_headers);

        let values = converted.0.get_all("key");
        let mut i = values.iter();
        assert_eq!(*i.next().unwrap(), MetadataValue::from_static("value1"));
        assert_eq!(*i.next().unwrap(), MetadataValue::from_static("value2"));
        assert_eq!(None, i.next());

        // Convert back to proto MetadataMap
        let and_back: Vec<proto::MetadataMap> = converted.into();

        assert_eq!(and_back.len(), 1);
        assert!(and_back[0].key == "key");
        assert_eq!(and_back[0].value.len(), 2);
        assert_eq!(and_back[0].value[0], "value1");
        assert_eq!(and_back[0].value[1], "value2");

        //let converted: RequestHeaders = RequestHeaders::from(converted);
        //panic!("{:?}", converted);
    }

    #[test]
    fn test_from_tonic_metadata_map_to_proto_and_back() {
        let mut map = MetadataMap::new();

        assert_eq!(0, map.len());

        map.insert("x-host-ip", "127.0.0.1".parse().unwrap());
        map.append("x-host-ip", "text/html".parse().unwrap());
        assert_eq!(2, map.len());

        // Create object to test
        let request_headers = RequestHeaders(map);

        // Convert to proto MetadataMap
        let converted: Vec<proto::MetadataMap> = request_headers.into();

        assert_eq!(converted.len(), 1);
        assert!(converted[0].key == "x-host-ip");
        assert_eq!(converted[0].value.len(), 2);
        assert_eq!(converted[0].value[0], "127.0.0.1");

        // Convert back to tonic MetadataMap
        let and_back: RequestHeaders = RequestHeaders::from(converted);
        assert_eq!(2, and_back.0.len());
        let values = and_back.0.get_all("x-host-ip");
        let mut i = values.iter();
        assert_eq!(*i.next().unwrap(), MetadataValue::from_static("127.0.0.1"));
        assert_eq!(*i.next().unwrap(), MetadataValue::from_static("text/html"));
        assert_eq!(None, i.next());
    }
}
