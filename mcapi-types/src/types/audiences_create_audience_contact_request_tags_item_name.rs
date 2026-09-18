pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAudienceContactRequestTagsItemName {
    #[serde(default)]
    pub name: String,
    pub status: CreateAudienceContactRequestTagsItemNameStatus,
}

impl CreateAudienceContactRequestTagsItemName {
    pub fn builder() -> CreateAudienceContactRequestTagsItemNameBuilder {
        <CreateAudienceContactRequestTagsItemNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudienceContactRequestTagsItemNameBuilder {
    name: Option<String>,
    status: Option<CreateAudienceContactRequestTagsItemNameStatus>,
}

impl CreateAudienceContactRequestTagsItemNameBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreateAudienceContactRequestTagsItemNameStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAudienceContactRequestTagsItemName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateAudienceContactRequestTagsItemNameBuilder::name)
    /// - [`status`](CreateAudienceContactRequestTagsItemNameBuilder::status)
    pub fn build(self) -> Result<CreateAudienceContactRequestTagsItemName, BuildError> {
        Ok(CreateAudienceContactRequestTagsItemName {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
