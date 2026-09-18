pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSendChecklistResponseItemsItem {
    /// Details about the specific feedback item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// The heading for the specific item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<String>,
    /// The ID for the specific item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The item type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListSendChecklistResponseItemsItemType>,
}

impl ListSendChecklistResponseItemsItem {
    pub fn builder() -> ListSendChecklistResponseItemsItemBuilder {
        <ListSendChecklistResponseItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSendChecklistResponseItemsItemBuilder {
    details: Option<String>,
    heading: Option<String>,
    id: Option<i64>,
    r#type: Option<ListSendChecklistResponseItemsItemType>,
}

impl ListSendChecklistResponseItemsItemBuilder {
    pub fn details(mut self, value: impl Into<String>) -> Self {
        self.details = Some(value.into());
        self
    }

    pub fn heading(mut self, value: impl Into<String>) -> Self {
        self.heading = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn r#type(mut self, value: ListSendChecklistResponseItemsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSendChecklistResponseItemsItem`].
    pub fn build(self) -> Result<ListSendChecklistResponseItemsItem, BuildError> {
        Ok(ListSendChecklistResponseItemsItem {
            details: self.details,
            heading: self.heading,
            id: self.id,
            r#type: self.r#type,
        })
    }
}
