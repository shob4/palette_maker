use crate::{
    color_spaces::{Color as dis_color, Hsl},
    input::TextInput,
};

#[derive(Debug, Clone, Default)]
pub enum UiMode {
    #[default]
    Normal,
    Monochrome {
        column: usize,
        options: Vec<dis_color>,
        selected: usize,
    },
    Save {
        input: TextInput,
    },
    Open {
        input: TextInput,
        matches: Vec<String>,
        selected: usize,
    },
}

#[derive(Debug, Clone)]
pub enum RetryAction {
    Startup,
    Save(Vec<dis_color>),
    Generate(usize),
    GenerateFrom(Vec<dis_color>, usize),
    GenerateSingle,
    Monochrome(Hsl),
    Load(String),
    List,
}
