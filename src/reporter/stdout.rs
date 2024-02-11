use crate::location::MatchLocation;
use crate::reporter::Reporter;
use std::path::PathBuf;

pub struct ReporterOptions {
    pub full_paths: bool,
}

pub struct StdoutReporter {
    options: ReporterOptions,
    prev_root: PathBuf,
    has_matches: bool,
}

impl StdoutReporter {
    pub fn new(options: ReporterOptions) -> Self {
        Self {
            options,
            prev_root: Default::default(),
            has_matches: false,
        }
    }
}

impl Reporter for StdoutReporter {
    fn report(&mut self, location: &MatchLocation, message: &str) {
        let current_root = match location {
            MatchLocation::Root(loc) => loc.root,
            MatchLocation::File(loc) => loc.root,
            MatchLocation::Line(loc) => loc.root,
        };

        if current_root != self.prev_root {
            println!("in {}", current_root.display());
            self.prev_root = current_root.to_path_buf();
        }

        match location {
            MatchLocation::Root(_) => println!("- {}", message),
            MatchLocation::File(loc) => {
                if self.options.full_paths {
                    println!("- {}: {}", loc.root.join(loc.file).display(), message)
                } else {
                    println!("- {}: {}", loc.file.display(), message)
                }
            }
            MatchLocation::Line(loc) => {
                if self.options.full_paths {
                    println!(
                        "- {}:{}: {}",
                        loc.root.join(loc.file).display(),
                        loc.line,
                        message
                    )
                } else {
                    println!("- {}:{}: {}", loc.file.display(), loc.line, message)
                }
            }
        }

        self.has_matches = true;
    }

    fn flush(&self) {}

    fn has_matches(&self) -> bool {
        self.has_matches
    }
}
