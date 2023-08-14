#![allow(unused)]

use std::sync::{Arc, RwLock};

use std::fmt;

/// WiFi phy
pub struct WiFi {
    mac: [u8; 6],
}

impl WiFi {
    pub fn new() -> WiFi {
        WiFi {
            mac: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
        }
    }
}

impl fmt::Display for WiFi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.mac[0], self.mac[1], self.mac[2], self.mac[3], self.mac[4], self.mac[5],
        )
    }
}

/// mesh node instance
pub struct Node {
    phy: WiFi,
}

impl Node {
    pub fn new() -> Self {
        Node { phy: WiFi::new() }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "phy: {}\n", self.phy)
    }
}

/// mesh nodes registry
// pub type Registry = Arc<RwLock<Vec<Node>>>;
pub type Registry = Vec<Node>;

// impl fmt::Display for Registry {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Registry:\n")
//     }
// }
