use gpui::{div, App, IntoElement, ParentElement, Pixels, RenderOnce, Styled, Window};
use gpui_component::{avatar::Avatar, ActiveTheme, Sizable};

#[derive(IntoElement)]
pub struct GroupAvatar {
    members: Vec<String>,
    size: Pixels,
}

impl GroupAvatar {
    pub fn new(members: Vec<String>) -> Self {
        Self {
            members: members.into_iter().take(4).collect(),
            size: crate::ui::constants::avatar_large(),
        }
    }
}

impl RenderOnce for GroupAvatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let half_size = self.size / 2.0;
        let theme = cx.theme();

        let mut members = self.members.clone();
        while members.len() < 4 {
            members.push("".to_string());
        }

        div()
            .size(self.size)
            .rounded(crate::ui::constants::avatar_small_radius())
            .overflow_hidden()
            .bg(theme.muted)
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_wrap()
                    .child(
                        div().size(half_size).child(
                            Avatar::new()
                                .src(crate::ui::avatar::avatar_for_key(&members[0]))
                                .xsmall(),
                        ),
                    )
                    .child(
                        div().size(half_size).child(
                            Avatar::new()
                                .src(crate::ui::avatar::avatar_for_key(&members[1]))
                                .xsmall(),
                        ),
                    )
                    .child(
                        div().size(half_size).child(
                            Avatar::new()
                                .src(crate::ui::avatar::avatar_for_key(&members[2]))
                                .xsmall(),
                        ),
                    )
                    .child(
                        div().size(half_size).child(
                            Avatar::new()
                                .src(crate::ui::avatar::avatar_for_key(&members[3]))
                                .xsmall(),
                        ),
                    ),
            )
    }
}
