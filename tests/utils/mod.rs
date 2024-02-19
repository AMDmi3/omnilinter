// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use assert_cmd::prelude::*;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempdir::TempDir;

pub struct TestCase {
    temp_dir: TempDir,
    extra_args: Vec<String>,
    had_asserts: bool,
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

#[allow(dead_code)]
impl TestCase {
    pub fn new() -> Self {
        let temp_dir = TempDir::new("omnilinter-test").unwrap();

        fs::create_dir(temp_dir.path().join("root")).unwrap();

        Self {
            temp_dir: temp_dir,
            extra_args: Default::default(),
            had_asserts: false,
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

    pub fn add_rule(&mut self, rule: &str) -> &mut Self {
        let ruleset_path = self.temp_dir.path().join("omnilinter.conf");
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(ruleset_path)
            .unwrap();

        file.write_all(b"[]\n").unwrap();
        file.write_all(rule.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();

        self
    }

    pub fn generate_files(&mut self, num_files: usize, num_lines: usize) -> &mut Self {
        for num_file in 1..=num_files {
            let mut file = File::create(
                self.temp_dir
                    .path()
                    .join("root")
                    .join(format!("{num_file}.txt")),
            )
            .unwrap();
            for num_line in 1..=num_lines {
                writeln!(file, "{num_file}:{num_line}").unwrap();
            }
            file.sync_all().unwrap();
        }

        self
    }

    fn run(&self) -> std::process::Output {
        let mut cmd = Command::cargo_bin("omnilinter").unwrap();

        cmd.current_dir(self.temp_dir.path())
            .arg("--config=omnilinter.conf")
            .arg("--json")
            .arg("root");

        for arg in &self.extra_args {
            cmd.arg(arg);
        }

        let res = cmd.output().unwrap();
        io::stderr().write_all(&res.stderr).unwrap();
        res
    }

    pub fn run_no_assert(&mut self) {
        self.had_asserts = true;
        self.run();
    }

    pub fn run_assert_matches(&mut self, expected: Vec<&str>) {
        self.had_asserts = true;
        let output = self.run();
        assert!(output.status.success());

        let mut res: Vec<String> =
            serde_json::from_str::<Vec<Match>>(std::str::from_utf8(&output.stdout).unwrap())
                .unwrap()
                .iter()
                .map(|m| match (&m.file, &m.line) {
                    (Some(file), Some(line)) => format!("{file}:{line}"),
                    (Some(file), None) => file.to_string(),
                    (None, None) => Default::default(),
                    _ => panic!("line number without file cannot happen"),
                })
                .collect();
        res.sort();

        let mut expected = expected.clone();
        expected.sort();

        assert_eq!(res, expected);
    }

    pub fn run_assert_exit_code(&mut self, expected: i32) {
        self.had_asserts = true;
        let output = self.run();
        assert_eq!(output.status.code(), Some(expected));
    }

    pub fn run_assert_success(&mut self) {
        self.had_asserts = true;
        let output = self.run();
        assert!(output.status.success());
    }

    pub fn run_assert_failure(&mut self) {
        self.had_asserts = true;
        let output = self.run();
        assert!(!output.status.success());
    }
}

impl Drop for TestCase {
    fn drop(&mut self) {
        if !self.had_asserts {
            panic!("test case had no asserts");
        }
    }
}
