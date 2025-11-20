use gpui::{div, img, px, IntoElement, ParentElement, Styled};
use gpui_component::{h_flex, v_flex, ActiveTheme};

use super::SettingsWindow;

impl SettingsWindow {
    pub(crate) fn render_plugins_tab(&self, cx: &gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let header = v_flex()
            .gap(px(2.))
            .child(
                div()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .text_center()
                    .child("暂无可用的插件"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .text_center()
                    .child("以下为腾讯官方出品的其他应用"),
            );

        let plugins = [
            (
                "plugin-download-yuanbao",
                "腾讯元宝",
                "长文总结 | 图表处理 | 写作翻译",
                "setting/tencent_yuanbao.svg",
            ),
            (
                "plugin-download-input-method",
                "微信输入法",
                "无广告干扰，界面简洁打字快",
                "setting/wechat_input_method.svg",
            ),
        ];

        let plugin_cards = plugins.into_iter().map(|(id, name, desc, icon_path)| {
            div()
                .w_full()
                .rounded(crate::ui::constants::radius_lg())
                .border_1()
                .border_color(theme.border)
                .bg(theme.background)
                .child(
                    h_flex()
                        .w_full()
                        .items_center()
                        .justify_between()
                        .px_4()
                        .py_3()
                        .child(
                            h_flex()
                                .items_center()
                                .gap_3()
                                .child(img(icon_path).w(px(38.)).h(px(38.)))
                                .child(
                                    v_flex()
                                        .gap(px(2.))
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(theme.foreground)
                                                .child(name),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.muted_foreground)
                                                .child(desc),
                                        ),
                                ),
                        )
                        .child(
                            crate::ui::base::settings_button::SettingsButton::new(id)
                                .label("下载"),
                        ),
                )
        });

        v_flex().gap_4().child(header).children(plugin_cards)
    }
}
