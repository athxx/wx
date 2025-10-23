use crate::app::state::WeixinApp;
use crate::components::settings_window::SettingsWindow;
use crate::components::{SessionList, ToolBar};
use crate::domain::events::{OpenSettingsEvent, SessionSelectEvent, ToolbarClickEvent};
use gpui::{
    px, App, AppContext, Bounds, Context, Entity, Size, Window, WindowBounds, WindowKind,
    WindowOptions,
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
        cx.subscribe(
            toolbar,
            |_this, _toolbar, event: &ToolbarClickEvent, _cx| {
                println!("Toolbar item clicked: {:?}", event.item);
            },
        )
        .detach();

        // 订阅会话选择事件
        cx.subscribe(
            session_list,
            |this, _list, event: &SessionSelectEvent, cx| {
                this.on_session_selected(&event.contact_id, cx);
            },
        )
        .detach();

        // 订阅工具栏设置事件
        cx.subscribe(
            toolbar,
            |_this, _toolbar, _event: &OpenSettingsEvent, cx| {
                Self::open_settings_window(cx);
            },
        )
        .detach();
    }

    /// 打开设置窗口
    pub fn open_settings_window(cx: &mut App) {
        let window_size = Size {
            width: px(550.0),
            height: px(680.0),
        };

        let window_bounds = Bounds::centered(None, window_size, cx);

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(Size {
                width: px(550.),
                height: px(680.),
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
