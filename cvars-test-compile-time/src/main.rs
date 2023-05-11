#[cfg(feature = "cvars100")]
mod cvars100;
#[cfg(feature = "cvars1000")]
mod cvars1000;
#[cfg(feature = "cvars10000")]
mod cvars10000;

#[cfg(feature = "cvars100")]
use cvars100::Cvars;
#[cfg(feature = "cvars1000")]
use cvars1000::Cvars;
#[cfg(feature = "cvars10000")]
use cvars10000::Cvars;

#[cfg(feature = "derive")]
use cvars::SetGet;
#[cfg(feature = "derive-dummy")]
use cvars::SetGetDummy as SetGet;

#[cfg(feature = "nomacro")]
#[derive(Default)]
struct Cvars;
#[cfg(feature = "nomacro")]
impl Cvars {
    fn get_string(&self, _cvar_name: &str) -> Result<String, String> {
        Ok("nomacro".to_string())
    }
    fn set_str(&mut self, _cvar_name: &str, _str_value: &str) -> Result<(), String> {
        Ok(())
    }
}

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
