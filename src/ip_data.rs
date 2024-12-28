use std::collections::VecDeque;

#[derive(Debug)]
pub struct IpData {
    pub(crate) addr: String,
    pub(crate) ip: String,
    pub(crate) rtts: VecDeque<f64>,
    pub(crate) last_attr: f64,
    pub(crate) min_rtt: f64,
    pub(crate) max_rtt: f64,
    pub(crate) sent: usize,
    pub(crate) received: usize,
    pub(crate) pop_count: usize,
}
