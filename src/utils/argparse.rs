use std::env::args;

pub fn read_arg(at: usize, description: &str) -> String {
    args().nth(at).unwrap_or_else(|| panic!("{} not provided", description))
}

pub fn parse_arg<T>(at: usize, description: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let arg = read_arg(at, description);
    arg.parse::<T>().unwrap_or_else(|_| panic!("cannot parse {} to desired type: {}", arg, description))
}
