use crate::{
    access_policy::{
        AccessPolicy,
    },
};

pub enum Job {
    BlockwheelFs(blockwheel_fs::job::Job<AccessPolicy>),
}

impl From<blockwheel_fs::job::Job<AccessPolicy>> for Job {
    fn from(job: blockwheel_fs::job::Job<AccessPolicy>) -> Job {
        Job::BlockwheelFs(job)
    }
}

impl edeltraud::Job for Job {
    fn run<P>(self, thread_pool: &P) where P: edeltraud::ThreadPool<Self> {
        match self {
            Job::BlockwheelFs(job) => {
                job.run(&edeltraud::ThreadPoolMap::new(thread_pool));
            },
        }
    }
}
