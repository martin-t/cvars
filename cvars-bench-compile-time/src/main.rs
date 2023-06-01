// Notes:
// - Replacing derive(Debug) with a manual impl
//   reduces the compile time by on nomacro/dummy 10k cvars by a bit over 10%
//   but makes rustc crash when using the fast compiles configuration.
// - Dummy 10k cvars: derive(Default) 703.1 ms, no Default 552.6 ms, manual new() impl 657.4 ms

#[cfg(feature = "nomacro")]
mod bench {
    // When the structs derive only default, 10k cvars recompiles in 2s (after changing the struct).
    // When they also derive Clone, they take 8s.
    // My theory is that without Clone all unused fields get eliminated (they're assiged but never read)
    // while Clone forces them all to go through codegen which is expensive.

    #[cfg(feature = "cvars-100")]
    include!("nomacro-100.in");
    #[cfg(feature = "cvars-1000")]
    include!("nomacro-1000.in");
    #[cfg(feature = "cvars-10000")]
    include!("nomacro-10000.in");

    // Originally i thought these functions should use Cvars in some way
    // so that code wouldn't get optimized out
    // but it appears there's no difference.
    #[cfg_attr(not(any(feature = "typed", feature = "string")), allow(unused))]
    impl Cvars {
        pub fn get(&self, _cvar_name: &str) -> Result<i32, String> {
            unimplemented!();
        }
        pub fn get_string(&self, _cvar_name: &str) -> Result<String, String> {
            unimplemented!();
        }
        pub fn set(&mut self, _cvar_name: &str, _value: i32) -> Result<(), String> {
            unimplemented!();
        }
        pub fn set_str(&mut self, _cvar_name: &str, _str_value: &str) -> Result<(), String> {
            unimplemented!();
        }
    }
}

#[cfg(any(feature = "derive-dummy", feature = "derive"))]
mod bench {
    #[cfg(feature = "derive")]
    use cvars::SetGet;
    #[cfg(feature = "derive-dummy")]
    use cvars::SetGetDummy as SetGet;

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

#[cfg_attr(not(any(feature = "typed", feature = "string")), allow(unused))]
fn main() {
    // Do something with cvars that depends on external input so this can't all be optimized away.
    let mut args = std::env::args();
    let path = args.next().unwrap();
    println!("path: {}", path);
    let number = path.len();
    let set = args.next().unwrap();
    let get = args.next().unwrap();

    let mut cvars = Cvars::default();

    // Use both the string and typed API, otherwise they get compiled out
    // even before generating LLVM IR (which is currently the slowest part).

    #[cfg(feature = "typed")]
    {
        cvars.set(&set, number as i32).unwrap();
        let val: i32 = cvars.get(&get).unwrap();
        println!("typed set {set} -> {number}, get {get} -> {val}");
    }

    #[cfg(feature = "string")]
    {
        cvars.set_str(&set, &number.to_string()).unwrap();
        let val = cvars.get_string(&get).unwrap();
        println!("string set {set} -> {number}, get {get} -> {val}");
    }
}
