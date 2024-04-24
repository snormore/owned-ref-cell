#![feature(test)]

extern crate test;
use owned_ref_cell::OwnedRefCell;
use test::Bencher;

#[bench]
fn bench_owned_ref_cell_borrow_mut(b: &mut Bencher) {
    let cell = OwnedRefCell::new(42);
    b.iter(|| {
        // Perform the mutable borrow inside the testing loop
        for _ in 0..1000 {
            test::black_box(cell.try_borrow_mut());
        }
    });
}

#[bench]
fn bench_owned_ref_cell_borrow(b: &mut Bencher) {
    let cell = OwnedRefCell::new(42);
    b.iter(|| {
        // Perform the immutable borrow inside the testing loop
        for _ in 0..1000 {
            test::black_box(cell.try_borrow());
        }
    });
}
