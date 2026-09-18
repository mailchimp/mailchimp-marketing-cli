pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about the contact.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSurveyQuestionAnswersResponseAnswersItemContact {
    /// URL for the contact's avatar or profile image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Indicates whether a contact consents to 1:1 messaging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consents_to_one_to_one_messaging: Option<bool>,
    /// The ID of this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// The contact's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The contact's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// The contact's sms phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The contact's current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListSurveyQuestionAnswersResponseAnswersItemContactStatus>,
}

impl ListSurveyQuestionAnswersResponseAnswersItemContact {
    pub fn builder() -> ListSurveyQuestionAnswersResponseAnswersItemContactBuilder {
        <ListSurveyQuestionAnswersResponseAnswersItemContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSurveyQuestionAnswersResponseAnswersItemContactBuilder {
    avatar_url: Option<String>,
    consents_to_one_to_one_messaging: Option<bool>,
    contact_id: Option<String>,
    email: Option<String>,
    email_id: Option<String>,
    full_name: Option<String>,
    phone: Option<String>,
    status: Option<ListSurveyQuestionAnswersResponseAnswersItemContactStatus>,
}

impl ListSurveyQuestionAnswersResponseAnswersItemContactBuilder {
    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    pub fn consents_to_one_to_one_messaging(mut self, value: bool) -> Self {
        self.consents_to_one_to_one_messaging = Some(value);
        self
    }

    pub fn contact_id(mut self, value: impl Into<String>) -> Self {
        self.contact_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
        self
    }

    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.full_name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListSurveyQuestionAnswersResponseAnswersItemContactStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSurveyQuestionAnswersResponseAnswersItemContact`].
    pub fn build(self) -> Result<ListSurveyQuestionAnswersResponseAnswersItemContact, BuildError> {
        Ok(ListSurveyQuestionAnswersResponseAnswersItemContact {
            avatar_url: self.avatar_url,
            consents_to_one_to_one_messaging: self.consents_to_one_to_one_messaging,
            contact_id: self.contact_id,
            email: self.email,
            email_id: self.email_id,
            full_name: self.full_name,
            phone: self.phone,
            status: self.status,
        })
    }
}
