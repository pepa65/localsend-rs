use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),
	#[error(transparent)]
	Receive(#[from] crate::localsend_lib::receive::ReceiveError),
	#[error(transparent)]
	Send(#[from] crate::localsend_lib::send::SendError),
	#[error(transparent)]
	WalkDir(#[from] walkdir::Error),
}
