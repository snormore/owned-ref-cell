#![feature(test)]

extern crate test;
use owned_ref_cell::OwnedRefCell;
use test::Bencher;

#[bench]
fn bench_borrow_mut(b: &mut Bencher) {
    let cell = OwnedRefCell::new(42);
    b.iter(|| {
        // Perform the mutable borrow inside the testing loop
        for _ in 0..1000 {
            test::black_box(cell.try_borrow_mut());
        }
    });
}

#[bench]
fn bench_borrow(b: &mut Bencher) {
    let cell = OwnedRefCell::new(42);
    b.iter(|| {
        // Perform the immutable borrow inside the testing loop
        for _ in 0..1000 {
            test::black_box(cell.try_borrow());
        }
    });
}

#[bench]
fn bench_borrow_mut_borrow(b: &mut Bencher) {
    let cell = OwnedRefCell::new(OwnedRefCell::new(42));
    b.iter(|| {
        let outer_borrow = cell.borrow_mut();
        for _ in 0..1000 {
            // Perform an inner borrow operation within an outer borrow
            let _inner_borrow = outer_borrow.borrow();
        }
    });
}

#[bench]
fn bench_borrow_mut_borrow_mut(b: &mut Bencher) {
    let cell = OwnedRefCell::new(OwnedRefCell::new(42));
    b.iter(|| {
        let outer_borrow = cell.borrow_mut();
        for _ in 0..1000 {
            // Perform an inner borrow operation within an outer borrow
            let _inner_borrow = outer_borrow.try_borrow_mut();
        }
    });
}

#[bench]
fn bench_borrow_borrow_mut(b: &mut Bencher) {
    let cell = OwnedRefCell::new(OwnedRefCell::new(42));
    b.iter(|| {
        let outer_borrow = cell.borrow();
        for _ in 0..1000 {
            // Perform an inner borrow operation within an outer borrow
            let _inner_borrow = outer_borrow.try_borrow_mut();
        }
    });
}
