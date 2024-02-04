use std::{
    sync::atomic::{
        AtomicPtr,
        Ordering::{Acquire, Release},
    },
    thread,
};

use rand::Rng;

struct Data {
    data: i32,
}

fn main() {
    thread::scope(|s| {
        s.spawn(|| {
            let data = get_data();
            println!("result: {}", data.data);
        });

        s.spawn(|| {
            let data = get_data();
            println!("result: {}", data.data);
        });

        s.spawn(|| {
            let data = get_data();
            println!("result: {}", data.data);
        });
    });
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        let data = rand::thread_rng().gen_range(0..100);
        println!("gen: {}", data);
        p = Box::into_raw(Box::new(Data { data }));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            // Safety: p comes from Box::into_raw right above,
            // and wasn't shared with any other thread.
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // Safety: p is not null and points to a properly initialized value.
    unsafe { &*p }
}
