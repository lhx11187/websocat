//! Manually maintained registry of all known Websocat nodes

/// Get `ClassRegistrar` with all WebSocat's nodes registered
pub fn all_node_classes() -> websocat_api::ClassRegistrar {
    let mut reg = websocat_api::ClassRegistrar::default();
    reg.register::<websocat_basic::net::Tcp>();
    reg.register::<websocat_basic::net::TcpListen>();
    reg.register::<websocat_basic::io_std::Stdio>();
    reg.register::<websocat_syncnodes::net::TcpConnect>();
    reg.register::<websocat_syncnodes::net::TcpListen>();
    reg.register::<websocat_syncnodes::net::UdpConnect>();
    reg.register::<websocat_syncnodes::net::UdpListen>();
    reg.register::<websocat_http::HttpClient>();
    reg.register::<websocat_http::HttpUpgradeClient>();
    reg
}
    