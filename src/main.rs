pub mod app;
mod assets;
pub mod components;
mod models;
mod infra;
mod ui;

use app::WeixinApp;
use assets::Assets;

use gpui::{
    px, AppContext, Application, Bounds, Size, WindowBounds, WindowKind, WindowOptions,
};
use gpui_component::{Root, TitleBar};

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.activate(true);

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
        .expect("failed to open window");
    });
}
