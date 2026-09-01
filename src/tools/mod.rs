mod disk_usage;
mod recent_logins;
mod service_status;
mod socket_list;

pub use disk_usage::DiskUsageTool;
pub use recent_logins::RecentLoginsTool;
pub use service_status::{ServiceStatusArgs, ServiceStatusTool};
pub use socket_list::SocketListTool;

#[derive(Debug, serde::Deserialize)]
pub struct NoArgs {}
