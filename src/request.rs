use futures::Future;

pub enum RequestKind {
    ParamCaching,
    Inference,
}

pub enum Priority {
    // 0
    High,
    // 5
    Medium,
    // 10
    Low,
}

pub struct Request {}
impl Future for Request {
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        ()
    }
}
