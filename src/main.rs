use std::{
    env,
    io::{stdin, stdout, BufRead, Write},
    path::Path,
    process::{Child, Command, Stdio},
};

mod tokens;
use tap::Tap;
use tokens::*;

const PUKA: &str = "puka";

fn main() {
    println!("Aloha");
    loop {
        let cmd = prompt("\u{03bb} ");
        if matches!(cmd.as_str(), PUKA) {
            println!("Aloha");
            return;
        }

        let tokens: Tokens = cmd.tokenize();

        let mut commands = tokens.split(|x| x == &Token::Pipe).peekable();
        let mut last = None;

        while let Some(cmd) = commands.next() {
            let mut parts = cmd.iter().map(|x| x.str());
            let cmd = parts.next().unwrap();
            let args = parts;

            last = process_command(cmd, args, last, commands.peek().is_some());
        }
    }
}

fn process_command<'a>(
    cmd: &str,
    args: impl Iterator<Item = &'a str>,
    previous: Option<Child>,
    peek_is_some: bool,
) -> Option<Child> {
    let stdin = previous.map_or(Stdio::inherit(), |output: Child| {
        Stdio::from(output.stdout.unwrap())
    });

    let stdout = if peek_is_some {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };

    let mut r = None;

    match cmd {
        "cd" => cd(args),
        "exit" => println!("try {PUKA:?}"),
        _ => {
            match Command::new(cmd)
                .stdin(stdin)
                .stdout(stdout)
                .args(args)
                .spawn()
            {
                Ok(x) => r = Some(x),
                Err(e) => {
                    eprintln!("os error: {}", e.raw_os_error().unwrap());
                    eprintln!("{e}");
                }
            };
        }
    };

    if let Some(ref mut x) = &mut r {
        x.wait().unwrap();
    }

    r
}

fn cd<'a>(args: impl Iterator<Item = &'a str>) {
    let new_dir = args.peekable().peek().map_or("/", |x| x);
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(root) {
        eprintln!("{}", e);
    }
}

fn prompt(prefix: &str) -> String {
    stdout()
        .tap(|mut x| x.write_all(prefix.as_bytes()).unwrap())
        .flush()
        .map(|_| stdin())
        .unwrap()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .to_string()
}
