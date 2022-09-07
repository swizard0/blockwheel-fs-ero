use futures::{
    channel::{
        mpsc,
    },
    stream,
    StreamExt,
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
    restart,
    supervisor::{
        SupervisorPid,
    },
    ErrorSeverity,
    RestartStrategy,
};

use crate::{
    job,
    proto,
    ftd_sklave,
    access_policy::{
        AccessPolicy,
    },
};

#[derive(Debug)]
pub enum Error {
    BlockwheelFsVersklaven(blockwheel_fs::Error),
    FtdSendegeraetStarten(komm::Error),
    FtdVersklaven(arbeitssklave::Error),
    RequestInfoBefehl(arbeitssklave::Error),
}

pub async fn run<P>(
    fused_request_rx: stream::Fuse<mpsc::Receiver<proto::Request>>,
    parent_supervisor: SupervisorPid,
    params: blockwheel_fs::Params,
    blocks_pool: BytesPool,
    thread_pool: &P,
)
where P: edeltraud::ThreadPool<job::Job> + Clone + Send + 'static,
{
    let terminate_result =
        restart::restartable(
            ero::Params {
                name: format!(
                    "blockwheel on {:?}",
                    match params.interpreter {
                        blockwheel_fs::InterpreterParams::FixedFile(ref interpreter_params) =>
                            format!("fixed file: {:?}", interpreter_params.wheel_filename),
                        blockwheel_fs::InterpreterParams::Ram(ref interpreter_params) =>
                            format!("ram file of {} bytes", interpreter_params.init_wheel_size_bytes),
                    },
                ),
                restart_strategy: RestartStrategy::InstantCrash,
            },
            State {
                parent_supervisor,
                params,
                blocks_pool,
                thread_pool: thread_pool.clone(),
                fused_request_rx,
            },
            |mut state| async move {
                let child_supervisor_gen_server = state.parent_supervisor.child_supervisor();
                let child_supervisor_pid = child_supervisor_gen_server.pid();
                state.parent_supervisor.spawn_link_temporary(
                    child_supervisor_gen_server.run(),
                );
                busyloop_init(child_supervisor_pid, state).await
            },
        )
        .await;
    if let Err(error) = terminate_result {
        log::error!("fatal error: {:?}", error);
    }
}

struct State<P> {
    parent_supervisor: SupervisorPid,
    params: blockwheel_fs::Params,
    blocks_pool: BytesPool,
    thread_pool: P,
    fused_request_rx: stream::Fuse<mpsc::Receiver<proto::Request>>,
}

impl<P> From<Error> for ErrorSeverity<State<P>, Error> {
    fn from(error: Error) -> Self {
        ErrorSeverity::Fatal(error)
    }
}

async fn busyloop_init<P>(supervisor_pid: SupervisorPid, state: State<P>) -> Result<(), ErrorSeverity<State<P>, Error>>
where P: edeltraud::ThreadPool<job::Job> + Clone + Send + 'static,
{
    let blockwheel_fs_meister = blockwheel_fs::Freie::new()
        .versklaven(
            state.params.clone(),
            state.blocks_pool.clone(),
            &edeltraud::ThreadPoolMap::new(state.thread_pool.clone()),
        )
        .map_err(Error::BlockwheelFsVersklaven)?;
    let ftd_sklave_freie = arbeitssklave::Freie::new();
    let ftd_sendegeraet = komm::Sendegeraet::starten(&ftd_sklave_freie, state.thread_pool.clone())
        .map_err(Error::FtdSendegeraetStarten)?;
    let ftd_sklave_meister = ftd_sklave_freie
        .versklaven(
            ftd_sklave::Welt {
                blockwheel_fs_meister,
            },
            &state.thread_pool,
        )
        .map_err(Error::FtdVersklaven)?;

    busyloop(
        supervisor_pid,
        state,
        ftd_sendegeraet,
        ftd_sklave_meister,
    ).await
}

async fn busyloop<P>(
    supervisor_pid: SupervisorPid,
    mut state: State<P>,
    ftd_sendegeraet: komm::Sendegeraet<ftd_sklave::Order>,
    ftd_sklave_meister: ftd_sklave::Meister,
)
    -> Result<(), ErrorSeverity<State<P>, Error>>
where P: edeltraud::ThreadPool<job::Job> + Clone + Send + 'static,
{
    while let Some(request) = state.fused_request_rx.next().await {
        match request {
            proto::Request::Info(request_info) => {
                ftd_sklave_meister
                    .befehl(ftd_sklave::Order::RequestInfo(request_info), &state.thread_pool)
                    .map_err(Error::RequestInfoBefehl)?;
            },
            proto::Request::Flush(proto::RequestFlush { reply_tx, }) => {

                todo!();
            },
            proto::Request::WriteBlock(proto::RequestWriteBlock { block_bytes, reply_tx, }) => {

                todo!();
            },
            proto::Request::ReadBlock(proto::RequestReadBlock { block_id, reply_tx, }) => {

                todo!();
            },
            proto::Request::DeleteBlock(proto::RequestDeleteBlock { block_id, reply_tx, }) => {

                todo!();
            },
            proto::Request::IterBlocks(proto::RequestIterBlocks { reply_tx, }) => {

                todo!();
            },
        }
    }

    log::debug!("request channel is depleted: terminating busyloop");
    Ok(())
}
