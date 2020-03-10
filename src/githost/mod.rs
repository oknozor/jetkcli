use std::io::Result;

/// Currently unimplemented
pub trait GitHost {
    fn submit_pr() -> Result<()>;
    fn edit_pr() -> Result<()>;
    fn assign_pr(
        user_id: &str,
        pr_id: &str,
    ) -> Result<()>;
}
