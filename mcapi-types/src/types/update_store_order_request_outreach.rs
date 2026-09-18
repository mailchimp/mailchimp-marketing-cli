pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The outreach associated with this order. For example, an email campaign or Facebook ad.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateStoreOrderRequestOutreach {
    /// A unique identifier for the outreach. Can be an email campaign ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateStoreOrderRequestOutreach {
    pub fn builder() -> UpdateStoreOrderRequestOutreachBuilder {
        <UpdateStoreOrderRequestOutreachBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreOrderRequestOutreachBuilder {
    id: Option<String>,
}

impl UpdateStoreOrderRequestOutreachBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoreOrderRequestOutreach`].
    pub fn build(self) -> Result<UpdateStoreOrderRequestOutreach, BuildError> {
        Ok(UpdateStoreOrderRequestOutreach {
            id: self.id,
        })
    }
}
