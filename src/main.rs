mod app;
mod assets;
mod components;
mod data;
mod models;
mod theme;

use app::WeixinApp;
use assets::Assets;
use theme::Theme;

use gpui::{
    px, App, AppContext, Application, Bounds, Size, Window, WindowBounds, WindowKind, WindowOptions,
};
use gpui_component::{Root, TitleBar};

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        // 初始化默认主题（亮色）
        Theme::set(Theme::light(), cx);

        cx.activate(true);

        let window_size = Size {
            width: px(900.0),
            height: px(650.0),
        };

        let window_bounds = Bounds::centered(None, window_size, cx);

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            titlebar: Some(TitleBar::title_bar_options()),
            window_min_size: Some(Size {
                width: px(800.),
                height: px(600.),
            }),
            kind: WindowKind::Normal,
            ..Default::default()
        };

        cx.open_window(options, |window, cx| {
            let app_view = WeixinApp::view(window, cx);
            cx.new(|cx| Root::new(app_view.into(), window, cx))
        })
        .expect("failed to open window");
    });
}
