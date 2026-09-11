pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of an individual Automation workflow's settings and content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationWorkflow {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<AutomationWorkflowLinksItem>>,
    /// The date and time the Automation was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub create_time: Option<DateTime<FixedOffset>>,
    /// The total number of emails sent for the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<i64>,
    /// A string that identifies the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// List settings for the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<AutomationWorkflowRecipients>,
    /// A summary of opens and clicks for sent campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<AutomationWorkflowReportSummary>,
    /// The settings for the Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AutomationWorkflowSettings>,
    /// The date and time the Automation was started in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub start_time: Option<DateTime<FixedOffset>>,
    /// The current status of the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AutomationWorkflowStatus>,
    /// The tracking options for the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<AutomationWorkflowTracking>,
    /// Available triggers for Automation workflows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_settings: Option<AutomationWorkflowTriggerSettings>,
}

impl AutomationWorkflow {
    pub fn builder() -> AutomationWorkflowBuilder {
        <AutomationWorkflowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowBuilder {
    links: Option<Vec<AutomationWorkflowLinksItem>>,
    create_time: Option<DateTime<FixedOffset>>,
    emails_sent: Option<i64>,
    id: Option<String>,
    recipients: Option<AutomationWorkflowRecipients>,
    report_summary: Option<AutomationWorkflowReportSummary>,
    settings: Option<AutomationWorkflowSettings>,
    start_time: Option<DateTime<FixedOffset>>,
    status: Option<AutomationWorkflowStatus>,
    tracking: Option<AutomationWorkflowTracking>,
    trigger_settings: Option<AutomationWorkflowTriggerSettings>,
}

impl AutomationWorkflowBuilder {
    pub fn links(mut self, value: Vec<AutomationWorkflowLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn create_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.create_time = Some(value);
        self
    }

    pub fn emails_sent(mut self, value: i64) -> Self {
        self.emails_sent = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: AutomationWorkflowRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn report_summary(mut self, value: AutomationWorkflowReportSummary) -> Self {
        self.report_summary = Some(value);
        self
    }

    pub fn settings(mut self, value: AutomationWorkflowSettings) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn status(mut self, value: AutomationWorkflowStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tracking(mut self, value: AutomationWorkflowTracking) -> Self {
        self.tracking = Some(value);
        self
    }

    pub fn trigger_settings(mut self, value: AutomationWorkflowTriggerSettings) -> Self {
        self.trigger_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflow`].
    pub fn build(self) -> Result<AutomationWorkflow, BuildError> {
        Ok(AutomationWorkflow {
            links: self.links,
            create_time: self.create_time,
            emails_sent: self.emails_sent,
            id: self.id,
            recipients: self.recipients,
            report_summary: self.report_summary,
            settings: self.settings,
            start_time: self.start_time,
            status: self.status,
            tracking: self.tracking,
            trigger_settings: self.trigger_settings,
        })
    }
}
