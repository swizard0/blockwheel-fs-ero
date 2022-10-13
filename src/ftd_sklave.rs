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

use arbeitssklave::{
    komm,
};

use crate::{
    job,
    proto,
    block,
    Info,
    Flushed,
    Deleted,
    RequestReadBlockError,
    RequestWriteBlockError,
    RequestDeleteBlockError,
};

pub type SklaveJob = arbeitssklave::SklaveJob<Welt, Order>;

pub enum Order {
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

pub struct RequestIterBlocksInit {
    pub iter_blocks_init_tx: oneshot::Sender<blockwheel_fs::IterBlocks>,
}

pub struct RequestIterBlocksNext {
    pub iter_blocks_next_tx: oneshot::Sender<blockwheel_fs::IterBlocksItem>,
}

pub struct Welt;

#[derive(Debug)]
pub enum Error {
    ReceiveOrder(arbeitssklave::Error),
    GenServerIsLostOnRequestInfo,
    GenServerIsLostOnRequestFlush,
    GenServerIsLostOnRequestWriteBlock,
    GenServerIsLostOnRequestReadBlock,
    GenServerIsLostOnRequestDeleteBlock,
    GenServerIsLostOnRequestIterBlocksInit,
    GenServerIsLostOnRequestIterBlocksNext,
}

pub fn job<P>(sklave_job: SklaveJob, thread_pool: &P) where P: edeltraud::ThreadPool<job::Job> {
    if let Err(error) = run_job(sklave_job, thread_pool) {
        log::error!("job terminated with error: {:?}", error);
    }
}

fn run_job<P>(mut sklave_job: SklaveJob, _thread_pool: &P) -> Result<(), Error> where P: edeltraud::ThreadPool<job::Job> {
    loop {
        let mut befehle = match sklave_job.zu_ihren_diensten() {
            Ok(arbeitssklave::Gehorsam::Machen { befehle, }) =>
                befehle,
            Ok(arbeitssklave::Gehorsam::Rasten) =>
                return Ok(()),
            Err(error) =>
                return Err(Error::ReceiveOrder(error)),
        };
        loop {
            match befehle.befehl() {
                arbeitssklave::SklavenBefehl::Mehr { befehl, mehr_befehle, } => {
                    befehle = mehr_befehle;
                    match befehl {
                        Order::InfoCancel(komm::UmschlagAbbrechen { .. }) =>
                            return Err(Error::GenServerIsLostOnRequestInfo),
                        Order::Info(komm::Umschlag { inhalt: info, stamp: reply_tx, }) =>
                            if let Err(_send_error) = reply_tx.send(info) {
                                log::debug!("client is gone during RequestInfo");
                            },
                        Order::FlushCancel(komm::UmschlagAbbrechen { .. }) =>
                            return Err(Error::GenServerIsLostOnRequestFlush),
                        Order::Flush(komm::Umschlag { inhalt: Flushed, stamp: reply_tx, }) =>
                            if let Err(_send_error) = reply_tx.send(Flushed) {
                                log::debug!("client is gone during RequestFlush");
                            },
                        Order::WriteBlockCancel(komm::UmschlagAbbrechen { .. }) =>
                            return Err(Error::GenServerIsLostOnRequestWriteBlock),
                        Order::WriteBlock(komm::Umschlag { inhalt: write_block_result, stamp: reply_tx, }) =>
                            if let Err(_send_error) = reply_tx.send(write_block_result) {
                                log::debug!("client is gone during RequestWriteBlock");
                            },
                        Order::ReadBlockCancel(komm::UmschlagAbbrechen { .. }) =>
                            return Err(Error::GenServerIsLostOnRequestReadBlock),
                        Order::ReadBlock(komm::Umschlag { inhalt: read_block_result, stamp: reply_tx, }) =>
                            if let Err(_send_error) = reply_tx.send(read_block_result) {
                                log::debug!("client is gone during RequestReadBlock");
                            },
                        Order::DeleteBlockCancel(komm::UmschlagAbbrechen { .. }) =>
                            return Err(Error::GenServerIsLostOnRequestDeleteBlock),
                        Order::DeleteBlock(komm::Umschlag { inhalt: delete_block_result, stamp: reply_tx, }) =>
                            if let Err(_send_error) = reply_tx.send(delete_block_result) {
                                log::debug!("client is gone during RequestDeleteBlock");
                            },
                        Order::IterBlocksInitCancel(komm::UmschlagAbbrechen { .. }) =>
                            return Err(Error::GenServerIsLostOnRequestIterBlocksInit),
                        Order::IterBlocksInit(komm::Umschlag {
                            inhalt: iter_blocks,
                            stamp: RequestIterBlocksInit { iter_blocks_init_tx, },
                        }) =>
                            if let Err(_send_error) = iter_blocks_init_tx.send(iter_blocks) {
                                log::debug!("iter blocks stream process is gone during RequestIterBlocksInit");
                            },
                        Order::IterBlocksNextCancel(komm::UmschlagAbbrechen { .. }) =>
                            return Err(Error::GenServerIsLostOnRequestIterBlocksNext),
                        Order::IterBlocksNext(komm::Umschlag {
                            inhalt: iter_blocks_item,
                            stamp: RequestIterBlocksNext { iter_blocks_next_tx, },
                        }) =>
                            if let Err(_send_error) = iter_blocks_next_tx.send(iter_blocks_item) {
                                log::debug!("iter blocks stream process is gone during RequestIterBlocksNext");
                            },
                    }
                },
                arbeitssklave::SklavenBefehl::Ende { sklave_job: next_sklave_job, } => {
                    sklave_job = next_sklave_job;
                    break;
                },
            }
        }
    }
}

impl From<komm::UmschlagAbbrechen<proto::RequestInfoReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestInfoReplyTx>) -> Order {
        Order::InfoCancel(v)
    }
}

impl From<komm::Umschlag<Info, proto::RequestInfoReplyTx>> for Order {
    fn from(v: komm::Umschlag<Info, proto::RequestInfoReplyTx>) -> Order {
        Order::Info(v)
    }
}

impl From<komm::UmschlagAbbrechen<proto::RequestFlushReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestFlushReplyTx>) -> Order {
        Order::FlushCancel(v)
    }
}

impl From<komm::Umschlag<Flushed, proto::RequestFlushReplyTx>> for Order {
    fn from(v: komm::Umschlag<Flushed, proto::RequestFlushReplyTx>) -> Order {
        Order::Flush(v)
    }
}

impl From<komm::UmschlagAbbrechen<proto::RequestWriteBlockReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestWriteBlockReplyTx>) -> Order {
        Order::WriteBlockCancel(v)
    }
}

impl From<komm::Umschlag<Result<block::Id, RequestWriteBlockError>, proto::RequestWriteBlockReplyTx>> for Order {
    fn from(v: komm::Umschlag<Result<block::Id, RequestWriteBlockError>, proto::RequestWriteBlockReplyTx>) -> Order {
        Order::WriteBlock(v)
    }
}

impl From<komm::UmschlagAbbrechen<proto::RequestReadBlockReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestReadBlockReplyTx>) -> Order {
        Order::ReadBlockCancel(v)
    }
}

impl From<komm::Umschlag<Result<Bytes, RequestReadBlockError>, proto::RequestReadBlockReplyTx>> for Order {
    fn from(v: komm::Umschlag<Result<Bytes, RequestReadBlockError>, proto::RequestReadBlockReplyTx>) -> Order {
        Order::ReadBlock(v)
    }
}

impl From<komm::UmschlagAbbrechen<proto::RequestDeleteBlockReplyTx>> for Order {
    fn from(v: komm::UmschlagAbbrechen<proto::RequestDeleteBlockReplyTx>) -> Order {
        Order::DeleteBlockCancel(v)
    }
}

impl From<komm::Umschlag<Result<Deleted, RequestDeleteBlockError>, proto::RequestDeleteBlockReplyTx>> for Order {
    fn from(v: komm::Umschlag<Result<Deleted, RequestDeleteBlockError>, proto::RequestDeleteBlockReplyTx>) -> Order {
        Order::DeleteBlock(v)
    }
}

impl From<komm::UmschlagAbbrechen<RequestIterBlocksInit>> for Order {
    fn from(v: komm::UmschlagAbbrechen<RequestIterBlocksInit>) -> Order {
        Order::IterBlocksInitCancel(v)
    }
}

impl From<komm::Umschlag<blockwheel_fs::IterBlocks, RequestIterBlocksInit>> for Order {
    fn from(v: komm::Umschlag<blockwheel_fs::IterBlocks, RequestIterBlocksInit>) -> Order {
        Order::IterBlocksInit(v)
    }
}

impl From<komm::UmschlagAbbrechen<RequestIterBlocksNext>> for Order {
    fn from(v: komm::UmschlagAbbrechen<RequestIterBlocksNext>) -> Order {
        Order::IterBlocksNextCancel(v)
    }
}

impl From<komm::Umschlag<blockwheel_fs::IterBlocksItem, RequestIterBlocksNext>> for Order {
    fn from(v: komm::Umschlag<blockwheel_fs::IterBlocksItem, RequestIterBlocksNext>) -> Order {
        Order::IterBlocksNext(v)
    }
}
