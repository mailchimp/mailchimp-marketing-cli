pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchAudienceContactRequestSmsChannel {
    /// A contact's current consent status for SMS marketing communications. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_consent: Option<PatchAudienceContactRequestSmsChannelMarketingConsent>,
    /// SMS Phone Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_phone: Option<String>,
}

impl PatchAudienceContactRequestSmsChannel {
    pub fn builder() -> PatchAudienceContactRequestSmsChannelBuilder {
        <PatchAudienceContactRequestSmsChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchAudienceContactRequestSmsChannelBuilder {
    marketing_consent: Option<PatchAudienceContactRequestSmsChannelMarketingConsent>,
    sms_phone: Option<String>,
}

impl PatchAudienceContactRequestSmsChannelBuilder {
    pub fn marketing_consent(mut self, value: PatchAudienceContactRequestSmsChannelMarketingConsent) -> Self {
        self.marketing_consent = Some(value);
        self
    }

    pub fn sms_phone(mut self, value: impl Into<String>) -> Self {
        self.sms_phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PatchAudienceContactRequestSmsChannel`].
    pub fn build(self) -> Result<PatchAudienceContactRequestSmsChannel, BuildError> {
        Ok(PatchAudienceContactRequestSmsChannel {
            marketing_consent: self.marketing_consent,
            sms_phone: self.sms_phone,
        })
    }
}
