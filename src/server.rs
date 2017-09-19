use std::io::{Read, Result};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;
use futures::Stream;
use byteorder::{BigEndian, ReadBytesExt};
use config::Config;

const ID_TYPE: usize = 0;
const ID_IP0: i32 = 1;
const ID_DM_LEN: usize = 1;
const ID_DM0: i32 = 2;

const TYPE_IPV4: i32 = 1;
const TYPE_DM: i32 = 3;
const TYPE_IPV6: i32 = 4;

const LEN_IPV4: i32 = 6;
const LEN_IPV6: i32 = 18;
const LEN_DM_BASE: i32 = 2;

const ADDR_MASK: u8 = 0xf;

pub fn run_server(config: &Config) {
  println!("config: {:?}", config);

  let mut core = Core::new().unwrap();
  let handle = core.handle();

  let addr = config.server_addr.parse().unwrap();
  let listener = TcpListener::bind(&addr, &handle).unwrap();

  let server = listener.incoming().for_each(|(mut sock, _)| {
    let addr = get_address_info(&mut sock);

    Ok(())
  });

  core.run(server).unwrap();
}

fn get_address_info(reader: &mut TcpStream) -> Result<SocketAddr> {
  let buffer: &mut [u8] = &mut [0; 269];

  reader.read_exact(buffer)?;

  let address_type = buffer[ID_TYPE];

  let req_start: i32;
  let req_end: i32;

  match address_type as i32 & ADDR_MASK as i32 {
    TYPE_IPV4 => {
      req_start = ID_IP0;
      req_end = ID_IP0 + LEN_IPV4;
    }
    TYPE_IPV6 => {
      req_start = ID_IP0;
      req_end = ID_IP0 + LEN_IPV6;
    }
    TYPE_DM => {
      reader.read_exact(&mut buffer[ID_TYPE + 1..ID_DM_LEN + 1])?;
      req_start = ID_DM0;
      req_end = ID_DM0 + buffer[ID_DM_LEN] as i32 + LEN_DM_BASE;
    }
    _ => unreachable!(),
  }

  reader.read_exact(&mut buffer[req_start as usize..req_end as usize])?;

  let ip_addr: IpAddr;

  match address_type as i32 & ADDR_MASK as i32 {
    TYPE_IPV4 => {
      ip_addr = IpAddr::V4(Ipv4Addr::new(
        buffer[ID_IP0 as usize],
        buffer[ID_IP0 as usize + 1],
        buffer[ID_IP0 as usize + 2],
        buffer[ID_IP0 as usize + 3],
      ));
    }
    TYPE_IPV6 => {
      let i = ID_IP0 as usize;
      ip_addr = IpAddr::V6(Ipv6Addr::new(
        ((buffer[i] as u16) << 8) | (buffer[i + 1] as u16),
        ((buffer[i + 2] as u16) << 8) | (buffer[i + 3] as u16),
        ((buffer[i + 4] as u16) << 8) | (buffer[i + 5] as u16),
        ((buffer[i + 6] as u16) << 8) | (buffer[i + 7] as u16),
        ((buffer[i + 8] as u16) << 8) | (buffer[i + 9] as u16),
        ((buffer[i + 10] as u16) << 8) | (buffer[i + 11] as u16),
        ((buffer[i + 12] as u16) << 8) | (buffer[i + 13] as u16),
        ((buffer[i + 14] as u16) << 8) | (buffer[i + 15] as u16),
      ))
    }
    TYPE_DM => unimplemented!(),
    _ => unreachable!(),
  }

  let mut ip_port = &buffer[req_end as usize - 2..req_end as usize];

  Ok(SocketAddr::new(
    ip_addr,
    ip_port.read_u16::<BigEndian>().unwrap(),
  ))
}
