mod shared;

use shared::Cvars;

impl Cvars {
    fn extra_method() {}
}

fn main() {
    let cvars = Cvars::default();
    cvars.extra_method();
}
