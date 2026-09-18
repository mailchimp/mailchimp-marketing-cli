pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An authorized app.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListResponseAppsItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListResponseAppsItemLinksItem>>,
    /// A short description of the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID for the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An array of usernames for users who have linked the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
}

impl ListResponseAppsItem {
    pub fn builder() -> ListResponseAppsItemBuilder {
        <ListResponseAppsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseAppsItemBuilder {
    links: Option<Vec<ListResponseAppsItemLinksItem>>,
    description: Option<String>,
    id: Option<i64>,
    name: Option<String>,
    users: Option<Vec<String>>,
}

impl ListResponseAppsItemBuilder {
    pub fn links(mut self, value: Vec<ListResponseAppsItemLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn users(mut self, value: Vec<String>) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListResponseAppsItem`].
    pub fn build(self) -> Result<ListResponseAppsItem, BuildError> {
        Ok(ListResponseAppsItem {
            links: self.links,
            description: self.description,
            id: self.id,
            name: self.name,
            users: self.users,
        })
    }
}
