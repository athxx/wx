use std::{f32::consts::PI, rc::Rc};

use gpui::{
    div, prelude::FluentBuilder, relative, App, IntoElement, ParentElement, Radians, RenderOnce,
    Styled, Window,
};
use gpui_component::{h_flex, v_flex, ActiveTheme, Icon};

use crate::models::Message;
use crate::ui::theme::Theme;
use crate::utils::time::format_time_hhmm;

#[derive(IntoElement)]
pub struct MessageBubble {
    message: Rc<Message>,
    is_group: bool,
}

impl MessageBubble {
    pub fn new(message: Rc<Message>) -> Self {
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

        // [重构] 提前确定气泡的背景色和文字颜色
        let bubble_bg_color = if is_self {
            weixin_colors.message_bubble_self
        } else {
            weixin_colors.message_bubble_other
        };

        let bubble_text_color = if is_self {
            weixin_colors.message_text_self
        } else {
            weixin_colors.message_text_other
        };

        let arrow_icon = div()
            .flex()
            .items_center()
            .flex_none()
            .text_color(bubble_bg_color)
            .when(is_self, |t| t.mr_neg_1())
            .when(!is_self, |t| t.ml_neg_1())
            .child(
                Icon::default()
                    .w(crate::ui::constants::message_bubble_arrow_icon_size())
                    .h(crate::ui::constants::message_bubble_arrow_icon_size())
                    .path(crate::ui::constants::message_bubble_arrow_path())
                    .rotate(if is_self { Radians(0.) } else { Radians(PI) }),
            );

        let colored_bubble = div()
            .px(crate::ui::constants::message_bubble_inner_padding_x())
            .py(crate::ui::constants::message_bubble_inner_padding_y())
            .rounded(crate::ui::constants::bubble_radius())
            .bg(bubble_bg_color)
            .text_color(bubble_text_color)
            .text_sm()
            .line_height(relative(crate::ui::constants::message_bubble_line_height()))
            .child(
                div()
                    .max_w(crate::ui::constants::bubble_max_width())
                    .whitespace_normal()
                    .child(self.message.content.clone()),
            );

        let bubble_and_arrow_wrapper = div()
            .flex()
            .items_center()
            .when(is_self, |this| this.flex_row_reverse())
            .child(arrow_icon)
            .child(colored_bubble);

        div()
            .w_full()
            .px(crate::ui::constants::message_bubble_outer_padding_x())
            .py(crate::ui::constants::message_bubble_outer_padding_y())
            .child(
                div()
                    .flex()
                    .w_full()
                    .when(is_self, |this| this.flex_row_reverse())
                    .gap(crate::ui::constants::message_bubble_gap_avatar_content())
                    .child(
                        crate::ui::base::avatar::Avatar::new(crate::ui::avatar::avatar_for_key(
                            &self.message.sender_id,
                        ))
                        .w(crate::ui::constants::avatar_small())
                        .h(crate::ui::constants::avatar_small())
                        .rounded(crate::ui::constants::avatar_small_radius()),
                    )
                    .child(
                        v_flex()
                            .gap(crate::ui::constants::message_bubble_gap_header_bubble())
                            .when(is_self, |this| this.items_end())
                            .when(self.is_group && !is_self, |this| this.items_start())
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
                            // [修改] 这里替换为新的包裹容器
                            .child(bubble_and_arrow_wrapper),
                    ),
            )
    }
}
