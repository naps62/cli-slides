struct Foo(i32);

impl std::ops::Add for Foo {
    type Output = Foo;

    fn add(self, other: Foo) -> Foo {
        Foo(self.0 - other.0)
    }
}

#[allow(unused)]
fn constants() {
    let x = 3.14156;
}

#[allow(unused)]
const DANGER: &str = "there's a zero-width space or soft hyphen some­where in this text.";
