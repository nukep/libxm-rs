#![feature(env, core, path)]
extern crate gcc;

fn main() {
    fn parse_env(key: &str, default: bool) -> bool {
        use std::env::{var_string, VarError};

        match var_string(key) {
            Ok(val) => {
                match val.as_slice() {
                    "0" => false,
                    "1" => true,
                    _ => default
                }
            },
            Err(VarError::NotPresent) => default,
            Err(VarError::NotUnicode(_)) => panic!("Environment variable is not unicode: {}", key),
        }
    }

    let linear_interpolation = parse_env("XM_LINEAR_INTERPOLATION", true);
    let ramping = parse_env("XM_RAMPING", true);
    let debug = parse_env("XM_DEBUG", false);
    let big_endian = parse_env("XM_BIG_ENDIAN", false);

    fn on_off(value: bool) -> Option<String> {
        Some(if value { "1" } else { "0" }.to_string())
    }

    let config = gcc::Config {
        include_directories: vec![Path::new("libxm/include")],
        definitions: vec![
            ("XM_LINEAR_INTERPOLATION".to_string(), on_off(linear_interpolation)),
            ("XM_RAMPING".to_string(), on_off(ramping)),
            ("XM_DEBUG".to_string(), on_off(debug)),
            ("XM_BIG_ENDIAN".to_string(), on_off(big_endian)),
        ],
        objects: vec![],
        flags: vec!["--std=c11".to_string()]
    };

    gcc::compile_library(
        "libxm.a",
        &config,
        &["libxm/src/context.c", "libxm/src/load.c", "libxm/src/play.c", "libxm/src/xm.c"]
    );
}
