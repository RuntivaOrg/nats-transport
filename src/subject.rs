pub struct SubjectName {}

impl SubjectName {
    pub fn chat_event(domain: &str, evt_name: &str) -> String {
        format!("chat.{}.event.{}", domain, evt_name)
    }
}
