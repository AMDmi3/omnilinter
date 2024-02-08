pub use glob::Pattern as Glob;
pub use regex::Regex;

pub struct Rule {
    pub glob: Glob,
    pub regex: Regex,
}

pub struct Ruleset {
    pub rules: Vec<Rule>,
}
