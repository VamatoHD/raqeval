use super::Rational;
use super::to_nonzeroU128;

macro_rules! const_rational {
    ($a:literal / $b:literal) => {{
        const {
            Rational {
                num: $a,
                den: to_nonzeroU128!($b),
                neg: false,
            }
        }
    }};
}

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
            Consts::E => const_rational!(848456353 / 312129649),
            Consts::PI => const_rational!(1146408 / 364913),
        }
    }
}
