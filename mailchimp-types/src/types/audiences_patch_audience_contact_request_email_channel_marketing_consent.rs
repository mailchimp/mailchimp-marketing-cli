pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A contact's current consent status for email marketing communications. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchAudienceContactRequestEmailChannelMarketingConsent {
    /// The source from which the parent's entity was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PatchAudienceContactRequestEmailChannelMarketingConsentSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PatchAudienceContactRequestEmailChannelMarketingConsentStatus>,
    /// The ISO 8601 timestamp when the email marketing consent state was recorded; accepted and returned only when status is `confirmed` or `consented`; defaults to the current time if omitted; ignored if older than an existing stored timestamp (staleness guard).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub captured_at: Option<DateTime<FixedOffset>>,
}

impl PatchAudienceContactRequestEmailChannelMarketingConsent {
    pub fn builder() -> PatchAudienceContactRequestEmailChannelMarketingConsentBuilder {
        <PatchAudienceContactRequestEmailChannelMarketingConsentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchAudienceContactRequestEmailChannelMarketingConsentBuilder {
    source: Option<PatchAudienceContactRequestEmailChannelMarketingConsentSource>,
    status: Option<PatchAudienceContactRequestEmailChannelMarketingConsentStatus>,
    captured_at: Option<DateTime<FixedOffset>>,
}

impl PatchAudienceContactRequestEmailChannelMarketingConsentBuilder {
    pub fn source(mut self, value: PatchAudienceContactRequestEmailChannelMarketingConsentSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: PatchAudienceContactRequestEmailChannelMarketingConsentStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatchAudienceContactRequestEmailChannelMarketingConsent`].
    pub fn build(self) -> Result<PatchAudienceContactRequestEmailChannelMarketingConsent, BuildError> {
        Ok(PatchAudienceContactRequestEmailChannelMarketingConsent {
            source: self.source,
            status: self.status,
            captured_at: self.captured_at,
        })
    }
}
