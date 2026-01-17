use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use kubesolv_solvers::{cube::{moves::CubeMove, state::CubeState}};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Paragraph},
    Frame, Terminal,
};
use std::{io};

struct App {
    cube: CubeState,
}

impl App {
    fn new() -> Self {
        App {
            cube: CubeState::new_solved(),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('u') => self.cube.execute_move(CubeMove::U),
            KeyCode::Char('U') => self.cube.execute_move(CubeMove::UPrime),
            KeyCode::Char('d') => self.cube.execute_move(CubeMove::D),
            KeyCode::Char('D') => self.cube.execute_move(CubeMove::DPrime),
            KeyCode::Char('l') => self.cube.execute_move(CubeMove::L),
            KeyCode::Char('L') => self.cube.execute_move(CubeMove::LPrime),
            KeyCode::Char('r') => self.cube.execute_move(CubeMove::R),
            KeyCode::Char('R') => self.cube.execute_move(CubeMove::RPrime),
            KeyCode::Char('f') => self.cube.execute_move(CubeMove::F),
            KeyCode::Char('F') => self.cube.execute_move(CubeMove::FPrime),
            KeyCode::Char('b') => self.cube.execute_move(CubeMove::B),
            KeyCode::Char('B') => self.cube.execute_move(CubeMove::BPrime),
            KeyCode::Char('m') => self.cube.execute_move(CubeMove::M),
            KeyCode::Char('M') => self.cube.execute_move(CubeMove::MPrime),
            KeyCode::Char('e') => self.cube.execute_move(CubeMove::E),
            KeyCode::Char('E') => self.cube.execute_move(CubeMove::EPrime),
            KeyCode::Char('s') => self.cube.execute_move(CubeMove::S),
            KeyCode::Char('S') => self.cube.execute_move(CubeMove::SPrime),
            KeyCode::Char('x') => self.cube.execute_move(CubeMove::X),
            KeyCode::Char('X') => self.cube.execute_move(CubeMove::XPrime),
            KeyCode::Char('y') => self.cube.execute_move(CubeMove::Y),
            KeyCode::Char('Y') => self.cube.execute_move(CubeMove::YPrime),
            KeyCode::Char('z') => self.cube.execute_move(CubeMove::Z),
            KeyCode::Char('Z') => self.cube.execute_move(CubeMove::ZPrime),
            KeyCode::Char('c') => self.cube = CubeState::new_solved(),
            _ => {}
        }
        true
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::Hide
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if !app.handle_key(key) {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    draw_cube(f, f.size(), app);
}

fn draw_cube(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let mut lines = vec![Line::from("".to_string())];
    lines.extend(
        app.cube
            .format_colored()
            .lines()
            .map(|line| Line::from(line.to_string())),
    );
    lines.push(Line::from("".to_string()));
    lines.push(Line::from(format!("Solved: {}", app.cube.solved())));

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title("Kubesolv Simulator"))
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, area);
}
