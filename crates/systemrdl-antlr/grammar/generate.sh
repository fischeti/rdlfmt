#!/usr/bin/env bash
# Regenerate the Rust lexer/parser/listener/visitor from SystemRDL.g4.
#
# The Rust target lives in the bundled ANTLR snapshot jar -- stock ANTLR does
# not ship a Rust backend. The jar is the one from the antlr4rust fork's v0.5.0
# release, and must stay in step with the antlr4rust crate version in Cargo.toml
# (the generated code calls `check_version("0", "5")`).
#
#   https://github.com/antlr4rust/antlr4/releases
#
# Needs a JRE. Set JAVA to point at one if `java` is not on PATH, e.g.
#   JAVA=/opt/homebrew/opt/openjdk/bin/java ./generate.sh
set -euo pipefail

cd "$(dirname "$0")"

"${JAVA:-java}" -jar antlr4-4.13.3-SNAPSHOT-complete.jar \
    -Dlanguage=Rust \
    -listener \
    -visitor \
    -o ../src/parser \
    SystemRDL.g4

# ANTLR writes the .interp/.tokens sidecars next to the .rs output; keep them
# here in grammar/ instead, since they are grammar metadata, not Rust code.
mv -f ../src/parser/*.interp ../src/parser/*.tokens . 2>/dev/null || true

# -listener/-visitor also emit base listener/visitor traits. Those exist for the
# Java target, where the interface has no method bodies and you subclass a base
# class to override just the hooks you want. Rust traits already have default
# method bodies, so SystemRDLListener/SystemRDLVisitor serve that purpose
# directly and the base traits are pure duplication.
#
# They are also unusable: the parser only emits `impl Listenable<dyn
# SystemRDLListener>`, so passing a base-trait impl to SystemRDLTreeWalker::walk
# fails to compile with "the trait bound `_: CoerceTo<dyn SystemRDLListener>` is
# not satisfied". Their hook names for labeled alternatives disagree with the
# real listener too (`enter_binaryexpr` vs `enter_BinaryExpr`).
#
# Deleting them keeps ~1600 lines of dead code out of the build. If a future
# antlr4rust release wires them up, drop these two lines and add the modules
# back to src/parser/mod.rs.
rm -f ../src/parser/systemrdlbaselistener.rs ../src/parser/systemrdlbasevisitor.rs

# The checked-in generated sources are kept rustfmt-clean so that
# `cargo fmt -- --check` passes over the whole crate.
cargo fmt --manifest-path ../Cargo.toml

echo "Regenerated src/parser/*.rs from SystemRDL.g4."
