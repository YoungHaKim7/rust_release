trait Foo {
    fn method<'a>(&'a self) -> impl Sized;

    // Desugars to:
    type Implicit1<'a>: Sized
    where
        Self: 'a;
    fn method_desugared<'a>(&'a self) -> Self::Implicit1<'a>;

    // Precise capturing with `use<...>` is unstable and not usable in impls
    // We'll just use a normal `impl Sized` here
    fn precise<'a>(&'a self) -> impl Sized;

    // Desugars to:
    type Implicit2: Sized;
    fn precise_desugared<'a>(&'a self) -> Self::Implicit2;
}

#[derive(Debug)]
struct MyType {
    value: String,
}

impl Foo for MyType {
    fn method<'a>(&'a self) -> impl Sized {
        &self.value
    }

    type Implicit1<'a>
        = &'a str
    where
        Self: 'a;
    fn method_desugared<'a>(&'a self) -> Self::Implicit1<'a> {
        &self.value
    }

    fn precise<'a>(&'a self) -> impl Sized {
        self.value.clone()
    }

    type Implicit2 = String;
    fn precise_desugared<'a>(&'a self) -> Self::Implicit2 {
        self.value.clone()
    }
}

fn main() {
    let my = MyType {
        value: "Hello from MyType".to_string(),
    };

    let s1 = my.method();
    // println!("method: {}", s1);

    let s2 = my.method_desugared();
    println!("method_desugared: {}", s2);

    let s3 = my.precise();
    // println!("precise: {}", s3);

    let s4 = my.precise_desugared();
    println!("precise_desugared: {}", s4);
}
