pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Options for customizing your signup form header.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignupFormHeader {
    /// Image alignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_align: Option<SignupFormHeaderImageAlign>,
    /// Alt text for the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_alt: Option<String>,
    /// Image border color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_border_color: Option<String>,
    /// Image border style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_border_style: Option<SignupFormHeaderImageBorderStyle>,
    /// Image border width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_border_width: Option<String>,
    /// Image height, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_height: Option<String>,
    /// The URL that the header image will link to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_link: Option<String>,
    /// Image link target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_target: Option<SignupFormHeaderImageTarget>,
    /// Header image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Image width, in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_width: Option<String>,
    /// Header text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SignupFormHeader {
    pub fn builder() -> SignupFormHeaderBuilder {
        <SignupFormHeaderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignupFormHeaderBuilder {
    image_align: Option<SignupFormHeaderImageAlign>,
    image_alt: Option<String>,
    image_border_color: Option<String>,
    image_border_style: Option<SignupFormHeaderImageBorderStyle>,
    image_border_width: Option<String>,
    image_height: Option<String>,
    image_link: Option<String>,
    image_target: Option<SignupFormHeaderImageTarget>,
    image_url: Option<String>,
    image_width: Option<String>,
    text: Option<String>,
}

impl SignupFormHeaderBuilder {
    pub fn image_align(mut self, value: SignupFormHeaderImageAlign) -> Self {
        self.image_align = Some(value);
        self
    }

    pub fn image_alt(mut self, value: impl Into<String>) -> Self {
        self.image_alt = Some(value.into());
        self
    }

    pub fn image_border_color(mut self, value: impl Into<String>) -> Self {
        self.image_border_color = Some(value.into());
        self
    }

    pub fn image_border_style(mut self, value: SignupFormHeaderImageBorderStyle) -> Self {
        self.image_border_style = Some(value);
        self
    }

    pub fn image_border_width(mut self, value: impl Into<String>) -> Self {
        self.image_border_width = Some(value.into());
        self
    }

    pub fn image_height(mut self, value: impl Into<String>) -> Self {
        self.image_height = Some(value.into());
        self
    }

    pub fn image_link(mut self, value: impl Into<String>) -> Self {
        self.image_link = Some(value.into());
        self
    }

    pub fn image_target(mut self, value: SignupFormHeaderImageTarget) -> Self {
        self.image_target = Some(value);
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn image_width(mut self, value: impl Into<String>) -> Self {
        self.image_width = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SignupFormHeader`].
    pub fn build(self) -> Result<SignupFormHeader, BuildError> {
        Ok(SignupFormHeader {
            image_align: self.image_align,
            image_alt: self.image_alt,
            image_border_color: self.image_border_color,
            image_border_style: self.image_border_style,
            image_border_width: self.image_border_width,
            image_height: self.image_height,
            image_link: self.image_link,
            image_target: self.image_target,
            image_url: self.image_url,
            image_width: self.image_width,
            text: self.text,
        })
    }
}
