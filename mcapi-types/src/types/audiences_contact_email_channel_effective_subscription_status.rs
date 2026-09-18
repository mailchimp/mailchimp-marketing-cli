pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A computation performed by the Mailchimp platform, triggered whenever any of its inputs change. Some inputs are controlled by API users, while others are tracked internally by the platform. Computation is based on: audience opt-in configuration (single vs. double opt-in), marketing consent status, and deliverability status (an internal state for a contact, maintained by Mailchimp for a specific marketing channel instance). This new API field is distinct from how contacts are displayed in the UI. See the [Audiences (BETA) documentation](https://mailchimp.com/developer/marketing/docs/audiences-introduction) to learn about supported values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudiencesContactEmailChannelEffectiveSubscriptionStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AudiencesContactEmailChannelEffectiveSubscriptionStatusValue>,
}

impl AudiencesContactEmailChannelEffectiveSubscriptionStatus {
    pub fn builder() -> AudiencesContactEmailChannelEffectiveSubscriptionStatusBuilder {
        <AudiencesContactEmailChannelEffectiveSubscriptionStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudiencesContactEmailChannelEffectiveSubscriptionStatusBuilder {
    value: Option<AudiencesContactEmailChannelEffectiveSubscriptionStatusValue>,
}

impl AudiencesContactEmailChannelEffectiveSubscriptionStatusBuilder {
    pub fn value(mut self, value: AudiencesContactEmailChannelEffectiveSubscriptionStatusValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudiencesContactEmailChannelEffectiveSubscriptionStatus`].
    pub fn build(self) -> Result<AudiencesContactEmailChannelEffectiveSubscriptionStatus, BuildError> {
        Ok(AudiencesContactEmailChannelEffectiveSubscriptionStatus {
            value: self.value,
        })
    }
}
