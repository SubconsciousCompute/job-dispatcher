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
//!     let mut job = Job::new("trash", path);
//!
//!     // start a job
//!     job.start();
//!
//!     // wait for it to finish
//!     job.wait().await.expect("TODO: panic message");
//!
//!     println!("Job exited with code: {:?}", job.get_status());
//! }
//! ```
pub mod job;
