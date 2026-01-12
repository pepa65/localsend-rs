use serde::{Deserialize, Serialize};

use crate::localsend_proto::{DeviceType, constants::PROTOCOL_VERSION_2};

use super::ProtocolType;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticastDto {
	pub alias: String,
	pub version: Option<String>, // v2, format: major.minor
	pub device_model: Option<String>,
	pub device_type: Option<DeviceType>, // nullable since v2
	pub fingerprint: String,
	pub port: Option<u16>,              // v2
	pub protocol: Option<ProtocolType>, // v2
	pub download: Option<bool>,         // v2
	pub announcement: Option<bool>,     // v1
	pub announce: Option<bool>,         // v2
}

impl MulticastDto {
	pub fn v2(alias: impl ToString, device_model: Option<String>, device_type: DeviceType, fingerprint: impl ToString, port: u16, announcement: bool) -> Self {
		Self {
			alias: alias.to_string(),
			version: Some(PROTOCOL_VERSION_2.to_string()),
			device_model,
			device_type: Some(device_type),
			fingerprint: fingerprint.to_string(),
			port: Some(port),
			protocol: Some(ProtocolType::Http),
			download: None,
			announcement: Some(announcement),
			announce: None,
		}
	}
}
