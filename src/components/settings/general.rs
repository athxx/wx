use crate::ui::widgets::setting_card;
use gpui::{div, px, IntoElement, ParentElement, Styled, Window};
use gpui_component::{h_flex, v_flex, ActiveTheme};

use super::SettingsWindow;

impl SettingsWindow {
    pub(crate) fn render_general_tab(
        &self,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let theme_mode = cx.theme().mode;
        let theme = cx.theme();
        let foreground = theme.foreground;
        let muted = theme.muted_foreground;

        let language_row = {
            let label = div().text_sm().text_color(foreground).child("语言");
            let btn = self.render_language_button(window, cx);
            setting_card::row().py_3().child(label).child(btn)
        };

        let translate_language_row = {
            let description =
                v_flex()
                    .gap(px(2.))
                    .child(div().text_sm().text_color(foreground).child("将文字翻译为"))
                    .child(div().text_xs().text_color(muted).whitespace_normal().child(
                        "在微信聊天、网页及图片中使用翻译\r\n功能时，文字会被翻译为所选语言。",
                    ));
            let btn = self.render_translate_language_button(window, cx);
            setting_card::row().py_3().child(description).child(btn)
        };

        let appearance_card_content = {
            let theme_row = self.render_theme_setting(theme_mode, window, cx);
            let font_row = {
                let label = div().text_sm().text_color(foreground).child("字体大小");
                let slider = self.render_font_size_slider(window, cx);
                setting_card::row()
                    .py_3()
                    .child(label)
                    .child(h_flex().flex_1().justify_end().child(slider))
            };
            v_flex()
                .gap_0()
                .child(theme_row)
                .child(setting_card::divider(cx))
                .child(font_row)
        };

        let privacy_card_content = {
            let readonly_text = v_flex()
                .w_full()
                .flex_1()
                .min_w(px(0.))
                .gap(px(2.))
                .child(
                    div()
                        .text_sm()
                        .text_color(foreground)
                        .child("以只读的方式打开聊天中的文件"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(muted)
                        .whitespace_normal()
                        .child("开启后，可保护聊天中的文件不被修改。"),
                );
            let readonly_files_row = setting_card::row()
                .py_3()
                .child(readonly_text)
                .child(crate::ui::widgets::toggle::toggle_small(cx, true));

            let history_row = self.render_setting_row("显示网络搜索历史", true, cx);
            let voice_to_text_row =
                self.render_setting_row("聊天中的语音消息自动转成文字", false, cx);
            let system_browser_row =
                self.render_setting_row("使用系统默认浏览器打开第三方网页", false, cx);
            let keep_window_row = self.render_setting_row("点击截图按钮时保留当前窗口", true, cx);

            let hide_window_text = v_flex()
                .w_full()
                .flex_1()
                .min_w(px(0.))
                .gap(px(2.))
                .child(
                    div()
                        .text_sm()
                        .text_color(foreground)
                        .child("演示屏幕时隐藏微信主窗口"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(muted)
                        .whitespace_normal()
                        .child("开启后，使用其他软件共享屏幕、投屏和截\r\n屏时微信主窗口将不会显示在被演示的屏幕\r\n上。"),
                );
            let hide_window_row = setting_card::row()
                .py_3()
                .child(hide_window_text)
                .child(crate::ui::widgets::toggle::toggle_small(cx, false));

            v_flex()
                .gap_0()
                .child(readonly_files_row)
                .child(setting_card::divider(cx))
                .child(history_row)
                .child(setting_card::divider(cx))
                .child(voice_to_text_row)
                .child(setting_card::divider(cx))
                .child(system_browser_row)
                .child(setting_card::divider(cx))
                .child(keep_window_row)
                .child(setting_card::divider(cx))
                .child(hide_window_row)
        };

        let updates_card_content = {
            let auto_update_row = self.render_setting_row("有更新时自动升级微信", true, cx);
            let auto_launch_row = self.render_setting_row("开机时自动打开微信", false, cx);

            v_flex()
                .gap_0()
                .child(auto_update_row)
                .child(setting_card::divider(cx))
                .child(auto_launch_row)
        };

        let language_card_content = v_flex()
            .gap_0()
            .child(language_row)
            .child(setting_card::divider(cx))
            .child(translate_language_row);

        v_flex()
            .w_full()
            .gap_6()
            .child(setting_card::card(cx, language_card_content))
            .child(setting_card::card(cx, appearance_card_content))
            .child(setting_card::card(cx, privacy_card_content))
            .child(setting_card::card(cx, updates_card_content))
    }
}
