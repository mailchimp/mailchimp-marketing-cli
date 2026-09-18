pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The outreach associated with this order. For example, an email campaign or Facebook ad.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateStoreOrderRequestOutreach {
    /// A unique identifier for the outreach. Can be an email campaign ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateStoreOrderRequestOutreach {
    pub fn builder() -> CreateStoreOrderRequestOutreachBuilder {
        <CreateStoreOrderRequestOutreachBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreOrderRequestOutreachBuilder {
    id: Option<String>,
}

impl CreateStoreOrderRequestOutreachBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateStoreOrderRequestOutreach`].
    pub fn build(self) -> Result<CreateStoreOrderRequestOutreach, BuildError> {
        Ok(CreateStoreOrderRequestOutreach {
            id: self.id,
        })
    }
}
