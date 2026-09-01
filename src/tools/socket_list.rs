use std::{future::Future, sync::Arc};

use rig::tool::{Tool, ToolContext};

use serde_json::{Value, json};

use crate::probes::{CommandObservation, ProbeError, ReadOnlyFreeBsdProbe};

use super::NoArgs;

pub struct SocketListTool {
    probe: Arc<dyn ReadOnlyFreeBsdProbe>,
}

impl SocketListTool {
    pub fn new(probe: Arc<dyn ReadOnlyFreeBsdProbe>) -> Self {
        Self { probe }
    }
}

impl Tool for SocketListTool {
    const NAME: &'static str = "get_socket_list";

    type Args = NoArgs;

    type Output = CommandObservation;

    type Error = ProbeError;

    fn description(&self) -> String {
        concat!(
            "Read currently open IPv4 and IPv6 sockets ",
            "on the FreeBSD host. ",
            "This operation is read-only."
        )
        .into()
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties":
                false
        })
    }

    fn call(
        &self,
        _context: &mut ToolContext,
        _args: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send {
        let probe = Arc::clone(&self.probe);

        async move { probe.socket_list().await }
    }
}
