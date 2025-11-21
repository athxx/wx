use crate::ui::theme::Theme;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{avatar::Avatar as GpuiAvatar, badge::Badge, ActiveTheme, Sizable};

#[derive(IntoElement)]
pub struct Avatar {
    src: Option<SharedString>,
    is_group: bool,
    group_members: Vec<String>,
    unread_count: usize,
    size: Pixels,
    rounded: Pixels,
}

impl Avatar {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: Some(src.into()),
            is_group: false,
            group_members: Vec::new(),
            unread_count: 0,
            size: crate::ui::constants::avatar_large(),
            rounded: crate::ui::constants::avatar_small_radius(),
        }
    }

    pub fn group(members: Vec<String>) -> Self {
        Self {
            src: None,
            is_group: true,
            group_members: members.into_iter().take(4).collect(),
            unread_count: 0,
            size: crate::ui::constants::avatar_large(),
            rounded: crate::ui::constants::avatar_small_radius(),
        }
    }

    pub fn unread_count(mut self, count: usize) -> Self {
        self.unread_count = count;
        self
    }

    #[allow(dead_code)]
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into();
        self
    }

    pub fn rounded(mut self, rounded: impl Into<Pixels>) -> Self {
        self.rounded = rounded.into();
        self
    }

    pub fn w(mut self, w: impl Into<Pixels>) -> Self {
        self.size = w.into();
        self
    }

    pub fn h(self, _h: impl Into<Pixels>) -> Self {
        // We assume width = height for avatars
        self
    }

    #[allow(dead_code)]
    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(src.into());
        self
    }

    fn render_content(&self, _cx: &App) -> impl IntoElement {
        if self.is_group {
            self.render_group_avatar(_cx)
        } else {
            div()
                .overflow_hidden()
                .child(
                    GpuiAvatar::new()
                        .rounded(self.rounded)
                        .with_size(self.size)
                        .src(self.src.clone().unwrap_or_default()),
                )
                .into_any_element()
        }
    }

    fn render_group_avatar(&self, cx: &App) -> AnyElement {
        let half_size = self.size / 2.0;
        let theme = cx.theme();

        let mut members = self.group_members.clone();
        while members.len() < 4 {
            members.push("".to_string());
        }

        div()
            .size(self.size)
            .rounded(self.rounded)
            .overflow_hidden()
            .bg(theme.muted)
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_wrap()
                    .children(members.iter().map(|member| {
                        div().size(half_size).child(
                            GpuiAvatar::new()
                                .src(crate::ui::avatar::avatar_for_key(member))
                                .xsmall(),
                        )
                    })),
            )
            .into_any_element()
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let weixin_colors = Theme::weixin_colors(cx);

        div()
            .flex_shrink_0()
            .when(self.unread_count > 0, |this| {
                this.child(
                    Badge::new()
                        .count(self.unread_count)
                        .max(99)
                        .color(weixin_colors.unread_badge)
                        .child(self.render_content(cx)),
                )
            })
            .when(self.unread_count == 0, |this| {
                this.child(self.render_content(cx))
            })
    }
}
