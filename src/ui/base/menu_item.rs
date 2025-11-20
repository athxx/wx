use gpui::{
    div, prelude::FluentBuilder, rgb, App, InteractiveElement, IntoElement, MouseDownEvent,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window,
};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct MenuItem {
    id: SharedString,
    label: SharedString,
    hovered: bool,
    compact: bool,
    set_hover: Option<Rc<dyn Fn(bool, &mut App)>>,
    on_click: Option<Rc<dyn Fn(&MouseDownEvent, &mut Window, &mut App)>>,
}

impl MenuItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            hovered: false,
            compact: false,
            set_hover: None,
            on_click: None,
        }
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn on_hover(mut self, handler: impl Fn(bool, &mut App) + 'static) -> Self {
        self.set_hover = Some(Rc::new(handler));
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for MenuItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let el = div()
            .id(self.id)
            .w_full()
            .px_3()
            .rounded(crate::ui::constants::radius_sm())
            .cursor_pointer()
            .text_sm();

        let el = if self.compact { el.py_1() } else { el.py_2() };

        el.when(self.hovered, |this| this.bg(rgb(0x07C160))) // TODO: Use theme color
            .when_some(self.set_hover, |this, handler| {
                this.on_hover(move |&is_hovering, _, cx| {
                    handler(is_hovering, cx);
                })
            })
            .when_some(self.on_click, |this, handler| {
                this.on_mouse_down(gpui::MouseButton::Left, move |event, window, cx| {
                    handler(event, window, cx)
                })
            })
            .child(
                div()
                    .when(self.hovered, |this| this.text_color(gpui::white()))
                    .child(self.label),
            )
    }
}
