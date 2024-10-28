struct ScopeCall<F: FnMut()> {
    c: F,
}
impl<F: FnMut()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        (self.c)();
    }
}

macro_rules! defer {
    ($e:expr) => {
        let _scope_call = ScopeCall {
            c: || -> () {
                $e;
            },
        };
    };
}

fn main() {
    let x = String::new();
    defer!(println!("defer 1"));
    defer!({
        println!("defer 2");
        println!("inside defer {}", x)
    });
    println!("normal execution {}", x);
}
