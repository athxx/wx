use gpui::{px, App, AppContext, Bounds, Size, WindowBounds, WindowKind, WindowOptions};
use gpui_component::{Root, TitleBar};

use crate::app::actions::{OpenChatWindow, SelectSession, ToolbarClicked};
use crate::app::state::GlobalMainApp;
use crate::app::state::{Preferences, WeixinApp};
use crate::components::{ChatWindow, SettingsWindow};
use crate::ui::theme::{Theme, ThemeMode};
/// 应用级初始化：注册组件库、主题等。
pub fn init_app(cx: &mut App) {
    // 初始化 gpui-component（类似 story::init）
    gpui_component::init(cx);

    // 从偏好中恢复主题模式和字体大小
    let prefs = Preferences::load();

    match prefs.theme_mode {
        ThemeMode::Light => Theme::set_light(cx),
        ThemeMode::Dark => Theme::set_dark(cx),
    }

    // 应用字体大小
    gpui_component::Theme::global_mut(cx).font_size = px(prefs.font_size);

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
        cx.set_global(GlobalMainApp(app_view.clone()));
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

        // 双击会话时打开独立聊天窗口。
        cx.on_action(|action: &OpenChatWindow, cx_app: &mut App| {
            // 这里不依赖 WeixinApp 的内部状态，只根据 contact_id 打开一个新的聊天窗口。
            open_chat_window(action.contact_id.clone(), cx_app);
        });

        cx.new(|cx| Root::new(app_view, window, cx))
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
        is_resizable: false,
        kind: WindowKind::Normal,
        ..Default::default()
    };

    cx.open_window(options, |window, cx| {
        let settings_view = SettingsWindow::view(window, cx);
        cx.new(|cx| Root::new(settings_view, window, cx))
    })
    .ok();
}

/// 打开独立聊天窗口，内容与主窗口右侧聊天区域类似。
pub fn open_chat_window(contact_id: String, cx: &mut App) {
    let window_size = Size {
        width: crate::ui::constants::chat_window_width(),
        height: crate::ui::constants::app_window_height(),
    };

    let window_bounds = Bounds::centered(None, window_size, cx);

    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(window_bounds)),
        titlebar: Some(TitleBar::title_bar_options()),
        window_min_size: Some(Size {
            width: crate::ui::constants::chat_window_width(),
            height: crate::ui::constants::app_window_min_height(),
        }),
        kind: WindowKind::Normal,
        ..Default::default()
    };

    // 如果该会话窗口已经存在，则不再打开新的窗口。
    if !ChatWindow::try_reserve(&contact_id) {
        return;
    }

    cx.open_window(options, move |window, cx| {
        let chat_view = ChatWindow::view(window, cx, contact_id.clone());
        cx.new(|cx| Root::new(chat_view, window, cx))
    })
    .ok();
}
