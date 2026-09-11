pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The events that can trigger the webhook and whether they are enabled.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWebhooksEvents {
    /// Whether the webhook is triggered when a campaign is sent or cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<bool>,
    /// Whether the webhook is triggered when a subscriber's email address is cleaned from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleaned: Option<bool>,
    /// Whether the webhook is triggered when a contact's profile is updated. This includes email subscribers and SMS-only contacts [BETA].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<bool>,
    /// Whether the webhook is triggered when a list subscriber is added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
    /// Whether the webhook is triggered when a list member unsubscribes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe: Option<bool>,
    /// Whether the webhook is triggered when a subscriber's email address is changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upemail: Option<bool>,
    /// [BETA] Whether the webhook is triggered when a contact subscribes to SMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_subscribe: Option<bool>,
    /// [BETA] Whether the webhook is triggered when a contact unsubscribes from SMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_unsubscribe: Option<bool>,
    /// [BETA] Whether the webhook is triggered when a contact's SMS phone number is updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upsms: Option<bool>,
    /// [BETA] Whether the webhook is triggered when an SMS campaign is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_campaign: Option<bool>,
}

impl ListWebhooksEvents {
    pub fn builder() -> ListWebhooksEventsBuilder {
        <ListWebhooksEventsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhooksEventsBuilder {
    campaign: Option<bool>,
    cleaned: Option<bool>,
    profile: Option<bool>,
    subscribe: Option<bool>,
    unsubscribe: Option<bool>,
    upemail: Option<bool>,
    sms_subscribe: Option<bool>,
    sms_unsubscribe: Option<bool>,
    upsms: Option<bool>,
    sms_campaign: Option<bool>,
}

impl ListWebhooksEventsBuilder {
    pub fn campaign(mut self, value: bool) -> Self {
        self.campaign = Some(value);
        self
    }

    pub fn cleaned(mut self, value: bool) -> Self {
        self.cleaned = Some(value);
        self
    }

    pub fn profile(mut self, value: bool) -> Self {
        self.profile = Some(value);
        self
    }

    pub fn subscribe(mut self, value: bool) -> Self {
        self.subscribe = Some(value);
        self
    }

    pub fn unsubscribe(mut self, value: bool) -> Self {
        self.unsubscribe = Some(value);
        self
    }

    pub fn upemail(mut self, value: bool) -> Self {
        self.upemail = Some(value);
        self
    }

    pub fn sms_subscribe(mut self, value: bool) -> Self {
        self.sms_subscribe = Some(value);
        self
    }

    pub fn sms_unsubscribe(mut self, value: bool) -> Self {
        self.sms_unsubscribe = Some(value);
        self
    }

    pub fn upsms(mut self, value: bool) -> Self {
        self.upsms = Some(value);
        self
    }

    pub fn sms_campaign(mut self, value: bool) -> Self {
        self.sms_campaign = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhooksEvents`].
    pub fn build(self) -> Result<ListWebhooksEvents, BuildError> {
        Ok(ListWebhooksEvents {
            campaign: self.campaign,
            cleaned: self.cleaned,
            profile: self.profile,
            subscribe: self.subscribe,
            unsubscribe: self.unsubscribe,
            upemail: self.upemail,
            sms_subscribe: self.sms_subscribe,
            sms_unsubscribe: self.sms_unsubscribe,
            upsms: self.upsms,
            sms_campaign: self.sms_campaign,
        })
    }
}
