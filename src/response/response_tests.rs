#[cfg(test)]
mod response_tests {
    use crate::response::{ErrorReason, MetaKeys, StandardNatsResponse, Status};

    #[test]
    fn test_success_response() {
        let id = uuid::Uuid::new_v4();
        let success_response = StandardNatsResponse::new(SampleResponse {
            id,
            name: "test".to_string(),
        });

        assert_eq!(success_response.data.as_ref().unwrap().id, id);
        assert_eq!(
            success_response.data.as_ref().unwrap().name,
            "test".to_string()
        );
    }

    #[test]
    fn test_error_response() {
        let error_response: StandardNatsResponse<()> =
            StandardNatsResponse::with_error(Status::DatabaseError)
                .message("Unable to connect to database".to_string())
                .with_details(
                    ErrorReason::ApiKeyInvalid,
                    "chat.persist.server".to_string(),
                )
                .append_metadata(MetaKeys::DatabaseError, "127.0.0.1".to_string());

        assert!(error_response.data.is_none());

        let err = error_response.error.as_ref().unwrap();
        assert_eq!(err.code, Status::DatabaseError as i32);
        assert_eq!(err.status, Status::DatabaseError);
        assert_eq!(err.message, "Unable to connect to database".to_string());
        assert_eq!(err.details.len(), 1);
        assert_eq!(
            err.details.first().unwrap().reason,
            ErrorReason::ApiKeyInvalid
        );
        assert_eq!(
            err.details.first().unwrap().domain,
            "chat.persist.server".to_string()
        );
    }

    #[allow(dead_code)]
    struct SampleResponse {
        id: uuid::Uuid,
        name: String,
    }
}
