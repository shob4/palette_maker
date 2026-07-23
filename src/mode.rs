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
}

#[derive(Debug, Clone)]
pub enum RetryAction {
    Startup,
    Save(Vec<dis_color>),
    Generate(usize),
    GenerateSingle,
    Monochrome(Hsl),
}
