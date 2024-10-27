use crate::prelude::*;

#[derive(Deref, Resource)]
pub(crate) struct BaseballPreviewImage(Handle<Image>);

impl BaseballPreviewImage {
    pub(crate) fn new(image_handle: Handle<Image>) -> Self {
        Self(image_handle)
    }
}
