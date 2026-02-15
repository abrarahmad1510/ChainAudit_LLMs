//! Include generated protobuf modules
include!(concat!(env!("OUT_DIR"), "/google.rpc.rs"));
include!(concat!(env!("OUT_DIR"), "/trillian.rs"));

// Re-export commonly used types for easier access
pub use self::trillian::*;
pub use self::google::rpc::Status;
