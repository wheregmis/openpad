#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateEffect {
    RequestSessionDiff {
        session_id: String,
        message_id: Option<String>,
    },
}
