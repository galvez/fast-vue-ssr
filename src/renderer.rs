use std::fs::read_to_string;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use quick_js::Context;
use crate::vue::{
    RENDER,
    VUE,
    VUE_SERVER_RENDERER,
    VUE_ROUTER,
};

static BUNDLE_PATH: &'static str = "./dist/server.js";

static INJECT_SSR_CONTEXT: &'static str = r###"
Vue.mixin({
  beforeCreate () {
    this.$ssrContext = this.$root.$options.$ssrContext
  }
})

let ready = false

let $ssrContext = {
  req: {
  }
}

true
"###;

static ROUTER_READY: &'static str = r###"
ready = false
router.push($ssrContext.req.url)

new Promise((resolve) => {
    router.onReady(() => {
        resolve(true)
    })
})
"###;

type Job = Box<RendererRequest>;

struct RendererRequest {
    url: String,
    sender: mpsc::Sender<String>,
}

pub struct RendererPool {
    sender: mpsc::Sender<Job>,
}

impl RendererPool {
    pub fn new(size: usize) -> RendererPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        lazy_static! {
            static ref BUNDLE: String = read_to_string(BUNDLE_PATH).unwrap();
        }
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver), &BUNDLE));
        }
        RendererPool { sender }
    }
    pub fn render(&self, url: String) -> String {
        let (sender, receiver) = mpsc::channel();
        let ssr_request = RendererRequest { url, sender };
        let job = Box::new(ssr_request);
        self.sender.send(job).unwrap();
        receiver.recv().unwrap()
    }
}

struct Worker {}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>, bundle: &'static str) -> Worker {
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
                let _set_url = ctx.eval(format!("$ssrContext.req.url = '{}'; true", &job.url).as_str()).unwrap();
                let _push_url = ctx.eval(ROUTER_READY).unwrap();
                let result = ctx.eval(RENDER).unwrap();
                job.sender.send(result.into_string().unwrap()).unwrap();
            }
        });

        Worker {}
    }
}
