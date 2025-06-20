use crate::event::EngineEvent;
use common::types::MatchResult;

pub fn generate_events(results: Vec<MatchResult>) -> Vec<EngineEvent> {
    results.into_iter().map(EngineEvent::Match).collect()
}