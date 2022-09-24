//! Rust crate to execute jobs in an async way
//!
//! Example:
//!
//! ```rust
//! use job_dispatcher::job::Job;
//!
//! #[tokio::main]
//! async fn main() {
//!     let path = "C:\\Users\\sn99\\Downloads\\privacy-script.bat";
//!
//! let mut job = Job::new("trash", path);
//!
//! // start a job
//!     job.start();
//!
//! // check is the job is done (does not block)
//!     println!("Job done?: {:?}", job.try_wait());
//!
//! // wait for it to finish (will block), will error out if previous statement returns `Ok`, use `match` to handle them
//!     job.wait().await.expect("Job failed");
//!
//! println!("Job exited with code: {:?}", job.get_status());
//! }
//! ```
pub mod job;
