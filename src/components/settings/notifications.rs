use crate::ui::composites::setting_card;
use gpui::{div, px, IntoElement, ParentElement, Styled};
use gpui_component::{h_flex, v_flex, ActiveTheme, Icon, Sizable, Size};
use gpui_component::switch::Switch;

use super::SettingsWindow;

impl SettingsWindow {
    pub(crate) fn render_notifications_tab(&self, cx: &gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let foreground = theme.foreground;
        let muted = theme.muted_foreground;

        let sound_notifications = setting_card::SettingCard::new(
            v_flex()
                .gap_0()
                .child(self.render_setting_row("新消息通知声音", true, cx))
                .child(setting_card::SettingDivider::new())
                .child(self.render_setting_row("语音和视频通话通知声音", true, cx)),
        );

        let badge_description = v_flex()
            .gap(px(2.))
            .pl_4()
            .pb_neg_4()
            .child(div().text_sm().text_color(foreground).child("通知标记"))
            .child(
                div()
                    .text_xs()
                    .text_color(muted)
                    .whitespace_normal()
                    .child("有内容更新时，侧边栏中该功能图标将出现标记提示。"),
            );

        let badge_row = setting_card::SettingCard::new(
            setting_card::setting_row()
                .py_3()
                .child(
                    h_flex()
                        .items_center()
                        .gap_3()
                        .child(
                            Icon::default()
                                .path("moments.svg")
                                .w(px(20.))
                                .h(px(20.))
                                .text_color(foreground),
                        )
                        .child(div().text_sm().text_color(foreground).child("朋友圈")),
                )
                .child(Switch::new("badge_toggle").checked(true).with_size(Size::Small)),
        );

        v_flex()
            .gap_6()
            .child(sound_notifications)
            .child(badge_description)
            .child(badge_row)
    }
}
