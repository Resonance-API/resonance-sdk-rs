use send_event::SendEventRequest;
use std::{error::Error, fmt::Display};

mod send_event;

const URL_BASE: &str = "https://api.resonance-api.com";
const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub struct ResonanceClient {
    http_client: reqwest::Client,
    api_key: String,
}

impl ResonanceClient {
    pub fn new(api_key: String) -> Result<Self, String> {
        Ok(Self {
            http_client: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .build()
                .map_err(|e| {
                    let err_msg = format!("Failed to initialize ResonanceClient: {e:?}");
                    log::error!("{err_msg}");
                    err_msg
                })?,
            api_key,
        })
    }

    // event_at must be a valid, parsable timestamp, or the request will be rejected
    pub async fn send_event(&self, req: SendEventRequest) -> Result<(), ResonanceClientError> {
        self.http_client
            .post(format!("{URL_BASE}/events/v1/events"))
            .header("x-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await
            .map_err(|e| {
                let err_msg = format!("Failed making request to send event; {e:?}");
                log::error!("{err_msg}");
                ResonanceClientError::FailedToSendRequest(err_msg)
            })?
            .error_for_status()
            .map_err(ResonanceClientError::from)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ResonanceClientError {
    FailedToSendRequest(String),
    ClientSideError(u16, String),
    ServerSideError(u16, String),
    Unknown(String),
}

impl Display for ResonanceClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl Error for ResonanceClientError {}

impl From<reqwest::Error> for ResonanceClientError {
    fn from(e: reqwest::Error) -> Self {
        let err_msg = format!("Error response from send event request; {e:?}");
        log::error!("{err_msg}");
        if let Some(status) = e.status() {
            if status.is_client_error() {
                ResonanceClientError::ClientSideError(status.as_u16(), e.to_string())
            } else if status.is_server_error() {
                ResonanceClientError::ServerSideError(status.as_u16(), e.to_string())
            } else {
                ResonanceClientError::Unknown(err_msg)
            }
        } else {
            ResonanceClientError::Unknown(err_msg)
        }
    }
}
