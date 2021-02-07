extern crate compiler;

use compiler::{compile, get_io_info, term_color::red_bold, IOInfo};
use std::{
    env, fs,
    process::{self, Command},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_platform();

    let args: Vec<String> = env::args().collect();

    let first_arg = args
        .get(1)
        .unwrap_or_else(|| exit_failure("Please specify a source file"));

    let IOInfo {
        input: input_info,
        output: output_info,
    } = get_io_info(first_arg)?;

    let content = fs::read_to_string(input_info.src_path)
        .unwrap_or_else(|_| exit_failure("Failed to read the source file"));

    let asm_output = compile(&content).unwrap_or_else(|msg| {
        eprintln!("{}", msg);
        process::exit(1);
    });

    fs::write(&output_info.asm_path, asm_output)?;

    let mut nasm_cmd = get_nasm_cmd(vec![output_info.asm_path.to_str().unwrap()]);

    let status = nasm_cmd.status().unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!(
            "{}",
            red_bold("Unable to find `nasm`, perhaps install NASM and set PATH")
        );
        process::exit(1);
    });

    if !status.success() {
        exit_failure("Error occurs while executing `nasm`");
    }

    let mut ld_cmd = get_ld_cmd(vec![
        "-o",
        output_info.bin_path.to_str().unwrap(),
        output_info.obj_path.to_str().unwrap(),
    ]);

    let status = ld_cmd.status().unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!(
            "{}",
            red_bold("Unable to find `ld`, perhaps install Linker and set PATH")
        );
        process::exit(1);
    });

    if !status.success() {
        exit_failure("Error occurs while executing `ld`");
    }

    Ok(())
}

fn check_platform() {
    if cfg!(not(any(target_os = "linux", target_os = "macos"))) {
        exit_failure("This operating system is not supported");
    }

    if cfg!(not(target_arch = "x86_64")) {
        exit_failure("This architecture is not supported.");
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn get_nasm_cmd(_: Vec<&str>) -> Command {
    unreachable!();
}

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn get_nasm_cmd(rest_args: Vec<&str>) -> Command {
    let mut args = vec!["-f", "elf64"];
    args.extend(rest_args);

    let mut cmd = Command::new("nasm");
    cmd.args(args);
    cmd
}

#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
fn get_nasm_cmd(rest_args: Vec<&str>) -> Command {
    let mut args = vec!["-f", "macho64"];
    args.extend(rest_args);

    let mut cmd = Command::new("nasm");
    cmd.args(args);
    cmd
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn get_ld_cmd(_: Vec<&str>) -> Command {
    unreachable!();
}

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn get_ld_cmd(rest_args: Vec<&str>) -> Command {
    let mut args = Vec::<&str>::new();
    args.extend(rest_args);

    let mut cmd = Command::new("ld");
    cmd.args(args);
    cmd
}

#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
fn get_ld_cmd(rest_args: Vec<&str>) -> Command {
    let mut args = vec!["-lSystem"];
    args.extend(rest_args);

    let mut cmd = Command::new("ld");
    cmd.args(args);
    cmd
}

fn exit_failure(msg: &str) -> ! {
    eprintln!("{}", red_bold(msg));
    process::exit(1);
}
