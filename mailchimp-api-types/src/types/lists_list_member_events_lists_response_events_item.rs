pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A specific event for a contact.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMemberEventsListsResponseEventsItem {
    /// The name for this type of event ('purchased', 'visited', etc). Must be 2-30 characters in length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The date and time the event occurred in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub occurred_at: Option<DateTime<FixedOffset>>,
    /// An optional list of properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,
}

impl ListMemberEventsListsResponseEventsItem {
    pub fn builder() -> ListMemberEventsListsResponseEventsItemBuilder {
        <ListMemberEventsListsResponseEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberEventsListsResponseEventsItemBuilder {
    name: Option<String>,
    occurred_at: Option<DateTime<FixedOffset>>,
    properties: Option<HashMap<String, String>>,
}

impl ListMemberEventsListsResponseEventsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn properties(mut self, value: HashMap<String, String>) -> Self {
        self.properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMemberEventsListsResponseEventsItem`].
    pub fn build(self) -> Result<ListMemberEventsListsResponseEventsItem, BuildError> {
        Ok(ListMemberEventsListsResponseEventsItem {
            name: self.name,
            occurred_at: self.occurred_at,
            properties: self.properties,
        })
    }
}
