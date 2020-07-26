use std::fs::read_to_string;
use std::sync::{
    mpsc,
    Arc,
    Mutex,
};
use std::thread;
use quick_js::{
    JsValue,
    Context,
};
use crate::renderer::request::{
    RendererRequest,
};
use crate::jslib::{
    vue::SRC as VUE,
    vue_server_renderer::SRC as VUE_SERVER_RENDERER,
    vue_router::SRC as VUE_ROUTER,
};
use crate::renderer::jsctx::{
    INJECT_SSR_CONTEXT,
    WAIT_ROUTER_READY,
    RENDER_VUE_COMPONENT,
};

static BUNDLE_PATH: &'static str = "./dist/server.js";

pub struct RendererJob {
    pub request: RendererRequest,
    pub sender: mpsc::Sender<String>,   
}

pub struct RendererPool {
    sender: mpsc::Sender<RendererJob>,
}

impl RendererJob {
    pub fn new (request: RendererRequest, sender: mpsc::Sender<String>) -> RendererJob {
        RendererJob {
            request,
            sender
        }
    }
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
        let (sender, receiver) = mpsc::channel();        
        let job = RendererJob::new(request, sender);
        self.sender.send(job).unwrap();
        receiver.recv().unwrap()
    }
    fn start_worker(receiver: Arc<Mutex<mpsc::Receiver<RendererJob>>>, bundle: &'static str) {
        thread::spawn(move || {
            let ctx = Context::new().unwrap();
            // let _add_log = ctx.add_callback("warpLog", |msg: String| {
            //     println!("{:?}", msg);
            //     "true"
            // }).unwrap();
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
                // Set req.url
                RendererPool::run_js(
                    &ctx,
                    format!(
                        "$ssrContext.req.url = '{}'; true",
                        &job.request.url
                    ).as_str(),
                );
                // Set req.method
                RendererPool::run_js(
                    &ctx,
                    format!(
                        "$ssrContext.req.method = '{}'; true",
                        &job.request.method
                    ).as_str(),
                );
                // Set req.headers
                RendererPool::run_js(
                    &ctx,
                    format!(
                        "$ssrContext.req.headers = JSON.parse('{}'); true",
                        &job.request.headers
                    ).as_str(),
                );
                // Set async data
                RendererPool::run_js(
                    &ctx,
                    format!(
                        "$ssrContext.data = '{}'; true",
                        &job.request.async_data
                    ).as_str(),
                );
                let _wait_router_ready = ctx.eval(WAIT_ROUTER_READY).unwrap();
                let result = ctx.eval(RENDER_VUE_COMPONENT)
                    .unwrap()
                    .into_string()
                    .unwrap();
                job.sender.send(result).unwrap();
            }
        });
        ()
    }
    fn run_js (ctx: &Context, snippet: &str) {
        let _ = ctx.eval(format!("{}", snippet).as_str()).unwrap();
        ()
    }
}
