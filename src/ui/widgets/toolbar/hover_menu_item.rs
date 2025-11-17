use gpui::{
    div, prelude::FluentBuilder, rgb, App, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window,
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
    hover_menu_item_internal(id, label, hovered, set_hover, on_mouse_down, false)
}

pub fn hover_menu_item_compact<FSet, L>(
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
    hover_menu_item_internal(id, label, hovered, set_hover, on_mouse_down, true)
}

fn hover_menu_item_internal<FSet, L>(
    id: &'static str,
    label: &'static str,
    hovered: bool,
    set_hover: FSet,
    on_mouse_down: L,
    compact: bool,
) -> impl IntoElement
where
    FSet: Fn(bool, &mut App) + Clone + 'static,
    L: Fn(&gpui::MouseDownEvent, &mut Window, &mut App) + 'static,
{
    let el = div()
        .id(id)
        .w_full()
        .px_3()
        .rounded(crate::ui::constants::radius_sm())
        .cursor_pointer()
        .text_sm();

    let el = if compact { el.py_1() } else { el.py_2() };

    el.when(hovered, |this| this.bg(rgb(0x07C160)))
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
