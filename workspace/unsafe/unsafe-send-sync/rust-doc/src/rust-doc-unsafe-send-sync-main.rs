#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-unsafe-send-sync_bin --bin rust-doc-unsafe-send-sync-main```
///
/// ```cargo doc  --package rust-doc-unsafe-send-sync_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-unsafe-send-sync_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
struct NonSendSyncStruct;

impl NonSendSyncStruct {
    fn new() -> Self {
        println!("Creating NonSendSyncStruct");
        Self
    }
}

trait MyTrait {
    fn do_something(&self);
}
#[derive(Copy, Clone)]
struct MyStruct{
     value: i32,
}



impl MyTrait for MyStruct {
    fn do_something(&self) {
        println!("Doing something!");
    }
}


impl MyStruct {
    fn get_non_send_sync_struct(&self) -> NonSendSyncStruct {
        println!("Getting NonSendSyncStruct");
        NonSendSyncStruct::new()
    }
}
fn use_my_struct_in_unsafe_block(mut my_struct: MyStruct) {
    // Safe to move `my_struct` into the unsafe block because we're not sharing it or sending it across threads.
    unsafe {
        my_struct.do_something();
        // After using `my_struct` inside the unsafe block, we can still use it outside if needed.
        my_struct.value = 42; // This would normally cause a borrow error outside the unsafe block.
    }

    // Now that we're out of the unsafe block, Rust's safety checks apply again.
    my_struct.do_something(); // This works fine now.
}

fn main() {
    let mut my_struct = MyStruct { value: 40 };
    let _ = unsafe {
        // Move `my_struct` into the unsafe block.
        let my_struct_in_unsafe = std::mem::replace(&mut my_struct, MyStruct { value: 50 });

        // Now, `my_struct_in_unsafe` owns `my_struct`.
        // We can call methods on `my_struct_in_unsafe` that return `NonSendSyncStruct`.
        let non_send_sync_struct = my_struct_in_unsafe.get_non_send_sync_struct();

        // Here, we would typically do something with `non_send_sync_struct`.
        // However, since `NonSendSyncStruct` does not implement `Send` or `Sync`,
        // we cannot return it from the unsafe block or transfer its ownership outside.
        // Instead, we just drop it at the end of the block.
        drop(non_send_sync_struct);
        
        // The original `my_struct` variable is now invalid and should not be used.
        // It has been replaced with a new `MyStruct` instance.
        my_struct_in_unsafe
    };
     use_my_struct_in_unsafe_block(my_struct);

    // The struct is still valid after being used in the unsafe block.
    my_struct.do_something(); // This prints "Doing something with value: 42"
    
    // At this point, `my_struct` is no longer valid and attempting to use it would result in undefined behavior.
    // The `unsafe` block ensures that any operations involving `my_struct` are contained and safe,
    // even though `my_struct` itself is moved out of scope.
}