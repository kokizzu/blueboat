use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Config {
    /// Threshold for determining memory pressure. Defaults to 1G.
    #[structopt(
        long,
        env = "RW_HIGH_MEMORY_THRESHOLD_BYTES",
        default_value = "1073741824"
    )]
    pub high_memory_threshold_bytes: usize,

    /// Max number of cached and active instances.
    #[structopt(long, env = "RW_MAX_NUM_OF_INSTANCES", default_value = "50")]
    pub max_num_of_instances: usize,

    /// Size of isolate pool.
    #[structopt(long, env = "RW_ISOLATE_POOL_SIZE", default_value = "200")]
    pub isolate_pool_size: usize,

    /// Max number of concurrent RPC requests.
    #[structopt(long, env = "RW_MAX_CONCURRENT_REQUESTS", default_value = "20")]
    pub max_concurrent_requests: usize,

    /// Cache period for inactive workers. Defaults to 2 minutes.
    #[structopt(long, env = "RW_MAX_INACTIVE_TIME_MS", default_value = "120000")]
    pub max_inactive_time_ms: u64,

    /// Max isolate memory in bytes. Defaults to 16 MiB.
    #[structopt(long, env = "RW_MAX_ISOLATE_MEMORY_BYTES", default_value = "16777216")]
    pub max_isolate_memory_bytes: usize,
}
