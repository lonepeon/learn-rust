use std::io;

enum Letter {
    Green(char),
    Yellow(char),
    Gray(char),
}

impl Letter {
    fn from_guess(guess: &crate::word::Guess) -> [Self; 5] {
        guess.guessed_letters().map(|h| match h {
            crate::word::Hint::Misplaced(c) => Letter::Yellow(c),
            crate::word::Hint::Exact(c) => Letter::Green(c),
            crate::word::Hint::Absent(c) => Letter::Gray(c),
        })
    }

    fn extract(&self) -> (&char, tui::style::Color) {
        match self {
            Letter::Green(c) => (c, tui::style::Color::Green),
            Letter::Yellow(c) => (c, tui::style::Color::Yellow),
            Letter::Gray(c) => (c, tui::style::Color::Reset),
        }
    }
}

fn popup<B: tui::backend::Backend>(f: &mut tui::Frame<B>, rect: tui::layout::Rect, content: &str) {
    let block = tui::widgets::Paragraph::new(content)
        .wrap(tui::widgets::Wrap { trim: true })
        .alignment(tui::layout::Alignment::Center)
        .block(tui::widgets::Block::default().borders(tui::widgets::Borders::ALL));

    let popup_layout = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(
            [
                tui::layout::Constraint::Percentage(50 / 2),
                tui::layout::Constraint::Percentage(50),
                tui::layout::Constraint::Percentage(50 / 2),
            ]
            .as_ref(),
        )
        .split(rect);

    let area = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                tui::layout::Constraint::Percentage(30 / 2),
                tui::layout::Constraint::Percentage(70),
                tui::layout::Constraint::Percentage(30 / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1];

    f.render_widget(tui::widgets::Clear, area);
    f.render_widget(block, area);
}

fn print_word<B: tui::backend::Backend>(
    f: &mut tui::Frame<B>,
    parent: tui::layout::Rect,
    letters: &[Letter; 5],
) {
    let chunks = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                tui::layout::Constraint::Length(5),
                tui::layout::Constraint::Length(5),
                tui::layout::Constraint::Length(5),
                tui::layout::Constraint::Length(5),
                tui::layout::Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(parent);

    for (i, l) in letters.iter().enumerate() {
        let (letter, color) = l.extract();
        f.render_widget(
            tui::widgets::Paragraph::new(letter.to_string())
                .alignment(tui::layout::Alignment::Center)
                .style(tui::style::Style::default().fg(color))
                .block(
                    tui::widgets::Block::default()
                        .border_type(tui::widgets::BorderType::Double)
                        .borders(tui::widgets::Borders::ALL),
                ),
            chunks[i],
        );
    }
}

pub fn play(wordle_response: [char; 5]) {
    let mut game = crate::game::Game::new(crate::word::Word::new(wordle_response));
    let mut input = String::new();

    crossterm::terminal::enable_raw_mode().expect("set terminal to raw mode");
    let stdout = io::stdout();
    let backend = tui::backend::CrosstermBackend::new(stdout);
    let mut terminal =
        tui::terminal::Terminal::new(backend).expect("terminal to be setup properly");

    terminal.clear().expect("screen should be cleared");

    loop {
        terminal
            .draw(|f| {
                let mut size = f.size();
                size.width = 27;
                size.height = 20;

                let main_block = tui::widgets::Block::default()
                    .title("Wordle")
                    .borders(tui::widgets::Borders::ALL);

                let main_layout = tui::layout::Layout::default()
                    .direction(tui::layout::Direction::Vertical)
                    .constraints([tui::layout::Constraint::Percentage(100)])
                    .split(size);

                f.render_widget(main_block, main_layout[0]);

                let chunks = tui::layout::Layout::default()
                    .direction(tui::layout::Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            tui::layout::Constraint::Length(3),
                            tui::layout::Constraint::Length(3),
                            tui::layout::Constraint::Length(3),
                            tui::layout::Constraint::Length(3),
                            tui::layout::Constraint::Length(3),
                            tui::layout::Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(main_layout[0]);

                game.history
                    .iter()
                    .enumerate()
                    .for_each(|(i, guess)| print_word(f, chunks[i], &Letter::from_guess(guess)));

                match game.state {
                    crate::game::State::Lost => {
                        popup(
                            f,
                            size,
                            format!(
                                "🥺\n\nthe word was\n{}\nYou will do better next time",
                                String::from_iter(wordle_response),
                            )
                            .as_ref(),
                        );
                        f.set_cursor(size.left(), size.bottom());
                    }
                    crate::game::State::Win => {
                        popup(
                            f,
                            size,
                            format!("\n🎉\n\nyou found the word in {} tries", game.tries())
                                .as_ref(),
                        );
                        f.set_cursor(size.left(), size.bottom());
                    }
                    crate::game::State::InProgress => {
                        let mut chars = input.chars();
                        print_word(
                            f,
                            chunks[game.history.len()],
                            &[
                                Letter::Gray(chars.next().unwrap_or(' ')),
                                Letter::Gray(chars.next().unwrap_or(' ')),
                                Letter::Gray(chars.next().unwrap_or(' ')),
                                Letter::Gray(chars.next().unwrap_or(' ')),
                                Letter::Gray(chars.next().unwrap_or(' ')),
                            ],
                        );
                    }
                }
            })
            .expect("fail to draw UI");

        if game.state != crate::game::State::InProgress {
            std::process::exit(0);
        }

        if let crossterm::event::Event::Key(key) = crossterm::event::read().expect("read event") {
            match (key.modifiers, key.code) {
                (crossterm::event::KeyModifiers::CONTROL, crossterm::event::KeyCode::Char('c')) => {
                    std::process::exit(0);
                }
                (_, crossterm::event::KeyCode::Enter) => {
                    if input.len() == 5 {
                        let mut chars = input.chars();
                        let input_guess = [
                            chars.next().expect("cannot get character at position 0"),
                            chars.next().expect("cannot get character at position 1"),
                            chars.next().expect("cannot get character at position 2"),
                            chars.next().expect("cannot get character at position 3"),
                            chars.next().expect("cannot get character at position 4"),
                        ];

                        game.guess(input_guess);
                        input.clear();
                    }
                }

                (_, crossterm::event::KeyCode::Char(c)) => {
                    if input.len() == 5 {
                        input.pop();
                    }

                    input.push_str(&c.to_uppercase().to_string());
                }
                (_, crossterm::event::KeyCode::Backspace) => {
                    input.pop();
                }
                _ => {}
            }
        }
    }
}
