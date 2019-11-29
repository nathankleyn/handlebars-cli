use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

// These tests have to be integration tests as otherwise they cannnot use the
// built binary.

macro_rules! when_binary_run {
    () => {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    };
}

#[test]
fn test_fails_with_usage_if_no_params() {
    when_binary_run!()
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn test_fails_if_only_one_param() {
    when_binary_run!()
        .arg("{}")
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn test_fails_if_properties_are_not_valid_json() {
    when_binary_run!()
        .arg("^_^")
        .arg("/tmp/foo")
        .assert()
        .failure()
        .stderr("Unable to parse properties JSON: expected value at line 1 column 1\n");
}

#[test]
fn test_fails_if_template_file_not_found() {
    when_binary_run!()
        .arg("{}")
        .arg("/tmp/foo")
        .assert()
        .failure()
        .stderr("Unable to read template from '/tmp/foo'.\n");
}

#[test]
fn test_fails_if_template_file_uses_property_not_found() {
    when_binary_run!()
        .arg("{}")
        .arg("/tmp/foo")
        .assert()
        .failure()
        .stderr("Unable to read template from '/tmp/foo'.\n");
}

#[test]
fn test_succeeds_when_noop() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "Hello World!").unwrap();

    when_binary_run!()
        .arg("{}")
        .arg(file.path())
        .assert()
        .success()
        .stdout("Hello World!\n\n");
}


#[test]
fn test_succeeds_when_templating_successful() {
    let mut file = NamedTempFile::new().unwrap();
    // Note that we have to escape the brackets.
    writeln!(file, "Hello {{{{name.first}}}} {{{{name.last}}}}!").unwrap();

    when_binary_run!()
        .arg("{ \"name\": { \"first\": \"Foo\", \"last\": \"Bar\" }}")
        .arg(file.path())
        .assert()
        .success()
        .stdout("Hello Foo Bar!\n\n");
}
