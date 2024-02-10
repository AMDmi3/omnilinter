use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use tempdir::TempDir;

pub struct OmnilinterRunner {
    root: TempDir,
}

impl OmnilinterRunner {
    pub fn new(&root_constructor: RootConstructor) -> OmnilinterRunner {
        let mut cmd = Command::cargo_bin("omnilinter").unwrap();

        let result = cmd.current_dir(&root_constructor.path()).arg("--config=omnilinter.conf").ok();


        RootConstructor {
            root: TempDir::new("omniparser-test").unwrap(),
        }
    }

    pub fn add_file(&self, path: &str, text: &str) -> &RootConstructor {
        let path = Path::new(path);

        if let Some(parent) = path.ancestors().nth(1) {
            println!("{:?}", parent);
            if !parent.exists() {
                create_dir_all(self.root.path().join(parent)).unwrap();
            }
        }

        let mut f = File::create(self.root.path().join(path)).unwrap();

        f.write_all(text.as_bytes()).unwrap();

        &self
    }

    pub fn path(&self) -> &Path {
        self.root.path()
    }
}
