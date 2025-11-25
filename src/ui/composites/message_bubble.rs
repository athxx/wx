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

    /// 测量消息气泡的高度（纯计算，不渲染）
    /// 供虚拟列表在渲染前预计算高度使用
    pub fn measure(
        message: &Rc<Message>,
        width: gpui::Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::Size<gpui::Pixels> {
        use gpui::{div, relative, size, AvailableSpace, IntoElement, ParentElement, Styled};
        use gpui_component::{h_flex, v_flex};

        // 1. 获取与 MessageBubble 一致的布局常量
        let avatar_size = crate::ui::constants::avatar_small();
        let bubble_max_width = crate::ui::constants::bubble_max_width();
        let arrow_placeholder_width = crate::ui::constants::message_bubble_arrow_width();

        // 2. 构造布局代理 (Layout Proxy) - 结构必须与 render 方法完全一致

        // 模拟 Avatar 占位
        let avatar_placeholder = div().w(avatar_size).h(avatar_size);

        // 模拟 Header (名字+时间) 占位
        // 使用与 render 完全一致的 h_flex 结构，确保高度计算准确
        let header_placeholder = h_flex()
            .gap_2()
            .child(div().text_xs().child("Name"))
            .child(div().text_xs().child("00:00"));

        // 模拟消息气泡文本内容（合并容器，避免多一层 wrapper）
        let bubble_inner_padding = div()
            .px(crate::ui::constants::message_bubble_inner_padding_x())
            .py(crate::ui::constants::message_bubble_inner_padding_y())
            .max_w(bubble_max_width)
            .whitespace_normal() // 关键：允许文本换行
            .text_sm() // 关键：字体大小必须一致
            .line_height(relative(crate::ui::constants::message_bubble_line_height())) // 关键：行高必须一致
            .child(message.content.clone());

        // 模拟箭头和气泡的包裹容器
        let bubble_and_arrow_proxy = div()
            .flex()
            .items_start()
            .child(
                div()
                    .w(arrow_placeholder_width)
                    .h(crate::ui::constants::message_bubble_arrow_height())
                    .flex_none()
                    .mt(crate::ui::constants::message_bubble_arrow_offset_y()),
            )
            .child(bubble_inner_padding);

        // 组装整体结构
        let layout_proxy = div()
            .w_full()
            .px(crate::ui::constants::message_bubble_outer_padding_x())
            .py(crate::ui::constants::message_bubble_outer_padding_y())
            // 增加微小底部缓冲，防止计算精度误差导致重叠
            .pb(gpui::px(2.))
            .child(
                div()
                    .flex()
                    .gap(crate::ui::constants::message_bubble_gap_avatar_content())
                    .child(avatar_placeholder)
                    .child(
                        v_flex()
                            .gap(crate::ui::constants::message_bubble_gap_header_bubble())
                            .child(header_placeholder)
                            .child(bubble_and_arrow_proxy),
                    ),
            );

        // 3. 执行测量
        let mut element = layout_proxy.into_any_element();
        // 给予确定的宽度，让 GPUI 计算内容所需的高度
        let available_space = size(AvailableSpace::Definite(width), AvailableSpace::MinContent);
        element.layout_as_root(available_space, window, cx)
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
            // 使用固定偏移，让箭头中心位于第一行的中间位置
            .mt(crate::ui::constants::message_bubble_arrow_offset_y())
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
            .max_w(crate::ui::constants::bubble_max_width())
            .whitespace_normal()
            .child(self.message.content.clone());

        let bubble_and_arrow_wrapper = div()
            .flex()
            // 顶对齐，再配合 arrow_offset_y 精确控制箭头在第一行中间
            .items_start()
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
