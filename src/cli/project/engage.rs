use cli::data::project::engage::Engage;

use cli::project::GitHubProject;
use cli::project::get_project_dir;
use cli::project::project_exists;

use std::collections::BTreeSet;
use std::env;
use std::process::Command;
use std::process::exit;
use std::string::String;

/// Engage a project via tmux.
pub fn execute(engage: Engage) {
    let project = match GitHubProject::from(&engage.project) {
        Ok(p) => p,
        Err(_) => {
            error!("{} is not a valid GitHub project name.", &engage.project);
            exit(1)
        }
    };

    let repository = format!("{}/{}", project.owner(), project.repository());

    info!("Engaging project {}", engage.project);

    if !project_exists(&repository) {
        error!("Project does not exist, please bring it down first.");
        exit(1);
    } else {
        debug!("Project exists already.");
    }

    // cd into the project directory
    debug!("Changing directory into project.");
    env::set_current_dir(get_project_dir(&repository)).unwrap();

    // enter the session
    enter_tmux_session(&project.repository());
}

/// Determine whether a tmux session exists with the given session name.
fn has_tmux_session(session_name: &str) -> bool {
    let tmux_list_sessions = Command::new("tmux").arg("ls").output().expect("unable to list tmux sessions");
    let tmux_list_output = String::from_utf8_lossy(&tmux_list_sessions.stdout);
    let tmux_list_output = tmux_list_output.lines();

    let tmux_sessions: BTreeSet<&str> = tmux_list_output.map(|s| s.split(':').nth(0).unwrap()).collect();
    tmux_sessions.contains(session_name)
}

/// Enter a tmux session by either attaching to it or creating it.
fn enter_tmux_session(session_name: &str) {
    let command_status = if has_tmux_session(&session_name) {
        debug!("Attaching to already existing tmux session.");
        Command::new("tmux").arg("attach").arg("-t").arg(&session_name).status()
    } else {
        debug!("Creating new tmux session.");
        Command::new("tmux").arg("new").arg("-s").arg(&session_name).status()
    };

    match command_status {
        Ok(status) => info!("tmux has exited, status {}", status),
        Err(_)     => error!("tmux was killed"),
    };
}
