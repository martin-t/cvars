#[cfg(feature = "nomacro")]
mod bench {
    #[derive(Default)]
    pub struct Cvars;

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

    #[cfg(feature = "cvars100")]
    include!("derive-100.in");
    #[cfg(feature = "cvars1000")]
    include!("derive-1000.in");
    #[cfg(feature = "cvars10000")]
    include!("derive-10000.in");
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
