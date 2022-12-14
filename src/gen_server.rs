use futures::{
    channel::{
        mpsc,
        oneshot,
    },
    stream,
    SinkExt,
    StreamExt,
};

use alloc_pool::{
    bytes::{
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
    proto,
    ftd_sklave,
    echo_policy::{
        EchoPolicy,
    },
    Params,
    IterBlocks,
    IterBlocksItem,
    InterpreterParams,
};

#[derive(Debug)]
pub enum Error {
    BlockwheelFsVersklaven(blockwheel_fs::Error),
    FtdVersklaven(arbeitssklave::Error),
    RequestInfoBefehl(blockwheel_fs::Error),
    RequestFlushBefehl(blockwheel_fs::Error),
    RequestWriteBlockBefehl(blockwheel_fs::Error),
    RequestReadBlockBefehl(blockwheel_fs::Error),
    RequestDeleteBlockBefehl(blockwheel_fs::Error),
    RequestIterBlocksInitBefehl(blockwheel_fs::Error),
    RequestIterBlocksNextBefehl(blockwheel_fs::Error),
    FtdSklaveIsGoneDuringIterBlocksInit,
    FtdSklaveIsGoneDuringIterBlocksNext,
}

pub async fn run<J>(
    fused_request_rx: stream::Fuse<mpsc::Receiver<proto::Request>>,
    parent_supervisor: SupervisorPid,
    params: Params,
    blocks_pool: BytesPool,
    thread_pool: edeltraud::Handle<J>,
)
where J: From<blockwheel_fs::job::SklaveJob<EchoPolicy>>,
      J: From<ftd_sklave::SklaveJob>,
      J: Send + 'static,
{
    let terminate_result =
        restart::restartable(
            ero::Params {
                name: format!(
                    "blockwheel_fs on {:?}",
                    match params.interpreter {
                        InterpreterParams::FixedFile(ref interpreter_params) =>
                            format!("fixed file: {:?}", interpreter_params.wheel_filename),
                        InterpreterParams::Ram(ref interpreter_params) =>
                            format!("ram file of {} bytes", interpreter_params.init_wheel_size_bytes),
                        InterpreterParams::Dummy(ref interpreter_params) =>
                            format!("dummy file of {} bytes", interpreter_params.init_wheel_size_bytes),
                    },
                ),
                restart_strategy: RestartStrategy::InstantCrash,
            },
            State {
                parent_supervisor,
                params,
                blocks_pool,
                thread_pool,
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

struct State<J> {
    parent_supervisor: SupervisorPid,
    params: Params,
    blocks_pool: BytesPool,
    thread_pool: edeltraud::Handle<J>,
    fused_request_rx: stream::Fuse<mpsc::Receiver<proto::Request>>,
}

impl<J> From<Error> for ErrorSeverity<State<J>, Error> {
    fn from(error: Error) -> Self {
        ErrorSeverity::Fatal(error)
    }
}

async fn busyloop_init<J>(supervisor_pid: SupervisorPid, state: State<J>) -> Result<(), ErrorSeverity<State<J>, Error>>
where J: From<blockwheel_fs::job::SklaveJob<EchoPolicy>>,
      J: From<ftd_sklave::SklaveJob>,
      J: Send + 'static,
{
    let blockwheel_fs_meister =
        blockwheel_fs::Meister::versklaven(
            state.params.clone(),
            state.blocks_pool.clone(),
            &state.thread_pool,
        )
        .map_err(Error::BlockwheelFsVersklaven)?;
    let ftd_sklave_meister = arbeitssklave::Freie::new()
        .versklaven(ftd_sklave::Welt, &state.thread_pool)
        .map_err(Error::FtdVersklaven)?;
    let ftd_sendegeraet =
        komm::Sendegeraet::starten(&ftd_sklave_meister, state.thread_pool.clone());

    busyloop(
        supervisor_pid,
        state,
        blockwheel_fs_meister,
        ftd_sklave_meister,
        ftd_sendegeraet,
    ).await
}

async fn busyloop<J>(
    mut supervisor_pid: SupervisorPid,
    mut state: State<J>,
    blockwheel_fs_meister: blockwheel_fs::Meister<EchoPolicy>,
    _ftd_sklave_meister: arbeitssklave::Meister<ftd_sklave::Welt, ftd_sklave::Order>,
    ftd_sendegeraet: komm::Sendegeraet<ftd_sklave::Order>,
)
    -> Result<(), ErrorSeverity<State<J>, Error>>
where J: From<blockwheel_fs::job::SklaveJob<EchoPolicy>>,
      J: From<ftd_sklave::SklaveJob>,
      J: Send + 'static,
{
    while let Some(request) = state.fused_request_rx.next().await {
        match request {
            proto::Request::Info(proto::RequestInfo { reply_tx, }) => {
                blockwheel_fs_meister
                    .info(
                        ftd_sendegeraet.rueckkopplung(reply_tx),
                        &state.thread_pool,
                    )
                    .map_err(Error::RequestInfoBefehl)?;
            },
            proto::Request::Flush(proto::RequestFlush { reply_tx, }) => {
                blockwheel_fs_meister
                    .flush(
                        ftd_sendegeraet.rueckkopplung(reply_tx),
                        &state.thread_pool,
                    )
                    .map_err(Error::RequestFlushBefehl)?;
            },
            proto::Request::WriteBlock(proto::RequestWriteBlock { block_bytes, reply_tx, }) => {
                blockwheel_fs_meister
                    .write_block(
                        block_bytes,
                        ftd_sendegeraet.rueckkopplung(reply_tx),
                        &state.thread_pool,
                    )
                    .map_err(Error::RequestWriteBlockBefehl)?;
            },
            proto::Request::ReadBlock(proto::RequestReadBlock { block_id, reply_tx, }) => {
                blockwheel_fs_meister
                    .read_block(
                        block_id,
                        ftd_sendegeraet.rueckkopplung(reply_tx),
                        &state.thread_pool,
                    )
                    .map_err(Error::RequestReadBlockBefehl)?;
            },
            proto::Request::DeleteBlock(proto::RequestDeleteBlock { block_id, reply_tx, }) => {
                blockwheel_fs_meister
                    .delete_block(
                        block_id,
                        ftd_sendegeraet.rueckkopplung(reply_tx),
                        &state.thread_pool,
                    )
                    .map_err(Error::RequestDeleteBlockBefehl)?;
            },
            proto::Request::IterBlocks(proto::RequestIterBlocks { reply_tx, }) => {
                let blockwheel_fs_meister = blockwheel_fs_meister.clone();
                let ftd_sendegeraet = ftd_sendegeraet.clone();
                let thread_pool = state.thread_pool.clone();
                supervisor_pid.spawn_link_temporary(async move {
                    if let Err(error) = iter_blocks_loop(blockwheel_fs_meister, ftd_sendegeraet, reply_tx, &thread_pool).await {
                        log::warn!("blocks iterator loop exited with error: {:?}", error);
                    }
                });
            },
        }
    }

    log::debug!("request channel is depleted: terminating busyloop");
    Ok(())
}

async fn iter_blocks_loop<J>(
    blockwheel_fs_meister: blockwheel_fs::Meister<EchoPolicy>,
    ftd_sendegeraet: komm::Sendegeraet<ftd_sklave::Order>,
    reply_tx: proto::RequestIterBlocksReplyTx,
    thread_pool: &edeltraud::Handle<J>,
)
    -> Result<(), Error>
where J: From<blockwheel_fs::job::SklaveJob<EchoPolicy>>,
      J: From<ftd_sklave::SklaveJob>,
      J: Send + 'static,
{
    let (iter_blocks_init_tx, iter_blocks_init_rx) = oneshot::channel();
    blockwheel_fs_meister
        .iter_blocks_init(
            ftd_sendegeraet.rueckkopplung(ftd_sklave::RequestIterBlocksInit {
                iter_blocks_init_tx,
            }),
            thread_pool,
        )
        .map_err(Error::RequestIterBlocksInitBefehl)?;
    let iter_blocks = iter_blocks_init_rx.await
        .map_err(|oneshot::Canceled| Error::FtdSklaveIsGoneDuringIterBlocksInit)?;

    let (mut blocks_tx, blocks_rx) = mpsc::channel(0);
    let iter_blocks_reply = IterBlocks {
        blocks_total_count: iter_blocks.blocks_total_count,
        blocks_total_size: iter_blocks.blocks_total_size,
        blocks_rx,
    };
    if let Err(_send_error) = reply_tx.send(iter_blocks_reply) {
        log::debug!("client canceled iter IterBlocks request (init)");
        return Ok(());
    }

    let mut current_iterator_next = iter_blocks.iterator_next;
    loop {
        let (iter_blocks_next_tx, iter_blocks_next_rx) = oneshot::channel();
        blockwheel_fs_meister
            .iter_blocks_next(
                current_iterator_next,
                ftd_sendegeraet.rueckkopplung(ftd_sklave::RequestIterBlocksNext {
                    iter_blocks_next_tx,
                }),
                thread_pool,
            )
            .map_err(Error::RequestIterBlocksNextBefehl)?;
        let iter_blocks_item = iter_blocks_next_rx.await
            .map_err(|oneshot::Canceled| Error::FtdSklaveIsGoneDuringIterBlocksNext)?;
        match iter_blocks_item {
            blockwheel_fs::IterBlocksItem::Block { block_id, block_bytes, iterator_next, } => {
                let item = IterBlocksItem::Block { block_id, block_bytes, };
                if let Err(_send_error) = blocks_tx.send(item).await {
                    log::debug!("client canceled iter IterBlocks request (stream)");
                    return Ok(());
                }
                current_iterator_next = iterator_next;
            },
            blockwheel_fs::IterBlocksItem::NoMoreBlocks => {
                if let Err(_send_error) = blocks_tx.send(IterBlocksItem::NoMoreBlocks).await {
                    log::debug!("client canceled iter IterBlocks request (stream)");
                }
                return Ok(());
            },
        }
    }
}
