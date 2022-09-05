#[forbid(unsafe_code)]

use futures::{
    channel::{
        mpsc,
    },
    stream,
    StreamExt,
};

pub struct GenServer {
    request_tx: mpsc::Sender<Request>,
    fused_request_rx: stream::Fuse<mpsc::Receiver<Request>>,
}

enum Request {

}
