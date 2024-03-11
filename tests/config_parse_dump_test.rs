// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use assert_cmd::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempdir::TempDir;

fn parse_dump_config(input_config: &Path) -> String {
    let output = &Command::cargo_bin("omnilinter")
        .unwrap()
        .arg("--dump-config")
        .arg(input_config)
        .output()
        .unwrap();

    eprint!("{}", std::str::from_utf8(&output.stderr).unwrap());
    if !output.status.success() {
        panic!("omnilinter failed");
    }
    std::str::from_utf8(&output.stdout).unwrap().to_owned()
}

#[test]
fn config_parse_dump_test() {
    let temp_dir = TempDir::new("omnilinter-test").unwrap();

    let input_path = Path::new(".omnilinter.conf");
    let output_path = temp_dir.path().join("omnilinter.conf");

    let first_dump = parse_dump_config(input_path);

    {
        let mut f = File::create(&output_path).unwrap();
        f.write_all(first_dump.as_bytes()).unwrap();
    }

    let second_dump = parse_dump_config(&output_path);

    pretty_assertions::assert_eq!(first_dump, second_dump);
}
