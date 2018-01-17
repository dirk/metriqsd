use metriqs::db::Db;

#[derive(Debug)]
pub struct Runner {
    db: Db,
}

impl Runner {
    pub fn new(db: Db) -> Runner {
        Runner {
            db,
        }
    }
}

pub trait Runnable {
    fn run(&mut self);
}

pub type RunnableBox = Box<Runnable>;
