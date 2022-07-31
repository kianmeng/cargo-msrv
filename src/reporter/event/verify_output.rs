use crate::reporter::event::Message;
use crate::toolchain::OwnedToolchainSpec;
use crate::Event;

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyOutput {
    pub toolchain: OwnedToolchainSpec,
    /// True if compatible, false if incompatible
    decision: bool,
    pub compatibility_report: CompatibilityReport,
}

impl VerifyOutput {
    pub fn compatible(toolchain: impl Into<OwnedToolchainSpec>) -> Self {
        Self {
            toolchain: toolchain.into(),
            decision: true,
            compatibility_report: CompatibilityReport::Compatible,
        }
    }

    pub fn incompatible(toolchain: impl Into<OwnedToolchainSpec>, error: Option<String>) -> Self {
        Self {
            toolchain: toolchain.into(),
            decision: false,
            compatibility_report: CompatibilityReport::Incompatible {
                error: error.map(Into::into),
            },
        }
    }

    pub fn toolchain(&self) -> &OwnedToolchainSpec {
        &self.toolchain
    }

    pub fn is_compatible(&self) -> bool {
        self.decision
    }
}

impl From<VerifyOutput> for Event {
    fn from(it: VerifyOutput) -> Self {
        Message::Verify(it).into()
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityReport {
    Compatible,
    Incompatible { error: Option<String> },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporter::event::Message;
    use crate::reporter::TestReporter;
    use crate::{semver, Event};
    use storyteller::Reporter;

    #[test]
    fn reported_compatible_toolchain() {
        let reporter = TestReporter::default();
        let event = VerifyOutput::compatible(OwnedToolchainSpec::new(
            &semver::Version::new(1, 2, 3),
            "test_target",
        ));

        reporter.reporter().report_event(event.clone()).unwrap();

        assert_eq!(
            reporter.wait_for_events(),
            vec![Event::new(Message::Verify(event)),]
        );
    }

    #[yare::parameterized(
        none = { None },
        some = { Some("whoo!".to_string()) },
    )]
    fn reported_incompatible_toolchain(error_message: Option<String>) {
        let reporter = TestReporter::default();
        let event = VerifyOutput::incompatible(
            OwnedToolchainSpec::new(&semver::Version::new(1, 2, 3), "test_target"),
            error_message,
        );

        reporter.reporter().report_event(event.clone()).unwrap();

        assert_eq!(
            reporter.wait_for_events(),
            vec![Event::new(Message::Verify(event)),]
        );
    }
}
