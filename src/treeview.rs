use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use ratatui::{prelude::*, widgets::*};
use crossterm::{terminal, execute, event};

pub struct TreeModel {
    entries: Vec<TreeEntry>,
}

pub struct TreeEntry {
    pub name: String,
    pub is_dir: bool,
    pub depth: u32,
}

impl TreeModel {
    pub fn new() -> TreeModel {
        TreeModel {
            entries: vec![],
        }
    }

    pub fn add_entry(&mut self, entry: TreeEntry) {
        self.entries.push(entry);
    }

    pub fn lines(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|entry| {
                let indent = "│   ".repeat(entry.depth as usize);
                let prefix = "├── ";
                format!("{}{}{}", indent, prefix, entry.name)
            })
            .collect()
    }

    pub fn start(model: Arc<Mutex<TreeModel>>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut stdout = io::stdout();
            terminal::enable_raw_mode().unwrap();
            execute!(stdout, terminal::EnterAlternateScreen).unwrap();
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend).unwrap();

            loop {
                terminal.draw(|f| {
                    let model = model.lock().unwrap();
                    let items: Vec<ListItem> = model
                        .lines()
                        .iter()
                        .map(|line| ListItem::new(line.clone()))
                        .collect();

                    let widget = List::new(items)
                        .block(Block::default().title("Test Tree").borders(Borders::ALL));
                    f.render_widget(widget, f.area());
                }).unwrap();

                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        if key.code == event::KeyCode::Char('q') {
                            break;
                        }
                    }
                }
            }

            terminal::disable_raw_mode().unwrap();
            execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen).unwrap();
        })
    }
}