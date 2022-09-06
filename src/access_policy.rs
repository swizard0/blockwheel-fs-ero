use crate::{
    proto,
    gen_server,
};

pub struct AccessPolicy;

impl blockwheel_fs::AccessPolicy for AccessPolicy {
    type Order = gen_server::Order;
    type Info = proto::RequestInfoReplyTx;
    type Flush = proto::RequestFlushReplyTx;
    type WriteBlock = proto::RequestWriteBlockReplyTx;
    type ReadBlock = proto::RequestReadBlockReplyTx;
    type DeleteBlock = proto::RequestDeleteBlockReplyTx;
    type IterBlocksInit = proto::RequestIterBlocksReplyTx;
    type IterBlocksNext = gen_server::IterBlocksNext;
}
