use crate::ui::theme::Theme;
use crate::ui::widgets::setting_card;
use gpui::{div, px, InteractiveElement, IntoElement, MouseDownEvent, ParentElement, Styled};
use gpui_component::{avatar::Avatar, h_flex, input::Input, v_flex, ActiveTheme, Sizable};

use super::SettingsWindow;

impl SettingsWindow {
    pub(crate) fn render_account_and_storage_tab(
        &self,
        cx: &gpui::Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let weixin_colors = Theme::weixin_colors(cx);
        let foreground = theme.foreground;
        let muted = theme.muted_foreground;
        let input_bg = weixin_colors.input_field_bg;
        let input_focus_color = weixin_colors.input_field_focus;
        let default_border_color = theme.border;

        let account_card = {
            let avatar_and_info = h_flex()
                .items_center()
                .gap_3()
                .child(
                    Avatar::new()
                        .w(crate::ui::constants::title_avatar_size())
                        .h(crate::ui::constants::title_avatar_size())
                        .rounded(crate::ui::constants::radius_md())
                        .src(crate::ui::avatar::avatar_for_key("self")),
                )
                .child(
                    v_flex()
                        .gap_0()
                        .child(div().text_sm().text_color(foreground).child("@@@"))
                        .child(div().text_xs().text_color(muted).child("H1548772930")),
                );

            let header_row = setting_card::row().py_3().child(avatar_and_info).child(
                crate::ui::widgets::settings_button::settings_button(cx, "settings-account-logout")
                    .label("退出登录"),
            );

            let auto_login_row = setting_card::row()
                .py_3()
                .child(
                    v_flex()
                        .gap(px(2.))
                        .child(div().text_sm().text_color(foreground).child("自动登录"))
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted)
                                .child("在本机登录微信将无需手机确认"),
                        ),
                )
                .child(crate::ui::widgets::toggle::toggle_small(cx, true));

            let keep_history_row = setting_card::row()
                .py_3()
                .child(
                    v_flex()
                        .gap(px(2.))
                        .child(div().text_sm().text_color(foreground).child("保留聊天记录")),
                )
                .child(crate::ui::widgets::toggle::toggle_small(cx, true));

            setting_card::card(
                cx,
                v_flex()
                    .gap_0()
                    .child(header_row)
                    .child(setting_card::divider(cx))
                    .child(auto_login_row)
                    .child(setting_card::divider(cx))
                    .child(keep_history_row),
            )
        };

        let storage_card = {
            let storage_space_row = setting_card::row()
                .py_4()
                .child(div().text_sm().text_color(foreground).child("存储空间"))
                .child(
                    crate::ui::widgets::settings_button::settings_button(
                        cx,
                        "settings-storage-manage",
                    )
                    .child("管理"),
                );

            let path_value_color = weixin_colors.storage_path_text;

            let path_left = v_flex()
                .gap(px(2.))
                .child(div().text_sm().text_color(foreground).child("存储位置"))
                .child(
                    div()
                        .text_xs()
                        .text_color(path_value_color)
                        .child("D:\\wxwechat_files"),
                );

            let path_row = setting_card::row().py_4().child(path_left).child(
                crate::ui::widgets::settings_button::settings_button(cx, "settings-storage-change")
                    .child("更改"),
            );

            let blur_auto_input =
                cx.listener(|this: &mut SettingsWindow, _: &MouseDownEvent, window, _| {
                    if this.is_auto_download_input_focused() {
                        this.blur_focus(window);
                    }
                });

            let auto_download_row = setting_card::row()
                .py_4()
                .child(
                    h_flex()
                        .items_center()
                        .gap_2()
                        .child(div().text_sm().text_color(foreground).child("自动下载小于"))
                        .child(
                            div()
                                .w(crate::ui::constants::settings_small_input_width())
                                .rounded(crate::ui::constants::radius_sm())
                                .border_1()
                                .border_color(if self.is_auto_download_input_focused() {
                                    input_focus_color
                                } else {
                                    default_border_color
                                })
                                .bg(input_bg)
                                .on_mouse_down_out(blur_auto_input)
                                .child(
                                    Input::new(self.auto_download_limit_input())
                                        .xsmall()
                                        .appearance(false)
                                        .focus_bordered(false)
                                        .bordered(false)
                                        .text_sm()
                                        .text_color(foreground)
                                        .w_full(),
                                ),
                        )
                        .child(div().text_sm().text_color(foreground).child("MB 的文件")),
                )
                .child(crate::ui::widgets::toggle::toggle_small(cx, true));

            let clear_button_row = h_flex().justify_end().px_4().py_4().child(
                crate::ui::widgets::settings_button::settings_button(cx, "settings-clear-messages")
                    .label("清空全部聊天记录"),
            );

            setting_card::card(
                cx,
                v_flex()
                    .gap_0()
                    .child(storage_space_row)
                    .child(setting_card::divider(cx))
                    .child(path_row)
                    .child(setting_card::divider(cx))
                    .child(auto_download_row)
                    .child(setting_card::divider(cx))
                    .child(clear_button_row),
            )
        };

        v_flex().gap_6().child(account_card).child(storage_card)
    }
}
