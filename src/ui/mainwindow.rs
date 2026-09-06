//! # mainwindow.rs
//!
//!

use gpui::*;

use crate::ui::menubar::MenuBar;

pub struct MainWindow {
    menubar: Entity<MenuBar>,
}

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
            .child(self.menubar.clone())
            .child(MainWindow::render_main_content())
    }
}

impl MainWindow {
    fn new(cx: &mut Context<Self>) -> Self {
        let menubar = cx.new(|_| MenuBar::new());
        Self {
            menubar,
        }
    }

    /// # Register actions to cx
    ///
    /// MainWindowに必要なActionと、その他のUIで必要なActionをまとめて登録する
    fn register_actions(cx: &mut App) {
        MenuBar::register_actions(cx);
    }

    pub fn start() {
        Application::new().run(|cx| {
            MainWindow::register_actions(cx);

            if let Err(e) = cx.open_window(
                WindowOptions::default(),
                |_, cx| cx.new(|cx| Self::new(cx)),
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
