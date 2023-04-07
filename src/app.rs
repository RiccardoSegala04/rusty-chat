use std::{io, time::Duration};
use tui::{
    backend::{Backend},
    widgets::{Block, Paragraph, List, ListItem},
    layout::{Layout, Constraint, Direction},
    text::{Span, Spans},
    style::{Style, Color, Modifier},
    backend::CrosstermBackend,
    Terminal,
    Frame
};
use crossterm::{
    event::{
        self, poll,
        Event, KeyCode, EnableMouseCapture, DisableMouseCapture
    },
    terminal::{
        enable_raw_mode, disable_raw_mode, 
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    cursor::{SetCursorShape, EnableBlinking, CursorShape},
    execute,
};
use std::sync::{Mutex, Arc};
use std::thread;

use crate::network::Peer;

enum InputMode {
    Normal,
    Editing,
    Selecting,
}

struct Point {
    line: u16,
    col: u16,
}

pub struct App {
    input: String,
    input_mode: InputMode,
    messages: Arc<Mutex<Vec<String>>>,
    
    show_idx: usize,
    cursor_pos: Point,

    closed: Arc<Mutex<bool>>,
    
    peer: Peer,
}

impl App {
    pub fn new(peer: Peer) -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Editing,
            messages: Arc::new(Mutex::new(Vec::new())),
            closed: Arc::new(Mutex::new(false)),
            peer,
            show_idx: 0,
            cursor_pos: Point{line:0, col:0},
        }
    }

   pub fn run(&mut self) -> io::Result<()> {

        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(
            stdout, 
            EnterAlternateScreen, 
            EnableMouseCapture, 
        ).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        
        Self::start_reciever(
            self.messages.clone(), self.closed.clone(), &self.peer);

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
                                self.peer.close();
                                *self.closed.lock().unwrap() = true;
                            }
                            _ => self.check_move_cursor(&key.code),
                        },
                        InputMode::Editing => match key.code {
                            KeyCode::Enter => {
                                let to_send = self.input.trim();
                                if !to_send.is_empty() {
                                    self.peer.send(to_send).unwrap();
                                    self.messages.lock().unwrap().push(format!(
                                        "You: {}", to_send
                                    ));
                                    self.input.clear();
                                }
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
                            _ => self.check_move_cursor(&key.code),
                        }
                    }
                }
            }

        }

        disable_raw_mode().unwrap();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            SetCursorShape(CursorShape::Block),
            EnableBlinking,
        ).unwrap();
        terminal.show_cursor().unwrap();

        Ok(())

    }

    fn start_reciever(
        msg_vec: Arc<Mutex<Vec<String>>>, 
        closed: Arc<Mutex<bool>>, 
        peer: &Peer
    ) {
        
        let reciever_peer = Arc::new(Mutex::new(peer.clone()));

        thread::spawn(move || {
            let mut stream = reciever_peer.lock().unwrap();
            loop {
                if let Ok(line) = stream.recieve() {
                    if line.is_empty() {
                        break;
                    }
                    msg_vec.lock().unwrap().push(format!(
                        "{}: {}", stream.get_name(), line
                    ));
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
            format!("[Connected to {}]", self.peer.get_ip_str()))
        );
        f.render_widget(par, chunks[1]);

        let par = Block::default().style(Style::default()
            .bg(Color::Gray)
            .fg(Color::Black)
        );
        f.render_widget(par, chunks[1]);

        let tmp = self.messages.lock().unwrap();

        let chunk_size = chunks[0].height-1;
        
        let mut stdout = io::stdout();
        if matches!(self.input_mode, InputMode::Editing) {
            if usize::from(chunk_size)<tmp.len() {
                self.show_idx = tmp.len()-usize::from(chunk_size);
            } 
            execute!(stdout, SetCursorShape(CursorShape::Line)).unwrap();
            self.cursor_pos.col = self.input.len() as u16 + 5;
            self.cursor_pos.line = tmp.len() as u16;

        } else {
            execute!(stdout, SetCursorShape(CursorShape::Block)).unwrap();
            if self.cursor_pos.line < (self.show_idx as u16) {
                self.show_idx = usize::from(self.cursor_pos.line);
            } else if self.cursor_pos.line+1>=(self.show_idx as u16)+chunks[0].height {
                self.show_idx += 
                    usize::from(
                        self.cursor_pos.line+1-chunks[0].height)-self.show_idx;
            }
        }

        let mut messages = Vec::new();

        for i in 0..std::cmp::min(chunk_size, tmp.len().try_into().unwrap()) {
            let i = i as usize;
            let msg = tmp.get((i+self.show_idx) as usize).unwrap();
            let sep = msg.find(' ').unwrap_or(0);
            let content = vec![Spans::from(vec!(
                    Span::styled(
                        &msg.as_str()[0..sep], 
                        Style::default().add_modifier(Modifier::BOLD)),
                        
                    Span::raw(&msg.as_str()[sep..msg.len()])
                )
            )];
            messages.push(ListItem::new(content));
        }

        messages.push(ListItem::new(vec![Spans::from(vec!(
            Span::styled(
                "You: ", 
                Style::default().add_modifier(Modifier::BOLD)),
                
            Span::raw(self.input.as_str())
        ))]));

        let messages = List::new(messages).block(Block::default());
        f.render_widget(messages, chunks[0]);

        

        let mut cursor_y = 0;
        if self.cursor_pos.line>(self.show_idx as u16) {
            cursor_y = self.cursor_pos.line-self.show_idx as u16;
        }
        
        f.set_cursor(
            chunks[0].x + self.cursor_pos.col,
            chunks[0].y + cursor_y
        );

    }
    
    fn get_selected_line_len(&self) -> u16 {
        self.messages.lock().unwrap().get(
            usize::from(self.cursor_pos.line)
        ).unwrap_or(&"You: ".to_string()).len() as u16
    }

    fn check_move_cursor(&mut self, code: &KeyCode) {
        match code {
            KeyCode::Left => {
                if self.cursor_pos.col>0 {
                    self.cursor_pos.col-=1;
                }
            }
            KeyCode::Right => {
                if self.cursor_pos.col<self.get_selected_line_len()-1 {
                    self.cursor_pos.col+=1;
                }
            }
            KeyCode::Up => {
                if self.cursor_pos.line>0 {
                    self.cursor_pos.line-=1;

                    self.cursor_pos.col = std::cmp::min(
                        self.cursor_pos.col, 
                        self.get_selected_line_len()-1
                    ); 
                } 
            }
            KeyCode::Down => {
                if usize::from(self.cursor_pos.line)
                    < self.messages.lock().unwrap().len() {

                    self.cursor_pos.line+=1;
                    
                    self.cursor_pos.col = std::cmp::min(
                        self.cursor_pos.col, 
                        self.get_selected_line_len()-1
                    ); 
                }
            }
            _ => ()
        }
    }


}

