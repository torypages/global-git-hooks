use regex::Regex;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

fn get_git_project_dir() -> String {
    let git_command = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .expect("Could not run git command to get project dir.");

    return String::from_utf8(git_command.stdout)
        .unwrap()
        .trim()
        .to_string();
}

pub fn run_local_hook(hook: &String) {
    // Run the local hook (in the calling projects repo if possible).

    let git_dir = get_git_project_dir();

    let local_hook = Path::new(&git_dir).join(".git").join("hooks").join(hook);
    return match Command::new(local_hook).status() {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {
                println!("Permission denied");
            }
            _ => {}
        },
    };
}

pub fn get_hook(hook_executable_path: &String) -> String {
    return Path::new(&hook_executable_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
}

fn get_auto_message(prefix_length: usize) -> String {
    let command_out = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()
        .unwrap();
    let filenames: Vec<String> = String::from_utf8(command_out.stdout)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();

    let message = format!("update: {}", filenames.join(", "));
    truncate_message(message, prefix_length)
}

fn truncate_message(message: String, prefix_length: usize) -> String {
    let max_length = 50;
    if (message.len() + prefix_length) <= max_length {
        return message;
    }
    let elipsis = "...";
    let line_max = max_length - elipsis.len() - prefix_length;
    return format!(
        "{}{}\n{}",
        &message[..line_max],
        elipsis,
        &message[line_max..]
    );
}

fn convert_dummy_message(existing_message: String, prefix_length: usize) -> String {
    // If a dummy message, convert it to a generic message else return the existing message
    let mut lines = existing_message.lines();
    let first_line = lines.next().unwrap_or("");

    match first_line {
        "-" => get_auto_message(prefix_length),
        _ => existing_message,
    }
}

pub fn get_commit_message(existing_message: String) -> String {
    let branch_name: String = get_branch_name();
    let pattern = r"^(VA-\d+)";
    let re = Regex::new(pattern).unwrap();
    if let Some(captures) = re.captures(&branch_name) {
        let prefix = format!("{} - ", &captures[0]);
        let commit_message = convert_dummy_message(existing_message, prefix.len());
        return format!("{}{}", prefix, commit_message);
    }
    return existing_message;
}

fn get_branch_name() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Could not run git comand to get branch name.");
    return String::from_utf8(output.stdout).unwrap().trim().to_string();
}
