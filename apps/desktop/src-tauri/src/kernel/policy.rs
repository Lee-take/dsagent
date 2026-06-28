use serde::{Deserialize, Serialize};

use crate::kernel::models::AccessMode;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityKind {
    FileRead,
    FileWrite,
    NetworkSearch,
    BrowserBrowse,
    BrowserSubmit,
    EmailRead,
    EmailDraft,
    EmailSend,
    DriveRead,
    DriveWrite,
    TerminalRead,
    TerminalWrite,
    ComputerScreenshot,
    ComputerControl,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecision {
    Allow,
    Ask,
    Deny,
}

pub fn capability_risk(capability: CapabilityKind) -> RiskLevel {
    match capability {
        CapabilityKind::FileRead
        | CapabilityKind::NetworkSearch
        | CapabilityKind::EmailDraft
        | CapabilityKind::DriveRead
        | CapabilityKind::TerminalRead
        | CapabilityKind::ComputerScreenshot => RiskLevel::Low,
        CapabilityKind::BrowserBrowse | CapabilityKind::EmailRead => RiskLevel::Medium,
        CapabilityKind::FileWrite
        | CapabilityKind::BrowserSubmit
        | CapabilityKind::DriveWrite
        | CapabilityKind::TerminalWrite => RiskLevel::High,
        CapabilityKind::EmailSend | CapabilityKind::ComputerControl => RiskLevel::Critical,
    }
}

pub fn decide(access_mode: AccessMode, capability: CapabilityKind) -> PolicyDecision {
    match access_mode {
        AccessMode::AskEveryStep => PolicyDecision::Ask,
        AccessMode::AskOnRisk => match capability_risk(capability) {
            RiskLevel::Low => PolicyDecision::Allow,
            RiskLevel::Medium | RiskLevel::High | RiskLevel::Critical => PolicyDecision::Ask,
        },
        AccessMode::LimitedAuto => match capability_risk(capability) {
            RiskLevel::Low | RiskLevel::Medium => PolicyDecision::Allow,
            RiskLevel::High | RiskLevel::Critical => PolicyDecision::Ask,
        },
        AccessMode::FullAccess => match capability {
            CapabilityKind::EmailSend | CapabilityKind::ComputerControl => PolicyDecision::Ask,
            _ => PolicyDecision::Allow,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{decide, CapabilityKind, PolicyDecision};
    use crate::kernel::models::AccessMode;

    #[test]
    fn ask_every_step_always_asks() {
        assert_eq!(
            decide(AccessMode::AskEveryStep, CapabilityKind::FileRead),
            PolicyDecision::Ask
        );
        assert_eq!(
            decide(AccessMode::AskEveryStep, CapabilityKind::EmailSend),
            PolicyDecision::Ask
        );
    }

    #[test]
    fn ask_on_risk_allows_low_risk_only() {
        assert_eq!(
            decide(AccessMode::AskOnRisk, CapabilityKind::FileRead),
            PolicyDecision::Allow
        );
        assert_eq!(
            decide(AccessMode::AskOnRisk, CapabilityKind::BrowserBrowse),
            PolicyDecision::Ask
        );
        assert_eq!(
            decide(AccessMode::AskOnRisk, CapabilityKind::FileWrite),
            PolicyDecision::Ask
        );
        assert_eq!(
            decide(AccessMode::AskOnRisk, CapabilityKind::EmailSend),
            PolicyDecision::Ask
        );
    }

    #[test]
    fn limited_auto_allows_medium_but_asks_high_and_critical() {
        assert_eq!(
            decide(AccessMode::LimitedAuto, CapabilityKind::BrowserBrowse),
            PolicyDecision::Allow
        );
        assert_eq!(
            decide(AccessMode::LimitedAuto, CapabilityKind::FileWrite),
            PolicyDecision::Ask
        );
        assert_eq!(
            decide(AccessMode::LimitedAuto, CapabilityKind::EmailSend),
            PolicyDecision::Ask
        );
    }

    #[test]
    fn full_access_still_asks_for_email_send_and_computer_control() {
        assert_eq!(
            decide(AccessMode::FullAccess, CapabilityKind::FileWrite),
            PolicyDecision::Allow
        );
        assert_eq!(
            decide(AccessMode::FullAccess, CapabilityKind::EmailSend),
            PolicyDecision::Ask
        );
        assert_eq!(
            decide(AccessMode::FullAccess, CapabilityKind::ComputerControl),
            PolicyDecision::Ask
        );
    }
}
