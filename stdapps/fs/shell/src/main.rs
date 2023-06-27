use std::io::{stdin, stdout, Read, Result, Write};

mod cmd;

const LF: u8 = b'\n';
const CR: u8 = b'\r';
const DL: u8 = b'\x7f';
const BS: u8 = b'\x08';
const SPACE: u8 = b' ';

const MAX_CMD_LEN: usize = 256;

fn print_prompt() -> Result<()> {
    print!(
        "arceos:{}$ ",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    stdout().flush()
}

fn main() {
    let mut stdin = stdin();
    let mut stdout = stdout();

    let mut buf = [0; MAX_CMD_LEN];
    let mut cursor = 0;
    cmd::run_cmd("help".as_bytes());
    print_prompt().unwrap();

    loop {
        if stdin.read(&mut buf[cursor..cursor + 1]).ok() != Some(1) {
            continue;
        }
        if buf[cursor] == b'\x1b' {
            buf[cursor] = b'^';
        }
        match buf[cursor] {
            CR | LF => {
                println!();
                if cursor > 0 {
                    cmd::run_cmd(&buf[..cursor]);
                    cursor = 0;
                }
                print_prompt().unwrap();
            }
            BS | DL => {
                if cursor > 0 {
                    stdout.write(&[BS, SPACE, BS]).unwrap();
                    stdout.flush().unwrap();
                    cursor -= 1;
                }
            }
            0..=31 => {}
            c => {
                if cursor < MAX_CMD_LEN - 1 {
                    stdout.write(&[c]).unwrap();
                    stdout.flush().unwrap();
                    cursor += 1;
                }
            }
        }
    }
}
