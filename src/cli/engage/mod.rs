use api::project::Project;

use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::process;
use std::process::Command;
use std::string::String;

#[derive(Debug,StructOpt)]
pub struct Engage {
    /// The project to engage in "$ORGANIZATION/$PROJECT" GitHub format.
    pub project: String,
}

/// Engage a project via tmux.
impl Engage {

    pub fn execute(&self) {
        let project = match Project::from(&self.project) {
            Ok(p) => p,
            Err(_) => {
                error!("{} is not a valid project name.", &self.project);
                process::exit(1)
            }
        };

        info!("Engaging project {}", self.project);

        // fetch it if it isn't already local
        if let Err(e) = project.clone(|_| true) {
            error!("Unable to fetch project {}: {}", project.url(), e.description());
            process::exit(1);
        }

        // configure it just because
        if let Err(e) = project.configure() {
            error!("Unable to configure project {}: {}", project.url(), e.description());
            process::exit(1);
        }

        // cd into the project directory
        debug!("Changing directory into project.");
        env::set_current_dir(project.dir()).unwrap();

        // enter the session
        enter_tmux_session(&project.repository());
    }
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
