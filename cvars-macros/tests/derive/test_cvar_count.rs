mod shared;

use shared::Cvars;

fn main() {
    let cvars = Cvars::default();

    assert_eq!(cvars.cvar_count(), 6);
    assert_eq!(Cvars::CVAR_COUNT, 6);
}
