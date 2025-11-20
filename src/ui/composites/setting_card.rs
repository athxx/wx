use crate::ui::theme::Theme;
use gpui::*;
use gpui_component::{h_flex, ActiveTheme};

#[derive(IntoElement)]
pub struct SettingCard {
    content: AnyElement,
}

impl SettingCard {
    pub fn new(content: impl IntoElement) -> Self {
        Self {
            content: content.into_any_element(),
        }
    }
}

impl RenderOnce for SettingCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let card_bg = Theme::weixin_colors(cx).session_list_bg;
        let border_color = cx.theme().border;

        div()
            .w_full()
            .bg(card_bg)
            .rounded(crate::ui::constants::radius_lg())
            .border_1()
            .border_color(border_color)
            .child(div().w_full().child(self.content))
    }
}

#[derive(IntoElement)]
pub struct SettingDivider;

impl SettingDivider {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for SettingDivider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border_color = cx.theme().border;

        h_flex()
            .w_full()
            .h(crate::ui::constants::hairline())
            // Leave a small inset on the left so the divider does not touch the card padding.
            .child(div().w(px(12.)).h_full())
            // The divider line occupies the remaining width and aligns with the right edge of the content.
            .child(
                div()
                    .flex_1()
                    .h(crate::ui::constants::hairline())
                    .bg(border_color),
            )
    }
}

pub fn setting_row() -> Div {
    h_flex().w_full().items_center().justify_between().px_4()
}
