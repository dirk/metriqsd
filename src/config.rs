#[derive(Debug, Deserialize)]
pub struct Config {
    db: Option<Db>,
    recv: Option<Recv>,
}

#[derive(Debug, Deserialize)]
struct Db {
    aggregation_interval: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct Recv {
    statsd: Option<Statsd>,
}

#[derive(Debug, Deserialize)]
struct Statsd {
    tcp: Option<Vec<StatsdTcp>>,
    udp: Option<Vec<()>>,
}

#[derive(Debug, Deserialize)]
struct StatsdTcp {
    port: Option<u16>,
}
