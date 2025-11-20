use gpui::{px, Pixels};
use gpui_component::PixelsExt;

pub fn toolbar_width() -> Pixels {
    px(67.)
}
pub fn title_bar_height() -> Pixels {
    px(67.)
}

pub fn settings_sidebar_width() -> Pixels {
    px(160.)
}
pub fn settings_title_height() -> Pixels {
    // Use the same height as the main window title bar so the settings close button
    // has the same height as the main window close button.
    px(67. / 2.)
}

pub fn window_button_width() -> Pixels {
    px(45.)
}
pub fn settings_close_button_width() -> Pixels {
    px(48.)
}

// Icon sizes
pub fn icon_xs() -> Pixels {
    px(16.)
}
pub fn icon_sm() -> Pixels {
    px(20.)
}
pub fn icon_md() -> Pixels {
    px(21.)
}

pub fn session_list_min_width() -> Pixels {
    px(200.)
}
pub fn session_list_max_width() -> Pixels {
    px(400.)
}

pub fn toolbar_trigger_size() -> Pixels {
    px(41.)
}

pub fn chat_input_default_height() -> Pixels {
    px(200.)
}
pub fn chat_input_min_height() -> Pixels {
    px(120.)
}
pub fn chat_input_max_height() -> Pixels {
    px(420.)
}

pub fn avatar_large() -> Pixels {
    px(46.)
}
pub fn avatar_small() -> Pixels {
    px(35.)
}

pub fn search_plus_button_size() -> Pixels {
    px(28.)
}

pub fn toolbar_popover_width() -> Pixels {
    px(130.)
}

// Toolbar specific paddings
pub fn toolbar_menu_padding_y() -> Pixels {
    px(4.)
}

// Radii
pub fn radius_sm() -> Pixels {
    px(4.)
}
pub fn radius_md() -> Pixels {
    px(6.)
}
pub fn radius_lg() -> Pixels {
    px(8.)
}

// Component-specific tokens
pub fn header_action_padding() -> Pixels {
    px(5.)
}
pub fn header_narrow_button_width() -> Pixels {
    px(15.)
}
pub fn header_narrow_button_height() -> Pixels {
    px(33.)
}

pub fn bubble_max_width() -> Pixels {
    px(300.)
}
pub fn bubble_radius() -> Pixels {
    px(4.)
}

pub fn avatar_small_radius() -> Pixels {
    px(5.)
}

pub fn icon_button_padding() -> Pixels {
    px(6.)
}

// App/window tokens
pub fn title_avatar_size() -> Pixels {
    px(40.)
}
pub fn drag_handle_height() -> Pixels {
    px(4.)
}
pub fn hairline() -> Pixels {
    px(0.7)
}

pub fn about_logo_size() -> Pixels {
    px(80.)
}

pub fn icon_badge_padding_xs() -> Pixels {
    px(1.5)
}

pub fn popover_width_sm() -> Pixels {
    px(100.)
}
pub fn popover_width_md() -> Pixels {
    px(120.)
}

pub fn settings_window_width() -> Pixels {
    px(550.)
}
pub fn settings_window_height() -> Pixels {
    px(680.)
}

pub fn settings_window_content_height() -> Pixels {
    let total = settings_window_height().as_f32();
    let header = settings_title_height().as_f32();
    px(total - header)
}

// Toggle tokens
pub fn toggle_width() -> Pixels {
    px(40.)
}
pub fn toggle_height() -> Pixels {
    px(20.)
}
pub fn toggle_radius() -> Pixels {
    px(10.)
}
pub fn toggle_handle_size() -> Pixels {
    px(16.)
}
pub fn toggle_handle_radius() -> Pixels {
    px(8.)
}
pub fn toggle_padding_x() -> Pixels {
    px(2.)
}

pub fn settings_small_input_width() -> Pixels {
    px(35.)
}

pub fn settings_shortcut_input_min_width() -> Pixels {
    px(80.)
}

pub fn settings_shortcut_input_max_width() -> Pixels {
    px(200.)
}

pub fn app_window_width() -> Pixels {
    px(900.)
}
pub fn app_window_height() -> Pixels {
    px(650.)
}
pub fn app_window_min_width() -> Pixels {
    px(800.)
}
pub fn app_window_min_height() -> Pixels {
    px(600.)
}

/// 独立聊天窗口宽度（约等于主窗口聊天区域宽度）。
pub fn chat_window_width() -> Pixels {
    // 主窗口宽度减去左侧工具栏和会话列表的最小宽度。
    let total = app_window_width().as_f32();
    let left_toolbar = toolbar_width().as_f32();
    let session_min = session_list_min_width().as_f32();
    px(total - left_toolbar - session_min)
}

pub fn toolbar_button_padding_y() -> Pixels {
    px(3.)
}
pub fn toolbar_item_padding() -> Pixels {
    px(10.)
}
