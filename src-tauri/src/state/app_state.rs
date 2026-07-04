#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState { Idle, Listening, Processing, Typing, Error }
