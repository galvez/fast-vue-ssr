use std::sync::mpsc;

pub type Job = Box<RendererRequest>;

pub struct RendererRequest {
    url: String,
    // method: &'static str,
    // headers: Vec<HeaderName, HeaderValue>,
    // asyncData: String,
    receiver: mpsc::Sender<String>,
    sender: mpsc::Sender<String>,
}

impl RendererRequest {
	pub fn new (url: String) {
        let (sender, receiver) = mpsc::channel();
        RendererRequest {
        	url,
        	receiver,
        	sender,
        }
	}
}
