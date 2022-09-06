use futures::{
    channel::{
        mpsc,
    },
};

use alloc_pool::{
    bytes::{
        Bytes,
        BytesPool,
    },
};

use arbeitssklave::{
    komm,
};

use ero::{
    supervisor::{
        SupervisorPid,
    },
    ErrorSeverity,
};

use crate::{
    job,
    proto,
    ftd_sklave,
};

#[derive(Debug)]
pub enum Error {
    BlockwheelFs(blockwheel_fs::Error),
}

pub fn run<P>(
    parent_supervisor: SupervisorPid,
    params: blockwheel_fs::Params,
    blocks_pool: BytesPool,
    thread_pool: &P,
)
    -> Result<(), Error>
where P: edeltraud::ThreadPool<job::Job> + Clone + Send + 'static,
{
    let blockwheel_fs_meister = blockwheel_fs::Freie::new()
        .versklaven(
            params.clone(),
            blocks_pool.clone(),
            &edeltraud::ThreadPoolMap::new(thread_pool.clone()),
        )
        .map_err(Error::BlockwheelFs)?;
    let ftd_sklave_freie = arbeitssklave::Freie::new();
    let ftd_sendegeraet = komm::Sendegeraet::starten(&ftd_sklave_freie, thread_pool.clone())
        .unwrap();
    let ftd_sklave_meister = ftd_sklave_freie
        .versklaven(ftd_sklave::Welt { blockwheel_fs_meister, }, &thread_pool)
        .unwrap();

    todo!()
}
