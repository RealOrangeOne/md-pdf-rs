use std::fmt::Debug;
use std::process::exit;
use std::io::{self, Write};


#[inline]
pub fn result_override<T, E: Debug>(r: Result<T, E>, msg: String) -> Result<T, String> {
    return match r {
        Ok(t) => Ok(t),
        Err(_) => Err(msg),
    };
}

#[inline]
pub fn result_prefix<T, E: Debug>(r: Result<T, E>, prefix: String) -> Result<T, String> {
    return match r {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("{}: {:?}", prefix, e)),
    };
}


pub fn ok_or_exit<T>(res: Result<T, String>) -> T {
    return match res {
        Ok(k) => k,
        Err(err) => {
            writeln!(io::stderr(), "Error: {:?}", err).unwrap();
            exit(1);
        }
    };
}
