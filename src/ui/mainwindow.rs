//! # mainwindow.rs
//!
//!

use gpui::*;

use crate::ui::menubar;

pub struct MainWindow;

impl Render for MainWindow {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .bg(rgb(0xffffff) /* White */)
            .items_center()
            .flex_col()
            .child(menubar::MenuBar::into_element())
            .child(MainWindow::render_main_content())
    }
}

impl MainWindow {
    pub fn start() {
        Application::new().run(|cx| {
            if let Err(e) = cx.open_window(
                WindowOptions::default(),
                |_, cx| cx.new(|_| Self),
            ) {
                eprintln!("Failed to open window: {}", e);
            }
        });
    }

    fn render_main_content() -> impl IntoElement {
        div()
            .flex()
            .child("Main Content")
    }
}

actions!(doors, [
    ShowAbout,
]);

struct AboutWindow;

impl Render for AboutWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x202020))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .text_color(rgb(0xffffff))
            .child("Doors")
            .child(format!("Version {}", std::env!("CARGO_PKG_VERSION")))
    }
}

impl AboutWindow {
    pub fn show_about(_: &ShowAbout, cx: &mut App) {
        let bounds = Bounds::centered(
            None,
            size(px(400.0), px(250.0)),
            cx);

        if let Err(e) = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Doors - Version")), // TODO: タイトルバーを日本語にすると、Linuxで文字化けする
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AboutWindow),
        ) {
            eprintln!("Failed to show About window: {e}")
        }
    }
}
