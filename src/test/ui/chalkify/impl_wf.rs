// compile-flags: -Z trait-solver=chalk

trait Foo: Sized { }

trait Bar {
    type Item: Foo;
}

impl Foo for i32 { }

impl Foo for str { }
//~^ ERROR the size for values of type `str` cannot be known at compilation time


// Implicit `T: Sized` bound.
impl<T> Foo for Option<T> { }

trait Baz<U: ?Sized> where U: Foo { }

impl Baz<i32> for i32 { }

impl Baz<f32> for f32 { }
//~^ ERROR the trait bound `f32: Foo` is not satisfied

fn main() {
}
