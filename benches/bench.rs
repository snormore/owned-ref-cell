#![feature(test)]

extern crate test;
use owned_ref_cell::OwnedRefCell;
use test::Bencher;

#[bench]
fn bench_borrow_mut(b: &mut Bencher) {
    let cell = OwnedRefCell::new(42);
    b.iter(|| {
        for _ in 0..1000 {
            test::black_box(cell.borrow_mut());
        }
    });
}

#[bench]
fn bench_borrow(b: &mut Bencher) {
    let cell = OwnedRefCell::new(42);
    b.iter(|| {
        for _ in 0..1000 {
            test::black_box(cell.borrow());
        }
    });
}

#[bench]
fn bench_borrow_after_borrow(b: &mut Bencher) {
    let cell = OwnedRefCell::new(OwnedRefCell::new(42));
    b.iter(|| {
        let _ref = cell.borrow();

        for _ in 0..1000 {
            test::black_box(cell.borrow());
        }
    });
}

#[bench]
fn bench_borrow_after_borrow_mut_fails(b: &mut Bencher) {
    let cell = OwnedRefCell::new(OwnedRefCell::new(42));
    b.iter(|| {
        let _ref = cell.borrow_mut();

        for _ in 0..1000 {
            assert!(
                cell.try_borrow().is_none(),
                "Expected failure, but succeeded"
            );
        }
    });
}

#[bench]
fn bench_borrow_mut_after_borrow_mut_fails(b: &mut Bencher) {
    let cell = OwnedRefCell::new(OwnedRefCell::new(42));
    b.iter(|| {
        let _ref = cell.borrow_mut();

        for _ in 0..1000 {
            assert!(
                cell.try_borrow_mut().is_none(),
                "Expected failure, but succeeded"
            );
        }
    });
}
