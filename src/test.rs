#[derive(Debug)]
struct A {
    a: i32,
    b: i32,
}

#[derive(Debug)]
struct B {
    a: u32,
    b: i32,
}
#[derive(Debug)]
pub enum E{
    A(A),
    B(B),
}

#[derive(Debug)]
struct C {
    arr: [u8; 1024],
}

fn main() {
    let mut x = E::B(B{a:10,b:50});
    
    let c = C {
        arr: *cast(&mut x)
    };
    
    println!("{:?}", c);
    
}

fn cast<F, T>(from: &mut F) -> &mut T {
    unsafe { &mut *(from as *mut F as *mut T) }
}