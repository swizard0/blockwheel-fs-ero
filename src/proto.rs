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

use crate::{
    block,
    Info,
    Flushed,
    Deleted,
    IterBlocks,
    RequestReadBlockError,
    RequestWriteBlockError,
    RequestDeleteBlockError,
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

pub type RequestInfoReplyTx = oneshot::Sender<Info>;

#[derive(Debug)]
pub struct RequestInfo {
    pub reply_tx: RequestInfoReplyTx,
}

pub type RequestFlushReplyTx = oneshot::Sender<Flushed>;

#[derive(Debug)]
pub struct RequestFlush {
    pub reply_tx: RequestFlushReplyTx,
}

pub type RequestWriteBlockReplyTx = oneshot::Sender<Result<block::Id, RequestWriteBlockError>>;

#[derive(Debug)]
pub struct RequestWriteBlock {
    pub block_bytes: Bytes,
    pub reply_tx: RequestWriteBlockReplyTx,
}

pub type RequestReadBlockReplyTx = oneshot::Sender<Result<Bytes, RequestReadBlockError>>;

#[derive(Debug)]
pub struct RequestReadBlock {
    pub block_id: block::Id,
    pub reply_tx: RequestReadBlockReplyTx,
}

pub type RequestDeleteBlockReplyTx = oneshot::Sender<Result<Deleted, RequestDeleteBlockError>>;

#[derive(Debug)]
pub struct RequestDeleteBlock {
    pub block_id: block::Id,
    pub reply_tx: RequestDeleteBlockReplyTx,
}

pub type RequestIterBlocksReplyTx = oneshot::Sender<IterBlocks>;

#[derive(Debug)]
pub struct RequestIterBlocks {
    pub reply_tx: RequestIterBlocksReplyTx,
}
