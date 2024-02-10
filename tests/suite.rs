mod utils;

use utils::TestCase;

#[test]
fn simple_matching() {
    TestCase::new()
        .add_file("a.txt", "foo\nbar\nbaz\n")
        .add_file("b.txt", "foo\nbar\nbaz\n")
        .add_file("c.txt", "foo\nbar\nbaz\n")
        .run_with_rule(
            "
            - title: single match
              files: 'a.txt'
              match: 'bar'
            ",
        )
        .assert_matches(vec!["a.txt:2"])
        .run_with_rule(
            "
            - title: multiple files
              files: '*.txt'
              match: 'bar'
            ",
        )
        .assert_matches(vec!["a.txt:2", "b.txt:2", "c.txt:2"])
        .run_with_rule(
            "
            - title: multiple lines
              files: 'a.txt'
              match: '...'
            ",
        )
        .assert_matches(vec!["a.txt:1", "a.txt:2", "a.txt:3"]);
}

#[test]
fn match_file_only() {
    TestCase::new()
        .add_file("a.txt", "")
        .add_file("b.txt", "")
        .add_file("c.txt", "")
        .run_with_rule(
            "
            - title: single match
              files: 'a.txt'
            ",
        )
        .assert_matches(vec!["a.txt"])
        .run_with_rule(
            "
            - title: single match
              files: '*.txt'
            ",
        )
        .assert_matches(vec!["a.txt", "b.txt", "c.txt"]);
}
