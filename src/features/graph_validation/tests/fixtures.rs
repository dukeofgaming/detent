mod linear_process;
mod branching_process;

pub(crate) use branching_process::branching_process;
pub(crate) use linear_process::{linear_process, linear_process_with_retargeted_exit};
