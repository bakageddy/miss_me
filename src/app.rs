use miss_me::Index;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    symbols::border,
    text::Line,
    widgets::{Block, List, Widget},
};
use std::{io, time::Duration};

pub enum ViewState {
    HomePage,
}

pub struct Application {
    state: Vec<Index>,
    current_page: ViewState,
    should_exit: bool,
    selected_index: u64,
}

impl Application {
    pub fn new(state: Vec<Index>) -> Application {
        Application {
            state,
            current_page: ViewState::HomePage,
            should_exit: false,
            selected_index: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_keypress(&mut self, e: KeyEvent) {
        match e.code {
            KeyCode::Down => {
                self.selected_index += 1;
            },
            KeyCode::Up => {
                self.selected_index -= 1;
            },
            KeyCode::Char('q') => {
                self.should_exit = true;
            },
            _ => {}
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        while let Ok(can_call) = event::poll(Duration::from_millis(16)) {
            if can_call {
                match event::read()? {
                    Event::Key(event) if event.kind == event::KeyEventKind::Press => {
                        self.handle_keypress(event);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn get_identifiers(&self) -> Vec<&str> {
        let mut items = Vec::new();
        for idx in &self.state {
            items.push(idx.identifier.as_str());
        }
        items
    }
}

impl Widget for &Application {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self.current_page {
            ViewState::HomePage => {
                let title = Line::from("Miss Me");
                let block = Block::bordered()
                    .title_top(title.centered())
                    .border_set(border::THICK);
                let list = List::new(self.get_identifiers());
                list.block(block).render(area, buf);
            }
        };
    }
}
