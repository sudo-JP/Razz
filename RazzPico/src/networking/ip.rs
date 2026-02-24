pub struct IPv4Addr {
    pub oct1: u8, 
    pub oct2: u8, 
    pub oct3: u8,
    pub oct4: u8
}

impl IPv4Addr {
    pub fn get_pico_ip() -> Self {
        let ip0: u8 = env!("PICO_ADDR_0").parse().unwrap();
        let ip1: u8 = env!("PICO_ADDR_1").parse().unwrap();
        let ip2: u8 = env!("PICO_ADDR_2").parse().unwrap();
        let ip3: u8 = env!("PICO_ADDR_3").parse().unwrap();
        Self {
            oct1: ip0,
            oct2: ip1,
            oct3: ip2,
            oct4: ip3,
        }
    }

    pub fn to_embassy_ip(&self) -> embassy_net::Ipv4Address {
        embassy_net::Ipv4Address::new(self.oct1, self.oct2, self.oct3, self.oct4)
    }

    pub fn get_gateway() -> Self {
        let ip0: u8 = env!("GATEWAY_0").parse().unwrap();
        let ip1: u8 = env!("GATEWAY_1").parse().unwrap();
        let ip2: u8 = env!("GATEWAY_2").parse().unwrap();
        let ip3: u8 = env!("GATEWAY_3").parse().unwrap();
        Self {
            oct1: ip0,
            oct2: ip1,
            oct3: ip2,
            oct4: ip3,
        }
    }

    pub fn to_embassy_gateway(&self) -> embassy_net::Ipv4Address {
        embassy_net::Ipv4Address::new(
            self.oct1,
            self.oct2,
            self.oct3,
            self.oct4
        )
    }

    pub fn get_broker() -> Self {
        let ip0: u8 = env!("BROKER_0").parse().unwrap();
        let ip1: u8 = env!("BROKER_1").parse().unwrap();
        let ip2: u8 = env!("BROKER_2").parse().unwrap();
        let ip3: u8 = env!("BROKER_3").parse().unwrap();
        Self {
            oct1: ip0,
            oct2: ip1,
            oct3: ip2,
            oct4: ip3,
        }
    }

    pub fn get_mask() -> u8 { 24 }
}

