use std::fmt::Debug;

pub fn result_override<T, E: Debug>(r: Result<T, E>, msg: String) -> Result<T, String> {
    return match r {
        Ok(t) => Ok(t),
        Err(_) => Err(msg)
    };
}

pub fn result_prefix<T: Debug, E: Debug>(r: Result<T, E>, prefix: String) -> Result<T, String> {
    return match r {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("{}: {:?}", prefix, e))
    };
}
