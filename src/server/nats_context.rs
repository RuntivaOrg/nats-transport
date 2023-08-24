use chat_proto::runtiva::nats::v1 as proto_nats;

pub struct NatsContext {
    pub user_id: String,
    pub metadata: Option<proto_nats::MetadataMap>,
}

impl NatsContext {
    pub fn new(user_id: String, metadata: Option<proto_nats::MetadataMap>) -> Self {
        Self { user_id, metadata }
    }
}
