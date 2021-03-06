pub use self::arch::{in_port_t};
pub use self::arch::{in_addr_t};
pub use self::arch::{INADDR_ANY};
pub use self::arch::{INADDR_BROADCAST};
pub use self::arch::{in_addr};
pub use self::arch::{IPPROTO_IP};
pub use self::arch::{IPPROTO_ICMP};
pub use self::arch::{IPPROTO_TCP};
pub use self::arch::{IPPROTO_UDP};
pub use self::arch::{IPPROTO_IPV6};
pub use self::arch::{IPPROTO_RAW};
pub use self::arch::{IPV6_JOIN_GROUP};
pub use self::arch::{IPV6_LEAVE_GROUP};
pub use self::arch::{IPV6_MULTICAST_HOPS};
pub use self::arch::{IPV6_MULTICAST_IF};
pub use self::arch::{IPV6_MULTICAST_LOOP};
pub use self::arch::{IPV6_UNICAST_HOPS};
pub use self::arch::{IPV6_V6ONLY};
pub use self::arch::{in6_addr};
pub use self::arch::{IN6_IS_ADDR_UNSPECIFIED};
pub use self::arch::{IN6_IS_ADDR_LOOPBACK};
pub use self::arch::{IN6_IS_ADDR_MULTICAST};
pub use self::arch::{IN6_IS_ADDR_LINKLOCAL};
pub use self::arch::{IN6_IS_ADDR_SITELOCAL};
pub use self::arch::{IN6_IS_ADDR_V4MAPPED};
pub use self::arch::{IN6_IS_ADDR_V4COMPAT};
pub use self::arch::{IN6_IS_ADDR_MC_NODELOCAL};
pub use self::arch::{IN6_IS_ADDR_MC_LINKLOCAL};
pub use self::arch::{IN6_IS_ADDR_MC_SITELOCAL};
pub use self::arch::{IN6_IS_ADDR_MC_ORGLOCAL};
pub use self::arch::{IN6_IS_ADDR_MC_GLOBAL};
pub use self::arch::{IN6ADDR_ANY_INIT};
pub use self::arch::{sockaddr_in};
pub use self::arch::{sockaddr_in6};
pub use self::arch::{ipv6_mreq};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

