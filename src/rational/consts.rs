use super::Rational;
use super::rat;

#[derive(Debug, Clone)]
pub enum Consts {
    E,
    PI,
}

impl std::fmt::Display for Consts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Consts::E => write!(f, "e"),
            Consts::PI => write!(f, "pi"),
        }
    }
}

impl Consts {
    pub const ALL: [&str; 2] = ["e", "pi"];

    pub fn from_str(v: &str) -> Option<Consts> {
        match v.trim().to_lowercase().as_str() {
            "e" => Some(Consts::E),
            "pi" => Some(Consts::PI),
            _ => None,
        }
    }

    pub const fn value(&self) -> Rational {
        match self {
            Consts::E => rat!(848456353 / 312129649),
            Consts::PI => rat!(1146408 / 364913),
        }
    }
}
