use crate::location::MatchLocation;
use crate::reporter::Reporter;

#[derive(serde::Serialize)]
struct Match {
    message: String,
    root: String,
    file: Option<String>,
    line: Option<usize>,
}

pub struct JsonReporter {
    reports: Vec<Match>,
}

impl JsonReporter {
    pub fn new() -> Self {
        Self {
            reports: Default::default(),
        }
    }
}

impl Reporter for JsonReporter {
    fn report(&mut self, location: &MatchLocation, message: &str) {
        self.reports.push(match location {
            MatchLocation::Root(loc) => Match {
                root: loc.root.display().to_string(),
                file: None,
                line: None,
                message: String::from(message),
            },
            MatchLocation::File(loc) => Match {
                root: loc.root.display().to_string(),
                file: Some(loc.file.display().to_string()),
                line: None,
                message: String::from(message),
            },
            MatchLocation::Line(loc) => Match {
                root: loc.root.display().to_string(),
                file: Some(loc.file.display().to_string()),
                line: Some(loc.line),
                message: String::from(message),
            },
        });
    }

    fn flush(&self) {
        println!("{}", serde_json::to_string_pretty(&self.reports).unwrap());
    }
}
