use std::fs::{self, File, FileType};
use std::io::{self, prelude::*};
use std::os::arceos::prelude::FileTypeExt;
use std::os::arceos::fs::PermissionsExt;

macro_rules! print_err {
    ($cmd: literal, $msg: literal) => {
        println!("{}: {}", $cmd, $msg);
    };
    ($cmd: literal, $err: ident) => {
        use std::io::Error::*;
        println!("{}: {}", $cmd, $err.to_string());
    };
    ($cmd: literal, $arg: expr, $err: ident) => {
        use std::io::Error::*;
        println!("{}: {}: {}", $cmd, $arg, $err.to_string());
    };
    ($cmd: literal, $arg: expr, $msg: expr) => {
        println!("{}: {}: {}", $cmd, $arg, $msg);
    };
}

type CmdHandler = fn(&str);

const CMD_TABLE: &[(&str, CmdHandler)] = &[
    ("cat", do_cat),
    ("cd", do_cd),
    ("echo", do_echo),
    ("exit", do_exit),
    ("help", do_help),
    ("ls", do_ls),
    ("mkdir", do_mkdir),
    ("pwd", do_pwd),
    ("rm", do_rm),
    ("uname", do_uname),
];

fn file_type_char(ft: FileType) -> char {
    if ft.is_dir() {
        'd'
    } else if ft.is_file() {
        '-'
    } else if ft.is_symlink() {
        'l'
    } else if ft.is_fifo() {
        'p'
    } else if ft.is_char_device() {
        'c'
    } else if ft.is_block_device() {
        'b'
    } else if ft.is_socket() {
        's'
    } else {
        ' '
    }
}

/// Owner has read permission.
const OWNER_READ: u32 = 0o400;
/// Owner has write permission.
const OWNER_WRITE: u32 = 0o200;
/// Owner has execute permission.
const OWNER_EXEC: u32 = 0o100;

/// Group has read permission.
const GROUP_READ: u32 = 0o40;
/// Group has write permission.
const GROUP_WRITE: u32 = 0o20;
/// Group has execute permission.
const GROUP_EXEC: u32 = 0o10;

/// Others have read permission.
const OTHER_READ: u32 = 0o4;
/// Others have write permission.
const OTHER_WRITE: u32 = 0o2;
/// Others have execute permission.
const OTHER_EXEC: u32 = 0o1;

/// Returns a 9-bytes string representation of the permission.
///
/// For example, `0o755` is represented as `rwxr-xr-x`.
fn rwx_buf(v: u32) -> [u8; 9] {
    let contains = |bit| -> bool {
        (v & bit) == bit
    };

    let mut perm = [b'-'; 9];
    if contains(OWNER_READ) {
        perm[0] = b'r';
    }
    if contains(OWNER_WRITE) {
        perm[1] = b'w';
    }
    if contains(OWNER_EXEC) {
        perm[2] = b'x';
    }
    if contains(GROUP_READ) {
        perm[3] = b'r';
    }
    if contains(GROUP_WRITE) {
        perm[4] = b'w';
    }
    if contains(GROUP_EXEC) {
        perm[5] = b'x';
    }
    if contains(OTHER_READ) {
        perm[6] = b'r';
    }
    if contains(OTHER_WRITE) {
        perm[7] = b'w';
    }
    if contains(OTHER_EXEC) {
        perm[8] = b'x';
    }
    perm
}

fn do_ls(args: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let args = if args.is_empty() {
        current_dir.to_str().unwrap()
    } else {
        args
    };
    let name_count = args.split_whitespace().count();

    fn show_entry_info(path: &str, entry: &str) -> io::Result<()> {
        let metadata = std::fs::metadata(path)?;
        let size = metadata.len();
        let file_type = metadata.file_type();
        let file_type_char = file_type_char(file_type);
        let rwx = rwx_buf(metadata.permissions().mode());
        let rwx = unsafe { core::str::from_utf8_unchecked(&rwx) };
        println!("{}{} {:>8} {}", file_type_char, rwx, size, entry);
        Ok(())
    }

    fn list_one(name: &str, print_name: bool) -> io::Result<()> {
        let is_dir = std::fs::metadata(name)?.is_dir();
        if !is_dir {
            return show_entry_info(name, name);
        }

        if print_name {
            println!("{}:", name);
        }
        let mut entries = std::fs::read_dir(name)?
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_os_string())
            .collect::<Vec<_>>();
        entries.sort();

        for entry in entries {
            let path = String::from(name) + "/" + entry.to_str().unwrap();
            if let Err(e) = show_entry_info(&path, entry.to_str().unwrap()) {
                print_err!("ls", path, e.to_string());
            }
        }
        Ok(())
    }

    for (i, name) in args.split_whitespace().enumerate() {
        if i > 0 {
            println!();
        }
        if let Err(e) = list_one(name, name_count > 1) {
            print_err!("ls", name, e.to_string());
        }
    }
}

fn do_cat(args: &str) {
    if args.is_empty() {
        print_err!("cat", "no file specified");
        return;
    }

    fn cat_one(fname: &str) -> io::Result<()> {
        let mut buf = [0; 1024];
        let mut file = File::open(fname)?;
        loop {
            let n = file.read(&mut buf)?;
            if n > 0 {
                std::io::stdout().write(&buf[..n])?;
            } else {
                return Ok(());
            }
        }
    }

    for fname in args.split_whitespace() {
        if let Err(e) = cat_one(fname) {
            print_err!("cat", fname, e.to_string());
        }
    }
}

fn do_echo(args: &str) {
    fn echo_file(fname: &str, text_list: &[&str]) -> io::Result<()> {
        println!("echo 1 {} {:?}", fname, text_list);
        let mut file = File::create(fname)?;
        println!("echo 2");
        for text in text_list {
            file.write_all(text.as_bytes())?;
        }
        Ok(())
    }

    if let Some(pos) = args.rfind('>') {
        let text_before = args[..pos].trim();
        let (fname, text_after) = split_whitespace(&args[pos + 1..]);
        if fname.is_empty() {
            print_err!("echo", "no file specified");
            return;
        };

        let text_list = [
            text_before,
            if !text_after.is_empty() { " " } else { "" },
            text_after,
            "\n",
        ];
        if let Err(e) = echo_file(fname, &text_list) {
            print_err!("echo", fname, e.to_string());
        }
    } else {
        println!("{}", args)
    }
}

fn do_mkdir(args: &str) {
    if args.is_empty() {
        print_err!("mkdir", "missing operand");
        return;
    }

    fn mkdir_one(path: &str) -> io::Result<()> {
        std::fs::create_dir(path)
    }

    for path in args.split_whitespace() {
        if let Err(e) = mkdir_one(path) {
            print_err!(
                "mkdir",
                format_args!("cannot create directory '{path}'"),
                e.to_string()
            );
        }
    }
}

fn do_rm(args: &str) {
    if args.is_empty() {
        print_err!("rm", "missing operand");
        return;
    }
    let mut rm_dir = false;
    for arg in args.split_whitespace() {
        if arg == "-d" {
            rm_dir = true;
        }
    }

    fn rm_one(path: &str, rm_dir: bool) -> io::Result<()> {
        if rm_dir && fs::metadata(path)?.is_dir() {
            std::fs::remove_dir(path)
        } else {
            std::fs::remove_file(path)
        }
    }

    for path in args.split_whitespace() {
        if path == "-d" {
            continue;
        }
        if let Err(e) = rm_one(path, rm_dir) {
            print_err!("rm", format_args!("cannot remove '{path}'"), e.to_string());
        }
    }
}

fn do_cd(mut args: &str) {
    if args.is_empty() {
        args = "/";
    }
    if !args.contains(char::is_whitespace) {
        if let Err(e) = std::env::set_current_dir(args) {
            print_err!("cd", args, e.to_string());
        }
    } else {
        print_err!("cd", "too many arguments");
    }
}

fn do_pwd(_args: &str) {
    let pwd = std::env::current_dir().unwrap();
    println!("{}", pwd.to_str().unwrap());
}

fn do_uname(_args: &str) {
    let arch = option_env!("ARCH").unwrap_or("");
    let platform = option_env!("PLATFORM").unwrap_or("");
    let smp = match option_env!("SMP") {
        None | Some("1") => "",
        _ => " SMP",
    };
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.1.0");
    println!(
        "ArceOS {ver}{smp} {arch} {plat}",
        ver = version,
        smp = smp,
        arch = arch,
        plat = platform,
    );
}

fn do_help(_args: &str) {
    println!("Available commands:");
    for (name, _) in CMD_TABLE {
        println!("  {}", name);
    }
}

fn do_exit(_args: &str) {
    unsafe { sys_terminate() };
}

pub fn run_cmd(line: &[u8]) {
    let line_str = unsafe { core::str::from_utf8_unchecked(line) };
    let (cmd, args) = split_whitespace(line_str);
    if !cmd.is_empty() {
        for (name, func) in CMD_TABLE {
            if cmd == *name {
                func(args);
                return;
            }
        }
        println!("{}: command not found", cmd);
    }
}

fn split_whitespace(str: &str) -> (&str, &str) {
    let str = str.trim();
    str.find(char::is_whitespace)
        .map_or((str, ""), |n| (&str[..n], str[n + 1..].trim()))
}

extern "Rust" {
    fn sys_terminate() -> !;
}
