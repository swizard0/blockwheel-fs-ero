use crate::{
    proto,
    ftd_sklave,
};

pub struct AccessPolicy;

impl blockwheel_fs::AccessPolicy for AccessPolicy {
    type Order = ftd_sklave::Order;
    type Info = proto::RequestInfoReplyTx;
    type Flush = proto::RequestFlushReplyTx;
    type WriteBlock = proto::RequestWriteBlockReplyTx;
    type ReadBlock = proto::RequestReadBlockReplyTx;
    type DeleteBlock = proto::RequestDeleteBlockReplyTx;
    type IterBlocksInit = proto::RequestIterBlocksReplyTx;
    type IterBlocksNext = ftd_sklave::IterBlocksNext;
}
