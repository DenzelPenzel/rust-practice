struct User {
    idx: u64,
}

impl User {
    fn new(idx: u64) -> Self {
        println!("Creating User: {:?}", idx);
        Self { idx }
    }
}

impl Drop for User {
    fn drop(&mut self) {
        println!("Dropping User: {:?}", self.idx);
    }
}

fn move_me(x: User) {
    // do nothing
}

struct HasDrop {
    x: User,
}

fn main() {
    alloc_mem_with_libc();
    alloc_mem_with_rust();

    let user = User::new(1);
    println!("User: {:?}", user.idx);

    {
        let user2 = User::new(2);
        println!("User2: {:?}", user2.idx);
    }

    move_me(user);

    let has_drop = HasDrop { x: User::new(3) };

    println!("Done")
}

fn alloc_mem_with_libc() {
    unsafe {
        let my_num: *mut i32 = libc::malloc(std::mem::size_of::<i32>() as libc::size_t) as *mut i32;
        if my_num.is_null() {
            panic!("Failed to allocate memory");
        }

        *my_num = 42;
        assert_eq!(42, *my_num);

        libc::free(my_num as *mut libc::c_void);
    }
}

fn alloc_mem_with_rust() {
    use std::alloc::{Layout, alloc, dealloc};

    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc(layout);

        *ptr = 42;
        assert_eq!(42, *ptr);

        // free the memory
        dealloc(ptr, layout);
    }
}
