pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Assign subscribers to interests to group them together. Interests are referred to as 'group names' in the Mailchimp application.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Interest {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<InterestLinksItem>>,
    /// The id for the interest category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// The display order for interests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// The ID for the interest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID for the list that this interest belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The name of the interest. This can be shown publicly on a subscription form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of subscribers associated with this interest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_count: Option<String>,
}

impl Interest {
    pub fn builder() -> InterestBuilder {
        <InterestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InterestBuilder {
    links: Option<Vec<InterestLinksItem>>,
    category_id: Option<String>,
    display_order: Option<i64>,
    id: Option<String>,
    list_id: Option<String>,
    name: Option<String>,
    subscriber_count: Option<String>,
}

impl InterestBuilder {
    pub fn links(mut self, value: Vec<InterestLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn category_id(mut self, value: impl Into<String>) -> Self {
        self.category_id = Some(value.into());
        self
    }

    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn subscriber_count(mut self, value: impl Into<String>) -> Self {
        self.subscriber_count = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Interest`].
    pub fn build(self) -> Result<Interest, BuildError> {
        Ok(Interest {
            links: self.links,
            category_id: self.category_id,
            display_order: self.display_order,
            id: self.id,
            list_id: self.list_id,
            name: self.name,
            subscriber_count: self.subscriber_count,
        })
    }
}
