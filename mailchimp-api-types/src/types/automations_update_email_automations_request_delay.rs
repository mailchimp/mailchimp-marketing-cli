pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The delay settings for an automation email.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateEmailAutomationsRequestDelay {
    /// The action that triggers the delay of an automation emails.
    pub action: UpdateEmailAutomationsRequestDelayAction,
    /// The delay amount for an automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Whether the delay settings describe before or after the delay action of an automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<UpdateEmailAutomationsRequestDelayDirection>,
    /// The type of delay for an automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UpdateEmailAutomationsRequestDelayType>,
}

impl UpdateEmailAutomationsRequestDelay {
    pub fn builder() -> UpdateEmailAutomationsRequestDelayBuilder {
        <UpdateEmailAutomationsRequestDelayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEmailAutomationsRequestDelayBuilder {
    action: Option<UpdateEmailAutomationsRequestDelayAction>,
    amount: Option<i64>,
    direction: Option<UpdateEmailAutomationsRequestDelayDirection>,
    r#type: Option<UpdateEmailAutomationsRequestDelayType>,
}

impl UpdateEmailAutomationsRequestDelayBuilder {
    pub fn action(mut self, value: UpdateEmailAutomationsRequestDelayAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn direction(mut self, value: UpdateEmailAutomationsRequestDelayDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn r#type(mut self, value: UpdateEmailAutomationsRequestDelayType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateEmailAutomationsRequestDelay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](UpdateEmailAutomationsRequestDelayBuilder::action)
    pub fn build(self) -> Result<UpdateEmailAutomationsRequestDelay, BuildError> {
        Ok(UpdateEmailAutomationsRequestDelay {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            amount: self.amount,
            direction: self.direction,
            r#type: self.r#type,
        })
    }
}
