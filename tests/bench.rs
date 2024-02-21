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

fn match_checks_with_little_matching_rules(c: &mut Criterion) {
    let mut testcase = TestCase::new_for_json_tests();
    testcase.generate_files(1, 100000);
    testcase.add_rule("files 1.txt\nmatch '^1:1$'");
    for i in 2..=100 {
        testcase.add_rule(&format!("files 1.txt\nmatch /^no-1:{i}/"));
    }

    c.bench_function("match checks with little matching rules", |b| {
        b.iter(|| {
            testcase.run();
        })
    });
}

fn match_checks_with_many_matching_rules(c: &mut Criterion) {
    let mut testcase = TestCase::new_for_json_tests();
    testcase.generate_files(1, 20000);
    for i in 1..=100 {
        testcase.add_rule(&format!("files 1.txt\nmatch /^1:{i}/"));
    }

    c.bench_function("match checks with many matching rules", |b| {
        b.iter(|| {
            testcase.run();
        })
    });
}

fn nomatch_checks_with_little_matching_rules(c: &mut Criterion) {
    let mut testcase = TestCase::new_for_json_tests();
    testcase.generate_files(1, 100000);
    for i in 1..=100 {
        testcase.add_rule(&format!("files 1.txt\nnomatch /^no-1:{i}/"));
    }

    c.bench_function("nomatch checks with little matching rules", |b| {
        b.iter(|| {
            testcase.run();
        })
    });
}

fn nomatch_checks_with_many_matching_rules(c: &mut Criterion) {
    let mut testcase = TestCase::new_for_json_tests();
    testcase.generate_files(1, 20000);
    for i in 1..=100 {
        testcase.add_rule(&format!("files 1.txt\nnomatch /^1:{i}/"));
    }

    c.bench_function("nomatch checks with many matching rules", |b| {
        b.iter(|| {
            testcase.run();
        })
    });
}

criterion_group!(
    name = file_benches;
    config = Criterion::default().sample_size(25);
    targets =
    file_checks_with_little_matching_rules,
    file_checks_with_many_matching_rules,
    file_checks_with_many_matching_rules_with_same_pattern,
    nofile_checks_with_little_matching_rules,
    nofile_checks_with_many_matching_rules,
);

criterion_group!(
    name = match_benches;
    config = Criterion::default().sample_size(25);
    targets =
    match_checks_with_little_matching_rules,
    match_checks_with_many_matching_rules,
    nomatch_checks_with_little_matching_rules,
    nomatch_checks_with_many_matching_rules,
);

criterion_main!(file_benches, match_benches);
