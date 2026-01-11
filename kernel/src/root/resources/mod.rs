pub mod bytespace;
pub mod stream;

#[derive(Clone)]
pub enum ResourceHandle {
    Bytespace(bytespace::BytespaceHandle),
    Stream(stream::StreamHandle),
}
