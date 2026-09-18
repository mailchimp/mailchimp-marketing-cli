pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Campaign feedback details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAdviceResponseAdviceItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListAdviceResponseAdviceItemLinksItem>>,
    /// The advice message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The sentiment type for a feedback message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListAdviceResponseAdviceItemType>,
}

impl ListAdviceResponseAdviceItem {
    pub fn builder() -> ListAdviceResponseAdviceItemBuilder {
        <ListAdviceResponseAdviceItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAdviceResponseAdviceItemBuilder {
    links: Option<Vec<ListAdviceResponseAdviceItemLinksItem>>,
    message: Option<String>,
    r#type: Option<ListAdviceResponseAdviceItemType>,
}

impl ListAdviceResponseAdviceItemBuilder {
    pub fn links(mut self, value: Vec<ListAdviceResponseAdviceItemLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ListAdviceResponseAdviceItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAdviceResponseAdviceItem`].
    pub fn build(self) -> Result<ListAdviceResponseAdviceItem, BuildError> {
        Ok(ListAdviceResponseAdviceItem {
            links: self.links,
            message: self.message,
            r#type: self.r#type,
        })
    }
}
