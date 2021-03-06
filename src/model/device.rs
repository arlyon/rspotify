use crate::senum::DeviceType;
use serde::{Deserialize, Serialize};

/// All objects related to device
/// [get a users available devices](https://developer.spotify.com/web-api/get-a-users-available-devices/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub is_active: bool,
    pub is_restricted: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: DeviceType,
    pub volume_percent: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevicePayload {
    pub devices: Vec<Device>,
}
