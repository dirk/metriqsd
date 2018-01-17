#[derive(Debug, Deserialize)]
pub struct Config {
    recv: Option<Recv>,
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
