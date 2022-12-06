use crate::{
    echo_policy::{
        EchoPolicy,
    },
    ftd_sklave,
};

pub enum Job {
    BlockwheelFs(blockwheel_fs::job::Job<EchoPolicy>),
    FtdSklave(ftd_sklave::SklaveJob),
}

impl From<blockwheel_fs::job::Job<EchoPolicy>> for Job {
    fn from(job: blockwheel_fs::job::Job<EchoPolicy>) -> Self {
        Self::BlockwheelFs(job)
    }
}

impl From<blockwheel_fs::job::SklaveJob<EchoPolicy>> for Job {
    fn from(sklave_job: blockwheel_fs::job::SklaveJob<EchoPolicy>) -> Self {
        Self::BlockwheelFs(sklave_job.into())
    }
}

impl From<ftd_sklave::SklaveJob> for Job {
    fn from(job: ftd_sklave::SklaveJob) -> Self {
        Self::FtdSklave(job)
    }
}

pub struct JobUnit<J>(edeltraud::JobUnit<J, Job>);

impl<J> From<edeltraud::JobUnit<J, Job>> for JobUnit<J> {
    fn from(job_unit: edeltraud::JobUnit<J, Job>) -> Self {
        Self(job_unit)
    }
}

impl<J> edeltraud::Job for JobUnit<J>
where J: From<blockwheel_fs::job::SklaveJob<EchoPolicy>>,
      J: From<blockwheel_fs::job::BlockPrepareWriteJob<EchoPolicy>>,
      J: From<blockwheel_fs::job::BlockPrepareDeleteJob<EchoPolicy>>,
      J: From<blockwheel_fs::job::BlockProcessReadJob<EchoPolicy>>,
{
    fn run(self) {
        match self.0.job {
            Job::BlockwheelFs(job) => {
                let job_unit = blockwheel_fs::job::JobUnit::from(edeltraud::JobUnit {
                    handle: self.0.handle,
                    job,
                });
                job_unit.run();
            },
            Job::FtdSklave(job) => {
                ftd_sklave::job(job, &self.0.handle);
            },
        }
    }
}
