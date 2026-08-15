//! End-to-end tests for the `systemrdl-fmt` command.
//!
//! What these are for is the behaviour that is not the formatter: which files
//! get touched, what the exit status is, and where the output goes. The
//! formatting itself is covered by `invariants.rs`.
//!
//! Exit status is the part worth being strict about, because it is the whole
//! interface to CI: 0 clean, 1 needs formatting, 2 something went wrong.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const EXE: &str = env!("CARGO_BIN_EXE_systemrdl-fmt");

const UNFORMATTED: &str = "addrmap a{name=\"x\";};\n";
const FORMATTED: &str = "addrmap a {\n    name = \"x\";\n};\n";

/// A scratch directory of its own, so tests can run in parallel.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> TempDir {
        // Enough to be unique across a parallel run without a dependency:
        // the tag is per-test and the pid is per-run.
        let path = std::env::temp_dir().join(format!("systemrdl-fmt-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn write(&self, name: &str, contents: &str) -> PathBuf {
        let path = self.0.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create parent");
        }
        std::fs::write(&path, contents).expect("write file");
        path
    }

    fn read(&self, name: &str) -> String {
        std::fs::read_to_string(self.0.join(name)).expect("read file")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn run(args: &[&str]) -> Output {
    Command::new(EXE).args(args).output().expect("run binary")
}

fn run_in(dir: &Path, args: &[&str]) -> Output {
    Command::new(EXE)
        .current_dir(dir)
        .args(args)
        .output()
        .expect("run binary")
}

fn pipe(stdin: &str, args: &[&str]) -> Output {
    let mut child = Command::new(EXE)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn binary");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(stdin.as_bytes())
        .expect("write stdin");
    child.wait_with_output().expect("wait")
}

#[track_caller]
fn code(output: &Output) -> i32 {
    output.status.code().expect("exit code")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

//--------------------------------------------------------------------------
// stdin
//--------------------------------------------------------------------------

#[test]
fn no_paths_means_stdin_to_stdout() {
    let out = pipe(UNFORMATTED, &[]);
    assert_eq!(code(&out), 0);
    assert_eq!(stdout(&out), FORMATTED);
}

#[test]
fn check_on_stdin_reports_without_writing_it_out() {
    let out = pipe(UNFORMATTED, &["--check"]);
    assert_eq!(code(&out), 1);
    assert!(!stdout(&out).contains("addrmap"), "formatted text leaked");
}

//--------------------------------------------------------------------------
// Writing, which is the default
//--------------------------------------------------------------------------

#[test]
fn a_path_is_rewritten_in_place() {
    let dir = TempDir::new("write");
    let path = dir.write("a.rdl", UNFORMATTED);

    let out = run(&[path.to_str().unwrap()]);
    assert_eq!(code(&out), 0);
    assert_eq!(dir.read("a.rdl"), FORMATTED);
}

#[test]
fn an_already_formatted_file_is_left_alone() {
    // Not merely "ends up with the same bytes": the file must not be written
    // at all, or formatting a tree would touch every mtime and trigger a
    // rebuild of everything downstream.
    let dir = TempDir::new("untouched");
    let path = dir.write("a.rdl", FORMATTED);
    let before = std::fs::metadata(&path).unwrap().modified().unwrap();

    let out = run(&[path.to_str().unwrap()]);
    assert_eq!(code(&out), 0);
    assert_eq!(stdout(&out), "", "an untouched file should not be reported");

    let after = std::fs::metadata(&path).unwrap().modified().unwrap();
    assert_eq!(before, after, "file was rewritten");
}

#[test]
fn stdout_mode_does_not_touch_the_file() {
    let dir = TempDir::new("stdout");
    let path = dir.write("a.rdl", UNFORMATTED);

    let out = run(&["--stdout", path.to_str().unwrap()]);
    assert_eq!(code(&out), 0);
    assert_eq!(stdout(&out), FORMATTED);
    assert_eq!(dir.read("a.rdl"), UNFORMATTED, "file was modified");
}

//--------------------------------------------------------------------------
// --check
//--------------------------------------------------------------------------

#[test]
fn check_leaves_the_file_alone_and_exits_one() {
    let dir = TempDir::new("check-dirty");
    let path = dir.write("a.rdl", UNFORMATTED);

    let out = run(&["--check", path.to_str().unwrap()]);
    assert_eq!(code(&out), 1);
    assert_eq!(dir.read("a.rdl"), UNFORMATTED, "check modified the file");
    assert!(stdout(&out).contains("a.rdl"), "got: {}", stdout(&out));
}

#[test]
fn check_exits_zero_when_everything_is_formatted() {
    let dir = TempDir::new("check-clean");
    let path = dir.write("a.rdl", FORMATTED);

    let out = run(&["--check", path.to_str().unwrap()]);
    assert_eq!(code(&out), 0);
    assert_eq!(stdout(&out), "");
}

//--------------------------------------------------------------------------
// Directories and failures
//--------------------------------------------------------------------------

#[test]
fn a_directory_is_searched_for_rdl_files() {
    let dir = TempDir::new("dir");
    dir.write("a.rdl", UNFORMATTED);
    dir.write("nested/b.rdl", UNFORMATTED);
    dir.write("notes.txt", "not rdl, must be left alone\n");
    dir.write(".hidden/c.rdl", UNFORMATTED);

    let out = run_in(&dir.0, &["."]);
    assert_eq!(code(&out), 0);
    assert_eq!(dir.read("a.rdl"), FORMATTED);
    assert_eq!(dir.read("nested/b.rdl"), FORMATTED);
    assert_eq!(dir.read("notes.txt"), "not rdl, must be left alone\n");
    assert_eq!(
        dir.read(".hidden/c.rdl"),
        UNFORMATTED,
        "hidden directories should be skipped"
    );
}

#[test]
fn a_file_that_does_not_parse_is_left_alone() {
    let dir = TempDir::new("broken");
    let path = dir.write("a.rdl", "addrmap a {\n");

    let out = run(&[path.to_str().unwrap()]);
    assert_eq!(code(&out), 2);
    assert_eq!(
        dir.read("a.rdl"),
        "addrmap a {\n",
        "broken file was written"
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("a.rdl:"), "no located error in: {stderr}");
}

#[test]
fn one_bad_file_does_not_stop_the_others() {
    let dir = TempDir::new("mixed");
    dir.write("good.rdl", UNFORMATTED);
    dir.write("bad.rdl", "addrmap a {\n");

    let out = run_in(&dir.0, &["."]);
    assert_eq!(code(&out), 2, "a failure anywhere is exit 2");
    assert_eq!(dir.read("good.rdl"), FORMATTED, "good file was skipped");
}

#[test]
fn a_missing_path_is_an_error() {
    let out = run(&["/nonexistent/nope.rdl"]);
    assert_eq!(code(&out), 2);
}

//--------------------------------------------------------------------------
// Arguments
//--------------------------------------------------------------------------

#[test]
fn indent_width_is_configurable() {
    let out = pipe(UNFORMATTED, &["--indent", "2"]);
    assert_eq!(code(&out), 0);
    assert_eq!(stdout(&out), "addrmap a {\n  name = \"x\";\n};\n");
}

#[test]
fn help_and_version_succeed() {
    for flag in ["--help", "-h", "--version", "-V"] {
        let out = run(&[flag]);
        assert_eq!(code(&out), 0, "{flag}");
        assert!(!stdout(&out).is_empty(), "{flag} printed nothing");
    }
}

#[test]
fn an_unknown_option_is_rejected() {
    let out = run(&["--nope"]);
    assert_eq!(code(&out), 2, "a usage error is exit 2, like any other");
    // The offending flag, not clap's phrasing, which is not ours to pin.
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("--nope"), "got: {stderr}");
}

#[test]
fn check_and_stdout_conflict() {
    // Writing nothing and writing to stdout cannot both be meant.
    let out = run(&["--check", "--stdout"]);
    assert_eq!(code(&out), 2);
}

#[test]
fn double_dash_ends_the_options() {
    let dir = TempDir::new("ddash");
    // A file whose name looks like a flag is still a path after `--`.
    let path = dir.write("--check", UNFORMATTED);

    let out = run(&["--", path.to_str().unwrap()]);
    assert_eq!(code(&out), 0);
    assert_eq!(dir.read("--check"), FORMATTED);
}
