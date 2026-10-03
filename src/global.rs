use std::{cell::UnsafeCell, ops::{Deref, DerefMut}};

pub struct Global<T>(UnsafeCell<T>);

impl<T> Global<T> {
    pub const fn new(value: T) -> Self {
        Global(UnsafeCell::new(value))
    }

    #[inline(always)]
    pub fn modify<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        // println!("Thread id: {:?} - Modifying global value", std::thread::current().id());
        // SAFETY: This is safe as long as the caller ensures that there are no concurrent accesses to the same Global instance.
        let value = unsafe { &mut *self.0.get() };
        f(value)
    }

    pub fn set(&self, value: T) {
        let slot = unsafe { &mut *self.0.get() };
        *slot = value;
    }
}

impl<T> Global<Option<T>> {
    #[inline(always)]
    pub fn modify_option(&self, f: impl FnOnce(&mut T)) {
        // println!("Thread id: {:?} - Modifying global option value", std::thread::current().id());
        // SAFETY: This is safe as long as the caller ensures that there are no concurrent accesses to the same Global instance.
        let value = unsafe { &mut *self.0.get() };
        if let Some(inner) = value {
            f(inner);
        }
    }
}


impl<T> Deref for Global<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: This is safe as long as the caller ensures that there are no concurrent accesses to the same Global instance.
        unsafe { &*self.0.get() }
    }
}

unsafe impl<T> Sync for Global<T> {}