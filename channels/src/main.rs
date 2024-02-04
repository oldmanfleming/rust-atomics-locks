use std::thread;

use one_shot::OneShot;

mod channel;
mod one_shot;

fn main() {
    let mut one_shot = OneShot::new();
    let t = thread::current();

    thread::scope(|s| {
        let (sender, receiver) = one_shot.split();

        s.spawn(move || {
            sender.send("Hello, world!");
            t.unpark();
        });
        println!("{}", receiver.receive());
    });
}
