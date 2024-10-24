use std::env;
use std::fs::File;
use std::io::{self};
use std::os::unix::io::AsRawFd;
use std::process::Command;
use nix::sched::setns;
use nix::unistd::{chroot, fchdir};
use std::os::unix::process::CommandExt;

fn main() -> io::Result<()> {
    let shell = "/bin/sh";
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Using default shell: {}", shell);
        args.push(shell.to_string());
    }

    let fdm = File::open("/proc/1/ns/mnt")?;
    let fdu = File::open("/proc/1/ns/uts")?;
    let fdn = File::open("/proc/1/ns/net")?;
    let fdi = File::open("/proc/1/ns/ipc")?;
    let froot = File::open("/proc/1/root")?;

    if let Err(e) = setns(fdm, nix::sched::CloneFlags::empty()) {
        eprintln!("setns:mnt: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = setns(fdu, nix::sched::CloneFlags::empty()) {
        eprintln!("setns:uts: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = setns(fdn, nix::sched::CloneFlags::empty()) {
        eprintln!("setns:net: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = setns(fdi, nix::sched::CloneFlags::empty()) {
        eprintln!("setns:ipc: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = fchdir(froot.as_raw_fd()) {
        eprintln!("fchdir: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = chroot(".") {
        eprintln!("chroot: {}", e);
        std::process::exit(1);
    }

    let cmd = &args[1];
    let args = if args.len() > 2 {
        args[2..].to_vec()
    } else {
        vec![]
    };
    println!("exec cmd: {} with args: {}", cmd, args.join(" "));

    let err = Command::new(cmd)
        .args(args)
        .envs(env::vars())
        .exec();

    eprintln!("execve: {}", err);
    std::process::exit(err.raw_os_error().unwrap_or(0));
}