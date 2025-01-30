use std::fmt::Display;

pub enum Site {
    One,
    Two,
    Three,
    Four,
}

impl Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Site::One => write!(f, "W1"),
            Site::Two => write!(f, "W2"),
            Site::Three => write!(f, "W3"),
            Site::Four => write!(f, "W4"),
        }
    }
}
