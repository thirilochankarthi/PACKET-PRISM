//! Module defining the `AddressPortPair` struct, which represents a network address:port pair.

use crate::Protocol;
use std::net::IpAddr;

/// Struct representing a network address:port pair.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct AddressPortPair {
    /// Network layer IPv4 or IPv6 source address.
    pub address1: IpAddr,
    /// Transport layer source port number (in the range 0..=65535).
    pub port1: Option<u16>,
    /// Network layer IPv4 or IPv6 destination address.
    pub address2: IpAddr,
    /// Transport layer destination port number (in the range 0..=65535).
    pub port2: Option<u16>,
    ///  Transport layer protocol carried through the associate address:port pair (TCP or UPD).
    pub protocol: Protocol,
}

impl AddressPortPair {
    /// Returns a new `AddressPort` element.
    ///
    /// # Arguments
    ///
    /// * `address` - A string representing the network layer IPv4 or IPv6 address.
    ///
    /// * `port` - An integer representing the transport layer port number (in the range 0..=65535).
    pub fn new(
        address1: IpAddr,
        port1: Option<u16>,
        address2: IpAddr,
        port2: Option<u16>,
        protocol: Protocol,
    ) -> Self {
        AddressPortPair {
            address1,
            port1,
            address2,
            port2,
            protocol,
        }
    }
}
