#![no_main]

use libfuzzer_sys::fuzz_target;
use owned_ref_cell::OwnedRefCell;
use std::vec::Vec;

fuzz_target!(|data: &[u8]| {
    let cell = OwnedRefCell::new(0);
    let mut immutable_borrows = Vec::new();
    let mut mutable_borrows = Vec::new();

    for &byte in data.iter() {
        match byte % 3 {
            0 => {
                if let Some(borrow) = cell.try_borrow() {
                    immutable_borrows.push(borrow); // hold the immutable borrow
                }
            }
            1 => {
                if let Some(mut borrow_mut) = cell.try_borrow_mut() {
                    *borrow_mut += 1; // mutate the content
                    mutable_borrows.push(borrow_mut); // hold the mutable borrow
                }
            }
            _ => {
                // Randomly drop borrows from both vectors
                if !immutable_borrows.is_empty() {
                    immutable_borrows.pop();
                }
                if !mutable_borrows.is_empty() {
                    mutable_borrows.pop();
                }
            }
        }
    }
});
