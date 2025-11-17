use gpui::InteractiveElement as _;
use gpui::{
    px, App, AppContext as _, ElementId, Entity, EventEmitter, IntoElement, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement as _, Pixels, RenderOnce, Styled,
    Window,
};
use gpui_component::ActiveTheme as _;
use std::ops::Range;

const HANDLE_WIDTH: Pixels = px(6.);

/// 分隔状态的事件，目前仅在松开鼠标时发送一次 Resized 用于持久化。
#[derive(Clone, Debug)]
pub enum FixedResizableEvent {
    Resized,
}

/// 固定像素宽度的可拖动分隔状态
///
/// - `left_width` 以像素为单位，记录左侧区域宽度
/// - `dragging` 表示当前是否正在拖动分隔条
/// - `drag_start_x` 按下时鼠标的 x 坐标（窗口坐标）
/// - `drag_start_width` 按下时的左侧宽度
#[derive(Clone)]
pub struct FixedResizableState {
    pub left_width: Pixels,
    pub dragging: bool,
    pub drag_start_x: Pixels,
    pub drag_start_width: Pixels,
}

impl FixedResizableState {
    /// 默认使用 200px，和 session_list_min_width 一致
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self {
            left_width: px(200.),
            dragging: false,
            drag_start_x: px(0.),
            drag_start_width: px(200.),
        })
    }
}

impl EventEmitter<FixedResizableEvent> for FixedResizableState {}

/// 创建一个水平分隔：左固定宽度，右自适应
pub fn fixed_h_resizable(
    id: impl Into<ElementId>,
    state: Entity<FixedResizableState>,
) -> FixedResizableGroup {
    FixedResizableGroup {
        id: id.into(),
        state,
        left_child: None,
        right_child: None,
        min_width: px(200.),
        max_width: px(400.),
    }
}

#[derive(IntoElement)]
pub struct FixedResizableGroup {
    id: ElementId,
    state: Entity<FixedResizableState>,
    left_child: Option<gpui::AnyElement>,
    right_child: Option<gpui::AnyElement>,
    min_width: Pixels,
    max_width: Pixels,
}

impl FixedResizableGroup {
    /// 限制左侧宽度范围
    pub fn width_range(mut self, range: Range<Pixels>) -> Self {
        self.min_width = range.start;
        self.max_width = range.end;
        self
    }

    /// 设置左侧内容
    pub fn left(mut self, child: impl IntoElement) -> Self {
        self.left_child = Some(child.into_any_element());
        self
    }

    /// 设置右侧内容
    pub fn right(mut self, child: impl IntoElement) -> Self {
        self.right_child = Some(child.into_any_element());
        self
    }
}

impl RenderOnce for FixedResizableGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        // 使用 theme.border 作为分割线颜色，使左右分割线与聊天区域上下分割线保持一致（约 d5d5d5）
        let handle_color = theme.border;
        let state = self.state.clone();
        let left_width = state.read(cx).left_width;
        let min_width = self.min_width;
        let max_width = self.max_width;

        // 容器相对布局，方便用绝对定位画出分隔条
        gpui::div()
            .id(self.id)
            .flex()
            .flex_row()
            .flex_1()
            .relative()
            .size_full()
            // 左侧固定宽度区域（右侧加一条可见边框，方便看见分割线）
            .child(
                gpui::div()
                    .w(left_width)
                    .h_full()
                    .border_r_1()
                    .border_color(handle_color)
                    .children(self.left_child),
            )
            // 中间分隔条：只负责显示和开始拖动（记录起始位置），具体宽度更新在容器的 on_mouse_move 中完成
            .child({
                let state = state.clone();
                gpui::div()
                    .absolute()
                    // 分隔条的左边正好在会话列表和聊天区域的边界上
                    .left(left_width)
                    .top_0()
                    .w(HANDLE_WIDTH)
                    .h_full()
                    .cursor_col_resize()
                    .on_mouse_down(
                        MouseButton::Left,
                        move |e: &MouseDownEvent, _window: &mut Window, cx: &mut App| {
                            state.update(cx, |s, _cx| {
                                s.dragging = true;
                                s.drag_start_x = e.position.x;
                                s.drag_start_width = s.left_width;
                            });
                            // 阻止事件继续向下传播，避免点击/拖动透传到下面的内容区域或窗口拖动区域
                            cx.stop_propagation();
                        },
                    )
                    // 拖动区域本身透明，只提供命中范围；真正的 1px 竖线由左侧容器的 border_r_1 提供
                    .child(gpui::div().w(HANDLE_WIDTH).h_full())
            })
            // 右侧自适应区域：使用 flex 列布局并占满可用空间，
            // 让 ChatArea 内部的 v_flex().flex_1() 能按预期分配高度。
            .child(
                gpui::div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .size_full()
                    .children(self.right_child),
            )
            // 容器级别的鼠标事件：根据拖动更新 left_width，并在松开时结束拖动
            .on_mouse_move({
                let state = state.clone();
                move |e: &MouseMoveEvent, _window: &mut Window, cx: &mut App| {
                    state.update(cx, |s, cx| {
                        if s.dragging {
                            let dx = e.position.x - s.drag_start_x;
                            let new_width = (s.drag_start_width + dx).clamp(min_width, max_width);
                            s.left_width = new_width;
                            cx.notify();
                        }
                    });
                    // 注意：这里不阻止事件传播，避免影响标题栏里的窗口控制按钮（最小化/最大化/关闭）。
                }
            })
            .on_mouse_up(MouseButton::Left, {
                let state = state.clone();
                move |_e: &MouseUpEvent, _window: &mut Window, cx: &mut App| {
                    state.update(cx, |s, cx| {
                        s.dragging = false;
                        // 通知监听者当前分隔状态已更新，可用于持久化。
                        cx.emit(FixedResizableEvent::Resized);
                    });
                    // 同样不在这里拦截事件，让 WindowControlArea 按钮能正常处理鼠标抬起。
                }
            })
    }
}

// 不再需要单独的 listener 元素
