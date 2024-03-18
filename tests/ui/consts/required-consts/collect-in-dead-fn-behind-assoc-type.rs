#![feature(impl_trait_in_assoc_type)]
//@revisions: noopt opt
//@ build-fail
//@[opt] compile-flags: -O
//! This fails without optimizations, so it should also fail with optimizations.

struct Fail<T>(T);
impl<T> Fail<T> {
    const C: () = panic!(); //~ERROR evaluation of `Fail::<i32>::C` failed
}

fn not_called<T>() {
    if false {
        let _ = Fail::<T>::C;
    }
}

fn callit_not(f: impl Fn()) {
    if false {
        f();
    }
}

// Using `Fn` here is important; with `FnOnce` another shim gets involved which somehow makes this
// easier to collect properly.
trait Hideaway {
    type T: Fn();
    const C: Self::T;
}
impl Hideaway for () {
    type T = impl Fn();
    const C: Self::T = not_called::<i32>;
}

fn reveal<T: Hideaway>() {
    if false {
        callit_not(T::C);
    }
}

fn main() {
    if false {
        reveal::<()>()
    }
}
