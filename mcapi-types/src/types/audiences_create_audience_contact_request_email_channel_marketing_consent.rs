pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A contact's current consent status for email marketing communications. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAudienceContactRequestEmailChannelMarketingConsent {
    /// Status of a contacts Marketing Consent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateAudienceContactRequestEmailChannelMarketingConsentStatus>,
}

impl CreateAudienceContactRequestEmailChannelMarketingConsent {
    pub fn builder() -> CreateAudienceContactRequestEmailChannelMarketingConsentBuilder {
        <CreateAudienceContactRequestEmailChannelMarketingConsentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudienceContactRequestEmailChannelMarketingConsentBuilder {
    status: Option<CreateAudienceContactRequestEmailChannelMarketingConsentStatus>,
}

impl CreateAudienceContactRequestEmailChannelMarketingConsentBuilder {
    pub fn status(mut self, value: CreateAudienceContactRequestEmailChannelMarketingConsentStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAudienceContactRequestEmailChannelMarketingConsent`].
    pub fn build(self) -> Result<CreateAudienceContactRequestEmailChannelMarketingConsent, BuildError> {
        Ok(CreateAudienceContactRequestEmailChannelMarketingConsent {
            status: self.status,
        })
    }
}
