use crate::RGB;

const ALPHABET_ALLOC: usize = 26 * 5;

const GRAY: RGB = (0.5, 0.5, 0.5);
const YELLOW: RGB = (1.0, 0.8, 0.0);
const GREEN: RGB = (0.0, 0.7, 0.0);

pub struct ValidWords([[char; 5]; 14855]);

include!("./words.rs");

impl ValidWords {
    pub const fn new() -> Self {
        // in words.rs
        Self(VALID_WORDS)
    }

    pub fn filter(&self, filter: &Filter, limit: usize) -> impl Iterator<Item = String> {
        self.0
            .iter()
            .filter(|word| filter.matches(word))
            .take(limit)
            .map(|word| word.iter().collect())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Filter(pub [[GridCell; 5]; 6]);

impl Filter {
    pub fn new() -> Self {
        Self(
            [[GridCell {
                character: None,
                color: CellColor::Gray,
            }; 5]; 6],
        )
    }

    pub fn set(&mut self, row: usize, col: usize, c: char, color: CellColor) {
        assert!(row < 6, "Row index isn't in 0..=5");
        assert!(col < 5, "Column index isn't in 0..=4");

        self.0[row][col] = GridCell {
            character: Some(c),
            color,
        };
    }

    pub fn matches(&self, word: &[char; 5]) -> bool {
        let mut greens: [Option<char>; 5] = [None; 5];
        let mut yellows: Vec<(char, usize)> = Vec::with_capacity(ALPHABET_ALLOC);
        let mut grays: Vec<(char, usize)> = Vec::with_capacity(ALPHABET_ALLOC);

        for row in &self.0 {
            for (i, cell) in row.iter().enumerate() {
                if let Some(c) = cell.character {
                    match cell.color {
                        CellColor::Green => greens[i] = Some(c),
                        CellColor::Yellow => yellows.push((c, i)),
                        CellColor::Gray => grays.push((c, i)),
                    }
                }
            }
        }

        // Green constraints
        for (i, g) in greens.iter().enumerate() {
            if let Some(expected) = g
                && word[i] != *expected
            {
                return false;
            }
        }

        // Yellow constraints
        for &(c, i) in &yellows {
            if word[i] == c || !word.contains(&c) {
                return false;
            }
        }

        // Gray constraints
        for &(c, i) in &grays {
            if word[i] == c {
                return false;
            }

            let used_elsewhere =
                greens.iter().any(|&g| g == Some(c)) || yellows.iter().any(|&(yc, _)| yc == c);

            if !used_elsewhere && word.contains(&c) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GridCell {
    pub character: Option<char>,
    pub color: CellColor,
}

impl Default for GridCell {
    fn default() -> Self {
        Self {
            character: None,
            color: CellColor::Gray,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellColor {
    Gray,
    Yellow,
    Green,
}

impl CellColor {
    pub fn to_color(&self) -> iced::Color {
        match self {
            CellColor::Gray => iced::Color::from_rgb(GRAY.0, GRAY.1, GRAY.2),
            CellColor::Yellow => iced::Color::from_rgb(YELLOW.0, YELLOW.1, YELLOW.2),
            CellColor::Green => iced::Color::from_rgb(GREEN.0, GREEN.1, GREEN.2),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            CellColor::Gray => CellColor::Yellow,
            CellColor::Yellow => CellColor::Green,
            CellColor::Green => CellColor::Gray,
        }
    }
}
