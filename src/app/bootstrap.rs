use gpui::{App, AppContext, Bounds, Size, WindowBounds, WindowKind, WindowOptions};
use gpui_component::{Root, TitleBar};

use crate::app::state::WeixinApp;

/// 应用级初始化：注册组件库、主题等。
pub fn init_app(cx: &mut App) {
    // 初始化 gpui-component（类似 story::init）
    gpui_component::init(cx);

    // 应用微信主题的光标颜色
    let colors = crate::ui::theme::WeixinThemeColors::light();
    gpui_component::Theme::global_mut(cx).caret = colors.caret;

    cx.activate(true);
}

/// 打开主窗口并挂载 WeixinApp 作为根视图。
pub fn open_main_window(cx: &mut App) {
    let window_size = Size {
        width: crate::ui::constants::app_window_width(),
        height: crate::ui::constants::app_window_height(),
    };

    let window_bounds = Bounds::centered(None, window_size, cx);

    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(window_bounds)),
        titlebar: Some(TitleBar::title_bar_options()),
        window_min_size: Some(Size {
            width: crate::ui::constants::app_window_min_width(),
            height: crate::ui::constants::app_window_min_height(),
        }),
        kind: WindowKind::Normal,
        ..Default::default()
    };

    cx.open_window(options, |window, cx| {
        let app_view = WeixinApp::view(window, cx);
        cx.new(|cx| Root::new(app_view.into(), window, cx))
    })
    .expect("failed to open main window");
}
