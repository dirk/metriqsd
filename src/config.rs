#[derive(Debug, Deserialize)]
pub struct Config {
    pub db: Option<Db>,
    pub recv: Option<Recv>,
}

#[derive(Debug, Deserialize)]
pub struct Db {
    pub aggregation_interval: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Recv {
    pub statsd: Option<Statsd>,
}

#[derive(Debug, Deserialize)]
pub struct Statsd {
    pub tcp: Option<Vec<StatsdTcp>>,
    pub udp: Option<Vec<()>>,
}

#[derive(Debug, Deserialize)]
pub struct StatsdTcp {
    pub port: Option<u16>,
}
