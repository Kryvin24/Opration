// cargo.exe shim: strips the Win32 verbatim path prefix from args and
// environment before delegating to the real cargo. The dbx-plugin packager
// canonicalizes its temp build dir to a verbatim path, which mingw-w64 ld in the
// GNU toolchain cannot parse (ld: cannot find symbols.o).
use std::process::Command;

const TOML_ESCAPED_PREFIX: &str = r"\\\\?\\"; // TOML basic-string source for the verbatim prefix
const RAW_PREFIX: &str = r"\\?\"; // raw Win32 verbatim prefix

fn fix(input: &str) -> String {
    // Handle the TOML-escaped form first, then the raw one. No UNC targets.
    input.replace(TOML_ESCAPED_PREFIX, "").replace(RAW_PREFIX, "")
}

fn main() {
    let real_cargo = r"C:\Users\zdiai\.cargo\bin\cargo.exe";

    let args: Vec<String> = std::env::args().skip(1).map(|a| fix(&a)).collect();

    let mut cmd = Command::new(real_cargo);
    cmd.args(&args);
    for (key, value) in std::env::vars() {
        cmd.env(key, fix(&value));
    }

    let status = cmd.status().expect("shim: failed to run real cargo");
    std::process::exit(status.code().unwrap_or(1));
}
