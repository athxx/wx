use chrono::Local;
use gpui::{div, IntoElement, ParentElement, Styled};
use gpui_component::{h_flex, v_flex};

use crate::models::Contact;

pub fn session_row_content(
    contact: &Contact,
    theme: &gpui_component::Theme,
    weixin_colors: &crate::ui::theme::WeixinThemeColors,
) -> impl IntoElement {
    let time_str = contact
        .last_message_time
        .as_ref()
        .map(|time| {
            let now = Local::now();
            let duration = now.signed_duration_since(*time);
            if duration.num_days() > 0 {
                time.format("%m/%d").to_string()
            } else if duration.num_hours() > 0 {
                time.format("%H:%M").to_string()
            } else {
                time.format("%H:%M").to_string()
            }
        })
        .unwrap_or_default();

    h_flex()
        .w_full()
        .gap_3()
        .items_center()
        .child(crate::ui::widgets::badge_avatar::badge_avatar(
            contact,
            weixin_colors,
        ))
        .child(
            v_flex()
                .flex_1()
                .gap_1()
                .overflow_hidden()
                .min_w_0()
                .child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .text_base()
                                .font_weight(gpui::FontWeight::NORMAL)
                                .text_color(theme.foreground)
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .text_ellipsis()
                                .text_sm()
                                .child(if contact.is_group {
                                    if let Some(count) = contact.member_count {
                                        format!("{} ({})", contact.name, count)
                                    } else {
                                        contact.name.clone()
                                    }
                                } else {
                                    contact.name.clone()
                                }),
                        )
                        .child(
                            div()
                                .flex_shrink_0()
                                .text_xs()
                                .text_color(theme.muted_foreground)
                                .child(time_str),
                        ),
                )
                .child(
                    div()
                        .w_full()
                        .text_sm()
                        .text_color(theme.muted_foreground)
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis()
                        .child({
                            let message = contact
                                .last_message
                                .clone()
                                .unwrap_or_else(|| "暂无消息".to_string());
                            if contact.is_group {
                                if let Some(sender) = &contact.last_sender_name {
                                    format!("{}: {}", sender, message)
                                } else {
                                    message
                                }
                            } else {
                                message
                            }
                        }),
                ),
        )
}
