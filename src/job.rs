//! Holds our [Job](Job) struct and its methods

use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::Stdio;

/// Contains the `Job` struct, which is used to represent a job in the shell.
/// - `name` is the name of the job
/// - `path` is the path to the executable
/// - `status` is the status of the job and is of type [Status](enum.Status.html)
#[derive(Debug)]
pub struct Job {
    /// Name of the job
    name: String,
    /// Path to the executable
    path: PathBuf,
    /// Status of the job
    status: Status,
}

/// Reports the current status of the job, can be of types:
/// - `Running`
/// - `Error(i32)`
/// - `Exit(i32)`
/// - `Standby`
#[derive(Debug)]
pub enum Status {
    /// The job is currently running
    Running(Box<tokio::process::Child>),
    /// The job has exited with an error code
    Error(i32),
    /// The job has exited normally with the given exit code
    Exit(i32),
    /// The job is currently in standby and yet to be [`started`](Job::start) or [`waited`](Job::wait)
    Standby,
}

/// Custom Display for Status
impl Display for Status {
    /// Display the status of the job in a human readable format
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Status::Running(_) => {
                write!(f, "Running")
            }
            Status::Error(code) => {
                write!(f, "Error({})", code)
            }
            Status::Exit(code) => {
                write!(f, "Exit({})", code)
            }
            Status::Standby => {
                write!(f, "Standby")
            }
        }
    }
}

/// Contains the methods for the `Job` struct
impl Job {
    /// Creates a new job with the given name and path
    ///
    /// Example:
    /// ```rust
    ///  use job_dispatcher::job::Job;
    ///
    /// let mut job = Job::new("trash", "C:\\Users\\sn99\\Downloads\\privacy-script.bat");
    /// ```
    pub fn new(name: &str, path: &str) -> Job {
        let name = name.to_string();
        let path = PathBuf::from(path);
        Job {
            name,
            path,
            status: Status::Standby,
        }
    }

    /// Start the job, see [wait](Job::wait) for further actions
    pub fn start(&mut self) {
        let mut cmd = tokio::process::Command::new(self.path.clone());
        // kill operation is invoked on a spawned child process when its corresponding Child handle
        // is dropped
        cmd.kill_on_drop(true)
        // .creation_flags(0x00000010)
        // .creation_flags(0x00000008)
        // .creation_flags(0x08000000)
        ;

        let child = cmd
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to spawn process");

        self.status = Status::Running(Box::from(child))
    }

    /// Call [`start`](Job::start) before this method
    ///
    /// Wait for the job to finish, best used with [spawn](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html)
    /// or [join](https://docs.rs/tokio/latest/tokio/macro.join.html)
    ///
    /// Returns `Ok(())` if the job has exited normally (the status of job itself could be
    /// [`Error`](Status::Error) but it was able to exit), otherwise returns: `Err(-1)` if the job
    /// was not [`Running`](Status::Running)
    pub async fn wait(&mut self) -> Result<(), i32> {
        match &mut self.status {
            Status::Running(child) => {
                let status = child.wait().await.expect("Failed to wait on child");
                match status.code() {
                    Some(code) => {
                        self.status = Status::Exit(code);
                        if code != 0 {
                            self.status = Status::Error(code);
                        }
                    }
                    None => {
                        self.status = Status::Error(-1);
                    }
                }
                Ok(())
            }
            _ => Err(-1),
        }
    }

    /// Returns the name of the job
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns the path of the job
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns the status of the job
    pub fn get_status(&self) -> &Status {
        &self.status
    }
}
