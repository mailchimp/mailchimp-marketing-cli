pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAudienceContactRequestEmailChannel {
    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// A contact's current consent status for email marketing communications. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_consent: Option<CreateAudienceContactRequestEmailChannelMarketingConsent>,
}

impl CreateAudienceContactRequestEmailChannel {
    pub fn builder() -> CreateAudienceContactRequestEmailChannelBuilder {
        <CreateAudienceContactRequestEmailChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudienceContactRequestEmailChannelBuilder {
    email: Option<String>,
    marketing_consent: Option<CreateAudienceContactRequestEmailChannelMarketingConsent>,
}

impl CreateAudienceContactRequestEmailChannelBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn marketing_consent(mut self, value: CreateAudienceContactRequestEmailChannelMarketingConsent) -> Self {
        self.marketing_consent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAudienceContactRequestEmailChannel`].
    pub fn build(self) -> Result<CreateAudienceContactRequestEmailChannel, BuildError> {
        Ok(CreateAudienceContactRequestEmailChannel {
            email: self.email,
            marketing_consent: self.marketing_consent,
        })
    }
}
