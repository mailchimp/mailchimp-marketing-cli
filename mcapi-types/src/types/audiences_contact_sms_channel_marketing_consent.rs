pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A contact's current consent status for SMS marketing communications. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudiencesContactSmsChannelMarketingConsent {
    /// The source from which the parent's entity was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AudiencesContactSmsChannelMarketingConsentSource>,
    /// The contact's SMS marketing consent status. Use `confirmed` for double opt-in audiences, `consented` for single opt-in audiences. `denied` is accepted on PATCH/PUT only (not POST) and drives an API-initiated unsubscribe; it cannot be used when creating a new contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AudiencesContactSmsChannelMarketingConsentStatus>,
    /// The timestamp when SMS marketing consent was captured (ISO 8601). Only accepted and returned when status is `confirmed`. The timestamp of the consent state change being recorded. Defaults to the current time if not provided. If the contact already has a consent timestamp on record that is equal to or newer than the supplied value, the supplied value is ignored (staleness guard); to update the consent timestamp supply a value strictly newer than the stored one.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub captured_at: Option<DateTime<FixedOffset>>,
}

impl AudiencesContactSmsChannelMarketingConsent {
    pub fn builder() -> AudiencesContactSmsChannelMarketingConsentBuilder {
        <AudiencesContactSmsChannelMarketingConsentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudiencesContactSmsChannelMarketingConsentBuilder {
    source: Option<AudiencesContactSmsChannelMarketingConsentSource>,
    status: Option<AudiencesContactSmsChannelMarketingConsentStatus>,
    captured_at: Option<DateTime<FixedOffset>>,
}

impl AudiencesContactSmsChannelMarketingConsentBuilder {
    pub fn source(mut self, value: AudiencesContactSmsChannelMarketingConsentSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: AudiencesContactSmsChannelMarketingConsentStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn captured_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.captured_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudiencesContactSmsChannelMarketingConsent`].
    pub fn build(self) -> Result<AudiencesContactSmsChannelMarketingConsent, BuildError> {
        Ok(AudiencesContactSmsChannelMarketingConsent {
            source: self.source,
            status: self.status,
            captured_at: self.captured_at,
        })
    }
}
