pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateJourneyStepActionTriggerCustomerJourneysRequest {
    /// The list member's email address.
    #[serde(default)]
    pub email_address: String,
}

impl CreateJourneyStepActionTriggerCustomerJourneysRequest {
    pub fn builder() -> CreateJourneyStepActionTriggerCustomerJourneysRequestBuilder {
        <CreateJourneyStepActionTriggerCustomerJourneysRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateJourneyStepActionTriggerCustomerJourneysRequestBuilder {
    email_address: Option<String>,
}

impl CreateJourneyStepActionTriggerCustomerJourneysRequestBuilder {
    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateJourneyStepActionTriggerCustomerJourneysRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_address`](CreateJourneyStepActionTriggerCustomerJourneysRequestBuilder::email_address)
    pub fn build(self) -> Result<CreateJourneyStepActionTriggerCustomerJourneysRequest, BuildError> {
        Ok(CreateJourneyStepActionTriggerCustomerJourneysRequest {
            email_address: self.email_address.ok_or_else(|| BuildError::missing_field("email_address"))?,
        })
    }
}

