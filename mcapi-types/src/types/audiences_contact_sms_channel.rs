pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudiencesContactSmsChannel {
    /// A computation performed by the Mailchimp platform, triggered whenever any of its inputs change. Some inputs are controlled by API users, while others are tracked internally by the platform. Computation is based on: audience opt-in configuration (single vs. double opt-in), marketing consent status, and deliverability status (an internal state for a contact, maintained by Mailchimp for a specific marketing channel instance). This new API field is distinct from how contacts are displayed in the UI. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_subscription_status: Option<AudiencesContactSmsChannelEffectiveSubscriptionStatus>,
    /// A contact's current consent status for SMS marketing communications. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_consent: Option<AudiencesContactSmsChannelMarketingConsent>,
    /// SMS Phone Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_phone: Option<String>,
    /// The source from which the parent's entity was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AudiencesContactSmsChannelSource>,
    /// SHA256 hash of the SMS phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashed_sms_phone: Option<String>,
}

impl AudiencesContactSmsChannel {
    pub fn builder() -> AudiencesContactSmsChannelBuilder {
        <AudiencesContactSmsChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudiencesContactSmsChannelBuilder {
    effective_subscription_status: Option<AudiencesContactSmsChannelEffectiveSubscriptionStatus>,
    marketing_consent: Option<AudiencesContactSmsChannelMarketingConsent>,
    sms_phone: Option<String>,
    source: Option<AudiencesContactSmsChannelSource>,
    hashed_sms_phone: Option<String>,
}

impl AudiencesContactSmsChannelBuilder {
    pub fn effective_subscription_status(mut self, value: AudiencesContactSmsChannelEffectiveSubscriptionStatus) -> Self {
        self.effective_subscription_status = Some(value);
        self
    }

    pub fn marketing_consent(mut self, value: AudiencesContactSmsChannelMarketingConsent) -> Self {
        self.marketing_consent = Some(value);
        self
    }

    pub fn sms_phone(mut self, value: impl Into<String>) -> Self {
        self.sms_phone = Some(value.into());
        self
    }

    pub fn source(mut self, value: AudiencesContactSmsChannelSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn hashed_sms_phone(mut self, value: impl Into<String>) -> Self {
        self.hashed_sms_phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudiencesContactSmsChannel`].
    pub fn build(self) -> Result<AudiencesContactSmsChannel, BuildError> {
        Ok(AudiencesContactSmsChannel {
            effective_subscription_status: self.effective_subscription_status,
            marketing_consent: self.marketing_consent,
            sms_phone: self.sms_phone,
            source: self.source,
            hashed_sms_phone: self.hashed_sms_phone,
        })
    }
}
