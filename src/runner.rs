use std::fmt;
use std::sync::Arc;
use std::thread;

use metriqs::db::Db;

use super::builder::RecvRunnerKind;

pub trait Runnable {
    fn run(&mut self);
}

pub struct Runner {
    db: Option<Db>,
    recv: Vec<RecvRunnerKind>,
}

impl Runner {
    pub fn new(db: Db, recv: Vec<RecvRunnerKind>) -> Runner {
        Runner {
            db: Some(db),
            recv,
        }
    }
}

impl fmt::Debug for Runner {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Runner")
            .finish()
    }
}

impl Runnable for Runner {
    /// Fire everything up. This consumes the values in the `Runner`, so you
    /// cannot call this method again.
    fn run(&mut self) {
        let mut join_handles = vec![];

        let db = Arc::new(self.db.take().unwrap());
        let db_aggregate = db.clone();
        join_handles.push(thread::spawn(move || {
            db_aggregate.sync_aggregate()
        }));
        let db_recv = db.clone();
        join_handles.push(thread::spawn(move || {
            db_recv.sync_recv()
        }));

        for mut recv in self.recv.drain((..)) {
            join_handles.push(thread::spawn(move || {
                recv.run()
            }));
        }

        // Block the calling thread waiting for all the running threads.
        for handle in join_handles {
            handle.join().expect("Unexpected thread exit");
        }
    }
}

