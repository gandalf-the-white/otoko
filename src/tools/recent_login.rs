use std::{convert::Infallible, future::Future, sync::Arc};

use rig::tool::{Tool, ToolContext};

use serde_json::{Value, json};

use crate::{
    domain::SystemObservation,
    probes::ReadOnlyFreeBsdProbe,
    tools::{ObservationHistory, result::to_system_observation},
};

use super::NoArgs;

pub struct RecentLoginsTool {
    probe: Arc<dyn ReadOnlyFreeBsdProbe>,
    history: ObservationHistory,
}

impl RecentLoginsTool {
    pub fn new(probe: Arc<dyn ReadOnlyFreeBsdProbe>, history: ObservationHistory) -> Self {
        Self { probe, history }
    }
}

impl Tool for RecentLoginsTool {
    const NAME: &'static str = "get_recent_logins";

    type Args = NoArgs;

    type Output = SystemObservation;

    type Error = Infallible;

    fn description(&self) -> String {
        concat!(
            "Read recent login history on the FreeBSD host. ",
            "This tool does not modify authentication state."
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
            let observation = to_system_observation(Self::NAME, probe.recent_logins().await);

            history.record(observation.clone()).await;

            Ok(observation)
        }
    }
}
