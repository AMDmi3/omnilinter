use crate::location::MatchLocation;
use std::path::PathBuf;

pub struct Reporter {
    prev_root: PathBuf,
}

impl Reporter {
    pub fn new() -> Reporter {
        Reporter {
            prev_root: Default::default(),
        }
    }

    pub fn report(&mut self, location: &MatchLocation, message: &str) {
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
            MatchLocation::Root(loc) => println!("- {}", message),
            MatchLocation::File(loc) => println!("- {}: {}", loc.file.display(), message),

            MatchLocation::Line(loc) => {
                println!("- {}:{}: {}", loc.file.display(), loc.line, message)
            }
        }
    }
}
