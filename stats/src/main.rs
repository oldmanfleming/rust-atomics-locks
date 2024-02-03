use std::{sync::atomic::{AtomicU64, AtomicUsize, Ordering}, thread, time::{Duration, Instant}};

use rand::Rng;

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(move || {

                for _ in 0..25 {
                    let start = Instant::now();
                    let mut rng = rand::thread_rng();
                    let sleep_time = rng.gen_range(0..10);
                    thread::sleep(Duration::from_micros(sleep_time));
                    let elapsed = start.elapsed().as_micros() as u64;

                    num_done.fetch_add(1, Ordering::Relaxed);
                    total_time.fetch_add(elapsed, Ordering::Relaxed);
                    max_time.fetch_max(elapsed, Ordering::Relaxed);
                }
            });
        }

        loop {
            let total_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            let num_done = num_done.load(Ordering::Relaxed);
            if num_done == 100 {
                break;
            }
            if num_done == 0 {
                continue;
            } else {
                println!("Working.. {num_done}/100, {:?} average, {:?} peak", total_time / num_done as u32, max_time);
            }

        }
    });

    println!("Done!");
}
