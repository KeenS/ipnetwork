//! The `ipnetwork` crate provides a set of APIs to work with IP CIDRs in
//! Rust. Implementation for IPv4 is more or less stable, IPv6 implementation
//! is still WIP.
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![crate_type = "lib"]

use std::net::IpAddr;

mod ipv4;
mod ipv6;
mod common;

pub use ipv4::{Ipv4Network, ipv4_mask_to_prefix};
pub use ipv6::{Ipv6Network, ipv6_mask_to_prefix};
pub use common::IpNetworkError;

/// Represents a generic network range. This type can have two variants:
/// the v4 and the v6 case.
#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
pub enum IpNetwork {
    V4(Ipv4Network),
    V6(Ipv6Network),
}

impl IpNetwork {
    /// Constructs a new `IpNetwork` from a given `IpAddr` and a prefix denoting the
    /// network size. If the prefix is larger than 32 (for IPv4) or 128 (for IPv6), this
    /// will raise an `IpNetworkError::InvalidPrefix` error. Support for IPv6 is not
    /// complete yet.
    pub fn new(ip: IpAddr, prefix: u8) -> Result<IpNetwork, IpNetworkError> {
        match ip {
            IpAddr::V4(a) => Ok(IpNetwork::V4(Ipv4Network::new(a, prefix)?)),
            IpAddr::V6(a) => Ok(IpNetwork::V6(Ipv6Network::new(a, prefix)?)),
        }
    }

    /// Returns the IP part of a given `IpNetwork`
    ///
    /// # Example
    /// ```
    /// use std::net::{Ipv4Addr, Ipv6Addr};
    /// use ipnetwork::IpNetwork;
    ///
    /// assert_eq!(IpNetwork::V4("10.9.0.32/16".parse().unwrap()).ip(), "10.9.0.32".parse().unwrap());
    /// assert_eq!(IpNetwork::V6("ff01::0/32".parse().unwrap()).ip(), "ff01::0".parse().unwrap());
    /// ```
    pub fn ip(&self) -> IpAddr {
        match *self {
            IpNetwork::V4(ref a) => IpAddr::V4(a.ip()),
            IpNetwork::V6(ref a) => IpAddr::V6(a.ip()),
        }
    }

    /// Returns the prefix of the given `IpNetwork`
    ///
    /// # Example
    /// ```
    /// use ipnetwork::IpNetwork;
    ///
    /// assert_eq!(IpNetwork::V4("10.9.0.32/16".parse().unwrap()).prefix(), 16u8);
    /// assert_eq!(IpNetwork::V6("ff01::0/32".parse().unwrap()).prefix(), 32u8);
    /// ```
    pub fn prefix(&self) -> u8 {
        match *self {
            IpNetwork::V4(ref a) => a.prefix(),
            IpNetwork::V6(ref a) => a.prefix(),
        }
    }

    /// Returns the mask of the given `IpNetwork`
    ///
    /// # Example
    /// ```
    /// use ipnetwork::IpNetwork;
    ///
    /// assert_eq!(IpNetwork::V4("10.9.0.32/16".parse().unwrap()).mask(), 16u8);
    /// assert_eq!(IpNetwork::V6("ff01::0/32".parse().unwrap()).mask(), 32u8);
    /// ```
    pub fn mask(&self) -> IpAddr {
        match *self {
            IpNetwork::V4(ref a) => IpAddr::V4(a.mask()),
            IpNetwork::V6(ref a) => IpAddr::V6(a.mask()),
        }
    }

    /// Returns true if the IP in this `IpNetwork` is a valid IPv4 address,
    /// false if it's a valid IPv6 address.
    ///
    /// # Example
    ///
    /// ```
    /// use ipnetwork::IpNetwork;
    ///
    /// let v4: IpNetwork = IpNetwork::V4("10.9.0.32/16".parse().unwrap());
    /// assert_eq!(v4.is_ipv4(), true);
    /// assert_eq!(v4.is_ipv6(), false);
    /// ```
    pub fn is_ipv4(&self) -> bool {
        match *self {
            IpNetwork::V4(_) => true,
            IpNetwork::V6(_) => false,
        }
    }

    /// Returns true if the IP in this `IpNetwork` is a valid IPv6 address,
    /// false if it's a valid IPv4 address.
    ///
    /// # Example
    ///
    /// ```
    /// use ipnetwork::IpNetwork;
    ///
    /// let v6: IpNetwork = IpNetwork::V6("ff01::0/32".parse().unwrap());
    /// assert_eq!(v6.is_ipv6(), true);
    /// assert_eq!(v6.is_ipv4(), false);
    /// ```
    pub fn is_ipv6(&self) -> bool {
        match *self {
            IpNetwork::V4(_) => false,
            IpNetwork::V6(_) => true,
        }
    }
}

impl From<Ipv4Network> for IpNetwork {
    fn from(v4: Ipv4Network) -> IpNetwork {
        IpNetwork::V4(v4)
    }
}

impl From<Ipv6Network> for IpNetwork {
    fn from(v6: Ipv6Network) -> IpNetwork {
        IpNetwork::V6(v6)
    }
}

/// Converts a `IpAddr` network mask into a prefix.
/// If the mask is invalid this will return an `IpNetworkError::InvalidPrefix`.
pub fn ip_mask_to_prefix(mask: IpAddr) -> Result<u8, IpNetworkError> {
    match mask {
        IpAddr::V4(mask) => ipv4_mask_to_prefix(mask),
        IpAddr::V6(mask) => ipv6_mask_to_prefix(mask),
    }
}
