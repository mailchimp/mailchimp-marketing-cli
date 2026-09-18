pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The delay settings for an automation email.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateEmailRequestDelay {
    /// The action that triggers the delay of an automation emails.
    pub action: UpdateEmailRequestDelayAction,
    /// The delay amount for an automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Whether the delay settings describe before or after the delay action of an automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<UpdateEmailRequestDelayDirection>,
    /// The type of delay for an automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UpdateEmailRequestDelayType>,
}

impl UpdateEmailRequestDelay {
    pub fn builder() -> UpdateEmailRequestDelayBuilder {
        <UpdateEmailRequestDelayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEmailRequestDelayBuilder {
    action: Option<UpdateEmailRequestDelayAction>,
    amount: Option<i64>,
    direction: Option<UpdateEmailRequestDelayDirection>,
    r#type: Option<UpdateEmailRequestDelayType>,
}

impl UpdateEmailRequestDelayBuilder {
    pub fn action(mut self, value: UpdateEmailRequestDelayAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn direction(mut self, value: UpdateEmailRequestDelayDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn r#type(mut self, value: UpdateEmailRequestDelayType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateEmailRequestDelay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](UpdateEmailRequestDelayBuilder::action)
    pub fn build(self) -> Result<UpdateEmailRequestDelay, BuildError> {
        Ok(UpdateEmailRequestDelay {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            amount: self.amount,
            direction: self.direction,
            r#type: self.r#type,
        })
    }
}
