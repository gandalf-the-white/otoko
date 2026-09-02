use std::{convert::Infallible, future::Future, sync::Arc};

use rig::tool::{Tool, ToolContext};

use serde_json::{Value, json};

use crate::{
    probes::ReadOnlyFreeBsdProbe,
    tools::{ObservationHistory, ToolObservation},
};

use super::NoArgs;

pub struct SocketListTool {
    probe: Arc<dyn ReadOnlyFreeBsdProbe>,
    history: ObservationHistory,
}

impl SocketListTool {
    pub fn new(probe: Arc<dyn ReadOnlyFreeBsdProbe>, history: ObservationHistory) -> Self {
        Self { probe, history }
    }
}

impl Tool for SocketListTool {
    const NAME: &'static str = "get_socket_list";

    type Args = NoArgs;

    type Output = ToolObservation;

    type Error = Infallible;

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
        let history = self.history.clone();

        async move {
            let observation = match probe.socket_list().await {
                Ok(value) => ToolObservation::success(Self::NAME, value),

                Err(error) => ToolObservation::from_probe_error(Self::NAME, error),
            };

            history.record(observation.clone()).await;

            Ok(observation)
        }
    }
}
