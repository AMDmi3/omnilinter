use assert_cmd::prelude::*;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempdir::TempDir;

pub struct TestCase {
    temp_dir: TempDir,
    last_matches: Vec<Match>,
}

#[derive(Deserialize)]
pub struct Match {
    #[allow(dead_code)]
    message: String,
    #[allow(dead_code)]
    root: String,
    file: Option<String>,
    line: Option<usize>,
}

impl TestCase {
    pub fn new() -> Self {
        let temp_dir = TempDir::new("omniparser-test").unwrap();

        fs::create_dir(temp_dir.path().join("root")).unwrap();

        Self {
            temp_dir: temp_dir,
            last_matches: Default::default(),
        }
    }

    pub fn add_file(&mut self, path: &str, text: &str) -> &mut Self {
        let path = Path::new(path);
        let root_path = self.temp_dir.path().join("root");

        if let Some(parent) = path.ancestors().nth(1) {
            fs::create_dir_all(root_path.join(parent)).unwrap();
        }

        let mut f = File::create(root_path.join(path)).unwrap();

        f.write_all(text.as_bytes()).unwrap();

        self
    }

    pub fn run_with_rule(&mut self, rule: &str) -> &mut Self {
        {
            let mut f = File::create(self.temp_dir.path().join("omnilinter.conf")).unwrap();
            f.write_all("rules:\n".as_bytes()).unwrap();
            f.write_all(rule.as_bytes()).unwrap();
        }

        let mut cmd = Command::cargo_bin("omnilinter").unwrap();

        let res = cmd
            .current_dir(self.temp_dir.path())
            .arg("--config=omnilinter.conf")
            .arg("--json")
            .arg("root")
            .ok();

        if let Ok(output) = res {
            self.last_matches =
                serde_json::from_str(std::str::from_utf8(&output.stdout).unwrap()).unwrap();
        } else {
            self.last_matches = Default::default();
            assert!(false, "omnilinter command failed: {:#?}", res);
        }

        self
    }

    pub fn assert_matches(&mut self, mut expected: Vec<&str>) -> &mut Self {
        let mut res: Vec<String> = self
            .last_matches
            .iter()
            .filter_map(|m| m.file.as_ref().map(|file| (file, &m.line)))
            .map(|(file, line)| match line {
                Some(line) => format!("{file}:{line}"),
                None => file.to_string(),
            })
            .collect();
        res.sort();
        expected.sort();
        assert_eq!(expected, res);

        self
    }
}
