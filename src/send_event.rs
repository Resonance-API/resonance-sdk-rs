use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct SendEventRequest {
    name: String,
    user_id: String,
    event_at: Option<String>, // must be a valid timestamp if provided
    data: HashMap<String, String>,
}

impl SendEventRequest {
    pub fn builder(name: impl Into<String>, user_id: impl Into<String>) -> SendEventRequestBuilder {
        SendEventRequestBuilder::new(name, user_id)
    }
}

pub struct SendEventRequestBuilder {
    name: String,
    user_id: String,
    event_at: Option<String>,
    data: HashMap<String, String>,
}

impl SendEventRequestBuilder {
    fn new(name: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            user_id: user_id.into(),
            event_at: None,
            data: HashMap::new(),
        }
    }

    pub fn event_at(mut self, at: &str) -> Result<Self, String> {
        let dt: DateTime<Utc> = DateTime::from_str(at).map_err(|e| {
            let err_msg = format!("Failed to parse timestamp {at} with error {e:?}");
            log::error!("{err_msg}");
            err_msg
        })?;
        self.event_at = Some(dt.to_rfc3339());
        Ok(self)
    }

    pub fn data(mut self, data: HashMap<String, String>) -> Self {
        self.data = data;
        self
    }

    pub fn build(self) -> SendEventRequest {
        SendEventRequest {
            name: self.name,
            user_id: self.user_id,
            event_at: self.event_at,
            data: self.data,
        }
    }
}
