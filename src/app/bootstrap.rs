use gpui::{App, AppContext, Bounds, Size, WindowBounds, WindowKind, WindowOptions};
use gpui_component::{Root, TitleBar};

use crate::app::actions::{SelectSession, ToolbarClicked};
use crate::app::state::WeixinApp;
use crate::components::SettingsWindow;

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

        // 在 App 级别路由 SelectSession / ToolbarClicked 动作到 WeixinApp 实例，
        // 确保通过 window.dispatch_action 触发的动作能够被根 Workspace 处理。
        {
            let app_view_for_select = app_view.clone();
            cx.on_action(move |action: &SelectSession, cx_app: &mut App| {
                let _ = app_view_for_select.update(cx_app, |app, cx_weixin| {
                    app.on_action_select_session(action, cx_weixin);
                });
            });
        }

        {
            let app_view_for_toolbar = app_view.clone();
            cx.on_action(move |action: &ToolbarClicked, cx_app: &mut App| {
                let _ = app_view_for_toolbar.update(cx_app, |app, cx_weixin| {
                    app.on_action_toolbar_clicked(action, cx_weixin);
                });
            });
        }

        cx.new(|cx| Root::new(app_view.into(), window, cx))
    })
    .expect("failed to open main window");
}

/// 打开设置窗口，并确保同一时间只打开一个实例。
pub fn open_settings_window(cx: &mut App) {
    use crate::components::settings::window::SETTINGS_WINDOW_OPEN;
    use std::sync::atomic::Ordering;

    if SETTINGS_WINDOW_OPEN.load(Ordering::SeqCst) {
        return;
    }

    SETTINGS_WINDOW_OPEN.store(true, Ordering::SeqCst);

    let window_size = Size {
        width: crate::ui::constants::settings_window_width(),
        height: crate::ui::constants::settings_window_height(),
    };

    let window_bounds = Bounds::centered(None, window_size, cx);

    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(window_bounds)),
        titlebar: Some(TitleBar::title_bar_options()),
        window_min_size: Some(Size {
            width: crate::ui::constants::settings_window_width(),
            height: crate::ui::constants::settings_window_height(),
        }),
        window_decorations: Some(gpui::WindowDecorations::Server),
        kind: WindowKind::Normal,
        ..Default::default()
    };

    cx.open_window(options, |window, cx| {
        let settings_view = SettingsWindow::view(window, cx);
        cx.new(|cx| Root::new(settings_view.into(), window, cx))
    })
    .ok();
}
