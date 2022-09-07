use futures::{
    channel::{
        mpsc,
        oneshot,
    },
};

use alloc_pool::{
    bytes::{
        Bytes,
    },
};

use arbeitssklave::{
    komm,
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
    job,
    proto,
    gen_server,
    access_policy::{
        AccessPolicy,
    },
    IterBlocks,
    IterBlocksItem,
};

pub type SklaveJob = arbeitssklave::SklaveJob<Welt, Order>;

pub enum Order {
    RequestInfo(proto::RequestInfo),
    RequestFlush(proto::RequestFlush),
    RequestWriteBlock(proto::RequestWriteBlock),
    RequestReadBlock(proto::RequestReadBlock),
    RequestDeleteBlock(proto::RequestDeleteBlock),
    RequestIterBlocksInit(RequestIterBlocksInit),
    RequestIterBlocksNext(RequestIterBlocksNext),
    Reply(OrderReply),
}

pub struct RequestIterBlocksInit {
    pub iter_blocks_init_tx: oneshot::Sender<blockwheel_fs::IterBlocks>,
}

pub struct RequestIterBlocksNext {
    pub iter_blocks_next_tx: oneshot::Sender<blockwheel_fs::IterBlocksItem>,
}

pub struct Welt;

pub fn job<P>(mut sklave_job: SklaveJob, thread_pool: &P) where P: edeltraud::ThreadPool<job::Job> {

    todo!()
}

pub enum OrderReply {
    InfoCancel(komm::UmschlagAbbrechen<proto::RequestInfoReplyTx>),
    Info(komm::Umschlag<Info, proto::RequestInfoReplyTx>),
    FlushCancel(komm::UmschlagAbbrechen<proto::RequestFlushReplyTx>),
    Flush(komm::Umschlag<Flushed, proto::RequestFlushReplyTx>),
    WriteBlockCancel(komm::UmschlagAbbrechen<proto::RequestWriteBlockReplyTx>),
    WriteBlock(komm::Umschlag<Result<block::Id, RequestWriteBlockError>, proto::RequestWriteBlockReplyTx>),
    ReadBlockCancel(komm::UmschlagAbbrechen<proto::RequestReadBlockReplyTx>),
    ReadBlock(komm::Umschlag<Result<Bytes, RequestReadBlockError>, proto::RequestReadBlockReplyTx>),
    DeleteBlockCancel(komm::UmschlagAbbrechen<proto::RequestDeleteBlockReplyTx>),
    DeleteBlock(komm::Umschlag<Result<Deleted, RequestDeleteBlockError>, proto::RequestDeleteBlockReplyTx>),
    IterBlocksInitCancel(komm::UmschlagAbbrechen<RequestIterBlocksInit>),
    IterBlocksInit(komm::Umschlag<blockwheel_fs::IterBlocks, RequestIterBlocksInit>),
    IterBlocksNextCancel(komm::UmschlagAbbrechen<RequestIterBlocksNext>),
    IterBlocksNext(komm::Umschlag<blockwheel_fs::IterBlocksItem, RequestIterBlocksNext>),
}

impl From<komm::UmschlagAbbrechen<proto::RequestInfoReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestInfoReplyTx>) -> Order {
        Order::Reply(OrderReply::InfoCancel(v))
    }
}

impl From<komm::Umschlag<Info, proto::RequestInfoReplyTx>> for Order {
    fn from(v: komm::Umschlag<Info, proto::RequestInfoReplyTx>) -> Order {
        Order::Reply(OrderReply::Info(v))
    }
}

impl From<komm::UmschlagAbbrechen<proto::RequestFlushReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestFlushReplyTx>) -> Order {
        Order::Reply(OrderReply::FlushCancel(v))
    }
}

impl From<komm::Umschlag<Flushed, proto::RequestFlushReplyTx>> for Order {
    fn from(v: komm::Umschlag<Flushed, proto::RequestFlushReplyTx>) -> Order {
        Order::Reply(OrderReply::Flush(v))
    }
}

impl From<komm::UmschlagAbbrechen<proto::RequestWriteBlockReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestWriteBlockReplyTx>) -> Order {
        Order::Reply(OrderReply::WriteBlockCancel(v))
    }
}

impl From<komm::Umschlag<Result<block::Id, RequestWriteBlockError>, proto::RequestWriteBlockReplyTx>> for Order {
    fn from(v: komm::Umschlag<Result<block::Id, RequestWriteBlockError>, proto::RequestWriteBlockReplyTx>) -> Order {
        Order::Reply(OrderReply::WriteBlock(v))
    }
}

impl From<komm::UmschlagAbbrechen<proto::RequestReadBlockReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestReadBlockReplyTx>) -> Order {
        Order::Reply(OrderReply::ReadBlockCancel(v))
    }
}

impl From<komm::Umschlag<Result<Bytes, RequestReadBlockError>, proto::RequestReadBlockReplyTx>> for Order {
    fn from(v: komm::Umschlag<Result<Bytes, RequestReadBlockError>, proto::RequestReadBlockReplyTx>) -> Order {
        Order::Reply(OrderReply::ReadBlock(v))
    }
}

impl From<komm::UmschlagAbbrechen<proto::RequestDeleteBlockReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestDeleteBlockReplyTx>) -> Order {
        Order::Reply(OrderReply::DeleteBlockCancel(v))
    }
}

impl From<komm::Umschlag<Result<Deleted, RequestDeleteBlockError>, proto::RequestDeleteBlockReplyTx>> for Order {
    fn from(v: komm::Umschlag<Result<Deleted, RequestDeleteBlockError>, proto::RequestDeleteBlockReplyTx>) -> Order {
        Order::Reply(OrderReply::DeleteBlock(v))
    }
}

impl From<komm::UmschlagAbbrechen<RequestIterBlocksInit>> for Order {
    fn from(v: komm::UmschlagAbbrechen<RequestIterBlocksInit>) -> Order {
        Order::Reply(OrderReply::IterBlocksInitCancel(v))
    }
}

impl From<komm::Umschlag<blockwheel_fs::IterBlocks, RequestIterBlocksInit>> for Order {
    fn from(v: komm::Umschlag<blockwheel_fs::IterBlocks, RequestIterBlocksInit>) -> Order {
        Order::Reply(OrderReply::IterBlocksInit(v))
    }
}

impl From<komm::UmschlagAbbrechen<RequestIterBlocksNext>> for Order {
    fn from(v: komm::UmschlagAbbrechen<RequestIterBlocksNext>) -> Order {
        Order::Reply(OrderReply::IterBlocksNextCancel(v))
    }
}

impl From<komm::Umschlag<blockwheel_fs::IterBlocksItem, RequestIterBlocksNext>> for Order {
    fn from(v: komm::Umschlag<blockwheel_fs::IterBlocksItem, RequestIterBlocksNext>) -> Order {
        Order::Reply(OrderReply::IterBlocksNext(v))
    }
}
