use ratatui::buffer::Buffer;
use ratatui::layout::{self, Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::{Block, Tabs, Widget, WidgetRef};
use ratatui::{Frame, Terminal};
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;
use ratatui_image::StatefulImage;
use strum::IntoEnumIterator;
use tokio::sync::mpsc::UnboundedSender;

use crate::backend::tui::Action;
use crate::view::pages::*;

use self::search::SearchPage;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum AppState {
    Runnning,
    Done,
}

pub struct App {
    pub current_tab: SelectedTabs,
    pub search_page: SearchPage,
    pub action_tx: UnboundedSender<Action>,
    pub state: AppState,
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let main_layout = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)]);

        let [top_tabs_area, page_are] = main_layout.areas(area);

        self.render_top_tabs(top_tabs_area, buf);

        self.render_pages(page_are, buf);
    }
}

impl App {
    pub fn new(action_tx: UnboundedSender<Action>) -> Self {
        // let mut picker = Picker::from_termios().unwrap();
        // // Guess the protocol.
        // picker.guess_protocol();
        //
        // // Load an image with the image crate.
        //
        // let dyn_img = image::io::Reader::new(Cursor::new("some".as_bytes()))
        //     .with_guessed_format()
        //     .unwrap();
        //
        // // Create the Protocol which will be used by the widget.
        // let image = picker.new_resize_protocol(dyn_img.decode().unwrap());

        App {
            current_tab: SelectedTabs::default(),
            search_page: SearchPage::new(),
            action_tx,
            state: AppState::Runnning,
        }
    }

    pub fn render_top_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles: Vec<String> = SelectedTabs::iter().map(|page| page.to_string()).collect();

        let tabs_block = Block::bordered();

        let current_page = self.current_tab.clone() as usize;

        Tabs::new(titles)
            .block(tabs_block)
            .highlight_style(Color::Yellow)
            .select(current_page)
            .padding("", "")
            .divider(" | ")
            .render(area, buf);
    }

    pub fn render_pages(&self, area: Rect, buf: &mut Buffer) {
        match self.current_tab {
            SelectedTabs::Home => {}
            SelectedTabs::Search => self.render_search_page(area, buf),
        }
    }

    pub fn render_search_page(&self, area: Rect, buf: &mut Buffer) {
        WidgetRef::render_ref(&self.search_page, area, buf);
    }

    pub fn render_home_page(&self, area: Rect, buf: &mut Buffer) {}

    pub fn render_manga_page(&self, area: Rect, buf: &mut Buffer) {}

    pub fn update_state(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.state = AppState::Done;
            }
            _ => {}
        }
    }
}
