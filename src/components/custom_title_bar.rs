use gpui::{
    div, px, App, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled, Window,
    WindowControlArea,
};
use gpui_component::{h_flex, ActiveTheme, Icon, IconName, Sizable};

/// 自定义标题栏 - 高度64px，无边框，带系统窗口控制
#[derive(IntoElement)]
pub struct CustomTitleBar {
    child: Option<gpui::AnyElement>,
}

impl CustomTitleBar {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for CustomTitleBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_maximized = window.is_maximized();
        div()
            .id("custom-title-bar")
            .flex_shrink_0()
            .h(px(64.))
            .flex()
            .items_center()
            .justify_between()
            .bg(cx.theme().title_bar)
            .child(
                // 内容区域（可拖动）
                div()
                    // .flex_1()
                    .h_full()
                    .flex()
                    .items_center()
                    .window_control_area(WindowControlArea::Drag)
                    .children(self.child),
            )
    }
}
