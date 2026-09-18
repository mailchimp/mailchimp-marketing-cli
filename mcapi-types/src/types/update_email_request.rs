pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEmailRequest {
    /// The delay settings for an automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<UpdateEmailRequestDelay>,
    /// Settings for the campaign including the email subject, from name, and from email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateEmailRequestSettings>,
}

impl UpdateEmailRequest {
    pub fn builder() -> UpdateEmailRequestBuilder {
        <UpdateEmailRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEmailRequestBuilder {
    delay: Option<UpdateEmailRequestDelay>,
    settings: Option<UpdateEmailRequestSettings>,
}

impl UpdateEmailRequestBuilder {
    pub fn delay(mut self, value: UpdateEmailRequestDelay) -> Self {
        self.delay = Some(value);
        self
    }

    pub fn settings(mut self, value: UpdateEmailRequestSettings) -> Self {
        self.settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateEmailRequest`].
    pub fn build(self) -> Result<UpdateEmailRequest, BuildError> {
        Ok(UpdateEmailRequest {
            delay: self.delay,
            settings: self.settings,
        })
    }
}

