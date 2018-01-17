use std::net::Ipv4Addr;
use std::time::Duration;

use metriqs::db::{Db, DbOptions};
use metriqs::recv::Collector;
use metriqs::recv::push::statsd::StatsdTcpListener;

use super::config::{
    Config,
    Db as DbConfig,
    Recv as RecvConfig,
    Statsd as StatsdConfig,
    StatsdTcp as StatsdTcpConfig
};
use super::runner::{Runnable, Runner};
use super::util::f64_to_duration;

macro_rules! option {
    ($config:ident.$property:ident ?: $default:expr) => {
        ($config.and_then(|config| config.$property)).unwrap_or_else(|| $default)
    };
    ($config:ident.$property:ident) => {
        ($config.and_then(|config| config.$property))
    }
}

struct BuildContext {
    db: Option<Db>,
}

impl BuildContext {
    fn collector(&self) -> Collector {
        if let Some(ref db) = self.db {
            db.collector()
        } else {
            panic!("Db not yet available")
        }
    }
}

trait Builder<T> {
    fn build(&self, &BuildContext) -> T;
}

#[derive(Debug)]
pub struct RootBuilder {
    db: DbBuilder,
    recv: RecvBuilder,
}

impl RootBuilder {
    pub fn build(&self) -> Runner {
        let mut ctxt = BuildContext {
            db: None,
        };

        // Set up the `Db` instance from the configuration and save it in the
        // context so that subsequent builders can use it.
        let db = self.db.build(&ctxt);
        ctxt.db = Some(db);

        // Build metrics receivers (aka. `recv` internally).
        let recv = self.recv.build(&ctxt);

        // Pull the database out of the context and pass it to the runner.
        Runner::new(ctxt.db.unwrap(), recv)
    }
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
    fn build(&self, _: &BuildContext) -> Db {
        let mut options: DbOptions = Default::default();
        options.aggregation_interval = self.aggregation_interval;
        Db::new(options)
    }
}

#[derive(Debug)]
pub struct RecvBuilder {
    kinds: Vec<RecvBuilderKind>,
}

impl RecvBuilder {
    /// Convenience function to convert a config `Option` to a
    /// `Vec<RecvBuilderKind>` via its `From` trait.
    fn config_to_kinds<T>(config: Option<T>) -> Vec<RecvBuilderKind>
        where Vec<RecvBuilderKind>: From<T>
    {
        match config {
            Some(config) => From::from(config),
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

impl Builder<Vec<RecvRunnerKind>> for RecvBuilder {
    fn build(&self, ctxt: &BuildContext) -> Vec<RecvRunnerKind> {
        self.kinds.iter()
            .map(|kind| kind.build(&ctxt))
            .collect()
    }
}

#[derive(Debug)]
pub enum RecvBuilderKind {
    StatsdTcp(StatsdTcpBuilder),
}

impl From<StatsdConfig> for Vec<RecvBuilderKind> {
    fn from(config: StatsdConfig) -> Vec<RecvBuilderKind> {
        let mut kinds: Vec<RecvBuilderKind> = vec![];
        if let Some(tcps) = config.tcp {
            for tcp in tcps {
                kinds.push(RecvBuilderKind::StatsdTcp(tcp.into()))
            }
        }
        kinds
    }
}

impl Builder<RecvRunnerKind> for RecvBuilderKind {
    fn build(&self, ctxt: &BuildContext) -> RecvRunnerKind {
        use self::RecvBuilderKind::*;

        match self {
            &StatsdTcp(ref builder) => RecvRunnerKind::StatsdTcp(builder.build(&ctxt)),
        }
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

impl Builder<StatsdTcpListener> for StatsdTcpBuilder {
    fn build(&self, ctxt: &BuildContext) -> StatsdTcpListener {
        let collector = ctxt.collector();

        let port = match self.port {
            Some(port) => port,
            None => 8125,
        };
        let addr = (Ipv4Addr::new(0, 0, 0, 0), port);

        StatsdTcpListener::new(collector, addr).unwrap()
    }
}

pub enum RecvRunnerKind {
    StatsdTcp(StatsdTcpListener),
}

impl Runnable for RecvRunnerKind {
    fn run(&mut self) {
        use self::RecvRunnerKind::*;

        match self {
            &mut StatsdTcp(ref mut listener) => listener.listen(),
        }
    }
}
