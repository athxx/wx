use gpui::{AnyElement, App, AvailableSpace, Pixels, Size, Window};

/// 测量一组竖直排列的元素高度，用于构建虚拟列表的 `item_sizes`。
///
/// - `items` 应该与实际渲染时的结构一致（例如外层再包一层 `div().w_full()`），
///   这样测量结果才会准确。
/// - `available_space` 用于控制测量时的宽度约束：
///   - 会话列表可以使用 `size(AvailableSpace::MinContent, AvailableSpace::MinContent)`；
///   - 聊天气泡可以使用固定宽度 + `AvailableSpace::MinContent` 高度，以获得正确的换行高度。
#[allow(dead_code)]
pub fn measure_vertical_items(
    items: Vec<AnyElement>,
    available_space: gpui::Size<AvailableSpace>,
    window: &mut Window,
    cx: &mut App,
) -> Vec<Size<Pixels>> {
    items
        .into_iter()
        .map(|mut el| el.layout_as_root(available_space, window, cx))
        .collect()
}
