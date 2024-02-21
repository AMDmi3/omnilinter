// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

mod utils;

use criterion::{criterion_group, criterion_main, Criterion};
use utils::TestCase;

fn file_checks_with_little_matching_rules(c: &mut Criterion) {
    let mut testcase = TestCase::new_for_json_tests();
    testcase.generate_files(10000, 1);
    testcase.add_rule("files 1.txt");
    for i in 2..=100 {
        testcase.add_rule(&format!("files no-{i}.txt"));
    }

    c.bench_function("file checks when little matching rules", |b| {
        b.iter(|| {
            testcase.run().assert_matches(vec!["1.txt"]);
        })
    });
}

fn file_checks_with_many_matching_rules(c: &mut Criterion) {
    let mut testcase = TestCase::new_for_json_tests();
    testcase.generate_files(10000, 1);
    for i in 1..=100 {
        testcase.add_rule(&format!("files {i}.txt"));
    }

    c.bench_function("file checks when many matching rules", |b| {
        b.iter(|| {
            testcase.run();
        })
    });
}

fn file_checks_with_many_matching_rules_with_same_pattern(c: &mut Criterion) {
    let mut testcase = TestCase::new_for_json_tests();
    testcase.generate_files(10000, 1);
    for _ in 1..=100 {
        testcase.add_rule(&format!("files 1.txt"));
    }

    c.bench_function(
        "file checks when many matching rules with same pattern",
        |b| {
            b.iter(|| {
                testcase.run();
            })
        },
    );
}

fn nofile_checks_with_little_matching_rules(c: &mut Criterion) {
    let mut testcase = TestCase::new_for_json_tests();
    testcase.generate_files(10000, 1);
    for i in 1..=100 {
        testcase.add_rule(&format!("nofiles no-{i}.txt"));
    }

    c.bench_function("nofile checks when little matching rules", |b| {
        b.iter(|| {
            testcase.run();
        })
    });
}

fn nofile_checks_with_many_matching_rules(c: &mut Criterion) {
    let mut testcase = TestCase::new_for_json_tests();
    testcase.generate_files(10000, 1);
    for i in 1..=100 {
        testcase.add_rule(&format!("nofiles {i}.txt"));
    }

    c.bench_function("nofile checks when many matching rules", |b| {
        b.iter(|| {
            testcase.run();
        })
    });
}

fn pattern_checks(c: &mut Criterion) {
    let mut testcase = TestCase::new_for_json_tests();
    testcase
        .generate_files(1, 20000)
        .add_rule("files 1.txt\nmatch '^1:1$'")
        .add_rule("files 1.txt\nmatch '^no2:2$'")
        .add_rule("files 1.txt\nmatch '^no3:3$'")
        .add_rule("files 1.txt\nmatch '^no4:4$'")
        .add_rule("files 1.txt\nmatch '^no5:5$'")
        .add_rule("files 1.txt\nmatch '^no6:6$'")
        .add_rule("files 1.txt\nmatch '^no7:7$'")
        .add_rule("files 1.txt\nmatch '^no8:8$'")
        .add_rule("files 1.txt\nmatch '^no9:9$'")
        .add_rule("files 1.txt\nmatch '^no10:10$'");

    c.bench_function("pattern checks", |b| {
        b.iter(|| {
            testcase.run().assert_matches(vec!["1.txt:1"]);
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(25);
    targets =
    file_checks_with_little_matching_rules,
    file_checks_with_many_matching_rules,
    file_checks_with_many_matching_rules_with_same_pattern,
    nofile_checks_with_little_matching_rules,
    nofile_checks_with_many_matching_rules,
    pattern_checks
);
criterion_main!(benches);
