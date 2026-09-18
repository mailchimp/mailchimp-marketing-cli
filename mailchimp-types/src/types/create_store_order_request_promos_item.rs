pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreOrderRequestPromosItem {
    pub amount_discounted: CreateStoreOrderRequestPromosItemAmountDiscounted,
    /// The Promo Code
    #[serde(default)]
    pub code: String,
    /// Type of discount. For free shipping set type to fixed
    pub r#type: CreateStoreOrderRequestPromosItemType,
}

impl CreateStoreOrderRequestPromosItem {
    pub fn builder() -> CreateStoreOrderRequestPromosItemBuilder {
        <CreateStoreOrderRequestPromosItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreOrderRequestPromosItemBuilder {
    amount_discounted: Option<CreateStoreOrderRequestPromosItemAmountDiscounted>,
    code: Option<String>,
    r#type: Option<CreateStoreOrderRequestPromosItemType>,
}

impl CreateStoreOrderRequestPromosItemBuilder {
    pub fn amount_discounted(mut self, value: CreateStoreOrderRequestPromosItemAmountDiscounted) -> Self {
        self.amount_discounted = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: CreateStoreOrderRequestPromosItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateStoreOrderRequestPromosItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_discounted`](CreateStoreOrderRequestPromosItemBuilder::amount_discounted)
    /// - [`code`](CreateStoreOrderRequestPromosItemBuilder::code)
    /// - [`r#type`](CreateStoreOrderRequestPromosItemBuilder::r#type)
    pub fn build(self) -> Result<CreateStoreOrderRequestPromosItem, BuildError> {
        Ok(CreateStoreOrderRequestPromosItem {
            amount_discounted: self.amount_discounted.ok_or_else(|| BuildError::missing_field("amount_discounted"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
