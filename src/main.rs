use global_git_hooks::functions::get_commit_message;
use global_git_hooks::functions::get_hook;
use global_git_hooks::functions::run_local_hook;
use std::env;
use std::fs;

// fn pre_commit() {
//     println!("Running pre-commit hook");
// }

fn commit_msg(args: &[String]) {
    // Commit message git hook
    let message_file = &args[1];
    let data = fs::read_to_string(&message_file).expect("Unable to read file");
    let commit_message = get_commit_message(data);

    fs::write(&args[1], commit_message).expect("Unable to write file");
}

fn run_custom_hook(hook: &String, args: &[String]) {
    match hook.as_str() {
        // "pre-commit" => {
        //     pre_commit();
        // }
        "commit-msg" => {
            commit_msg(args);
        }
        _ => {}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let hook = get_hook(&args[0]);
    run_custom_hook(&hook, &args);
    run_local_hook(&hook);
}
