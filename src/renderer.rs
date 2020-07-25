use std::fs::read_to_string;
use std::sync::{
    mpsc,
    Arc,
    Mutex,
};
use std::thread;
use quick_js::Context;
use crate::request::{
    RendererJob,
    RendererRequest,
};
use crate::vue::{
    RENDER,
    VUE,
    VUE_SERVER_RENDERER,
    VUE_ROUTER,
};

static BUNDLE_PATH: &'static str = "./dist/server.js";

pub struct RendererPool {
    sender: mpsc::Sender<RendererJob>,
}

impl RendererPool {
    pub fn new(size: usize) -> RendererPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        lazy_static! {
            static ref BUNDLE: String = read_to_string(BUNDLE_PATH).unwrap();
        }
        for _ in 0..size {
            RendererPool::start_worker(Arc::clone(&receiver), &BUNDLE);
        }
        RendererPool { sender }
    }
    pub fn render(&self, request: RendererRequest) -> String {
        let job = RendererJob::new(request);
        self.sender.send(job).unwrap();
        job.receiver.recv().unwrap()
    }
    fn start_worker(receiver: Arc<Mutex<mpsc::Receiver<RendererJob>>>, bundle: &'static str) {
        thread::spawn(move || {
            let ctx = Context::new().unwrap();
            let _set_vars = ctx
                .eval("let result; let error; true")
                .unwrap();
            let shared_ctx = Arc::new(Mutex::new(&ctx));
            let _loaded_vue = ctx.eval(VUE).unwrap();
            let _loaded_vue_server_renderer = ctx.eval(VUE_SERVER_RENDERER).unwrap();
            let _loaded_vue_router = ctx.eval(VUE_ROUTER).unwrap();
            let _injected_ssr_context = ctx.eval(INJECT_SSR_CONTEXT).unwrap();
            let _loaded_bundle = ctx.eval(format!("{}\ntrue", bundle).as_str()).unwrap();
            let shared_ctx = Arc::clone(&shared_ctx);
            loop {
                let ctx = shared_ctx.lock().unwrap();
                let job = receiver.lock().unwrap().recv().unwrap();
                // println!("$ssrContext.req.url = '{}';", &job.url);
                let _set_url = ctx.eval(format!("$ssrContext.req.url = '{}'; true", &job.request.url).as_str()).unwrap();
                let _push_url = ctx.eval(ROUTER_READY).unwrap();
                let result = ctx.eval(RENDER).unwrap();
                job.sender.send(result.into_string().unwrap()).unwrap();
            }
        });
        ()
    }
}
