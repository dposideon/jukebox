use mdns_sd::{ServiceDaemon, ServiceInfo};

pub fn register_mdns(port: u16) -> mdns_sd::Result<ServiceDaemon> {
    let mdns = ServiceDaemon::new()?;

    let service_type = "_http._tcp.local.";
    let instance_name = "music";

    let service = ServiceInfo::new(service_type, instance_name, "music.local.", "", port, None)?
        .enable_addr_auto();

    mdns.register(service)?;

    Ok(mdns)
}
