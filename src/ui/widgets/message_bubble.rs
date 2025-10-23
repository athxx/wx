use gpui::prelude::FluentBuilder;
use gpui::{div, relative, IntoElement, ParentElement, Styled};
use gpui_component::{h_flex, v_flex, Sizable};

use crate::models::Message;

pub fn message_bubble(
    message: &Message,
    is_group: bool,
    theme: &gpui_component::Theme,
    weixin_colors: &crate::ui::theme::WeixinThemeColors,
) -> impl IntoElement {
    let is_self = message.is_self;
    let time_str = message.timestamp.format("%H:%M").to_string();

    div().w_full().px_5().py_2().child(
        div()
            .flex()
            .w_full()
            .when(is_self, |this| this.flex_row_reverse())
            .gap_3()
            .child(
                gpui_component::avatar::Avatar::new()
                    .with_size(crate::ui::constants::avatar_small())
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
                            .when(is_group && !is_self, |this| {
                                this.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.muted_foreground)
                                        .font_weight(gpui::FontWeight::MEDIUM)
                                        .child(message.sender_name.clone()),
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
                                .text_base()
                                .line_height(relative(1.6))
                                .child(message.content.clone()),
                        ),
                    ),
            ),
    )
}
