use crate::models::Message;
use crate::ui::theme::Theme;
use crate::utils::time::format_time_hhmm;
use gpui::prelude::FluentBuilder;
use gpui::{div, relative, App, IntoElement, ParentElement, RenderOnce, Styled, Window};
use gpui_component::{h_flex, v_flex, ActiveTheme, Sizable};

#[derive(IntoElement)]
pub struct MessageBubble {
    message: Message,
    is_group: bool,
}

impl MessageBubble {
    pub fn new(message: Message) -> Self {
        Self {
            message,
            is_group: false,
        }
    }

    pub fn group(mut self, is_group: bool) -> Self {
        self.is_group = is_group;
        self
    }
}

impl RenderOnce for MessageBubble {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);

        let is_self = self.message.is_self;
        let time_str = format_time_hhmm(&self.message.timestamp);

        div().w_full().px_5().py_2().child(
            div()
                .flex()
                .w_full()
                .when(is_self, |this| this.flex_row_reverse())
                .gap_3()
                .child(
                    crate::ui::base::avatar::Avatar::new(crate::ui::avatar::avatar_for_key(
                        &self.message.sender_id,
                    ))
                    .w(crate::ui::constants::avatar_small())
                    .h(crate::ui::constants::avatar_small()) // Added explicit height to match previous code which used Sizable trait implicitly via with_size, but our Avatar uses w/h methods
                    .rounded(crate::ui::constants::avatar_small_radius()),
                )
                .child(
                    v_flex()
                        .gap_1p5()
                        .max_w(crate::ui::constants::bubble_max_width())
                        .when(is_self, |this| this.items_end())
                        .child(
                            h_flex()
                                .gap_2()
                                .when(is_self, |this| this.flex_row_reverse())
                                .when(self.is_group && !is_self, |this| {
                                    this.child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.muted_foreground)
                                            .font_weight(gpui::FontWeight::MEDIUM)
                                            .child(self.message.sender_name.clone()),
                                    )
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.muted_foreground)
                                        .child(time_str),
                                ),
                        )
                        .child(
                            div().relative().child(
                                div()
                                    .px_3()
                                    .py_2()
                                    .rounded(crate::ui::constants::bubble_radius())
                                    .bg(if is_self {
                                        weixin_colors.message_bubble_self
                                    } else {
                                        weixin_colors.message_bubble_other
                                    })
                                    .text_color(if is_self {
                                        weixin_colors.message_text_self
                                    } else {
                                        weixin_colors.message_text_other
                                    })
                                    .text_sm()
                                    .line_height(relative(1.6))
                                    .child(self.message.content.clone()),
                            ),
                        ),
                ),
        )
    }
}
