use std::{convert::Infallible, future::Future, sync::Arc};

use rig::{
    serde_json,
    tool::{Tool, ToolContext},
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{
    probes::ReadOnlyFreeBsdProbe,
    tools::{ObservationHistory, ToolObservation},
};

#[derive(Debug, Deserialize)]
pub struct ServiceStatusArgs {
    pub service: String,
}

pub struct ServiceStatusTool {
    probe: Arc<dyn ReadOnlyFreeBsdProbe>,
    history: ObservationHistory,
}

impl ServiceStatusTool {
    pub fn new(probe: Arc<dyn ReadOnlyFreeBsdProbe>, history: ObservationHistory) -> Self {
        Self { probe, history }
    }
}

impl Tool for ServiceStatusTool {
    const NAME: &'static str = "get_service_status";

    type Args = ServiceStatusArgs;

    type Output = ToolObservation;

    type Error = Infallible;

    fn description(&self) -> String {
        concat!(
            "Read the current status of a FreeBSD rc.d service. ",
            "This tool is read-only and does not start, stop, ",
            "restart, enable, or modify the service."
        )
        .into()
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "service": {
                    "type": "string",
                    "description":
                        "FreeBSD service name such as sshd or postgresql"
                }
            },
            "required": [
                "service"
            ],
            "additionalProperties":
                false
        })
    }

    fn call(
        &self,
        _context: &mut ToolContext,
        args: Self::Args,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send {
        let probe = Arc::clone(&self.probe);
        let history = self.history.clone();

        async move {
            let observation = match probe.service_status(&args.service).await {
                Ok(value) => ToolObservation::success(Self::NAME, value),

                Err(error) => ToolObservation::from_probe_error(Self::NAME, error),
            };

            history.record(observation.clone()).await;

            Ok(observation)
        }
    }
}
