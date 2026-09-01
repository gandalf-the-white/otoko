use std::{future::Future, sync::Arc};

use rig::tool::{Tool, ToolContext};

use serde_json::{Value, json};

use crate::probes::{CommandObservation, ProbeError, ReadOnlyFreeBsdProbe};

use super::NoArgs;

pub struct RecentLoginsTool {
    probe: Arc<dyn ReadOnlyFreeBsdProbe>,
}

impl RecentLoginsTool {
    pub fn new(probe: Arc<dyn ReadOnlyFreeBsdProbe>) -> Self {
        Self { probe }
    }
}

impl Tool for RecentLoginsTool {
    const NAME: &'static str = "get_recent_logins";

    type Args = NoArgs;

    type Output = CommandObservation;

    type Error = ProbeError;

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

        async move { probe.recent_logins().await }
    }
}
