use crate::ui::theme::Theme;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonCustomVariant, ButtonVariants as _},
    ActiveTheme, Sizable,
};

pub fn settings_button(cx: &App, id: &'static str) -> Button {
    let theme = cx.theme();
    let foreground = theme.foreground;

    let border_color = theme.border;

    let weixin_colors = Theme::weixin_colors(cx);
    let (bg, hover, active) = (
        weixin_colors.settings_button_bg,
        weixin_colors.settings_button_hover,
        weixin_colors.settings_button_active,
    );

    let settings_button_variant = ButtonCustomVariant::new(cx)
        .color(bg)
        .foreground(foreground)
        .border(border_color)
        .hover(hover)
        .active(active);

    Button::new(id)
        .xsmall()
        .p_3()
        .custom(settings_button_variant)
}
