//! Job management for shell instances.

use std::io::Write;

use crate::{error, extensions};

impl<SE: extensions::ShellExtensions> crate::Shell<SE> {
    /// Checks for completed jobs in the shell, reporting any changes found.
    pub fn check_for_completed_jobs(&mut self) -> Result<(), error::Error> {
        let results = self.jobs.poll()?;

        if self.options.enable_job_control {
            for (job, _result) in results {
                writeln!(self.stderr(), "{job}")?;
            }
        }

        Ok(())
    }

    /// Returns whether the shell currently has any stopped jobs.
    pub fn has_stopped_jobs(&self) -> bool {
        self.jobs
            .jobs
            .iter()
            .any(|job| matches!(job.state, crate::jobs::JobState::Stopped))
    }

    /// Checks whether an interactive shell should refuse to exit because it has stopped jobs.
    ///
    /// Returns `true` on the first such exit attempt, after recording that the warning was
    /// issued; a subsequent attempt, with no other command run in between, returns `false` so
    /// that the shell can exit. Non-interactive shells never block on stopped jobs.
    pub fn check_for_stopped_jobs_on_exit(&mut self) -> bool {
        if self.options.interactive && !self.stopped_jobs_warning_issued && self.has_stopped_jobs()
        {
            self.stopped_jobs_warning_issued = true;
            true
        } else {
            false
        }
    }

    /// Clears the record of a stopped-jobs exit warning, starting a fresh exit attempt.
    pub(crate) const fn clear_stopped_jobs_exit_warning(&mut self) {
        self.stopped_jobs_warning_issued = false;
    }
}
