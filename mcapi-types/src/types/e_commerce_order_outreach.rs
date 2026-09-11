pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The outreach associated with this order. For example, an email campaign or Facebook ad.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ECommerceOrderOutreach {
    /// A unique identifier for the outreach. Can be an email campaign ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name for the outreach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The date and time the Outreach was published in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub published_time: Option<DateTime<FixedOffset>>,
    /// The type of the outreach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ECommerceOrderOutreach {
    pub fn builder() -> ECommerceOrderOutreachBuilder {
        <ECommerceOrderOutreachBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceOrderOutreachBuilder {
    id: Option<String>,
    name: Option<String>,
    published_time: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl ECommerceOrderOutreachBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn published_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.published_time = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ECommerceOrderOutreach`].
    pub fn build(self) -> Result<ECommerceOrderOutreach, BuildError> {
        Ok(ECommerceOrderOutreach {
            id: self.id,
            name: self.name,
            published_time: self.published_time,
            r#type: self.r#type,
        })
    }
}
