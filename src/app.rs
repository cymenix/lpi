use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, MouseEventKind,
};
use ratatui::backend::Backend;
use ratatui::layout::Position;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Scrollbar, ScrollbarOrientation};
use ratatui::{Frame, Terminal};
use std::time::{Duration, Instant};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::moon::{Moon, Project, Task};

#[must_use]
pub(crate) struct App {
    state: TreeState<&'static str>,
    items: Vec<TreeItem<'static, &'static str>>,
}

impl From<Moon> for App {
    fn from(value: Moon) -> Self {
        let items: Vec<TreeItem<'static, &'static str>> = value
            .projects
            .iter()
            .map(|p| TreeItem::from(p.clone()))
            .collect();

        Self {
            state: TreeState::default(),
            items,
        }
    }
}

impl From<Project> for TreeItem<'static, &'static str> {
    fn from(value: Project) -> Self {
        let children: Vec<TreeItem<'static, &'static str>> = value
            .tasks
            .iter()
            .map(|t| TreeItem::from(t.clone()))
            .collect();
        let project = Box::leak(value.project.clone().into_boxed_str());
        Self::new(project, value.project.clone(), children).unwrap()
    }
}

impl From<Task> for TreeItem<'static, &'static str> {
    fn from(value: Task) -> Self {
        let task = Box::leak(value.command.clone().into_boxed_str());
        Self::new(task, value.task.clone(), vec![]).unwrap()
    }
}

impl App {
    pub fn new() -> Self {
        Self::from(Moon::generate())
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.size();
        let widget = Tree::new(&self.items)
            .expect("all item identifiers are unique")
            .block(Block::bordered().title("Workspace projects"))
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .track_symbol(None)
                    .end_symbol(None),
            ))
            .highlight_style(
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        frame.render_stateful_widget(widget, area, &mut self.state);
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> std::io::Result<Option<String>> {
        const DEBOUNCE: Duration = Duration::from_millis(16); // 60 FPS

        terminal.draw(|frame| self.draw(frame))?;

        let mut debounce: Option<Instant> = None;

        loop {
            let timeout = debounce.map_or(DEBOUNCE, |start| {
                DEBOUNCE.saturating_sub(start.elapsed())
            });
            if crossterm::event::poll(timeout)? {
                let update = match crossterm::event::read()?{
                    Event::Key(KeyEvent {
                        code,
                        kind: KeyEventKind::Press,
                        ..
                    }) => match code {
                        KeyCode::Char('q') => return Ok(None),
                        KeyCode::Char('\n' | ' ') => {
                            self.state.toggle_selected()
                        }
                        KeyCode::Left => self.state.key_left(),
                        KeyCode::Char('h') => self.state.key_left(),
                        KeyCode::Right => self.state.key_right(),
                        KeyCode::Char('l') => self.state.key_right(),
                        KeyCode::Down => self.state.key_down(),
                        KeyCode::Char('j') => self.state.key_down(),
                        KeyCode::Up => self.state.key_up(),
                        KeyCode::Char('k') => self.state.key_up(),
                        KeyCode::Esc => self.state.select(Vec::new()),
                        KeyCode::Home => self.state.select_first(),
                        KeyCode::End => self.state.select_last(),
                        KeyCode::PageDown => self.state.scroll_down(3),
                        KeyCode::PageUp => self.state.scroll_up(3),
                        KeyCode::Enter => {
                            self.state.key_right();
                            let selected = self.state.selected();
                            if !selected.is_empty() {
                                return Ok(Some(
                                    selected.last().unwrap().to_string(),
                                ));
                            }
                            false
                        }
                        _ => false,
                    },
                    Event::Mouse(mouse) => match mouse.kind {
                        MouseEventKind::ScrollDown => self.state.scroll_down(1),
                        MouseEventKind::ScrollUp => self.state.scroll_up(1),
                        MouseEventKind::Down(_button) => self
                            .state
                            .click_at(Position::new(mouse.column, mouse.row)),
                        _ => false,
                    },
                    Event::Resize(_, _) => true,
                    _ => false,
                };
                if update {
                    debounce.get_or_insert_with(Instant::now);
                }
            }
            if debounce.is_some_and(|debounce| debounce.elapsed() > DEBOUNCE) {
                terminal.draw(|frame| {
                    self.draw(frame);
                })?;

                debounce = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_tree_item_from_task() {
        let task = Task::new("p".to_string(), ":check | biome".to_string());
        let tree_item = TreeItem::from(task);
        dbg!(&tree_item);
        assert_eq!(tree_item.identifier().to_string().as_str(), "p:check")
    }

    #[test]
    fn should_create_task() {
        let task = Task::new("p".to_string(), ":check | biome".to_string());
        assert_eq!(task.command, "p:check")
    }
}
