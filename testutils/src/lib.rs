// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(dead_code)]

use assert_cmd::prelude::*;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;
use tempfile::{TempDir, tempdir};

#[macro_export]
macro_rules! lines {
    ($($s:expr),+) => {{
        concat!(
            $($s,'\n',)+
        )
    }};
}

#[macro_export]
macro_rules! paths {
    ($s:expr) => {{
        &$s.replace('/', std::path::MAIN_SEPARATOR_STR) // omnilinter: ignore
    }};
}

#[derive(Deserialize)]
pub struct Match {
    message: String,
    root: String,
    file: Option<String>,
    line: Option<usize>,
}

pub struct TestCase {
    temp_dir: TempDir,
    args: Vec<String>,
    silence_stderr: bool,
}

impl TestCase {
    pub fn new_for_json_tests() -> Self {
        let temp_dir = tempdir().unwrap();

        fs::create_dir(temp_dir.path().join("root")).unwrap();

        Self {
            temp_dir,
            args: vec!["--config=omnilinter.conf", "--format=json", "root"]
                .into_iter()
                .map(|a| a.to_string())
                .collect(),
            silence_stderr: false,
        }
    }

    pub fn new_for_stdout_tests() -> Self {
        let temp_dir = tempdir().unwrap();

        fs::create_dir(temp_dir.path().join("root")).unwrap();

        Self {
            temp_dir,
            args: vec!["--config=omnilinter.conf", "root"]
                .into_iter()
                .map(|a| a.to_string())
                .collect(),
            silence_stderr: false,
        }
    }

    pub fn add_raw_binary_file(&mut self, path: &str, text: &[u8]) -> &mut Self {
        let path = Path::new(path);
        let root_path = self.temp_dir.path();

        if let Some(parent) = path.ancestors().nth(1) {
            fs::create_dir_all(root_path.join(parent)).unwrap();
        }

        let mut f = File::create(root_path.join(path)).unwrap();

        f.write_all(text).unwrap();

        self
    }

    pub fn add_raw_file(&mut self, path: &str, text: &str) -> &mut Self {
        self.add_raw_binary_file(path, text.as_bytes());
        self
    }

    pub fn add_file(&mut self, path: &str, text: &str) -> &mut Self {
        self.add_raw_file(&("root/".to_owned() + path), text);
        self
    }

    pub fn add_binary_file(&mut self, path: &str, text: &[u8]) -> &mut Self {
        self.add_raw_binary_file(&("root/".to_owned() + path), text);
        self
    }

    pub fn add_arg(&mut self, arg: &str) -> &mut Self {
        self.args.push(arg.to_string());

        self
    }

    pub fn silence_stderr(&mut self) -> &mut Self {
        self.silence_stderr = true;

        self
    }

    pub fn add_rule(&mut self, rule: &str) -> &mut Self {
        self.add_named_rule("", rule);
        self
    }

    pub fn add_named_rule(&mut self, name: &str, rule: &str) -> &mut Self {
        self.add_raw_rule(&format!("[{}]\n{}\n", name, rule));
        self
    }

    pub fn add_raw_rule(&mut self, rule: &str) -> &mut Self {
        let ruleset_path = self.temp_dir.path().join("omnilinter.conf");
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(ruleset_path)
            .unwrap();

        file.write_all(rule.as_bytes())
            .unwrap();

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

    pub fn run(&mut self) -> TestRunResult {
        let mut cmd = Command::cargo_bin("omnilinter").unwrap();

        cmd.current_dir(self.temp_dir.path());

        for arg in &self.args {
            cmd.arg(arg);
        }

        let output = cmd.output().unwrap();

        if !self.silence_stderr {
            io::stderr().write_all(&output.stderr).unwrap();
        }

        TestRunResult { output }
    }
}

pub struct TestRunResult {
    output: std::process::Output,
}

impl TestRunResult {
    pub fn assert_matches(&self, expected: Vec<&str>) -> &Self {
        assert!(self.output.status.success());

        let mut res: Vec<String> =
            serde_json::from_str::<Vec<Match>>(std::str::from_utf8(&self.output.stdout).unwrap())
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

        self
    }

    pub fn assert_exit_code(&self, expected: i32) -> &Self {
        assert_eq!(self.output.status.code(), Some(expected));
        self
    }

    pub fn assert_success(&self) -> &Self {
        assert!(self.output.status.success());
        self
    }

    pub fn assert_failure(&self) -> &Self {
        assert!(!self.output.status.success());
        self
    }

    pub fn assert_stderr_contains(&self, sample: &str) -> &Self {
        assert!(from_utf8(&self.output.stderr).unwrap().contains(sample));
        self
    }

    pub fn assert_stdout_contains(&self, sample: &str) -> &Self {
        assert!(from_utf8(&self.output.stdout).unwrap().contains(sample));
        self
    }

    pub fn assert_stdout(&self, expected: &str) -> &Self {
        pretty_assertions::assert_eq!(from_utf8(&self.output.stdout).unwrap(), expected);
        self
    }
}
