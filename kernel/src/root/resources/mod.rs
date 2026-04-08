pub mod stream;

#[derive(Clone)]
pub enum ResourceHandle {
    Stream(stream::StreamHandle),
}
