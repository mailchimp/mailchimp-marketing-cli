pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FacebookAdsBudget {
    /// Currency code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// Duration of the ad in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Total budget of the ad
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
}

impl FacebookAdsBudget {
    pub fn builder() -> FacebookAdsBudgetBuilder {
        <FacebookAdsBudgetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdsBudgetBuilder {
    currency_code: Option<String>,
    duration: Option<i64>,
    total_amount: Option<f64>,
}

impl FacebookAdsBudgetBuilder {
    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdsBudget`].
    pub fn build(self) -> Result<FacebookAdsBudget, BuildError> {
        Ok(FacebookAdsBudget {
            currency_code: self.currency_code,
            duration: self.duration,
            total_amount: self.total_amount,
        })
    }
}
