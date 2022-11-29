use std::{
    cell::UnsafeCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
};

pub struct UnsafeRef<T: ?Sized> {
    value: NonNull<T>,
}
impl<T: ?Sized> Deref for UnsafeRef<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}
pub struct UnsafeRefMut<T: ?Sized> {
    value: NonNull<T>,
}

impl<T: ?Sized> Deref for UnsafeRefMut<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for UnsafeRefMut<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.value.as_mut() }
    }
}

impl<T: ?Sized + PartialEq> PartialEq<T> for UnsafeRef<T> {
    fn eq(&self, other: &T) -> bool {
        self.deref() == other
    }
}

pub struct UnsafeRefCell<T> {
    cell: UnsafeCell<T>,
}
impl<T: Debug> Debug for UnsafeRefCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}

impl<T> UnsafeRefCell<T> {
    pub fn new(value: T) -> UnsafeRefCell<T> {
        Self {
            cell: UnsafeCell::new(value),
        }
    }
    pub fn rc(value: T) -> Rc<UnsafeRefCell<T>> {
        Rc::new(Self::new(value))
    }
    pub fn borrow(&self) -> UnsafeRef<T> {
        let value = unsafe { NonNull::new_unchecked(self.cell.get()) };
        UnsafeRef { value }
    }
    pub fn borrow_mut(&self) -> UnsafeRefMut<T> {
        let value = unsafe { NonNull::new_unchecked(self.cell.get()) };
        UnsafeRefMut { value }
    }
}
