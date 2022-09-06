use crate::{
    access_policy::{
        AccessPolicy,
    },
    ftd_sklave,
};

pub enum Job {
    BlockwheelFs(blockwheel_fs::job::Job<AccessPolicy>),
    FtdSklave(ftd_sklave::SklaveJob),
}

impl From<blockwheel_fs::job::Job<AccessPolicy>> for Job {
    fn from(job: blockwheel_fs::job::Job<AccessPolicy>) -> Job {
        Job::BlockwheelFs(job)
    }
}

impl From<ftd_sklave::SklaveJob> for Job {
    fn from(job: ftd_sklave::SklaveJob) -> Job {
        Job::FtdSklave(job)
    }
}

impl edeltraud::Job for Job {
    fn run<P>(self, thread_pool: &P) where P: edeltraud::ThreadPool<Self> {
        match self {
            Job::BlockwheelFs(job) => {
                job.run(&edeltraud::ThreadPoolMap::new(thread_pool));
            },
            Job::FtdSklave(job) => {
                ftd_sklave::job(job, thread_pool);
            },
        }
    }
}
