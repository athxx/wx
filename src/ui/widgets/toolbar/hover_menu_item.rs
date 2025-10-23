use gpui::prelude::FluentBuilder;
use gpui::{
    div, rgb, App, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement,
    Styled, Window,
};

pub fn hover_menu_item<FSet, L>(
    id: &'static str,
    label: &'static str,
    hovered: bool,
    set_hover: FSet,
    on_mouse_down: L,
) -> impl IntoElement
where
    FSet: Fn(bool, &mut App) + Clone + 'static,
    L: Fn(&gpui::MouseDownEvent, &mut Window, &mut App) + 'static,
{
    div()
        .id(id)
        .w_full()
        .px_3()
        .py_2()
.rounded(crate::ui::constants::radius_sm())
        .cursor_pointer()
        .text_sm()
        .when(hovered, |this| this.bg(rgb(0x07C160)))
        .on_hover(move |&is_hovering, _, cx| {
            set_hover(is_hovering, cx);
        })
        .on_mouse_down(gpui::MouseButton::Left, on_mouse_down)
        .child(
            div()
                .when(hovered, |this| this.text_color(gpui::white()))
                .child(label),
        )
}
