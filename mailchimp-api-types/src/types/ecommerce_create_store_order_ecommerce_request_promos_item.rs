pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStoreOrderEcommerceRequestPromosItem {
    pub amount_discounted: CreateStoreOrderEcommerceRequestPromosItemAmountDiscounted,
    /// The Promo Code
    #[serde(default)]
    pub code: String,
    /// Type of discount. For free shipping set type to fixed
    pub r#type: CreateStoreOrderEcommerceRequestPromosItemType,
}

impl CreateStoreOrderEcommerceRequestPromosItem {
    pub fn builder() -> CreateStoreOrderEcommerceRequestPromosItemBuilder {
        <CreateStoreOrderEcommerceRequestPromosItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreOrderEcommerceRequestPromosItemBuilder {
    amount_discounted: Option<CreateStoreOrderEcommerceRequestPromosItemAmountDiscounted>,
    code: Option<String>,
    r#type: Option<CreateStoreOrderEcommerceRequestPromosItemType>,
}

impl CreateStoreOrderEcommerceRequestPromosItemBuilder {
    pub fn amount_discounted(mut self, value: CreateStoreOrderEcommerceRequestPromosItemAmountDiscounted) -> Self {
        self.amount_discounted = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: CreateStoreOrderEcommerceRequestPromosItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateStoreOrderEcommerceRequestPromosItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_discounted`](CreateStoreOrderEcommerceRequestPromosItemBuilder::amount_discounted)
    /// - [`code`](CreateStoreOrderEcommerceRequestPromosItemBuilder::code)
    /// - [`r#type`](CreateStoreOrderEcommerceRequestPromosItemBuilder::r#type)
    pub fn build(self) -> Result<CreateStoreOrderEcommerceRequestPromosItem, BuildError> {
        Ok(CreateStoreOrderEcommerceRequestPromosItem {
            amount_discounted: self.amount_discounted.ok_or_else(|| BuildError::missing_field("amount_discounted"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
