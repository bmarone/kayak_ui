use bevy::prelude::{Bundle, Component, Entity, Handle, Image, In, Query, Vec2};

use crate::{
    context::WidgetName,
    prelude::KayakWidgetContext,
    styles::{KStyle, RenderCommand, StyleProp},
    widget::Widget,
};

/// A widget that renders a texture atlas
/// Allows for the use of a partial square of an image such as in a sprite sheet
///
/// # Props
///
/// __Type:__ [`TextureAtlasProps`]
///
/// | Common Prop | Accepted |
/// | :---------: | :------: |
/// | `children`  |           |
/// | `styles`    | ✅        |
/// | `on_event`  | ✅        |
/// | `on_layout` | ✅        |
/// | `focusable` | ✅        |
///
#[derive(Component, PartialEq, Clone, Default, Debug)]
pub struct TextureAtlasProps {
    /// The handle to image
    pub handle: Handle<Image>,
    /// The position of the tile (in pixels)
    pub position: Vec2,
    /// The size of the tile (in pixels)
    pub tile_size: Vec2,
}

impl Widget for TextureAtlasProps {}

/// A widget that renders a bevy texture atlas
#[derive(Bundle)]
pub struct TextureAtlasBundle {
    pub atlas: TextureAtlasProps,
    pub styles: KStyle,
    pub widget_name: WidgetName,
}

impl Default for TextureAtlasBundle {
    fn default() -> Self {
        Self {
            atlas: Default::default(),
            styles: Default::default(),
            widget_name: TextureAtlasProps::default().get_name(),
        }
    }
}

pub fn texture_atlas_render(
    In((_widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut query: Query<(&mut KStyle, &TextureAtlasProps)>,
) -> bool {
    if let Ok((mut styles, texture_atlas)) = query.get_mut(entity) {
        *styles = KStyle {
            render_command: StyleProp::Value(RenderCommand::TextureAtlas {
                position: texture_atlas.position,
                size: texture_atlas.tile_size,
                handle: texture_atlas.handle.clone_weak(),
            }),
            ..styles.clone()
        };
    }

    true
}
