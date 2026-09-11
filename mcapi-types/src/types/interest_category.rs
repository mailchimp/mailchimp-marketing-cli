pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Interest categories organize interests, which are used to group subscribers based on their preferences. These correspond to Group Titles the application.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InterestCategory {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<InterestCategoryLinksItem>>,
    /// The order that the categories are displayed in the list. Lower numbers display first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// The id for the interest category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The unique list id for the category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The text description of this category. This field appears on signup forms and is often phrased as a question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Determines how this category’s interests appear on signup forms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<InterestCategoryType>,
}

impl InterestCategory {
    pub fn builder() -> InterestCategoryBuilder {
        <InterestCategoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InterestCategoryBuilder {
    links: Option<Vec<InterestCategoryLinksItem>>,
    display_order: Option<i64>,
    id: Option<String>,
    list_id: Option<String>,
    title: Option<String>,
    r#type: Option<InterestCategoryType>,
}

impl InterestCategoryBuilder {
    pub fn links(mut self, value: Vec<InterestCategoryLinksItem>) -> Self {
        self.links = Some(value);
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

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: InterestCategoryType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InterestCategory`].
    pub fn build(self) -> Result<InterestCategory, BuildError> {
        Ok(InterestCategory {
            links: self.links,
            display_order: self.display_order,
            id: self.id,
            list_id: self.list_id,
            title: self.title,
            r#type: self.r#type,
        })
    }
}
