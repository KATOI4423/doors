//! filelist.rs
//!
//! Show the file list with those information

use std::collections::VecDeque;
use std::path::{
    Path,
    PathBuf,
};

use gpui_component::{
    IndexPath,
    h_flex,
};
use gpui_component::input::{
    Input,
    InputEvent,
    InputState,
};
use gpui_component::label::Label;
use gpui_component::list::{
    ListDelegate,
    ListState,
    ListItem,
};
use gpui_kit::*;
use strum::Display;

use crate::api::file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Display)]
enum FileListColumn {
    #[strum(to_string="")] // ヘッダーに Icon の文字列は含めない
    Icon,
    Name,
    Size,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileListColumnState {
    kind: FileListColumn,
    width: Pixels,
}

#[derive(Debug, Clone)]
struct FileListColumns {
    states: Vec<FileListColumnState>,
}

impl FromIterator<FileListColumnState> for FileListColumns {
    fn from_iter<T: IntoIterator<Item = FileListColumnState>>(iter: T) -> Self {
        Self {
            states: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for FileListColumns {
    type Item = FileListColumnState;
    type IntoIter = std::vec::IntoIter<FileListColumnState>;

    fn into_iter(self) -> Self::IntoIter {
        self.states.into_iter()
    }
}

impl Default for FileListColumns {
    fn default() -> Self {
        FileListColumns::from_iter(vec![
            FileListColumnState { kind: FileListColumn::Icon, width: px(32.0) },
            FileListColumnState { kind: FileListColumn::Name, width: px(568.0) },
            FileListColumnState { kind: FileListColumn::Size, width: px(200.0) },
        ])
    }
}

impl FileListColumns {
    pub fn iter(&self) -> impl Iterator<Item = &FileListColumnState> + '_ {
        self.states.iter()
    }
}


pub struct FileList {
    current: PathBuf,
    back: VecDeque<PathBuf>,
    forward: VecDeque<PathBuf>,
    path_input: Entity<InputState>,

    columns: Entity<FileListColumns>,
    list: Entity<ListState<FileListDelegate>>,
}

impl Render for FileList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h_full()
            .w_full()
            .flex()
            .flex_col()
            .rounded_b_lg()
            .bg(rgb(0x202020))
            .child(self.render_toolbar(window, cx))
            .child(self.render_header(window, cx))
            .child(self.list.clone())
    }
}

impl FileList {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let path_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(path.to_string_lossy())
        });

        let columns = cx.new(|_cx| {
            FileListColumns::default()
        });

        let delegate = FileListDelegate::new(&path, columns.clone());
        let list = cx.new(|cx| {
            gpui_component::list::ListState::new(delegate, window, cx)
        });

        let filelist = Self {
            current: path,
            back: VecDeque::default(), // TODO: 履歴復元機能を追加
            forward: VecDeque::default(), // TODO: 履歴復元機能を追加
            path_input,

            columns,
            list,
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
        div()
            .w_full()
            .child(
                Input::new(&self.path_input)
                    .appearance(true)
                    .focus_bordered(true)
                    .bg(rgb(0xe0e0e0))
            )
            .on_mouse_down_out(|_, window, cx| {
                window.blur(cx);
            })
    }

    fn render_header(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut div = div()
            .h_6()
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .bg(rgb(0x181818))
            .text_color(rgb(0xffffff));

        for col in self.columns.read(cx).iter() {
            div = div
                .child(
                    gpui_kit::div()
                        .h_full()
                        .w(col.width - px(1.0))
                        .flex_shrink_0()
                        .px_3()
                        .child(col.kind.to_string())
                ).child(
                    gpui_kit::div()
                        .h_full()
                        .w(px(1.0))
                        .flex_shrink_0()
                        .bg(rgb(0x444444))
                );
        }

        div
    }
}

#[derive(Debug, Clone)]
pub struct FileListDelegate {
    columns: Entity<FileListColumns>,
    entries: Vec<file::FileEntry>,
    selected: Option<IndexPath>,
}

impl FileListDelegate {
    pub fn new(path: &Path, columns: Entity<FileListColumns>) -> Self {
        let entries = match file::FileEntry::read_directory(path) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Failed to get file entries from {}: {e}", path.display());
                return Self {
                    columns,
                    entries: Vec::default(),
                    selected: Option::default(),
                };
            }
        };

        Self {
            columns,
            entries,
            selected: None,
        }
    }
}

impl ListDelegate for FileListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.entries.len()
    }

    fn render_item(
        &mut self,
        ix: base::IndexPath,
        _window: &mut Window,
        cx: &mut Context<gpui_component::list::ListState<Self>>,
    ) -> Option<Self::Item>
    {
        const ROW_HEIGHT: Pixels = px(21.0);
        let columns = self.columns.read(cx);
        let icon_col = columns.iter().find(|elm| {
            elm.kind == FileListColumn::Icon
        })?;
        let name_col = columns.iter().find(|elm| {
            elm.kind == FileListColumn::Name
        })?;
        let size_col = columns.iter().find(|elm| {
            elm.kind == FileListColumn::Size
        })?;

        self.entries.get(ix.row).map(|entry| {
            Self::Item::new(ix)
                .p_0()
                .border_b_1()
                .border_color(rgb(0x444444))
                .child(
                    h_flex()
                        .w_full()
                        .flex_shrink_0()
                        .child(
                            div()
                                .flex()
                                .flex_shrink_0()
                                .h(ROW_HEIGHT)
                                .w(icon_col.width)
                                .border_r_1()
                                .border_color(rgb(0x444444))
                                .items_center()
                                .justify_center()
                                .child(
                                    svg()
                                        .path(entry.kind().icon_path().to_string_lossy())
                                        .size(px(16.0))
                                        .text_color(rgb(0xffffff))
                                )
                        )
                        .child(
                            div()
                                .flex()
                                .flex_shrink_0()
                                .h(ROW_HEIGHT)
                                .w(name_col.width)
                                .border_r_1()
                                .border_color(rgb(0x444444))
                                .items_center()
                                .px_3()
                                .child(
                                    Label::new(entry.name())
                                        .text_color(rgb(0xffffff))
                                )
                        )
                        .child(
                            div()
                                .flex()
                                .flex_shrink_0()
                                .h(ROW_HEIGHT)
                                .w(size_col.width)
                                .border_r_1()
                                .border_color(rgb(0x444444))
                                .items_center()
                                .px_3()
                                .child(
                                    Label::new(entry.kind().size())
                                        .text_color(rgb(0xffffff))
                                )
                        )
                ).selected(Some(ix) == self.selected)
        })
    }

    fn set_selected_index(
        &mut self,
        ix: Option<base::IndexPath>,
        window: &mut Window,
        cx: &mut Context<gpui_component::list::ListState<Self>>,
    )
    {
        self.selected = ix;
        cx.notify();
    }
}
