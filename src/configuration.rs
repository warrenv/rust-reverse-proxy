use serde::{Deserialize, Serialize};
use std::env;
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Configuration {
    addr: String,
    port: String,
    cms_host: String,
    frontend_host: String,
}

impl Configuration {
    pub fn load() -> Result<Configuration, ()> {
        let config = Self {
            addr: env::var("BURP_ADDR").expect("BURP_ADDR env var"),
            port: env::var("BURP_PORT").expect("BURP_PORT env var"),
            cms_host: env::var("BURP_CMS_HOST").expect("BURP_CMS_HOST env var"),
            frontend_host: env::var("BURP_FRONTEND_HOST").expect("BURP_FRONTEND_HOST env var"),
        };

        Ok(config)
    }

    pub fn addr(&self) -> IpAddr {
        self.addr
            .clone()
            .parse()
            .expect("parse of BURP_ADDR failed")
    }

    pub fn port(&self) -> u16 {
        self.port
            .clone()
            .parse()
            .expect("parse of BURP_PORT failed")
    }

    pub fn cms_host(&self) -> String {
        self.cms_host.clone()
    }

    pub fn frontend_host(&self) -> String {
        self.frontend_host.clone()
    }
}
