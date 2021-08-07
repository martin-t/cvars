// Make sure users can add their own impl bloks to the generated struct.

mod shared;

use shared::Cvars;

impl Cvars {
    fn extra_method(&self) {}
}

fn main() {
    let cvars = Cvars::default();
    cvars.extra_method();
}
