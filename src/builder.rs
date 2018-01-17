use std::time::Duration;

use metriqs::db::{Db, DbOptions};

use super::config::{
    Config,
    Db as DbConfig,
};
use super::util::f64_to_duration;

macro_rules! option {
    ($config:ident.$property:ident ?: $default:expr) => {
        ($config.and_then(|config| config.$property)).unwrap_or_else(|| $default)
    };
    ($config:ident.$property:ident) => {
        ($config.and_then(|config| config.$property))
    }
}

trait Builder<T> {
    fn build(&self) -> T;
}

pub struct RootBuilder {
    db: DbBuilder,
}

impl From<Config> for RootBuilder {
    fn from(config: Config) -> RootBuilder {
        RootBuilder {
            db: config.db.into(),
        }
    }
}

pub struct DbBuilder {
    aggregation_interval: Option<Duration>,
}

impl From<Option<DbConfig>> for DbBuilder {
    fn from(config: Option<DbConfig>) -> DbBuilder {
        DbBuilder {
            aggregation_interval: option!(config.aggregation_interval).map(f64_to_duration),
        }
    }
}

impl Builder<Db> for DbBuilder {
    fn build(&self) -> Db {
        let mut options: DbOptions = Default::default();
        options.aggregation_interval = self.aggregation_interval;
        Db::new(options)
    }
}

