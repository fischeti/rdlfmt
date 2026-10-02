//! Byte-for-byte snapshots of the formatter's output.
//!
//! Each file in `golden/input` is deliberately unformatted; its counterpart in
//! `golden/expected` is what `format` must produce from it. Where
//! `the_output_style_is_a_fixed_point` only shows that formatted input stays
//! put, these pin down how unformatted input is brought there.
//!
//! After a deliberate style change, regenerate the expected files with
//! `RDLFMT_BLESS=1 cargo test --test golden` and review the diff.

use std::fs;
use std::path::Path;

#[test]
fn golden() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden");
    let bless = std::env::var_os("RDLFMT_BLESS").is_some();

    let mut inputs: Vec<_> = fs::read_dir(root.join("input"))
        .expect("read golden/input")
        .map(|entry| entry.expect("read entry").path())
        .collect();
    inputs.sort();
    assert!(!inputs.is_empty(), "no golden inputs found");

    let mut failures = Vec::new();
    for input in &inputs {
        let name = input.file_name().expect("file name");
        let expected_path = root.join("expected").join(name);
        let src = fs::read_to_string(input).expect("read input");
        let out = rdlfmt::format(&src).unwrap_or_else(|err| panic!("{}: {err}", input.display()));

        if bless {
            fs::write(&expected_path, &out).expect("write expected");
            continue;
        }

        let expected = fs::read_to_string(&expected_path)
            .unwrap_or_else(|_| panic!("missing {}", expected_path.display()));
        if out != expected {
            failures.push(format!(
                "{}\n--- expected ---\n{expected}\n--- actual ---\n{out}",
                name.display()
            ));
        }
        if rdlfmt::format(&expected).as_deref() != Ok(expected.as_str()) {
            failures.push(format!(
                "{}: expected output is not a fixed point",
                name.display()
            ));
        }
    }

    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}
