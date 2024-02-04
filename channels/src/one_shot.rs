use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
    thread::{self, Thread},
};

pub struct OneShot<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for OneShot<T> where T: Send {}

impl<T> OneShot<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (
            Sender {
                one_shot: self,
                receiving_thread: thread::current(),
            },
            Receiver {
                one_shot: self,
                _no_send: PhantomData,
            },
        )
    }
}

impl<T> Drop for OneShot<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() };
        }
    }
}

pub struct Sender<'a, T> {
    one_shot: &'a OneShot<T>,
    receiving_thread: Thread,
}

impl<T> Sender<'_, T> {
    pub fn send(&self, message: T) {
        unsafe { (*self.one_shot.message.get()).write(message) };
        self.one_shot.ready.store(true, Ordering::Release);
        self.receiving_thread.unpark();
    }
}

pub struct Receiver<'a, T> {
    one_shot: &'a OneShot<T>,
    _no_send: PhantomData<*const ()>,
}

impl<T> Receiver<'_, T> {
    pub fn receive(&self) -> T {
        while !self.one_shot.ready.swap(false, Ordering::Acquire) {
            thread::park();
        }

        unsafe { (*self.one_shot.message.get()).assume_init_read() }
    }
}
