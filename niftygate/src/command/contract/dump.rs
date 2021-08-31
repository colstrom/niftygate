use ethcontract::{Event, EventStatus, StreamEvent};
use serde::Serialize;
use std::error::Error;

pub fn query<T>(events: Vec<Event<T>>)
where
  T: Serialize,
{
  for event in events {
    if let Ok(json) = serde_json::to_string(&event.data) {
      std::println!("{}", json)
    }
  }
}

const ADDED: &str = "Added";
const REMOVED: &str = "Removed";

pub async fn stream<T, E>(event: Result<StreamEvent<T>, E>)
where
  T: Serialize,
  E: Error,
{
  if let Ok(event) = event {
    if let Ok(json) = serde_json::to_string(event.inner_data()) {
      let status = match event.data {
        EventStatus::Added(_) => ADDED,
        EventStatus::Removed(_) => REMOVED,
      };
      async_std::println!("{{ \"status\": \"{}\", \"event\": {} }}", status, json).await
    }
  }
}
