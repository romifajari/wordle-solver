use iced::widget::{button, column, container, row, text, text_input};
use iced::{Center, Color, Element, Fill, Length};
use std::fmt::Write;

mod wordle;
use wordle::{CellColor, Filter, ValidWords};

type RGB = (f32, f32, f32);

const WORDS_TO_TAKE: usize = 10;
const DISPLAYED_TEXT_ALLOC: usize = 400;
static WORDS: ValidWords = ValidWords::new();

pub fn main() -> iced::Result {
    iced::application(App::init, App::update, App::view).run()
}

#[derive(Debug, Clone)]
struct App {
    filter: Filter,
    selected_cell: Option<(usize, usize)>,
    input_text: String,
    displayed_text: String,
    matching_words: [Option<String>; WORDS_TO_TAKE],
}

#[derive(Debug, Clone)]
enum Message {
    CellClicked(usize, usize),
    InputChanged(String),
    SetCharacter,
    CycleColor(usize, usize),
}

impl App {
    fn init() -> Self {
        Self {
            filter: Filter::new(),
            selected_cell: None,
            input_text: String::with_capacity(3),
            displayed_text: String::with_capacity(DISPLAYED_TEXT_ALLOC),
            matching_words: Default::default(),
        }
    }

    fn update_displayed_text(&mut self) {
        self.displayed_text.clear();
        for (row_idx, row) in self.filter.0.iter().enumerate() {
            unsafe { write!(self.displayed_text, "Row {}: ", row_idx + 1).unwrap_unchecked() };
            for cell in row {
                match cell.character {
                    Some(ch) => {
                        self.displayed_text.push(ch);
                        self.displayed_text.push_str(match cell.color {
                            CellColor::Gray => "-Gray ",
                            CellColor::Yellow => "-Yellow ",
                            CellColor::Green => "-Green ",
                        });
                    }
                    None => self.displayed_text.push_str("_ "),
                }
            }
            self.displayed_text.push('\n');
        }

        unsafe {
            writeln!(
                self.displayed_text,
                "\nMatching Words (Up to {}):",
                WORDS_TO_TAKE
            )
            .unwrap_unchecked();
        }

        if self.matching_words[0].is_none() {
            self.displayed_text.push_str("No matches found.");
        } else {
            for word in self.matching_words.iter().flatten() {
                self.displayed_text.push_str(&word);
                self.displayed_text.push('\n');
            }
        }
    }

    fn update_matching_words(
        filter: impl Iterator<Item = String>,
    ) -> [Option<String>; WORDS_TO_TAKE] {
        let mut array: [Option<String>; WORDS_TO_TAKE] = Default::default();
        for (i, word) in filter.enumerate() {
            array[i] = Some(word);
        }
        array
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CellClicked(row, col) => {
                self.selected_cell = Some((row, col));
                self.input_text.clear();
            }

            Message::InputChanged(text) => {
                self.input_text = text;

                if let Some((row, col)) = self.selected_cell
                    && let Some(ch) = self.input_text.chars().last()
                    && ch.is_alphabetic()
                {
                    let current_color = self.filter.0[row][col].color;
                    self.filter
                        .set(row, col, ch.to_ascii_lowercase(), current_color);
                    let words = WORDS.filter(&self.filter, WORDS_TO_TAKE);
                    self.matching_words = Self::update_matching_words(words);
                    self.update_displayed_text();
                }
            }

            Message::SetCharacter => {
                if let Some((row, col)) = self.selected_cell
                    && let Some(ch) = self.input_text.chars().last()
                    && ch.is_alphabetic()
                {
                    let current_color = self.filter.0[row][col].color;
                    self.filter
                        .set(row, col, ch.to_ascii_lowercase(), current_color);
                    let words = WORDS.filter(&self.filter, WORDS_TO_TAKE);
                    self.matching_words = Self::update_matching_words(words);
                    self.update_displayed_text();
                }
            }

            Message::CycleColor(row, col) => {
                let current_char = self.filter.0[row][col].character.unwrap_or('_');
                let new_color = self.filter.0[row][col].color.next();
                self.filter.set(row, col, current_char, new_color);
                let words = WORDS.filter(&self.filter, WORDS_TO_TAKE);
                self.matching_words = Self::update_matching_words(words);
                self.update_displayed_text();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let left_pane = self.view_grid();
        let right_pane = self.view_text_display();

        row![
            container(left_pane)
                .width(Length::FillPortion(1))
                .height(Fill)
                .padding(20),
            container(right_pane)
                .width(Length::FillPortion(1))
                .height(Fill)
                .padding(20)
        ]
        .spacing(10)
        .into()
    }

    fn view_grid(&self) -> Element<'_, Message> {
        let mut grid_column = column![].spacing(5);

        for (row_idx, row) in self.filter.0.iter().enumerate() {
            let mut grid_row = row![].spacing(5);

            for (col_idx, cell) in row.iter().enumerate() {
                let is_selected = self.selected_cell == Some((row_idx, col_idx));

                let cell_content = match cell.character {
                    Some(ch) => ch.to_ascii_uppercase(),
                    None => '_',
                };

                let cell_button = button(text(cell_content).size(24).width(Fill).align_x(Center))
                    .width(60)
                    .height(60)
                    .style(move |_theme, _status| {
                        let base_color = cell.color.to_color();
                        let border_color = if is_selected {
                            Color::from_rgb(0.0, 0.0, 1.0)
                        } else {
                            Color::from_rgb(0.2, 0.2, 0.2)
                        };

                        button::Style {
                            background: Some(base_color.into()),
                            border: iced::Border {
                                width: if is_selected { 3.0 } else { 1.0 },
                                color: border_color,
                                radius: 5.0.into(),
                            },
                            text_color: Color::WHITE,
                            ..button::Style::default()
                        }
                    })
                    .on_press(Message::CellClicked(row_idx, col_idx));

                grid_row = grid_row.push(cell_button);
            }

            grid_column = grid_column.push(grid_row);
        }

        let input_section = if let Some((row, col)) = self.selected_cell {
            column![
                text(format!("Selected: Row {}, Col {}", row + 1, col + 1)),
                text_input("Enter character...", &self.input_text)
                    .on_input(Message::InputChanged)
                    .on_submit(Message::SetCharacter),
                button("Cycle Color")
                    .on_press(Message::CycleColor(row, col))
                    .style(button::secondary),
            ]
            .spacing(10)
        } else {
            column![text("Click a cell to select it")]
        };

        column![text("Wordle Grid").size(20), grid_column, input_section,]
            .spacing(20)
            .into()
    }

    fn view_text_display(&self) -> Element<'_, Message> {
        column![
            text("Console*").size(20),
            container(text(&self.displayed_text).size(16).width(Fill))
                .width(Fill)
                .height(Fill)
                .padding(15)
                .style(|_theme| {
                    container::Style {
                        background: Some(Color::from_rgb(0.0, 0.0, 0.0).into()),
                        border: iced::Border {
                            width: 1.0,
                            color: Color::from_rgb(0.8, 0.8, 0.8),
                            radius: 5.0.into(),
                        },
                        ..container::Style::default()
                    }
                })
        ]
        .spacing(10)
        .into()
    }
}
