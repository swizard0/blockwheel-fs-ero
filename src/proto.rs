use futures::{
    channel::{
        oneshot,
    },
};

use alloc_pool::{
    bytes::{
        Bytes,
    },
};

use blockwheel_fs::{
    block,
    Info,
    Flushed,
    Deleted,
    RequestReadBlockError,
    RequestWriteBlockError,
    RequestDeleteBlockError,
};

use crate::{
    IterBlocks,
};

#[derive(Debug)]
pub enum Request {
    Info(RequestInfo),
    Flush(RequestFlush),
    WriteBlock(RequestWriteBlock),
    ReadBlock(RequestReadBlock),
    DeleteBlock(RequestDeleteBlock),
    IterBlocks(RequestIterBlocks),
}

#[derive(Debug)]
pub struct RequestInfo {
    pub reply_tx: oneshot::Sender<Info>,
}

#[derive(Debug)]
pub struct RequestFlush {
    pub reply_tx: oneshot::Sender<Flushed>,
}

#[derive(Debug)]
pub struct RequestWriteBlock {
    pub block_bytes: Bytes,
    pub reply_tx: oneshot::Sender<Result<block::Id, RequestWriteBlockError>>,
}

#[derive(Debug)]
pub struct RequestReadBlock {
    pub block_id: block::Id,
    pub reply_tx: oneshot::Sender<Result<Bytes, RequestReadBlockError>>,
}

#[derive(Debug)]
pub struct RequestDeleteBlock {
    pub block_id: block::Id,
    pub reply_tx: oneshot::Sender<Result<Deleted, RequestDeleteBlockError>>,
}

#[derive(Debug)]
pub struct RequestIterBlocks {
    pub reply_tx: oneshot::Sender<IterBlocks>,
}
