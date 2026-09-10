//! # mainwindow.rs
//!
//!

use gpui::*;
use rust_embed::RustEmbed;

use crate::ui::titlebar::TitleBar;

pub struct MainWindow {
    titlebar: Entity<TitleBar>,
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
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                this.titlebar.update(cx, |titlebar, cx| {
                    titlebar.close_setting_pulldown(cx);
                });
            }))
            .child(self.titlebar.clone())
            .child(MainWindow::render_main_content())
    }
}

impl MainWindow {
    fn new(cx: &mut Context<Self>) -> Self {
        let titlebar = cx.new(|_| TitleBar::default());
        Self {
            titlebar,
        }
    }

    /// # Register actions to cx
    ///
    /// MainWindowに必要なActionと、その他のUIで必要なActionをまとめて登録する
    fn register_actions(cx: &mut App) {
        TitleBar::register_actions(cx);
    }

    pub fn start() {
        Application::new()
            .with_assets(Assets)
            .run(|cx| {
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
