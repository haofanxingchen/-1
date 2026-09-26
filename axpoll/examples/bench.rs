use std::{
    sync::Arc,
    task::{Wake, Waker},
    time::Instant,
};

use axpoll::PollSet;

struct Noop;

impl Wake for Noop {
    fn wake(self: Arc<Self>) {}
    fn wake_by_ref(self: &Arc<Self>) {}
}

fn main() {
    let waker = Waker::from(Arc::new(Noop));

    // 1. register throughput.
    let n = 500_000;
    let ps = PollSet::new();
    let start = Instant::now();
    for _ in 0..n {
        ps.register(&waker);
    }
    let d = start.elapsed();
    println!(
        "register      {:>8} ops  {:>8.1} ns/op",
        n,
        d.as_nanos() as f64 / n as f64
    );

    // 2. Full drain cycle: register 64 wakers then wake() once.
    //    Each wake() allocates a fresh 64-slot buffer (64 * size_of::<Waker>()).
    let m = 100_000;
    let ps = PollSet::new();
    let start = Instant::now();
    for _ in 0..m {
        for _ in 0..64 {
            ps.register(&waker);
        }
        let _ = ps.wake();
    }
    let d = start.elapsed();
    println!(
        "64reg+wake    {:>8} ops  {:>8.1} ns/op",
        m,
        d.as_nanos() as f64 / m as f64
    );
}
