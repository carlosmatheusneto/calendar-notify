use anyhow::{Context, Result};
use notify_rust::{Notification, Timeout, Urgency};

pub const APP_NAME: &str = "calendar-notify";

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub spec_version: String,
    pub capabilities: Vec<String>,
}

impl ServerInfo {
    pub fn supports(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability) 
    }
}

/// Calls GetServerInformatio and GetCapabilities on org.freedesktop.Notifications
pub fn server_info() -> Result<ServerInfo> {
    let info = notify_rust::get_server_information().context(
        "No notification daemon answered on D-Bus - check if there's a daemon running"
    )?;
    let capabilities = notify_rust::get_capabilities().context(
        "Failed to consult GetCapabilities"
    )?;
    Ok(ServerInfo{
        name: info.name,
        vendor: info.vendor,
        version: info.version,
        spec_version: info.spec_version,
        capabilities,
    }) 
}

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Snooze,
    Join,
    Closed,
    Other(String),
}
