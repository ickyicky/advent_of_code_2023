use std::env::args;

pub fn read_arg(at: usize, description: &str) -> String {
    args().nth(at).expect(format!("{} not provided", description).as_str())
}

pub fn parse_arg<T>(at: usize, description: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let arg = read_arg(at, description);
    arg.parse::<T>().expect(format!("cannot parse {} to desired type: {}", arg, description).as_str())
}
