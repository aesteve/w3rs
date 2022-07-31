#[derive(Debug, PartialEq, Clone)]
pub enum Environment {
    Tree,
    GoldMine,
}

impl Environment {
    pub(crate) fn from_str(str: &str) -> Option<Environment> {
        if "LTlt" == str {
            Some(Environment::Tree)
        } else {
            None
        }
    }
}
