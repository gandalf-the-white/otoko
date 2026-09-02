use std::sync::Arc;

use rig::tool::{Tool, ToolContext};

use otoko::{
    probes::FakeFreeBsdProbe,
    tools::{ObservationHistory, ServiceStatusArgs, ServiceStatusTool},
};

#[tokio::test]
async fn service_status_tool_uses_probe() {
    let probe = Arc::new(FakeFreeBsdProbe);
    let history = ObservationHistory::new();

    let tool = ServiceStatusTool::new(probe, history);

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
    let history = ObservationHistory::new();
    let tool = ServiceStatusTool::new(Arc::new(FakeFreeBsdProbe), history);

    let description = tool.description();

    assert!(description.contains("read-only"));
}
