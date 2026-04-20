use std::mem::MaybeUninit;

#[cfg(any(
    all(feature = "ub-use-after-free", feature = "ub-uninitialized-read"),
    all(feature = "ub-use-after-free", feature = "ub-overlapping-copy"),
    all(feature = "ub-uninitialized-read", feature = "ub-overlapping-copy"),
))]
compile_error!("Enable at most one `ub-*` feature at a time.");

fn main() {
    println!("Miri highlights:");
    println!(
        "  use-after-free demo result: {}",
        use_after_free_demo()
    );
    println!(
        "  initialization demo result: {}",
        initialization_demo()
    );
    println!(
        "  copy demo result: {:?}",
        overlapping_copy_demo()
    );
}

#[cfg(not(feature = "ub-use-after-free"))]
fn use_after_free_demo() -> i32 {
    // Safe default: keep the allocation alive.
    let boxed = Box::new(41);
    *boxed
}

#[cfg(feature = "ub-use-after-free")]
fn use_after_free_demo() -> i32 {
    // Enable `ub-use-after-free` to let Miri catch a dangling pointer read.
    let ptr = {
        let boxed = Box::new(41);
        Box::into_raw(boxed)
    };
    unsafe {
        drop(Box::from_raw(ptr));
        *ptr
    }
}

#[cfg(not(feature = "ub-uninitialized-read"))]
fn initialization_demo() -> i32 {
    // Safe default: initialize the value before reading it.
    let value = MaybeUninit::new(42);
    unsafe { value.assume_init() }
}

#[cfg(feature = "ub-uninitialized-read")]
fn initialization_demo() -> i32 {
    // Enable `ub-uninitialized-read` to let Miri catch the invalid read.
    let value = MaybeUninit::<i32>::uninit();
    unsafe { value.assume_init() }
}

#[cfg(not(feature = "ub-overlapping-copy"))]
fn overlapping_copy_demo() -> [u8; 4] {
    // Safe default: use an API that permits overlapping ranges.
    let mut values = [1_u8, 2, 3, 4];
    values.copy_within(0..3, 1);
    values
}

#[cfg(feature = "ub-overlapping-copy")]
fn overlapping_copy_demo() -> [u8; 4] {
    // Enable `ub-overlapping-copy` to let Miri catch the violated intrinsic precondition.
    let mut values = [1_u8, 2, 3, 4];
    unsafe {
        let base = values.as_mut_ptr();
        std::ptr::copy_nonoverlapping(base, base.add(1), 3);
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_after_free_demo_returns_expected_value_in_safe_mode() {
        assert_eq!(use_after_free_demo(), 41);
    }

    #[test]
    fn initialization_demo_returns_expected_value_in_safe_mode() {
        assert_eq!(initialization_demo(), 42);
    }

    #[test]
    fn overlapping_copy_demo_moves_bytes_without_ub_in_safe_mode() {
        assert_eq!(overlapping_copy_demo(), [1, 1, 2, 3]);
    }
}
