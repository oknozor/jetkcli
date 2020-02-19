// TODO: custom error handling
use std::io::Result;

pub trait GitHost {
    fn submit_pr() -> Result<()>;
    fn edit_pr() -> Result<()>;
    fn assign_pr(user_id: &str, pr_id: &str) -> Result<()>;
}
