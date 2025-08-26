use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use crate::impl_identity;

impl_identity!(IpAddr, Ipv4Addr, Ipv6Addr);
impl_identity!(SocketAddr, SocketAddrV4, SocketAddrV6);
