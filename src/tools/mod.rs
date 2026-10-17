mod disk_usage;
mod history;
mod recent_login;
mod result;
mod service_status;
mod socket_list;

pub use disk_usage::DiskUsageTool;
pub use history::ObservationHistory;
pub use recent_login::RecentLoginsTool;
pub use service_status::{ServiceStatusArgs, ServiceStatusTool};
pub use socket_list::SocketListTool;

#[derive(Debug, serde::Deserialize)]
pub struct NoArgs {}
