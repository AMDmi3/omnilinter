// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use criterion::{criterion_group, criterion_main, Criterion};
use utils::TestCase;

fn file_checks(c: &mut Criterion) {
    let mut testcase = TestCase::new();
    testcase
        .generate_files(1000, 1)
        .add_rule("- files: 1.txt")
        .add_rule("- files: no2,txt")
        .add_rule("- files: no3,txt")
        .add_rule("- files: no4,txt")
        .add_rule("- files: no5,txt")
        .add_rule("- files: no6,txt")
        .add_rule("- files: no7,txt")
        .add_rule("- files: no8,txt")
        .add_rule("- files: no9,txt")
        .add_rule("- files: no10,txt");

    c.bench_function("file checks", |b| {
        b.iter(|| testcase.run_assert_matches(vec!["1.txt"]))
    });
}

fn pattern_checks(c: &mut Criterion) {
    let mut testcase = TestCase::new();
    testcase
        .generate_files(1, 20000)
        .add_rule("- files: 1.txt\n  match: '^1:1$'")
        .add_rule("- files: 1.txt\n  match: '^no2:2$'")
        .add_rule("- files: 1.txt\n  match: '^no3:3$'")
        .add_rule("- files: 1.txt\n  match: '^no4:4$'")
        .add_rule("- files: 1.txt\n  match: '^no5:5$'")
        .add_rule("- files: 1.txt\n  match: '^no6:6$'")
        .add_rule("- files: 1.txt\n  match: '^no7:7$'")
        .add_rule("- files: 1.txt\n  match: '^no8:8$'")
        .add_rule("- files: 1.txt\n  match: '^no9:9$'")
        .add_rule("- files: 1.txt\n  match: '^no10:10$'");

    c.bench_function("pattern checks", |b| {
        b.iter(|| testcase.run_assert_matches(vec!["1.txt:1"]))
    });
}

criterion_group!(benches, file_checks, pattern_checks);
criterion_main!(benches);
