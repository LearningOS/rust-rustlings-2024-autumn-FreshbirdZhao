// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.

use std::env;

fn main() {
    // 获取环境变量 TEST_FOO
    let test_foo = env::var("TEST_FOO").unwrap();

    // 将环境变量的值解析为 u64 类型
    let test_foo_value: u64 = test_foo.parse().expect("TEST_FOO must be a number");

    // 获取当前的 Unix 时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 检查 TEST_FOO 的值是否在当前时间前后10秒内
    if now >= test_foo_value && now < test_foo_value + 10 {
        // 如果在范围内，告诉 Cargo 继续构建过程
        println!("cargo:rerun-if-env-changed=TEST_FOO");
    } else {
        // 如果不在范围内，输出错误信息并退出构建过程
        panic!("TEST_FOO is not within 10 seconds of the current time.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
