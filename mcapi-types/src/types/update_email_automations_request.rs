pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEmailAutomationsRequest {
    /// The delay settings for an automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<UpdateEmailAutomationsRequestDelay>,
    /// Settings for the campaign including the email subject, from name, and from email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateEmailAutomationsRequestSettings>,
}

impl UpdateEmailAutomationsRequest {
    pub fn builder() -> UpdateEmailAutomationsRequestBuilder {
        <UpdateEmailAutomationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEmailAutomationsRequestBuilder {
    delay: Option<UpdateEmailAutomationsRequestDelay>,
    settings: Option<UpdateEmailAutomationsRequestSettings>,
}

impl UpdateEmailAutomationsRequestBuilder {
    pub fn delay(mut self, value: UpdateEmailAutomationsRequestDelay) -> Self {
        self.delay = Some(value);
        self
    }

    pub fn settings(mut self, value: UpdateEmailAutomationsRequestSettings) -> Self {
        self.settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateEmailAutomationsRequest`].
    pub fn build(self) -> Result<UpdateEmailAutomationsRequest, BuildError> {
        Ok(UpdateEmailAutomationsRequest {
            delay: self.delay,
            settings: self.settings,
        })
    }
}

