pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of an individual Automation workflow email.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationWorkflowEmail {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<AutomationWorkflowEmailLinksItem>>,
    /// The link to the campaign's archive version in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<String>,
    /// How the campaign's content is put together ('template', 'drag_and_drop', 'html', 'url').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The date and time the campaign was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub create_time: Option<DateTime<FixedOffset>>,
    /// The delay settings for an Automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<AutomationWorkflowEmailDelay>,
    /// The total number of emails sent for this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<i64>,
    /// Determines if the campaign contains the *|BRAND:LOGO|* merge tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_logo_merge_tag: Option<bool>,
    /// A string that uniquely identifies the Automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Determines if the automation email needs its blocks refreshed by opening the web-based campaign editor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_block_refresh: Option<bool>,
    /// The position of an Automation email in a workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    /// List settings for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<AutomationWorkflowEmailRecipients>,
    /// For sent campaigns, a summary of opens and clicks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<AutomationWorkflowEmailReportSummary>,
    /// The date and time a campaign was sent in ISO 8601 format
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub send_time: Option<DateTime<FixedOffset>>,
    /// Settings for the campaign including the email subject, from name, and from email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AutomationWorkflowEmailSettings>,
    /// The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_card: Option<AutomationWorkflowEmailSocialCard>,
    /// The date and time the campaign was started in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub start_time: Option<DateTime<FixedOffset>>,
    /// The current status of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AutomationWorkflowEmailStatus>,
    /// The tracking options for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<AutomationWorkflowEmailTracking>,
    /// Available triggers for Automation workflows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_settings: Option<AutomationWorkflowEmailTriggerSettings>,
    /// The ID used in the Mailchimp web application. View this automation in your Mailchimp account at `https://{dc}.admin.mailchimp.com/campaigns/show/?id={web_id}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<i64>,
    /// A string that uniquely identifies an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

impl AutomationWorkflowEmail {
    pub fn builder() -> AutomationWorkflowEmailBuilder {
        <AutomationWorkflowEmailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowEmailBuilder {
    links: Option<Vec<AutomationWorkflowEmailLinksItem>>,
    archive_url: Option<String>,
    content_type: Option<String>,
    create_time: Option<DateTime<FixedOffset>>,
    delay: Option<AutomationWorkflowEmailDelay>,
    emails_sent: Option<i64>,
    has_logo_merge_tag: Option<bool>,
    id: Option<String>,
    needs_block_refresh: Option<bool>,
    position: Option<i64>,
    recipients: Option<AutomationWorkflowEmailRecipients>,
    report_summary: Option<AutomationWorkflowEmailReportSummary>,
    send_time: Option<DateTime<FixedOffset>>,
    settings: Option<AutomationWorkflowEmailSettings>,
    social_card: Option<AutomationWorkflowEmailSocialCard>,
    start_time: Option<DateTime<FixedOffset>>,
    status: Option<AutomationWorkflowEmailStatus>,
    tracking: Option<AutomationWorkflowEmailTracking>,
    trigger_settings: Option<AutomationWorkflowEmailTriggerSettings>,
    web_id: Option<i64>,
    workflow_id: Option<String>,
}

impl AutomationWorkflowEmailBuilder {
    pub fn links(mut self, value: Vec<AutomationWorkflowEmailLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn archive_url(mut self, value: impl Into<String>) -> Self {
        self.archive_url = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn create_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.create_time = Some(value);
        self
    }

    pub fn delay(mut self, value: AutomationWorkflowEmailDelay) -> Self {
        self.delay = Some(value);
        self
    }

    pub fn emails_sent(mut self, value: i64) -> Self {
        self.emails_sent = Some(value);
        self
    }

    pub fn has_logo_merge_tag(mut self, value: bool) -> Self {
        self.has_logo_merge_tag = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn needs_block_refresh(mut self, value: bool) -> Self {
        self.needs_block_refresh = Some(value);
        self
    }

    pub fn position(mut self, value: i64) -> Self {
        self.position = Some(value);
        self
    }

    pub fn recipients(mut self, value: AutomationWorkflowEmailRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn report_summary(mut self, value: AutomationWorkflowEmailReportSummary) -> Self {
        self.report_summary = Some(value);
        self
    }

    pub fn send_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.send_time = Some(value);
        self
    }

    pub fn settings(mut self, value: AutomationWorkflowEmailSettings) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn social_card(mut self, value: AutomationWorkflowEmailSocialCard) -> Self {
        self.social_card = Some(value);
        self
    }

    pub fn start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn status(mut self, value: AutomationWorkflowEmailStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tracking(mut self, value: AutomationWorkflowEmailTracking) -> Self {
        self.tracking = Some(value);
        self
    }

    pub fn trigger_settings(mut self, value: AutomationWorkflowEmailTriggerSettings) -> Self {
        self.trigger_settings = Some(value);
        self
    }

    pub fn web_id(mut self, value: i64) -> Self {
        self.web_id = Some(value);
        self
    }

    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowEmail`].
    pub fn build(self) -> Result<AutomationWorkflowEmail, BuildError> {
        Ok(AutomationWorkflowEmail {
            links: self.links,
            archive_url: self.archive_url,
            content_type: self.content_type,
            create_time: self.create_time,
            delay: self.delay,
            emails_sent: self.emails_sent,
            has_logo_merge_tag: self.has_logo_merge_tag,
            id: self.id,
            needs_block_refresh: self.needs_block_refresh,
            position: self.position,
            recipients: self.recipients,
            report_summary: self.report_summary,
            send_time: self.send_time,
            settings: self.settings,
            social_card: self.social_card,
            start_time: self.start_time,
            status: self.status,
            tracking: self.tracking,
            trigger_settings: self.trigger_settings,
            web_id: self.web_id,
            workflow_id: self.workflow_id,
        })
    }
}
