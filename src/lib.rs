mod localsend_lib;
mod localsend_proto;

pub type Result<T> = std::result::Result<T, crate::localsend_lib::Error>;
