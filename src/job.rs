use std::path::PathBuf;
use std::process::Stdio;

/// Contains the `Job` struct, which is used to represent a job in the shell.
/// - `name` is the name of the job
/// - `path` is the path to the executable
/// - `status` is the status of the job and is of type [Status](enum.Status.html)
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum Status {
    /// The job is currently running
    Running,
    /// The job has exited with an error code
    Error(i32),
    /// The job has exited normally with the given exit code
    Exit(i32),
    /// The job is currently in standby and yet to be [executed](struct.Job.html#method.execute)
    Standby,
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

    /// Executes the job in async and returns the exit code of the job
    pub async fn execute(&mut self) {
        self.status = Status::Running;
        let mut cmd = tokio::process::Command::new(self.path.clone());
        cmd.kill_on_drop(true).creation_flags(0x00000008);
        let mut child = cmd
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to spawn process");

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
