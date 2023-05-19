// TODO compare all manual / code generation

// Notes:
// - Replacing derive(Debug) with a manual impl
//   reduces the compile time by on nomacro/dummy 10k cvars by a bit over 10%
//   but makes rustc crash when using the fast compiles configuration.
// - Dummy 10k cvars: derive(Default) 703.1 ms, no Default 552.6 ms, manual new() impl 657.4 ms

#[cfg(feature = "nomacro")]
mod bench {
    // Hack so we can reuse the same code as for the derive benchmarks.
    use std::clone::Clone as SetGet;

    #[cfg(feature = "cvars-100")]
    include!("derive-100.in");
    #[cfg(feature = "cvars-1000")]
    include!("derive-1000.in");
    #[cfg(feature = "cvars-10000")]
    include!("derive-10000.in");

    impl Cvars {
        pub fn get_string(&self, _cvar_name: &str) -> Result<String, String> {
            Ok("nomacro".to_string())
        }
        pub fn set_str(&mut self, _cvar_name: &str, _str_value: &str) -> Result<(), String> {
            Ok(())
        }
    }
}

#[cfg(any(feature = "derive-dummy", feature = "derive"))]
mod bench {
    #[cfg(feature = "derive-dummy")]
    use cvars::SetGetDummy as SetGet;
    #[cfg(feature = "derive")]
    use cvars::SetGet;

    #[cfg(feature = "cvars-100")]
    include!("derive-100.in");
    #[cfg(feature = "cvars-1000")]
    include!("derive-1000.in");
    #[cfg(feature = "cvars-10000")]
    include!("derive-10000.in");
}

#[cfg(feature = "fnlike")]
mod bench {
    use cvars::cvars;

    #[cfg(feature = "cvars-100")]
    include!("fnlike-100.in");
    #[cfg(feature = "cvars-1000")]
    include!("fnlike-1000.in");
    #[cfg(feature = "cvars-10000")]
    include!("fnlike-10000.in");
}

use bench::*;

fn main() {
    // Do something with cvars that depends on external input so this can't all be optimized away.
    let mut args = std::env::args();
    let path = args.next().unwrap();
    println!("path: {}", path);
    let number = path.len();
    let set = args.next().unwrap();
    let get = args.next().unwrap();
    let mut cvars = Cvars::default();
    cvars.set_str(&set, &number.to_string()).unwrap();
    let val = cvars.get_string(&get).unwrap();
    println!("set {set} -> {number}, get {get} -> {val}");
}
