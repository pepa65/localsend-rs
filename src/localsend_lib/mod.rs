pub mod error;
pub mod receive;
pub mod scanner;
pub mod send;
pub mod server;
pub mod settings;
pub mod util;

pub use error::*;

pub use {
	Error, settings::Settings,
	scanner::MulticastDeviceScanner,
	send::{SendError, SendSession, SendingFiles, UploadProgress},
	server::{ClientMessage, ServerMessage, ServerState, start_api_server},
	util::device,
};

pub use crate::localsend_proto::{
	dto::FileType,
	DeviceType,
	route::ApiRoute,
};

pub type Result<T> = std::result::Result<T, Error>;
