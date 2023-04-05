use std::{io, time::Duration};
use tui::{
    backend::{Backend},
    widgets::{Block, Paragraph, List, ListItem},
    layout::{Layout, Constraint, Direction},
    text::{Span, Spans},
    style::{Style, Color},
    Terminal,
    Frame
};
use crossterm::{
    event::{
        self, poll,
        Event, KeyCode
    },
};
use std::sync::{Mutex, Arc};
use std::thread;

use crate::network::Peer;

enum InputMode {
    Normal,
    Editing,
    Selecting,
}

pub struct App {
    input: String,
    input_mode: InputMode,
    messages: Arc<Mutex<Vec<String>>>,
    closed: Arc<Mutex<bool>>,
    
    peer_name: String,
    peer_addr: String,
}

impl App {
    pub fn new(peer_name: String, peer_addr: String) -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Arc::new(Mutex::new(Vec::new())),
            closed: Arc::new(Mutex::new(false)),
            peer_name,
            peer_addr,
        }
    }

    pub fn run<B: Backend>(
        &mut self, 
        terminal: &mut Terminal<B>, 
        peer: &mut Peer
    ) -> io::Result<()> {
        
        Self::start_reciever(self.messages.clone(), self.closed.clone(), peer);

        while !*self.closed.lock().unwrap() {
            terminal.draw(|f| self.ui(f)).unwrap();
            
            if poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match self.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('i') => {
                                self.input_mode = InputMode::Editing;
                            }
                            KeyCode::Char('v') => {
                                self.input_mode = InputMode::Selecting;
                            }
                            KeyCode::Char('q') => {
                                peer.close();
                                return Ok(());
                            }
                            _ => {}
                        },
                        InputMode::Editing => match key.code {
                            KeyCode::Enter => {
                                peer.send(self.input.as_str()).unwrap();
                                self.messages.lock().unwrap().push(self.input.drain(..).collect());
                            }
                            KeyCode::Char(c) => {
                                self.input.push(c);
                            }
                            KeyCode::Backspace => {
                                self.input.pop();
                            }
                            KeyCode::Esc => {
                                self.input_mode = InputMode::Normal;
                            }
                            _ => {}
                        },
                        InputMode::Selecting => match key.code {
                            KeyCode::Esc => {
                                self.input_mode = InputMode::Normal;
                            }
                            _ => {}
                        }
                    }
                }
            }

        }
        Ok(())

    }

    fn start_reciever(msg_vec: Arc<Mutex<Vec<String>>>, closed: Arc<Mutex<bool>>, peer: &Peer) {
        
        let reciever_peer = Arc::new(Mutex::new(peer.clone()));

        thread::spawn(move || {
            let mut stream = reciever_peer.lock().unwrap();
            loop {
                if let Ok(line) = stream.recieve() {
                    if line.len() == 0 {
                        break;
                    }
                    msg_vec.lock().unwrap().push(format!("{}: {}", stream.get_name(), line));
                } else {
                    break;
                }
            }
            let mut flag = closed.lock().unwrap();
            *flag = true;
        });
    }

    fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(f.size());

        let text = match self.input_mode {
            InputMode::Normal => String::from("Press Q to quit"),
            InputMode::Editing => String::from("-- INSERT --"),
            InputMode::Selecting => String::from("-- VISUAL --"),
        };

        let par = Paragraph::new(Span::raw(text));
        f.render_widget(par, chunks[2]);

        let par = Paragraph::new(Span::raw(
            format!("[Connected to {}]", self.peer_addr))
        );
        f.render_widget(par, chunks[1]);

        let par = Block::default().style(Style::default()
            .bg(Color::Gray)
            .fg(Color::Black)
        );
        f.render_widget(par, chunks[1]);

        let tmp = self.messages.lock().unwrap();
        let mut messages: Vec<ListItem> = tmp
            .iter()
            .map(|m| {
                let content = vec![Spans::from(Span::raw(m))];
                ListItem::new(content)
            })
            .collect();
        
        if(matches!(self.input_mode, InputMode::Editing)||self.input.len()>0) {
            messages.push(ListItem::new(Span::raw(
                format!("You: {}", self.input)
            )));
        }

        let messages = List::new(messages).block(Block::default());
        f.render_widget(messages, chunks[0]);

        if matches!(self.input_mode, InputMode::Editing) {
            f.set_cursor(
                chunks[0].x + self.input.len() as u16 + 5,
                chunks[0].y + tmp.len() as u16);
        }

    }
}
/*
fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout, 
        EnterAlternateScreen, 
        EnableMouseCapture, 
        SetCursorShape(CursorShape::Line)
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("Mario".to_string(), "192.168.1.35".to_string());
    app.run(&mut terminal)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        SetCursorShape(CursorShape::Block),
        EnableBlinking,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
*/

