use futures::{
    channel::{
        mpsc,
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
    proto,
    IterBlocks,
    IterBlocksItem,
};

pub fn gen_server_start() {

}

pub enum Order {
    Request(proto::Request),
    Reply(OrderReply),
}

pub struct IterBlocksNext {
    blocks_tx: mpsc::Sender<IterBlocksItem>,
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
    IterBlocksInitCancel(komm::UmschlagAbbrechen<proto::RequestIterBlocksReplyTx>),
    IterBlocksInit(komm::Umschlag<blockwheel_fs::IterBlocks, proto::RequestIterBlocksReplyTx>),
    IterBlocksNextCancel(komm::UmschlagAbbrechen<IterBlocksNext>),
    IterBlocksNext(komm::Umschlag<blockwheel_fs::IterBlocksItem, IterBlocksNext>),
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

impl From<komm::UmschlagAbbrechen<proto::RequestIterBlocksReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestIterBlocksReplyTx>) -> Order {
        Order::Reply(OrderReply::IterBlocksInitCancel(v))
    }
}

impl From<komm::Umschlag<blockwheel_fs::IterBlocks, proto::RequestIterBlocksReplyTx>> for Order {
    fn from(v: komm::Umschlag<blockwheel_fs::IterBlocks, proto::RequestIterBlocksReplyTx>) -> Order {
        Order::Reply(OrderReply::IterBlocksInit(v))
    }
}

impl From<komm::UmschlagAbbrechen<IterBlocksNext>> for Order {
    fn from(v: komm::UmschlagAbbrechen<IterBlocksNext>) -> Order {
        Order::Reply(OrderReply::IterBlocksNextCancel(v))
    }
}

impl From<komm::Umschlag<blockwheel_fs::IterBlocksItem, IterBlocksNext>> for Order {
    fn from(v: komm::Umschlag<blockwheel_fs::IterBlocksItem, IterBlocksNext>) -> Order {
        Order::Reply(OrderReply::IterBlocksNext(v))
    }
}
