use std::sync::Arc;

use rig::tool::{Tool, ToolContext};

use otoko::{
    probes::FakeFreeBsdProbe,
    tools::{DiskUsageTool, ObservationHistory, ServiceStatusArgs, ServiceStatusTool},
};

#[tokio::test]
async fn service_status_tool_uses_probe() {
    let probe = Arc::new(FakeFreeBsdProbe);

    let tool = ServiceStatusTool::new(probe, ObservationHistory::new());

    let mut context = ToolContext::new();

    let result = tool
        .call(
            &mut context,
            ServiceStatusArgs {
                service: "sshd".into(),
            },
        )
        .await
        .expect("tool should succeed");

    assert!(result.stdout.contains("sshd is running"));
}

#[test]
fn service_status_tool_has_safe_description() {
    let tool = ServiceStatusTool::new(Arc::new(FakeFreeBsdProbe), ObservationHistory::new());

    let description = tool.description();

    assert!(description.contains("read-only"));
}

#[tokio::test]
async fn disk_usage_tool_uses_probe() {
    let probe = Arc::new(FakeFreeBsdProbe);

    let tool = DiskUsageTool::new(probe, ObservationHistory::new());

    let mut context = ToolContext::new();

    let result = tool
        .call(&mut context, otoko::tools::NoArgs {})
        .await
        .expect("tool should succeed");

    assert!(result.stdout.contains("/dev/ada0p2"));
}

#[test]
fn service_status_tool_has_read_only_description() {
    let tool = ServiceStatusTool::new(Arc::new(FakeFreeBsdProbe), ObservationHistory::new());

    assert!(tool.description().contains("read-only"));
}
