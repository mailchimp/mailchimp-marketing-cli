pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Extra options for some merge field types.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMergeFieldListsRequestOptions {
    /// In a radio or dropdown non-group field, the available options for contacts to pick from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<String>>,
    /// In a date or birthday field, the format of the date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    /// In an address field, the default country code if none supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_country: Option<i64>,
    /// In a phone field, the phone number type: US or International.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_format: Option<String>,
    /// In a text field, the default length of the text field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl CreateMergeFieldListsRequestOptions {
    pub fn builder() -> CreateMergeFieldListsRequestOptionsBuilder {
        <CreateMergeFieldListsRequestOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMergeFieldListsRequestOptionsBuilder {
    choices: Option<Vec<String>>,
    date_format: Option<String>,
    default_country: Option<i64>,
    phone_format: Option<String>,
    size: Option<i64>,
}

impl CreateMergeFieldListsRequestOptionsBuilder {
    pub fn choices(mut self, value: Vec<String>) -> Self {
        self.choices = Some(value);
        self
    }

    pub fn date_format(mut self, value: impl Into<String>) -> Self {
        self.date_format = Some(value.into());
        self
    }

    pub fn default_country(mut self, value: i64) -> Self {
        self.default_country = Some(value);
        self
    }

    pub fn phone_format(mut self, value: impl Into<String>) -> Self {
        self.phone_format = Some(value.into());
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMergeFieldListsRequestOptions`].
    pub fn build(self) -> Result<CreateMergeFieldListsRequestOptions, BuildError> {
        Ok(CreateMergeFieldListsRequestOptions {
            choices: self.choices,
            date_format: self.date_format,
            default_country: self.default_country,
            phone_format: self.phone_format,
            size: self.size,
        })
    }
}
