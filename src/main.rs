use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1..].join(" ");
    let mut elements: Vec<&str> = query.split('>').collect();
    elements = elements.into_iter().map(|e| e.trim()).collect();
    let globs = get_globs(elements[0]);
    let keyword = elements[1];

    let mut cmd_args = vec![
        "--column",
        "--line-number",
        "--no-heading",
        "--color=always",
        "--smart-case",
    ];

    for g in globs {
        cmd_args.push("--iglob");
        cmd_args.push(g);
    }

    cmd_args.push("--");
    cmd_args.push(keyword);

    let output = Command::new("rg")
        .args(&cmd_args)
        .output()
        .expect("Failed!");

    let result_text = String::from_utf8_lossy(&output.stdout);

    println!("{}", result_text)
}

fn get_globs(glob_string: &str) -> Vec<&str> {
    return glob_string
        .split(' ')
        .map(|g| g.trim())
        .filter(|g| !g.is_empty())
        .collect();
}

