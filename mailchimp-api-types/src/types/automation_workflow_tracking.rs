pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The tracking options for the Automation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowTracking {
    /// Deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capsule: Option<AutomationWorkflowTrackingCapsule>,
    /// The custom slug for [ClickTale](https://mailchimp.com/help/additional-tracking-options-for-campaigns/) tracking (max of 50 bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicktale: Option<String>,
    /// Whether to enable e-commerce tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecomm360: Option<bool>,
    /// Deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_tracking: Option<bool>,
    /// The custom slug for [Google Analytics](https://mailchimp.com/help/integrate-google-analytics-with-mailchimp/) tracking (max of 50 bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_analytics: Option<String>,
    /// Whether to [track clicks](https://mailchimp.com/help/enable-and-view-click-tracking/) in the HTML version of the Automation. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_clicks: Option<bool>,
    /// Whether to [track opens](https://mailchimp.com/help/about-open-tracking/). Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<bool>,
    /// Deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<AutomationWorkflowTrackingSalesforce>,
    /// Whether to [track clicks](https://mailchimp.com/help/enable-and-view-click-tracking/) in the plain-text version of the Automation. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_clicks: Option<bool>,
}

impl AutomationWorkflowTracking {
    pub fn builder() -> AutomationWorkflowTrackingBuilder {
        <AutomationWorkflowTrackingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowTrackingBuilder {
    capsule: Option<AutomationWorkflowTrackingCapsule>,
    clicktale: Option<String>,
    ecomm360: Option<bool>,
    goal_tracking: Option<bool>,
    google_analytics: Option<String>,
    html_clicks: Option<bool>,
    opens: Option<bool>,
    salesforce: Option<AutomationWorkflowTrackingSalesforce>,
    text_clicks: Option<bool>,
}

impl AutomationWorkflowTrackingBuilder {
    pub fn capsule(mut self, value: AutomationWorkflowTrackingCapsule) -> Self {
        self.capsule = Some(value);
        self
    }

    pub fn clicktale(mut self, value: impl Into<String>) -> Self {
        self.clicktale = Some(value.into());
        self
    }

    pub fn ecomm360(mut self, value: bool) -> Self {
        self.ecomm360 = Some(value);
        self
    }

    pub fn goal_tracking(mut self, value: bool) -> Self {
        self.goal_tracking = Some(value);
        self
    }

    pub fn google_analytics(mut self, value: impl Into<String>) -> Self {
        self.google_analytics = Some(value.into());
        self
    }

    pub fn html_clicks(mut self, value: bool) -> Self {
        self.html_clicks = Some(value);
        self
    }

    pub fn opens(mut self, value: bool) -> Self {
        self.opens = Some(value);
        self
    }

    pub fn salesforce(mut self, value: AutomationWorkflowTrackingSalesforce) -> Self {
        self.salesforce = Some(value);
        self
    }

    pub fn text_clicks(mut self, value: bool) -> Self {
        self.text_clicks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowTracking`].
    pub fn build(self) -> Result<AutomationWorkflowTracking, BuildError> {
        Ok(AutomationWorkflowTracking {
            capsule: self.capsule,
            clicktale: self.clicktale,
            ecomm360: self.ecomm360,
            goal_tracking: self.goal_tracking,
            google_analytics: self.google_analytics,
            html_clicks: self.html_clicks,
            opens: self.opens,
            salesforce: self.salesforce,
            text_clicks: self.text_clicks,
        })
    }
}
