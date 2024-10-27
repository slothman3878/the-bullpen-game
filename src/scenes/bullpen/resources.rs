use crate::prelude::*;

#[derive(Debug, Deref, Resource)]
pub(crate) struct BaseballPreviewImage(Handle<Image>);

impl BaseballPreviewImage {
    pub(crate) fn new(image_handle: Handle<Image>) -> Self {
        Self(image_handle)
    }
}

// Add these new resources
#[derive(Debug, Resource, Default)]
pub(crate) struct MenuVisibility(pub bool);

pub(crate) fn menu_visibility_is(
    visibility: bool,
) -> impl FnMut(Res<MenuVisibility>) -> bool + Clone {
    move |menu_visibility| menu_visibility.0 == visibility
}
