use std::alloc::{Layout, alloc, dealloc};

struct SmartPointer<T> {
    ptr: *mut u8,
    data: *mut T,
    layout: Layout,
}

impl<T> SmartPointer<T> {
    fn new() -> SmartPointer<T> {
        println!("Allocating data on the heap");

        unsafe {
            let layout = Layout::new::<T>();
            let ptr = alloc(layout);

            SmartPointer {
                ptr,
                data: ptr as *mut T,
                layout,
            }
        }
    }

    fn set(&mut self, value: T) {
        unsafe {
            *self.data = value;
        }
    }

    fn get(&self) -> &T {
        unsafe {
            // convert the pointer to a reference
            self.data.as_ref().unwrap()
        }
    }
}

impl<T> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping SmartPointer");
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}

fn main() {
    let mut x = SmartPointer::<u32>::new();
    x.set(42);
    println!("x: {}", x.get());

    let num = Box::new(42);
    println!("Num: {}", num);
}
