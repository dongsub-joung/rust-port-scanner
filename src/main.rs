use std::net::Ipv4Addr;

struct Segment{
    // @TODO
    shadow_hand_shaking: bool
    // data: L1, segment: L3
}

struct Port{
    date: Option<Segment>,
    well_known: WellKnown,
    port_number: u32,
    established: bool,
}

// 0 ~ 1023
struct WellKnown{
    ip: Ipv4Addr,
    segment: Packet,

    flag: bool,

    // @TODO Add well known ports
    ftp: u32,
    ftp_two: u32,
}
impl WellKnown{
    pub fn new(network_data_bus: &NetworkDevice, flag: bool) -> Self{
        // @TODO IO
        let ip= Ipv4Addr::new(196, 168, 0, 1);

        // @TODO add specific ports
        let ftp= 20;
        let ftp_two= 21;

        // Read
        // copy and save vector vs smart pointer(4byte)
        let v_data_bus= network_data_bus.from_data();
        let segment= Box::from(v_data_bus);

        if flag != true{
            panic!("not varified packet {:?}", segment)       
        }else {
            WellKnown { ip, flag, ftp, ftp_two }
        }

        WellKnown { ip, flag, ftp, ftp_two  }
    }
}



fn main() {
    println!("Hello, world!");
}