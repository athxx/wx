use crate::ui::base::icon_button::IconButton;
use gpui::{div, App, IntoElement, ParentElement, RenderOnce, Styled, Window, prelude::FluentBuilder};
use gpui_component::h_flex;

#[derive(IntoElement)]
pub struct ChatToolbar {
    is_group: bool,
}

impl ChatToolbar {
    pub fn new() -> Self {
        Self { is_group: false }
    }

    pub fn is_group(mut self, is_group: bool) -> Self {
        self.is_group = is_group;
        self
    }
}

impl RenderOnce for ChatToolbar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex()
            .w_full()
            .items_center()
            .child(
                h_flex()
                    .gap_2()
                    .child(IconButton::new("emoji.svg"))
                    .child(IconButton::new("favorite.svg"))
                    .child(IconButton::new("file.svg"))
                    .child(IconButton::new("scissors.svg"))
                    .child(
                        IconButton::new("down.svg")
                            .padding(crate::ui::constants::header_action_padding())
                            .rounded(crate::ui::constants::radius_md())
                            .w(crate::ui::constants::header_narrow_button_width())
                            .h(crate::ui::constants::header_narrow_button_height())
                    ),
            )
            .child(div().flex_1())
            .child(
                h_flex()
                    .gap_2()
                    .when(self.is_group, |this| {
                        this.child(IconButton::new("circle.svg"))
                            .child(IconButton::new("video-call.svg"))
                    })
                    .when(!self.is_group, |this| {
                        this.child(IconButton::new("phone-call.svg"))
                            .child(IconButton::new("video.svg"))
                    }),
            )
    }
}
