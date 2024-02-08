pub use glob::Pattern as Glob;
pub use regex::Regex;

pub struct Rule {
    pub title: String,
    pub glob: Glob,
    pub regex: Option<Regex>,
}

pub struct Ruleset {
    pub rules: Vec<Rule>,
}
