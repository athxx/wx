use gpui::{div, px, App, IntoElement, ParentElement, Pixels, RenderOnce, Styled, Window};
use gpui_component::{avatar::Avatar, ActiveTheme, Sizable};

/// 群组头像组件 - 显示2x2网格的成员头像
#[derive(IntoElement)]
pub struct GroupAvatar {
    members: Vec<String>,
    size: Pixels,
}

impl GroupAvatar {
    pub fn new(members: Vec<String>) -> Self {
        Self {
            members: members.into_iter().take(4).collect(),
            size: px(43.),
        }
    }

    pub fn with_size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for GroupAvatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let half_size = self.size / 2.0;
        let avatar_size = half_size - px(1.); // 留出1px间隙
        let theme = cx.theme();

        // 确保至少有4个成员（不足的用空字符串填充）
        let mut members = self.members.clone();
        while members.len() < 4 {
            members.push("".to_string());
        }

        div()
            .size(self.size)
            .rounded(px(4.))
            .overflow_hidden()
            .bg(theme.muted)
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_wrap()
                    .child(
                        // 左上
                        div()
                            .size(half_size)
                            .child(Avatar::new().name(&members[0]).xsmall()),
                    )
                    .child(
                        // 右上
                        div()
                            .size(half_size)
                            .child(Avatar::new().name(&members[1]).xsmall()),
                    )
                    .child(
                        // 左下
                        div()
                            .size(half_size)
                            .child(Avatar::new().name(&members[2]).xsmall()),
                    )
                    .child(
                        // 右下
                        div()
                            .size(half_size)
                            .child(Avatar::new().name(&members[3]).xsmall()),
                    ),
            )
    }
}
