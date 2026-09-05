//! # mainwindow.rs
//!
//!

use gpui::*;

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
            .child(MainWindow::render_menu_bar())
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

    fn render_menu_bar() -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .h(px(28.0))
            .child("Menu Bar")
    }

    fn render_main_content() -> impl IntoElement {
        div()
            .flex()
            .child("Main Content")
    }
}
