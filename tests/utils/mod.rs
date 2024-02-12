// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

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
    extra_args: Vec<String>,
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
        let temp_dir = TempDir::new("omnilinter-test").unwrap();

        fs::create_dir(temp_dir.path().join("root")).unwrap();

        Self {
            temp_dir: temp_dir,
            last_matches: Default::default(),
            extra_args: Default::default(),
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

    pub fn add_arg(&mut self, arg: &str) -> &mut Self {
        self.extra_args.push(arg.to_string());

        self
    }

    pub fn run_with_rule(&mut self, rule: &str) -> &mut Self {
        {
            let mut f = File::create(self.temp_dir.path().join("omnilinter.conf")).unwrap();
            f.write_all("rules:\n".as_bytes()).unwrap();
            f.write_all(rule.as_bytes()).unwrap();
        }

        let mut cmd = Command::cargo_bin("omnilinter").unwrap();

        cmd.current_dir(self.temp_dir.path())
            .arg("--config=omnilinter.conf")
            .arg("--json")
            .arg("root");

        for arg in &self.extra_args {
            cmd.arg(arg);
        }

        let res = cmd.ok();

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
            .map(|m| match (&m.file, &m.line) {
                (Some(file), Some(line)) => format!("{file}:{line}"),
                (Some(file), None) => file.to_string(),
                (None, None) => Default::default(),
                _ => panic!("line number without file cannot happen"),
            })
            .collect();
        res.sort();
        expected.sort();
        assert_eq!(expected, res);

        self
    }
}
