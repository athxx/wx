use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{tooltip::Tooltip, ActiveTheme, Icon};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct IconButton {
    icon_path: SharedString,
    tooltip: Option<SharedString>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    icon_size: Pixels,
    padding: Pixels,
    rounded: Pixels,
    width: Option<Pixels>,
    height: Option<Pixels>,
    selected: bool,
}

impl IconButton {
    pub fn new(icon_path: impl Into<SharedString>) -> Self {
        Self {
            icon_path: icon_path.into(),
            tooltip: None,
            on_click: None,
            icon_size: crate::ui::constants::icon_sm(),
            padding: crate::ui::constants::icon_button_padding(),
            rounded: crate::ui::constants::radius_sm(),
            width: None,
            height: None,
            selected: false,
        }
    }

    pub fn icon_size(mut self, size: impl Into<Pixels>) -> Self {
        self.icon_size = size.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn rounded(mut self, rounded: impl Into<Pixels>) -> Self {
        self.rounded = rounded.into();
        self
    }

    pub fn w(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn h(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let bg_color = if self.selected {
            theme.secondary
        } else {
            theme.transparent
        };

        div()
            .id(SharedString::from(format!("btn-{}", self.icon_path)))
            .flex()
            .items_center()
            .justify_center()
            .p(self.padding)
            .rounded(self.rounded)
            .cursor_pointer()
            .bg(bg_color)
            .hover(|s| s.bg(theme.secondary_hover))
            .active(|s| s.bg(theme.secondary_active))
            .when_some(self.width, |this, w| this.w(w))
            .when_some(self.height, |this, h| this.h(h))
            .child(
                Icon::default()
                    .path(self.icon_path)
                    .size(self.icon_size)
                    .text_color(theme.foreground),
            )
            .when_some(self.on_click, |this, handler| {
                this.on_click(move |event, window, cx| handler(event, window, cx))
            })
            .when_some(self.tooltip, |this, text| {
                this.tooltip(move |window, cx| Tooltip::new(text.clone()).build(window, cx))
            })
    }
}
