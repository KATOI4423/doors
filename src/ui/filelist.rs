//! filelist.rs
//!
//! Show the file list with those information

use std::collections::VecDeque;
use std::path::PathBuf;

use gpui_component::input::{
    Input,
    InputEvent,
};
use gpui_kit::base::input::InputState;
use gpui_kit::*;

pub struct FileList {
    current: PathBuf,
    back: VecDeque<PathBuf>,
    forward: VecDeque<PathBuf>,
    path_input: Entity<InputState>,
}

impl Render for FileList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h_full()
            .w_full()
            .rounded_b_lg()
            .bg(rgb(0x202020))
            .child(self.render_toolbar(window, cx))
    }
}

impl FileList {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let path_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(path.to_string_lossy())
        });

        let filelist = Self {
            current: path,
            back: VecDeque::default(), // TODO: 履歴復元機能を追加
            forward: VecDeque::default(), // TODO: 履歴復元機能を追加
            path_input,
        };

        filelist.create_subscribe(window, cx);

        filelist
    }

    fn create_subscribe(&self, window: &mut Window, cx: &mut Context<Self>) {
        // path_input のイベント処理
        cx.subscribe_in(&self.path_input, window, |this, _state, event, window, cx| {
            match event {
                InputEvent::PressEnter { secondary: _, shift: _ } => {
                    let value = this.path_input.read(cx).value();
                    let path = PathBuf::from(value.to_string());

                    // TODO: パス移動処理 (移動先があるかどうかをチェック？)
                    dbg!(&path);
                    Self::update_stack(&mut this.back, this.current.clone());
                    this.set_current_path(window, cx, path);
                    this.forward.clear();
                },
                _ => {},
            }
        }).detach();
    }

    fn render_toolbar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .h_8()
            .w_full()
            .justify_start()
            .items_start()
            .child(Self::render_back_button(window, cx))
            .child(Self::render_front_button(window, cx))
            .child(Self::render_parent_button(window, cx))
            .child(Self::render_renew_button(window, cx))
            .child(self.render_path_input(window, cx))
    }

    /// # Update Stack
    ///
    /// ## Args:
    ///  - stack: 更新対象の VecDeque
    ///  - element: stack に入れる要素
    ///
    /// ## Notes:
    ///  - stack の要素数が MAX_HISTORY を超えないように、stack の要素を削除して自動調整する
    fn update_stack(stack: &mut VecDeque<PathBuf>, element: PathBuf) {
        const MAX_HISTORY: usize = 10;
        if stack.len() >= MAX_HISTORY {
            stack.pop_front();
        }
        stack.push_back(element);
    }

    fn set_current_path(&mut self, window: &mut Window, cx: &mut Context<Self>, current: impl Into<PathBuf>) {
        self.current = current.into();
        self.path_input.update(cx, |input, cx| {
            input.set_value(self.current.to_string_lossy(), window, cx);
        })
    }

    fn render_button(
        id: impl Into<ElementId>,
        icon_path: impl Into<SharedString>,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Stateful<Div> {
        div()
            .id(id)
            .size(px(32.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                svg()
                    .path(icon_path)
                    .size(px(16.0))
                    .text_color(rgb(0xffffff))
            )
            .on_click(listener)
    }

    fn render_back_button(_window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("back", "icons/chevron-left.svg", cx.listener(|this, _event, window, cx| {
            let Some(back) = this.back.pop_back() else {
                eprintln!("back is empty");
                return;
            };

            dbg!(&back);
            Self::update_stack(&mut this.forward, this.current.clone());
            this.set_current_path(window, cx, back);
        }))
    }

    fn render_front_button(_window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("front", "icons/chevron-right.svg", cx.listener(|this, _event, window, cx| {
            let Some(forward) = this.forward.pop_back() else {
                eprintln!("forward is empty");
                return;
            };

            dbg!(&forward);
            Self::update_stack(&mut this.back, this.current.clone());
            this.set_current_path(window, cx, forward);
        }))
    }

    /// # Render Parrent Button
    ///
    /// Note: forward の履歴は削除する
    fn render_parent_button(_window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("parent", "icons/allow-up.svg", cx.listener(|this, _event, window, cx| {
            let Some(parent) = this.current.parent() else {
                eprintln!("there are no parent directory");
                return;
            };

            dbg!(&parent);
            this.forward.clear();
            Self::update_stack(&mut this.back, this.current.clone());
            this.set_current_path(window, cx, parent.to_owned());
        }))
    }

    fn render_renew_button(_window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_button("renew", "icons/rotate.svg", cx.listener(|this, _, _, _| {
            eprintln!("Renew!");
        }))
    }

    fn render_path_input(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Input::new(&self.path_input)
            .appearance(true)
            .bg(rgb(0xe0e0e0))
            .w_full()
    }


}
