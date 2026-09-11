pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateStoreOrderEcommerceRequestPromosItem {
    pub amount_discounted: UpdateStoreOrderEcommerceRequestPromosItemAmountDiscounted,
    /// The Promo Code
    #[serde(default)]
    pub code: String,
    /// Type of discount. For free shipping set type to fixed
    pub r#type: UpdateStoreOrderEcommerceRequestPromosItemType,
}

impl UpdateStoreOrderEcommerceRequestPromosItem {
    pub fn builder() -> UpdateStoreOrderEcommerceRequestPromosItemBuilder {
        <UpdateStoreOrderEcommerceRequestPromosItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreOrderEcommerceRequestPromosItemBuilder {
    amount_discounted: Option<UpdateStoreOrderEcommerceRequestPromosItemAmountDiscounted>,
    code: Option<String>,
    r#type: Option<UpdateStoreOrderEcommerceRequestPromosItemType>,
}

impl UpdateStoreOrderEcommerceRequestPromosItemBuilder {
    pub fn amount_discounted(mut self, value: UpdateStoreOrderEcommerceRequestPromosItemAmountDiscounted) -> Self {
        self.amount_discounted = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: UpdateStoreOrderEcommerceRequestPromosItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoreOrderEcommerceRequestPromosItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_discounted`](UpdateStoreOrderEcommerceRequestPromosItemBuilder::amount_discounted)
    /// - [`code`](UpdateStoreOrderEcommerceRequestPromosItemBuilder::code)
    /// - [`r#type`](UpdateStoreOrderEcommerceRequestPromosItemBuilder::r#type)
    pub fn build(self) -> Result<UpdateStoreOrderEcommerceRequestPromosItem, BuildError> {
        Ok(UpdateStoreOrderEcommerceRequestPromosItem {
            amount_discounted: self.amount_discounted.ok_or_else(|| BuildError::missing_field("amount_discounted"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
