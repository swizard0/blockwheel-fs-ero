use arbeitssklave::{
    komm,
};

use crate::{
    proto,
    ftd_sklave,
};

pub struct EchoPolicy;

impl blockwheel_fs::EchoPolicy for EchoPolicy {
    type Info = komm::Rueckkopplung<ftd_sklave::Order, proto::RequestInfoReplyTx>;
    type Flush = komm::Rueckkopplung<ftd_sklave::Order, proto::RequestFlushReplyTx>;
    type WriteBlock = komm::Rueckkopplung<ftd_sklave::Order, proto::RequestWriteBlockReplyTx>;
    type ReadBlock = komm::Rueckkopplung<ftd_sklave::Order, proto::RequestReadBlockReplyTx>;
    type DeleteBlock = komm::Rueckkopplung<ftd_sklave::Order, proto::RequestDeleteBlockReplyTx>;
    type IterBlocksInit = komm::Rueckkopplung<ftd_sklave::Order, ftd_sklave::RequestIterBlocksInit>;
    type IterBlocksNext = komm::Rueckkopplung<ftd_sklave::Order, ftd_sklave::RequestIterBlocksNext>;
}
