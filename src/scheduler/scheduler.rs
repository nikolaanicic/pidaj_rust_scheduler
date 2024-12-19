use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::Duration,
};

use tokio::{
    self,
    sync::{Mutex, Notify, Semaphore},
    task::{self, JoinHandle},
    time,
};

use crate::common::Response;

pub struct Scheduler {
    task_queue: Mutex<VecDeque<(i32, JoinHandle<Response>)>>,
    results: Mutex<HashMap<i32, Response>>,
    notify: Arc<Notify>,
    semaphore: Arc<Semaphore>,
    schedule_interval: f32,
    shutdown_signal: Mutex<i32>,
}

impl Scheduler {
    pub fn new(max_tasks: usize) -> Arc<Self> {
        Arc::new(Self {
            task_queue: Mutex::new(VecDeque::new()),
            results: Mutex::new(HashMap::new()),
            notify: Arc::new(Notify::new()),
            semaphore: Arc::new(Semaphore::new(max_tasks)),
            schedule_interval: 0.001,
            shutdown_signal: Mutex::new(0),
        })
    }

    async fn store_result(&self, id: i32, result: Response) {
        let mut map = self.results.lock().await;
        map.insert(id, result);

        self.notify.notify_waiters();
    }

    pub async fn get_result(&self, id: i32) -> Option<Response> {
        self.results.lock().await.get(&id).cloned()
    }

    async fn is_shutdown(&self) -> bool {
        let signal = self.shutdown_signal.try_lock().unwrap();
        *signal == 1
    }

    pub async fn shutdown(self: &Arc<Self>) {
        let mut shutdown_signal = self.shutdown_signal.lock().await;
        *shutdown_signal = 1;
    }

    pub async fn add_task(self: &Arc<Self>, id: i32, t: JoinHandle<Response>) -> Arc<Notify> {
        if let Ok(permit) = self.semaphore.clone().try_acquire_owned() {
            let scheduler = Arc::clone(self);
            task::spawn(async move {
                scheduler.store_result(id, t.await.unwrap()).await;
                drop(permit);
            });

            return Arc::clone(&self.notify);
        } else {
            let mut queue = self.task_queue.lock().await;
            queue.push_back((id, t));
            return Arc::clone(&self.notify);
        }
    }

    async fn periodic_schedule(self: &Arc<Self>) {
        let interval = time::interval(Duration::from_secs_f32(self.schedule_interval));
        tokio::pin!(interval);

        loop {
            interval.tick().await;

            if self.is_shutdown().await {
                break;
            }

            let mut queue = self.task_queue.lock().await;

            let available_slots = self.semaphore.available_permits();

            for _ in 0..available_slots {
                if let Some((id, task)) = queue.pop_front() {
                    if let Ok(permit) = self.semaphore.clone().try_acquire_owned() {
                        let scheduler = Arc::clone(self);

                        task::spawn(async move {
                            let result = task.await.unwrap();
                            scheduler.store_result(id, result).await;
                            drop(permit);
                        });
                    }
                }
            }
        }
    }

    pub fn run(self: &Arc<Self>) {
        let scheduler = Arc::clone(self);
        task::spawn(async move {
            scheduler.periodic_schedule().await;
        });
    }
}
