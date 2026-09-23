use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};
use std::env;
use std::process::{Command, exit};

enum BootMode {
    Uefi,
    Bios,
}

fn cli() -> clap::Command {
    clap::Command::new("boot")
        .about("Boots a UEFI or BIOS image using QEMU")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            clap::Arg::new("serial")
                .short('s')
                .long("serial")
                .help("Redirects output to stdout/stdio instead of GUI window")
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand(clap::Command::new("uefi").about("Boot in UEFI mode"))
        .subcommand(clap::Command::new("bios").about("Boot in BIOS mode"))
} 

fn main() {
    let cli = cli();
    let matches = cli.get_matches();

    let is_serial = matches.get_flag("serial");

    let boot_mode = match matches.subcommand() {
        Some(("uefi", _)) => BootMode::Uefi,
        Some(("bios", _)) => BootMode::Bios,
        _ => unreachable!("clap should ensure we don't get here"),
    };

    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    let mut cmd = Command::new("qemu-system-x86_64");

    if is_serial {
        cmd.arg("-serial").arg("mon:stdio");
        cmd.arg("-display").arg("none");
    }

    cmd.arg("-device")
        .arg("isa-debug-exit,iobase=0xf4,iosize=0x04");

    if let BootMode::Uefi = boot_mode {
        let prebuilt =
            Prebuilt::fetch(Source::LATEST, "target/ovmf").expect("failed to update prebuilt");

        let code = prebuilt.get_file(Arch::X64, FileType::Code);
        let vars = prebuilt.get_file(Arch::X64, FileType::Vars);

        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
        cmd.arg("-drive").arg(format!(
            "if=pflash,format=raw,unit=0,file={},readonly=on",
            code.display()
        ));
        // copy vars and enable rw instead of snapshot if you want to store data (e.g. enroll secure boot keys)
        cmd.arg("-drive").arg(format!(
            "if=pflash,format=raw,unit=1,file={},snapshot=on",
            vars.display()
        ));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }

    let mut child = cmd.spawn().expect("failed to start qemu-system-x86_64");
    let status = child.wait().expect("failed to wait on qemu");
    let exit_code = match status.code().unwrap_or(1) {
        0x10 => 0, // success
        0x11 => 1, // failure
        _ => 2,    // unknown fault
    };

    exit(exit_code)
}