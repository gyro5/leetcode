use std::sync::{Arc, Condvar, Mutex};

#[derive(PartialEq)]
enum Ordering {
    First,
    Second,
    Third,
}

struct Foo {
    blocking: Arc<(Mutex<Ordering>, Condvar)>,
}

/*
https://leetcode.com/problems/print-in-order/

This is an example of simple ordering of threads using Condition Variable.

Notes that notify before wait is NOT a problem here, because that can only
happen when first thread run to finish first, as it hold the mutex until the end.

If other threads go first, they will wait and sleep, and they will be woken up
by the first thread's notify_all later. When waking up, they will try to acquire
the lock again, which will happen after the first thread finish and release the lock.
Note how the ordering variable is already changed by the first thread, and the
other threads will see this in their while loop's condition.
*/
impl Foo {
    fn new() -> Self {
        Foo {
            blocking: Arc::new((Mutex::new(Ordering::First), Condvar::new())),
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        let (mutex, cvar) = &*self.blocking;
        let mut ordering = mutex.lock().unwrap();
        while *ordering != Ordering::First {
            ordering = cvar.wait(ordering).unwrap();
        }
        print_first();
        *ordering = Ordering::Second;
        cvar.notify_all();
        // Mutex released here, after ordering is updated
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        let (mutex, cvar) = &*self.blocking;
        let mut ordering = mutex.lock().unwrap();
        while *ordering != Ordering::Second {
            ordering = cvar.wait(ordering).unwrap();
        }
        print_second();
        *ordering = Ordering::Third;
        cvar.notify_all();
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        let (mutex, cvar) = &*self.blocking;
        let mut ordering = mutex.lock().unwrap();
        while *ordering != Ordering::Third {
            ordering = cvar.wait(ordering).unwrap();
        }
        print_third();
        *ordering = Ordering::First;
        cvar.notify_all();
    }
}
