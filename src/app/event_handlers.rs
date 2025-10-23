use crate::app::events::AppEvent;
use crate::app::state::WeixinApp;
use crate::components::SettingsWindow;
use crate::components::{SessionList, ToolBar};
use gpui::{
    App, AppContext, Bounds, Context, Entity, Size, WindowBounds, WindowKind, WindowOptions,
};
use gpui_component::{Root, TitleBar};

impl WeixinApp {
    /// 设置事件订阅
    pub(super) fn setup_event_subscriptions(
        toolbar: &Entity<ToolBar>,
        session_list: &Entity<SessionList>,
        cx: &mut Context<Self>,
    ) {
        // 订阅工具栏点击事件
        cx.subscribe(toolbar, |_this, _toolbar, event: &AppEvent, _cx| {
            if let AppEvent::ToolbarClicked { item } = event {
                println!("Toolbar item clicked: {:?}", item);
            }
        })
        .detach();

        // 订阅会话选择事件
        cx.subscribe(session_list, |this, _list, event: &AppEvent, cx| {
            if let AppEvent::SessionSelected { contact_id } = event {
                this.on_session_selected(contact_id, cx);
            }
        })
        .detach();
    }

    /// 打开设置窗口
    pub fn open_settings_window(cx: &mut App) {
        use crate::components::settings::window::SETTINGS_WINDOW_OPEN;
        use std::sync::atomic::Ordering;

        // 若已打开则直接返回
        if SETTINGS_WINDOW_OPEN.load(Ordering::SeqCst) {
            return;
        }
        // 标记为已打开（在 SettingsWindow Drop 时会复位）
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
}
