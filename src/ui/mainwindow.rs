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
            .items_center()
            .justify_center()
            .child("Hello, Doors!")
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
}
