use quick_js::{Context, JsValue};
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Job = Box<SSRRequest>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct SSRRequest {
    path: String,
    sender: mpsc::Sender<String>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                Context::new().unwrap(),
            ));
        }
        ThreadPool { workers, sender }
    }
    pub fn execute(&self, path: String) -> String {
        let (sender, receiver) = mpsc::channel();
        let ssr_request = SSRRequest { path, sender };
        let job = Box::new(ssr_request);
        self.sender.send(job).unwrap();
        receiver.recv().unwrap()
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, ctx: Context) -> Worker {
        let thread = thread::spawn(move || {
            let ctx = Context::new().unwrap();
            let shared_ctx = Arc::new(Mutex::new(&ctx));
            let _loaded_renderer = ctx
                .eval("function renderer (str) { return `${str} is rendered`; }")
                .unwrap();
            let (tx, rx): (mpsc::Sender<Context>, mpsc::Receiver<Context>) = channel();
            let (shared_ctx, tx) = (Arc::clone(&shared_ctx), tx.clone());
            loop {
                let ctx = shared_ctx.lock().unwrap();
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                let result = ctx.eval("renderer('foobar')").unwrap();
                job.sender.send(result.into_string().unwrap()).unwrap();
            }
        });

        Worker { id, thread }
    }
}
