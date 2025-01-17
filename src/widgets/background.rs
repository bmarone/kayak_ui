use bevy::prelude::{Bundle, Commands, Component, Entity, In, Query};

use crate::{
    children::KChildren,
    context::WidgetName,
    on_event::OnEvent,
    prelude::KayakWidgetContext,
    styles::{KStyle, RenderCommand, StyleProp},
    widget::Widget,
};

#[derive(Component, PartialEq, Eq, Clone, Default)]
pub struct Background;

impl Widget for Background {}

/// Background Widget
///
/// The name of this widget is slightly misleading.
/// In actuality this widget renders a quad or multiple quads if a border is used.
/// You can customize the colors, border, border-radius, by passing in custom styles.
/// Children are rendered inside of the quad.
#[derive(Bundle)]
pub struct BackgroundBundle {
    pub background: Background,
    pub styles: KStyle,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for BackgroundBundle {
    fn default() -> Self {
        Self {
            background: Default::default(),
            styles: Default::default(),
            children: Default::default(),
            on_event: Default::default(),
            widget_name: Background::default().get_name(),
        }
    }
}

pub fn background_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    _: Commands,
    mut query: Query<(&mut KStyle, &KChildren)>,
) -> bool {
    if let Ok((mut style, children)) = query.get_mut(entity) {
        style.render_command = StyleProp::Value(RenderCommand::Quad);
        children.process(&widget_context, Some(entity));
    }
    true
}
