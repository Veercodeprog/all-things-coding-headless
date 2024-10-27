use std::net::Ipv4Addr;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Database {
    // the ip for the database conncetion
    pub ip: Ipv4Addr,
    //port for the database conncetion
    pub port: u16,
}
impl Database {
    pub fn new(ip: &str, port: u16) -> anyhow::Result<Self> {
        let ip = ip.parse::<Ipv4Addr>()?;
        Ok(Database { ip, port })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
