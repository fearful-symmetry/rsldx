use clap::{Parser, Subcommand, ValueEnum};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Clone, Subcommand)]
pub enum Commands {
    /// Print USB info about the connected display
    Info,
    /// Reset the display
    Reset,
    /// Print a message on the display
    Print{
        message: String
    },
    /// Print a scrolling message. Must be less than 45 characters.
    Scroll {
        message: String,
       #[arg(long, short='d', value_enum, default_value_t=Direction::Left)]
        direction: Direction,
        #[arg(long, short='p', value_enum, default_value_t=Position::Top)]
        position: Position
    },
    /// Print a "smart scroll" message, with each string part scrolling on and off the screen one at a time.
    /// Each part must be less than 20 characters, with a total maximum of 45 characters.
    Smart {
        message: Vec<String>,
        #[arg(long, short='d', value_enum, default_value_t=Direction::Left)]
        direction: Direction,
        #[arg(long, short='p', value_enum, default_value_t=Position::Top)]
        position: Position

    },
    /// Enable or disable the terminal's blinking cursor
    #[command(subcommand)]
    Cursor(CursorMode),
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Position {
    Top,
    Bottom
}

impl From<Position> for linedisplayrs::ScrollPosition {
    fn from(val: Position) -> Self {
        match val {
            Position::Bottom => linedisplayrs::ScrollPosition::Bottom,
            Position::Top => linedisplayrs::ScrollPosition::Top
        }
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right   
}

impl From<Direction> for linedisplayrs::ScrollDirection {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Left => linedisplayrs::ScrollDirection::Left,
            Direction::Right => linedisplayrs::ScrollDirection::Right
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum CursorMode {
    On,
    Off
}

impl From<CursorMode> for linedisplayrs::CursorMode {
    fn from(val: CursorMode) -> Self {
        match val {
            CursorMode::On => linedisplayrs::CursorMode::On,
            CursorMode::Off => linedisplayrs::CursorMode::Off
        }
    }
}