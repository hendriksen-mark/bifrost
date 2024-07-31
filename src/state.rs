use mac_address::MacAddress;
use uuid::{uuid, Uuid};

use crate::hue::v1::{ApiConfig, ApiShortConfig, Capabilities};
use crate::hue::v2::{Bridge, ClipResourceType, Resource, ResourceLink, TimeZone};

#[derive(Clone, Debug)]
pub struct AppState {
    mac: MacAddress,
}

impl AppState {
    pub fn new(mac: MacAddress) -> Self {
        Self { mac }
    }

    pub fn bridge_id(&self) -> String {
        let mac = self.mac.bytes();
        format!(
            "{:02x}{:02x}{:02x}FFFE{:02x}{:02x}{:02x}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5],
        )
    }

    pub fn api_short_config(&self) -> ApiShortConfig {
        ApiShortConfig {
            bridgeid: self.bridge_id(),
            mac: self.mac,
            ..Default::default()
        }
    }

    pub fn api_config(&self) -> ApiConfig {
        let config = ApiConfig::default();
        ApiConfig {
            short_config: self.api_short_config(),
            ..config
        }
    }

    pub fn get_bridge(&self) -> Resource {
        let bridge_id = self.bridge_id();
        let bridge = Bridge {
            id: Uuid::new_v5(
                &Uuid::NAMESPACE_URL,
                format!("{bridge_id}device").as_bytes(),
            ),
            bridge_id,
            owner: ResourceLink {
                rid: uuid!("00000000-0000-0000-0000-000000000000"),
                rtype: ClipResourceType::Device,
            },
            time_zone: TimeZone {
                time_zone: "Europe/London".to_string(),
            },
        };
        Resource::Bridge(bridge)
    }

    pub fn capabilities(&self) -> Capabilities {
        Capabilities::new()
    }
}
