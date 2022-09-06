use crate::{
    access_policy::{
        AccessPolicy,
    },
};

pub enum Job {
    BlockwheelFs(blockwheel_fs::job::Job<AccessPolicy>),
}
