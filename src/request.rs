use std::sync::mpsc;

pub struct RendererRequest {
    pub url: String,
    // method: &'static str,
    // headers: Vec<HeaderName, HeaderValue>,
    // asyncData: String,
}

impl RendererRequest {
	pub fn new(url: String) -> RendererRequest {
        RendererRequest {
        	url,
        }
	}
}

pub struct RendererJob {
	pub request: RendererRequest,
    pub receiver: mpsc::Receiver<String>,
    pub sender: mpsc::Sender<String>,	
}

impl RendererJob {
	pub fn new (request: RendererRequest) -> RendererJob {
		let (sender, receiver) = mpsc::channel();
		RendererJob {
			request,
			sender,
			receiver
		}
	}
}

// pub type Job = Box<&'a RendererRequest>;
