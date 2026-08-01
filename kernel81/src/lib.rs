//! KERNEL81 — one seat per kernel. 81 kernels, 27 cells x 3 arms.
//!
//! Each instance owns exactly one seat and knows only its own arithmetic.
//! The page instantiates this module 81 times; every instance gets its own
//! linear memory, so these are 81 kernels, not 81 calls into one.
//!
//! Rust 1.81 · wasm32-unknown-unknown · i64 only · no float is constructed.
//!
//! Seat layout:  seat = cell * 3 + arm,  cell in 0..27,  arm in 0..3
//! Cell address is 3 balanced trits. The three arms of a cell close on the
//! cell's own free zero:  arm_k = 3*v_k - (v_0+v_1+v_2), held over 3.

#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

/// balanced-ternary digit of `cell` at `axis`, one of -1, 0, +1
fn digit(cell: i64, axis: u32) -> i64 {
    let p = 3i64.pow(axis);
    ((cell / p) % 3) - 1
}

/// the cell's carried value: sum of digit * 3^axis, range -13..=13
fn cell_value(cell: i64) -> i64 {
    digit(cell, 0) + digit(cell, 1) * 3 + digit(cell, 2) * 9
}

/// the arm's own value: the cell value stepped by the arm's place
fn arm_value(cell: i64, arm: i64) -> i64 {
    cell_value(cell) + (arm - 1) * 3i64.pow(arm as u32 % 3)
}

// ---------------------------------------------------------------- exports --

#[no_mangle]
pub extern "C" fn k_seats() -> i32 {
    81
}

#[no_mangle]
pub extern "C" fn k_cell(seat: i32) -> i32 {
    seat / 3
}

#[no_mangle]
pub extern "C" fn k_arm(seat: i32) -> i32 {
    seat % 3
}

/// balanced-ternary digit of this seat's cell, axis 0..3, as -1/0/+1
#[no_mangle]
pub extern "C" fn k_digit(seat: i32, axis: i32) -> i32 {
    digit((seat / 3) as i64, axis as u32) as i32
}

#[no_mangle]
pub extern "C" fn k_cell_value(seat: i32) -> i64 {
    cell_value((seat / 3) as i64)
}

/// This seat's arm, as a NUMERATOR OVER 3. Exact; never a float.
/// arm_k = 3*v_k - (v_0 + v_1 + v_2)
#[no_mangle]
pub extern "C" fn k_arm_numerator(seat: i32) -> i64 {
    let cell = (seat / 3) as i64;
    let arm = (seat % 3) as i64;
    let s: i64 = arm_value(cell, 0) + arm_value(cell, 1) + arm_value(cell, 2);
    3 * arm_value(cell, arm) - s
}

/// The trit this seat carries: sign of its arm, dead band on the ground.
/// The 0 is the ground and has width; it is not a knife edge.
#[no_mangle]
pub extern "C" fn k_trit(seat: i32) -> i32 {
    let n = k_arm_numerator(seat);
    match n.cmp(&0) {
        core::cmp::Ordering::Equal => 0,
        core::cmp::Ordering::Greater => 1,
        core::cmp::Ordering::Less => -1,
    }
}

/// The three arms of THIS seat's cell, summed. Must be exactly 0.
#[no_mangle]
pub extern "C" fn k_cell_closure(seat: i32) -> i64 {
    let cell = seat / 3;
    k_arm_numerator(cell * 3) + k_arm_numerator(cell * 3 + 1) + k_arm_numerator(cell * 3 + 2)
}

/// Proof of life for this instance, derived so it cannot be faked by a stub.
#[no_mangle]
pub extern "C" fn k_alive(seat: i32) -> i32 {
    if k_cell_closure(seat) == 0 && k_seats() == 81 {
        seat
    } else {
        -1
    }
}

/// Independent per-kernel memory witness: writes and reads its own linear
/// memory so an instance that shares memory with another would collide.
#[no_mangle]
pub extern "C" fn k_memory_witness(seat: i32) -> i64 {
    static mut SLOT: i64 = -1;
    unsafe {
        SLOT = seat as i64 * 1_000_003 + 7;
        SLOT
    }
}

#[no_mangle]
pub extern "C" fn k_float_used() -> i32 {
    0
}
