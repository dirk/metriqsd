use std::time::Duration;

use metriqs::db::{Db, DbOptions};

use super::config::{
    Config,
    Db as DbConfig,
    Recv as RecvConfig,
    Statsd as StatsdConfig,
    StatsdTcp as StatsdTcpConfig
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

#[derive(Debug)]
pub struct RootBuilder {
    db: DbBuilder,
    recv: RecvBuilder,
}

impl From<Config> for RootBuilder {
    fn from(config: Config) -> RootBuilder {
        RootBuilder {
            db: config.db.into(),
            recv: config.recv.into(),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct RecvBuilder {
    kinds: Vec<RecvKind>,
}

impl RecvBuilder {
    /// Convenience function to convert a config `Option` to a
    /// `Vec<RecvKind>` via its `Into` trait.
    fn config_to_kinds<T: Into<Vec<RecvKind>>>(config: Option<T>) -> Vec<RecvKind> {
        match config {
            Some(config) => Into::into(config),
            None => vec![],
        }
    }
}

impl From<Option<RecvConfig>> for RecvBuilder {
    fn from(config: Option<RecvConfig>) -> RecvBuilder {
        let mut kinds = vec![];
        if let Some(config) = config {
            kinds.extend(RecvBuilder::config_to_kinds(config.statsd))
        }
        RecvBuilder {
            kinds,
        }
    }
}

#[derive(Debug)]
pub enum RecvKind {
    StatsdTcp(StatsdTcpBuilder),
}

impl From<StatsdConfig> for Vec<RecvKind> {
    fn from(config: StatsdConfig) -> Vec<RecvKind> {
        let mut kinds: Vec<RecvKind> = vec![];
        if let Some(tcps) = config.tcp {
            for tcp in tcps {
                kinds.push(RecvKind::StatsdTcp(tcp.into()))
            }
        }
        kinds
    }
}

#[derive(Debug)]
pub struct StatsdTcpBuilder {
    port: Option<u16>,
}

impl From<StatsdTcpConfig> for StatsdTcpBuilder {
    fn from(config: StatsdTcpConfig) -> StatsdTcpBuilder {
        StatsdTcpBuilder {
            port: config.port,
        }
    }
}
