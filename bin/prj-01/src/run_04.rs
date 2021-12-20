pub mod run_03;

use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
        pin::Pin, 
        thread,
    },

    run_03::TimerFuture,
};

struct Executor{
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner{
    task_sender: SyncSender<Arc<Task>>,
}

struct Task{
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner{
    fn spawn(&self, future: impl Future<Output=()>+'static+Send){
        println!("send task, Spawner::spawn()");
        let future = future.boxed();
        let task = Arc::new(Task{
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl ArcWake for Task{
    fn wake_by_ref(arc_self: &Arc<Self>){
        println!("send task, Task::wake_by_ref()");
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many queued");
    }
}

impl Executor{
    fn run(&self){
        while let Ok(task) = self.ready_queue.recv(){
            // println!("consume ready_queue");
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take(){
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);

                if future.as_mut().poll(context).is_pending(){
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn new_executor_and_spawner()->(Executor, Spawner){
    const MAX_QUEUED_TASKS :usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor{ready_queue}, Spawner{task_sender})
}

fn main(){
    std::thread::spawn(move || {
        for _ in 0..100{
            thread::sleep(Duration::new(0, 300_000_000));
            println!(".");
        }
    });

    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(async{
        println!("start");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done");
    });


    drop(spawner);
    executor.run();
}