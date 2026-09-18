pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A contact's current consent status for SMS marketing communications. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAudienceContactRequestSmsChannelMarketingConsent {
    /// The source from which the parent's entity was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CreateAudienceContactRequestSmsChannelMarketingConsentSource>,
    /// The contact's SMS marketing consent status. Use `confirmed` for double opt-in audiences, `consented` for single opt-in audiences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateAudienceContactRequestSmsChannelMarketingConsentStatus>,
    /// The timestamp when SMS marketing consent was captured (ISO 8601). Only accepted and returned when status is `confirmed`. The timestamp of the consent state change being recorded. Defaults to the current time if not provided. If the contact already has a consent timestamp on record that is equal to or newer than the supplied value, the supplied value is ignored (staleness guard); to update the consent timestamp supply a value strictly newer than the stored one.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub captured_at: Option<DateTime<FixedOffset>>,
}

impl CreateAudienceContactRequestSmsChannelMarketingConsent {
    pub fn builder() -> CreateAudienceContactRequestSmsChannelMarketingConsentBuilder {
        <CreateAudienceContactRequestSmsChannelMarketingConsentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudienceContactRequestSmsChannelMarketingConsentBuilder {
    source: Option<CreateAudienceContactRequestSmsChannelMarketingConsentSource>,
    status: Option<CreateAudienceContactRequestSmsChannelMarketingConsentStatus>,
    captured_at: Option<DateTime<FixedOffset>>,
}

impl CreateAudienceContactRequestSmsChannelMarketingConsentBuilder {
    pub fn source(mut self, value: CreateAudienceContactRequestSmsChannelMarketingConsentSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: CreateAudienceContactRequestSmsChannelMarketingConsentStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAudienceContactRequestSmsChannelMarketingConsent`].
    pub fn build(self) -> Result<CreateAudienceContactRequestSmsChannelMarketingConsent, BuildError> {
        Ok(CreateAudienceContactRequestSmsChannelMarketingConsent {
            source: self.source,
            status: self.status,
            captured_at: self.captured_at,
        })
    }
}
