//! # mainwindow.rs
//!
//!

use gpui::*;
use rust_embed::RustEmbed;

use crate::ui::menubar::MenuBar;
use crate::ui::titlebar::TitleBar;

pub struct MainWindow {
    titlebar: Entity<TitleBar>,
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
            .child(self.titlebar.clone())
            .child(self.menubar.clone())
            .child(MainWindow::render_main_content())
    }
}

impl MainWindow {
    fn new(cx: &mut Context<Self>) -> Self {
        let titlebar = cx.new(|_| TitleBar::default());
        let menubar = cx.new(|_| MenuBar::new());
        Self {
            titlebar,
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
        Application::new()
            .with_assets(Assets)
            .run(|cx| {
                eprintln!("GPUI compositer: {}", gpui::guess_compositor());
                MainWindow::register_actions(cx);

                if let Err(e) = cx.open_window(
                    // GNOME wayland や Ghostty など、 xdg-decoration を実装していない環境では、
                    // WindowDecorations::Server (=Default) を使用するとタイトルバーが描画されない
                    // そのため、 WindowDecorations::Client (=独自実装) を選択する
                    WindowOptions {
                        titlebar: None,
                        window_decorations: Some(WindowDecorations::Client),
                        ..Default::default()
                    },
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

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        eprintln!("AssetSource::load: {path}");
        Ok(Self::get(path).map(|file| file.data))
    }

    fn list(&self, prefix: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|asset_path| {
                asset_path.strip_prefix(prefix)
                    .map(|path| SharedString::from(path.to_owned()))
            })
            .collect())
    }
}
