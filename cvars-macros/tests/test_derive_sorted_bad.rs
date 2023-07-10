use cvars::SetGet;

#[derive(SetGet)]
#[cvars(sorted)]
pub struct Cvars {
    a: i32,
    aa: i32,
    ab: i32,
    aaa: i32,

    b: i32,
    c: i32,
    d: i32,
}

fn main() {}
