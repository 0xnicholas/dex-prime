use common::types::MatchResult;

#[derive(Debug)]
pub enum EngineEvent {
    Match(MatchResult),
    OrderPlaced(String),
    OrderCancelled(String),
}