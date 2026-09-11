pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateActionTestCampaignsRequest {
    /// Choose the type of test email to send.
    pub send_type: CreateActionTestCampaignsRequestSendType,
    /// An array of email addresses to send the test email to.
    #[serde(default)]
    pub test_emails: Vec<String>,
}

impl CreateActionTestCampaignsRequest {
    pub fn builder() -> CreateActionTestCampaignsRequestBuilder {
        <CreateActionTestCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateActionTestCampaignsRequestBuilder {
    send_type: Option<CreateActionTestCampaignsRequestSendType>,
    test_emails: Option<Vec<String>>,
}

impl CreateActionTestCampaignsRequestBuilder {
    pub fn send_type(mut self, value: CreateActionTestCampaignsRequestSendType) -> Self {
        self.send_type = Some(value);
        self
    }

    pub fn test_emails(mut self, value: Vec<String>) -> Self {
        self.test_emails = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateActionTestCampaignsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`send_type`](CreateActionTestCampaignsRequestBuilder::send_type)
    /// - [`test_emails`](CreateActionTestCampaignsRequestBuilder::test_emails)
    pub fn build(self) -> Result<CreateActionTestCampaignsRequest, BuildError> {
        Ok(CreateActionTestCampaignsRequest {
            send_type: self.send_type.ok_or_else(|| BuildError::missing_field("send_type"))?,
            test_emails: self.test_emails.ok_or_else(|| BuildError::missing_field("test_emails"))?,
        })
    }
}

