use crate::ui::composites::setting_card;
use gpui::{div, px, IntoElement, ParentElement, Styled};
use gpui_component::{h_flex, v_flex, ActiveTheme};

use super::SettingsWindow;

impl SettingsWindow {
    pub(crate) fn render_about_tab(&self, cx: &gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let version_row = setting_card::SettingCard::new(
            h_flex()
                .w_full()
                .items_center()
                .justify_between()
                .px_4()
                .py_3()
                .child(
                    v_flex()
                        .gap(px(2.))
                        .child(div().text_sm().text_color(theme.foreground).child("版本信息"))
                        .child(div().text_xs().text_color(theme.muted_foreground).child("4.1.2.17")),
                )
                .child(
                    crate::ui::base::settings_button::SettingsButton::new("about-check-update")
                        .label("检查更新"),
                ),
        );

        let help_row = setting_card::SettingCard::new(
            h_flex()
                .w_full()
                .items_center()
                .justify_between()
                .px_4()
                .py_3()
                .child(div().text_sm().text_color(theme.foreground).child("微信帮助"))
                .child(
                    crate::ui::base::settings_button::SettingsButton::new("about-view-help")
                        .label("查看帮助"),
                ),
        );

        v_flex().gap_4().child(version_row).child(help_row)
    }
}
