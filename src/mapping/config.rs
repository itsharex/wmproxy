use std::{net::SocketAddr, vec};

use serde::{Deserialize, Serialize};

fn default_domain() -> String {
    "".to_string()
}


fn default_header() -> Vec<Vec<String>> {
    vec![]
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MappingConfig {
    pub name: String,
    pub mode: String,
    pub local_addr: Option<SocketAddr>,
    #[serde(default = "default_domain")]
    pub domain: String,
    #[serde(default = "default_header")]
    pub headers: Vec<Vec<String>>,
}

impl MappingConfig {
    pub fn new(name: String, mode: String, domain: String, headers: Vec<Vec<String>>) -> Self {
        MappingConfig {
            name,
            mode,
            local_addr: None,
            domain,
            headers,
        }
    }

    pub fn is_http(&self) -> bool {
        self.mode.eq_ignore_ascii_case("http")
    }

    pub fn is_https(&self) -> bool {
        self.mode.eq_ignore_ascii_case("https")
    }

    pub fn is_tcp(&self) -> bool {
        self.mode.eq_ignore_ascii_case("tcp")
    }
}