mod spin_lock;

use std::thread;

use spin_lock::SpinLock;

fn main() {
    let x = SpinLock::new(Vec::new());

    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(3);
        });
    });

    let g = x.lock();

    println!("{:?}", g.as_slice());

    assert!(g.as_slice() == [1, 2, 3] || g.as_slice() == [2, 3, 1]);
}
