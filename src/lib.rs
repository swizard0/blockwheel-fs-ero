#![forbid(unsafe_code)]

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
        Bytes,
        BytesPool,
    },
};

pub use blockwheel_fs::{
    block,
    Info,
    Params,
    Flushed,
    Deleted,
    InterpreterParams,
    RamInterpreterParams,
    RequestReadBlockError,
    RequestWriteBlockError,
    RequestDeleteBlockError,
    FixedFileInterpreterParams,
};

pub mod job;

mod proto;
mod gen_server;
mod ftd_sklave;
mod echo_policy;

pub struct GenServer {
    request_tx: mpsc::Sender<proto::Request>,
    fused_request_rx: stream::Fuse<mpsc::Receiver<proto::Request>>,
}

#[derive(Clone)]
pub struct Pid {
    request_tx: mpsc::Sender<proto::Request>,
}

impl Default for GenServer {
    fn default() -> Self {
        Self::new()
    }
}

impl GenServer {
    pub fn new() -> GenServer {
        let (request_tx, request_rx) = mpsc::channel(0);
        GenServer {
            request_tx,
            fused_request_rx: request_rx.fuse(),
        }
    }

    pub fn pid(&self) -> Pid {
        Pid {
            request_tx: self.request_tx.clone(),
        }
    }

    pub async fn run<P>(
        self,
        parent_supervisor: ero::supervisor::SupervisorPid,
        params: blockwheel_fs::Params,
        blocks_pool: BytesPool,
        thread_pool: P,
    )
    where P: edeltraud::ThreadPool<job::Job> + Clone + Send + Sync + 'static,
    {
        gen_server::run(
            self.fused_request_rx,
            parent_supervisor,
            params,
            blocks_pool,
            thread_pool,
        ).await
    }
}

#[derive(Debug)]
pub enum WriteBlockError {
    GenServer(ero::NoProcError),
    NoSpaceLeft,
}

#[derive(Debug)]
pub enum ReadBlockError {
    GenServer(ero::NoProcError),
    NotFound,
}

#[derive(Debug)]
pub enum DeleteBlockError {
    GenServer(ero::NoProcError),
    NotFound,
}

#[derive(Debug)]
pub enum IterBlocksError {
    GenServer(ero::NoProcError),
}

#[derive(Debug)]
pub struct IterBlocks {
    pub blocks_total_count: usize,
    pub blocks_total_size: usize,
    pub blocks_rx: mpsc::Receiver<IterBlocksItem>,
}

#[derive(Debug)]
pub enum IterBlocksItem {
    Block { block_id: block::Id, block_bytes: Bytes, },
    NoMoreBlocks,
}

impl Pid {
    pub async fn info(&mut self) -> Result<Info, ero::NoProcError> {
        loop {
            let (reply_tx, reply_rx) = oneshot::channel();
            self.request_tx.send(proto::Request::Info(proto::RequestInfo { reply_tx, })).await
                .map_err(|_send_error| ero::NoProcError)?;
            match reply_rx.await {
                Ok(info) =>
                    return Ok(info),
                Err(oneshot::Canceled) =>
                    (),
            }
        }
    }

    pub async fn flush(&mut self) -> Result<Flushed, ero::NoProcError> {
        loop {
            let (reply_tx, reply_rx) = oneshot::channel();
            self.request_tx.send(proto::Request::Flush(proto::RequestFlush { reply_tx, })).await
                .map_err(|_send_error| ero::NoProcError)?;
            match reply_rx.await {
                Ok(Flushed) =>
                    return Ok(Flushed),
                Err(oneshot::Canceled) =>
                    (),
            }
        }
    }

    pub async fn write_block(&mut self, block_bytes: Bytes) -> Result<block::Id, WriteBlockError> {
        loop {
            let (reply_tx, reply_rx) = oneshot::channel();
            self.request_tx
                .send(proto::Request::WriteBlock(proto::RequestWriteBlock {
                    block_bytes: block_bytes.clone(),
                    reply_tx,
                }))
                .await
                .map_err(|_send_error| WriteBlockError::GenServer(ero::NoProcError))?;

            match reply_rx.await {
                Ok(Ok(block_id)) =>
                    return Ok(block_id),
                Ok(Err(RequestWriteBlockError::NoSpaceLeft)) =>
                    return Err(WriteBlockError::NoSpaceLeft),
                Err(oneshot::Canceled) =>
                    (),
            }
        }
    }

    pub async fn read_block(&mut self, block_id: block::Id) -> Result<Bytes, ReadBlockError> {
        loop {
            let (reply_tx, reply_rx) = oneshot::channel();
            self.request_tx
                .send(proto::Request::ReadBlock(proto::RequestReadBlock {
                    block_id: block_id.clone(),
                    reply_tx,
                }))
                .await
                .map_err(|_send_error| ReadBlockError::GenServer(ero::NoProcError))?;

            match reply_rx.await {
                Ok(Ok(block_bytes)) =>
                    return Ok(block_bytes),
                Ok(Err(RequestReadBlockError::NotFound)) =>
                    return Err(ReadBlockError::NotFound),
                Err(oneshot::Canceled) =>
                    (),
            }
        }
    }

    pub async fn delete_block(&mut self, block_id: block::Id) -> Result<Deleted, DeleteBlockError> {
        loop {
            let (reply_tx, reply_rx) = oneshot::channel();
            self.request_tx
                .send(proto::Request::DeleteBlock(proto::RequestDeleteBlock {
                    block_id: block_id.clone(),
                    reply_tx,
                }))
                .await
                .map_err(|_send_error| DeleteBlockError::GenServer(ero::NoProcError))?;

            match reply_rx.await {
                Ok(Ok(Deleted)) =>
                    return Ok(Deleted),
                Ok(Err(RequestDeleteBlockError::NotFound)) =>
                    return Err(DeleteBlockError::NotFound),
                Err(oneshot::Canceled) =>
                    (),
            }
        }
    }

    pub async fn iter_blocks(&mut self) -> Result<IterBlocks, IterBlocksError> {
        loop {
            let (reply_tx, reply_rx) = oneshot::channel();
            self.request_tx
                .send(proto::Request::IterBlocks(proto::RequestIterBlocks {
                    reply_tx,
                }))
                .await
                .map_err(|_send_error| IterBlocksError::GenServer(ero::NoProcError))?;

            match reply_rx.await {
                Ok(iter_blocks) =>
                    return Ok(iter_blocks),
                Err(oneshot::Canceled) =>
                    (),
            }
        }
    }
}
