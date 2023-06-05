// Change cvars according to user input from stdin, then print the new values.
//
// In a real game you would use the engine's console instead of stdin
// or at least you'd check for new lines every frame without blocking.

use std::io::BufRead;

use cvars::cvars;

cvars! {
    g_respawn_delay: f64 = 3.0,
    g_respawn_health: i32 = 100,
}

fn main() {
    let mut cvars = Cvars::default();

    println!("Type cvar name and value to update settings (e.g. g_respawn_delay 5):");

    for line in std::io::stdin().lock().lines() {
        // Read line from stdin, split it into the cvar's name and new value.
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let cvar_name = parts.next().unwrap();
        let cvar_value = parts.next().unwrap();

        // Update the cvar and print the new value - this is a stringly typed API.
        cvars.set_str(cvar_name, cvar_value).unwrap();
        println!(
            "Cvar updated: {} = {}",
            cvar_name,
            cvars.get_string(cvar_name).unwrap()
        );

        // In gamecode, you'll use cvars as any other struct - with strong and static typing.
        println!(
            "Players will respawn after {} s with {} health",
            cvars.g_respawn_delay, cvars.g_respawn_health
        );
    }
}
