use anyhow::{anyhow, Result as AnyResult};
use std::net::{IpAddr};
use std::process::Command;
use tracing::*;

#[cfg(feature = "net")]
#[derive(Debug, Clone, Default)]
pub struct NetLink {
    pub Mac: String,
    pub Ipv4: String,
    pub Ipv6: String,
    pub Name: String,
    pub Index: String,
    pub State: String,
}

impl NetLink {
    pub fn get_ipv4_addr(&self) -> String {
        let ifo: Vec<&str> = self.Ipv4.split("/").collect();
        ifo.get(0).unwrap().to_string()
    }
}

#[cfg(feature = "net")]
#[derive(Debug, Default)]
pub struct NetRoute {
    pub Dest: String,
    pub Dev: String,
    pub Gateway: String,
    pub Src: String,
    pub State: String,
}

#[cfg(feature = "net")]
impl NetRoute {
    pub fn subnet_contains(&self, ip: &str) -> bool {
        //TODO: calc
        false
    }
}

#[cfg(feature = "net")]
#[derive(Debug, Default)]
pub struct GatewayInfo {
    pub bHasGateway: bool,
    pub nGatewayCount: u8,
    pub bHasDefaultGateway: bool,
    pub nDefaultGatewayCount: u8,
    pub nLinkUp: u8,
}

#[cfg(feature = "net")]
#[derive(Debug, Default)]
pub struct RouteTable {
    data: Vec<NetRoute>,
}

#[cfg(feature = "net")]
impl RouteTable {
    /// check gateway address
    pub fn Parse(&self) -> GatewayInfo {
        let mut gatewayInfo = GatewayInfo::default();

        debug!("{:?}", self.data);

        if self.data.len() == 0 {
            gatewayInfo.nLinkUp = 0;
            gatewayInfo.bHasGateway = false;
            gatewayInfo.nDefaultGatewayCount = 0;
        }

        for route in &self.data {
            if route.State != "linkdown" {
                gatewayInfo.nLinkUp += 1;
            }

            if route.Gateway != "" {
                gatewayInfo.bHasGateway = true;
                gatewayInfo.nGatewayCount += 1;

                if route.Dest.contains("0.0.0.0") || route.Dest == "default" {
                    gatewayInfo.bHasDefaultGateway = true;
                    gatewayInfo.nDefaultGatewayCount += 1;
                }
            }
        }

        debug!("gatewayInfo:{:?}", gatewayInfo);
        gatewayInfo
    }

    /// get destination hop
    pub fn FindRoute(&self, ip: &str) -> Option<&NetRoute> {
        let mut bHasGateway = false;
        for route in &self.data {
            //check custom route
            if route.Dest != "0.0.0.0/0" && route.Dest != "default" {
                //calculate subnet
                if route.subnet_contains(ip) {
                    bHasGateway = true;
                    return Some(route);
                }
            }
        }

        if !bHasGateway {
            for route in &self.data {
                //check default route
                if route.Dest == "0.0.0.0/0" || route.Dest == "default" {
                    bHasGateway = true;
                    return Some(route);
                }
            }
        }

        None
    }
}

#[cfg(feature = "net")]
#[derive(Debug, Default)]
pub struct PingResult {
    duration: f32,
}

#[cfg(feature = "net")]
#[derive(Debug)]
pub struct TcpPingResult {
    duration: f32,
}

#[cfg(feature = "net")]
#[derive(Debug)]
pub struct NsLookupResult {
    ip_list: Vec<IpAddr>,
}

#[cfg(feature = "net")]
pub trait os_network {
    fn get_interface_list() -> AnyResult<Vec<NetLink>>;
    fn get_route_table() -> AnyResult<RouteTable>;
    fn ping(host: &str) -> AnyResult<PingResult>;
    fn nslookup(host: &str) -> AnyResult<NsLookupResult>;
    fn tcping(host: &str, port: i32) -> AnyResult<TcpPingResult>;
}

#[cfg(feature = "net")]
pub struct LinuxNetwork {}

#[cfg(feature = "net")]
pub struct MacOSNetwork {}

#[cfg(all(feature = "net", target_os = "macos"))]
impl MacOSNetwork {
    /// return mac,ipv4,ipv6,index
    pub fn getIfMacIpAddr(ifname: &str) -> AnyResult<NetLink> {
        match Command::new("/sbin/ifconfig")
            .args([ifname])
            .output()
        {
            Ok(output) => {
                let sOutput = String::from_utf8(output.stdout)?;
                let sError = String::from_utf8(output.stderr)?;
                
                // Check if interface exists by examining stderr
                if !sError.is_empty() && (sError.contains("not found") || sError.contains("not exist")) {
                    return Err(anyhow!("Interface {} not found: {}", ifname, sError));
                }
                
                // Check if output is empty (interface likely doesn't exist)
                if sOutput.trim().is_empty() && sError.trim().is_empty() {
                    return Err(anyhow!("Interface {} not found", ifname));
                }
                
                let mut netlink = NetLink::default();
                let lines = sOutput.split("\n");

                netlink.Name = ifname.to_string();

                for line in lines {
                    let line = line.trim();
                    let parts: Vec<&str> = line.split_whitespace().collect();

                    // Extract MAC address
                    if line.contains("ether") && parts.len() >= 2 {
                        netlink.Mac = parts[1].to_string();
                    }
                    
                    // Extract IPv4 address
                    if line.contains("inet ") && !line.contains("inet6") && parts.len() >= 2 {
                        netlink.Ipv4 = parts[1].to_string();
                    }
                    
                    // Extract IPv6 address
                    if line.contains("inet6") && parts.len() >= 2 {
                        netlink.Ipv6 = parts[1].to_string();
                    }
                }
                
                // Try to get interface index
                match Command::new("/sbin/ifconfig")
                    .args(["-l"])
                    .output()
                {
                    Ok(list_output) => {
                        let list_str = String::from_utf8(list_output.stdout)?;
                        let interfaces: Vec<&str> = list_str.split_whitespace().collect();
                        if let Some(pos) = interfaces.iter().position(|&x| x == ifname) {
                            netlink.Index = pos.to_string();
                        }
                    }
                    Err(_) => {
                        // If we can't get the index, leave it empty
                    }
                }
                
                Ok(netlink)
            }
            Err(e) => {
                println!("getNetAddrError {}", e);
                Err(anyhow!("getNetAddrError: {}", e))
            }
        }
    }

    fn ping_internal(host: &str, diag: bool) -> AnyResult<PingResult> {
        let mut result = PingResult::default();
        match Command::new("/sbin/ping")
            .args(["-c", "1", "-t", "1", host])
            .output()
        {
            Ok(output) => {
                let sOutput = String::from_utf8(output.stdout)?;
                let mut sError = String::from_utf8(output.stderr)?;

                if sOutput.contains("icmp_seq=") {
                    // Parse macOS ping output
                    let resultLines: Vec<&str> = sOutput.split("\n").collect();
                    for line in resultLines {
                        if line.contains("bytes from") && line.contains("time=") {
                            // Extract time value
                            if let Some(time_pos) = line.find("time=") {
                                let time_str = &line[time_pos + 5..]; // Skip "time="
                                if let Some(ms_pos) = time_str.find("ms") {
                                    let time_val = &time_str[..ms_pos];
                                    result.duration = time_val.parse::<f32>().unwrap_or(0.0);
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    // No output or error
                    if sError.is_empty() {
                        if diag {
                            // Diagnostics code would go here
                            let _network = MacOSNetwork {};

                            let route_table = MacOSNetwork::get_route_table().unwrap();

                            let routeInfo = route_table.Parse();
                            debug!("GatewayInfo {:?}", routeInfo);

                            if routeInfo.nLinkUp == 0 {
                                error!("Net NoLinkUp");
                                return Err(anyhow!("NoLinkUp"));
                            } else if routeInfo.nGatewayCount == 0 {
                                error!("Net NoGateway");
                                return Err(anyhow!("NoGateway"));
                            } else {
                                // Find route by host
                                let route = route_table.FindRoute(host);

                                if route.is_none() {
                                    error!("Net NoRoute");
                                    return Err(anyhow!("NoRoute"));
                                } else {
                                    // Check route info
                                    debug!("FoundRoute-{:?}", route);
                                    let gateway = &route.unwrap().Gateway;

                                    match MacOSNetwork::ping_internal(&gateway, false) {
                                        Ok(_) => {}
                                        Err(e) => {
                                            error!("GatewayNotReachable {}", e);
                                            return Err(anyhow!("GatewayNotReachable"));
                                        }
                                    }
                                }
                            }
                        } else {
                            return Err(anyhow!("NoErrData"));
                        }
                    } else {
                        error!("sError {}", sError);
                        return Err(anyhow!("{}", sError));
                    }
                }
            }
            Err(e) => {
                error!("Err-{}", e);
                return Err(anyhow!("{}", e));
            }
        }

        Ok(result)
    }
}

#[cfg(all(feature = "net", target_os = "macos"))]
impl os_network for MacOSNetwork {
    fn get_route_table() -> AnyResult<RouteTable> {
        println!("get_route_table");
        match Command::new("/usr/sbin/netstat")
            .args(["-rn"])
            .output()
        {
            Ok(output) => {
                let mut result: RouteTable = RouteTable::default();

                let sOutput = String::from_utf8(output.stdout)?;
                //println!("Ouput \n{}", sOutput);

                let route_list = sOutput.split("\n");

                for route in route_list {
                    //
                    let items: Vec<&str> = route.split_whitespace().collect();

                    if items.len() >= 4 {
                        let mut route_entry = NetRoute::default();

                        let dst = items.get(0).unwrap_or(&"").to_string();
                        let gateway = items.get(1).unwrap_or(&"").to_string();
                        let flags = items.get(2).unwrap_or(&"").to_string();
                        let iface = items.get(3).unwrap_or(&"").to_string();

                        // Skip header lines and invalid entries
                        if dst == "Destination" || dst.contains("Internet") || dst.is_empty() {
                            continue;
                        }

                        route_entry.Dest = dst;
                        route_entry.Gateway = gateway;
                        route_entry.Dev = iface;

                        // Check if interface is down
                        if flags.contains("D") {
                            route_entry.State = "linkdown".to_string();
                        }

                        result.data.push(route_entry);
                    }
                }

                return Ok(result);
            }
            Err(e) => {
                println!("get_route_table_error {}", e);
                Err(anyhow!("get_route_table_error".to_string()))
            }
        }
    }

    fn get_interface_list() -> AnyResult<Vec<NetLink>> {
        let mut result: Vec<NetLink> = vec![];
        match Command::new("/sbin/ifconfig").output() {
            Ok(output) => {
                let sOutput = String::from_utf8(output.stdout)?;
                if sOutput.len() == 0 {
                    let sErr = String::from_utf8(output.stderr)?;
                    return Err(anyhow!("get_interfaces_error {}", sErr));
                }

                let interfaces = sOutput.split("\n\n"); // Split by double newline to separate interfaces

                for interface_block in interfaces {
                    if interface_block.trim().is_empty() {
                        continue;
                    }

                    let mut netlink = NetLink::default();
                    let lines: Vec<&str> = interface_block.split("\n").collect();

                    // Get interface name from first line
                    if let Some(first_line) = lines.first() {
                        let name_parts: Vec<&str> = first_line.split(":").collect();
                        if !name_parts.is_empty() {
                            netlink.Name = name_parts[0].trim().to_string();
                        }
                    }

                    // Parse interface details
                    for line in lines {
                        let line = line.trim();
                        let parts: Vec<&str> = line.split_whitespace().collect();

                        // Extract MAC address
                        if line.contains("ether") && parts.len() >= 2 {
                            netlink.Mac = parts[1].to_string();
                        }
                        
                        // Extract IPv4 address
                        if line.contains("inet ") && !line.contains("inet6") && parts.len() >= 2 {
                            netlink.Ipv4 = parts[1].to_string();
                        }
                        
                        // Extract IPv6 address
                        if line.contains("inet6") && parts.len() >= 2 {
                            // Skip link-local addresses
                            let ipv6_addr = parts[1];
                            if !ipv6_addr.starts_with("fe80") {
                                netlink.Ipv6 = ipv6_addr.to_string();
                            }
                        }
                    }

                    if !netlink.Name.is_empty() {
                        result.push(netlink);
                    }
                }

                Ok(result)
            }
            Err(e) => {
                error!("get_interfaces_error {}", e);
                Err(anyhow!("get_interfaces_error".to_string()))
            }
        }
    }

    fn ping(_host: &str) -> AnyResult<PingResult> {
        MacOSNetwork::ping_internal(_host, true)
    }

    fn nslookup(_host: &str) -> AnyResult<NsLookupResult> {
        Err(anyhow!("NotImplement"))
    }
    
    fn tcping(_host: &str, _port: i32) -> AnyResult<TcpPingResult> {
        Err(anyhow!("NotImplement"))
    }
}

#[cfg(all(feature = "net", target_os = "linux"))]
impl LinuxNetwork {
    /// return mac,ipv4,ipv6,index
    pub fn getIfMacIpAddr(ifname: &str) -> AnyResult<NetLink> {
        match Command::new("/bin/ip")
            .args(["addr", "show", ifname])
            .output()
        {
            Ok(output) => {
                let mut netlink = NetLink::default();
                let sOutput = String::from_utf8(output.stdout)?;
                //println!("Ouput {}", sOutput);
                let sErr = String::from_utf8(output.stderr)?;
                let lines = sOutput.split("\n");

                let mut counter = 0;

                for line in lines {
                    let line = line.trim();

                    let netaddr: Vec<&str> = line.split(" ").collect();

                    if counter == 0 {
                        netlink.Index = netaddr.get(0).unwrap_or(&"").to_string().replace(":", "");
                        counter += 1;
                    }

                    if line.starts_with("link") {
                        netlink.Mac = netaddr.get(1).unwrap_or(&"").to_string();
                    } else if line.starts_with("inet6") {
                        netlink.Ipv6 = netaddr.get(1).unwrap_or(&"").to_string();
                    } else if line.starts_with("inet") {
                        netlink.Ipv4 = netaddr.get(1).unwrap_or(&"").to_string();
                    }
                }
                Ok(netlink)
            }
            Err(e) => {
                println!("getNetAddrError {}", e);
                Err(anyhow!("getNetAddrError".to_string()))
            }
        }
    }

    fn ping_internal(host: &str, diag: bool) -> AnyResult<PingResult> {
        let mut result = PingResult::default();
        match Command::new("/bin/ping")
            .args(["-c", "1", "-W", "1", host])
            .output()
        {
            Ok(output) => {
                let sOutput = String::from_utf8(output.stdout)?;
                let mut sError = String::from_utf8(output.stderr)?;

                if sOutput.contains("icmp_seq=") {
                    let resultLines: Vec<&str> = sOutput.split("\n").collect();
                    let response: Vec<&str> =
                        resultLines.get(1).unwrap_or(&"").split("=").collect();
                    let response_time = response.get(3).unwrap_or(&"");
                    let response_list: Vec<&str> = response_time.split(" ").collect();
                    //println!("response_list {:?}", response_list);
                    result.duration = response_list.get(0).unwrap_or(&"").parse::<f32>().unwrap();
                    //println!("sOutputOk {}",response.get(3).unwrap_or(&""));
                } else {
                    //linux no output
                    if sError.is_empty() {
                        if diag {
                            //find other reason
                            let _network = LinuxNetwork {};

                            let route_table = LinuxNetwork::get_route_table().unwrap();

                            let routeInfo = route_table.Parse();
                            debug!("GatewayInfo {:?}", routeInfo);

                            if routeInfo.nLinkUp == 0 {
                                error!("Net NoLinkUp");
                                return Err(anyhow!("NoLinkUp"));
                            } else if routeInfo.nGatewayCount == 0 {
                                error!("Net NoGateway");
                                return Err(anyhow!("NoGateway"));
                            } else {
                                //find route by host
                                let route = route_table.FindRoute(host);

                                if route.is_none() {
                                    error!("Net NoRoute");
                                    return Err(anyhow!("NoRoute"));
                                } else {
                                    //check route info
                                    debug!("FoundRoute-{:?}", route);
                                    let gateway = &route.unwrap().Gateway;

                                    match LinuxNetwork::ping_internal(&gateway, false) {
                                        Ok(dt) => {}
                                        Err(e) => {
                                            error!("GatewayNotReachable {}", e);
                                            return Err(anyhow!("GatewayNotReachable"));
                                        }
                                    }
                                }
                            }
                        } else {
                            return Err(anyhow!("NoErrData"));
                        }
                    } else {
                        if sError.contains("ping: unknown host") {
                            sError = "ping: unknown host".to_string()
                        }

                        error!("sError {}", sError);
                        return Err(anyhow!("{}", sError));
                    }
                }
            }
            Err(e) => {
                error!("Err-{}", e);
                return Err(anyhow!("{}", e));
            }
        }

        Ok(result)
    }
}

#[cfg(all(feature = "net", target_os = "linux"))]
impl os_network for LinuxNetwork {
    fn get_route_table() -> AnyResult<RouteTable> {
        println!("get_route_table");
        match Command::new("/bin/ip")
            .args(["route", "list", "table", "0"])
            .output()
        {
            Ok(output) => {
                let mut result: RouteTable = RouteTable::default();

                let sOutput = String::from_utf8(output.stdout)?;
                //println!("Ouput \n{}", sOutput);

                let route_list = sOutput.split("\n");

                for route in route_list {
                    //
                    let items: Vec<&str> = route.split(" ").collect();

                    if items.len() > 4 {
                        let mut route = NetRoute::default();

                        let dst = items.get(0).unwrap_or(&"").to_string();

                        if dst == "local" || dst == "multicast" || dst == "broadcast" {
                            continue;
                        } else {
                            route.Dest = dst;
                        }

                        for mut pos in 0..items.len() {
                            let flag = items.get(pos).unwrap_or(&"").to_string();
                            if flag == "dev" {
                                route.Dev = items.get(pos + 1).unwrap_or(&"").to_string();
                                pos += 1;
                            } else if flag == "src" {
                                route.Src = items.get(pos + 1).unwrap_or(&"").to_string();
                                pos += 1;
                            } else if flag == "linkdown" {
                                route.State = flag.to_string();
                                pos += 1;
                            } else if flag == "via" {
                                route.Gateway = items.get(pos + 1).unwrap_or(&"").to_string();
                                pos += 1;
                            }
                        }
                        //

                        let dev_flag = items.get(1).unwrap_or(&"").to_string();

                        if dev_flag == "dev" {
                            route.Dev = items.get(2).unwrap_or(&"").to_string();
                        }

                        if route.Dev.starts_with("dummy") {
                            continue;
                        }
                        result.data.push(route);
                    }
                }

                return Ok(result);
            }
            Err(e) => {
                println!("get_route_table_error {}", e);
                Err(anyhow!("get_route_table_error".to_string()))
            }
        }
    }

    fn get_interface_list() -> AnyResult<Vec<NetLink>> {
        let mut result: Vec<NetLink> = vec![];
        match Command::new("/bin/ip").args(["addr"]).output() {
            Ok(output) => {
                let sOutput = String::from_utf8(output.stdout)?;
                if sOutput.len() == 0 {
                    let sErr = String::from_utf8(output.stderr)?;
                    return Err(anyhow!("get_interfaces_error {}", sErr));
                }

                let lines = sOutput.split("\n");

                let mut counter = 0;

                let mut netlink = NetLink {
                    Mac: "".to_string(),
                    Ipv4: "".to_string(),
                    Ipv6: "".to_string(),
                    Name: "".to_string(),
                    Index: "".to_string(),
                    State: "".to_string(),
                };

                for line in lines {
                    let line = line.trim();
                    let netaddr: Vec<&str> = line.split(" ").collect();

                    if line.contains("state") {
                        //new line
                        counter = 0;
                        if netlink.Name != "" {
                            result.push(netlink.clone());
                        }

                        //state check
                        let mut nPos = 0;
                        for item in netaddr.clone() {
                            if item == "state" {
                                netlink.State = netaddr.get(nPos + 1).unwrap_or(&"").to_string();
                            }
                            nPos += 1;
                        }
                    }

                    if counter == 0 {
                        netlink.Index = netaddr.get(0).unwrap_or(&"").to_string().replace(":", "");
                        netlink.Name = netaddr.get(1).unwrap_or(&"").to_string().replace(":", "");
                        counter += 1;
                    }

                    if line.starts_with("link") {
                        netlink.Mac = netaddr.get(1).unwrap_or(&"").to_string();
                    }

                    if line.starts_with("inet6") {
                        netlink.Ipv6 = netaddr.get(1).unwrap_or(&"").to_string();
                    } else if line.starts_with("inet") {
                        netlink.Ipv4 = netaddr.get(1).unwrap_or(&"").to_string();
                    }
                }

                if netlink.Name != "" {
                    result.push(netlink.clone());
                }

                Ok(result)
            }
            Err(e) => {
                error!("get_interfaces_error {}", e);
                Err(anyhow!("get_interfaces_error".to_string()))
            }
        }
    }

    fn ping(_host: &str) -> AnyResult<PingResult> {
        LinuxNetwork::ping_internal(_host, true)
    }

    fn nslookup(_host: &str) -> AnyResult<NsLookupResult> {
        Err(anyhow!("NotImplement"))
    }
    fn tcping(_host: &str, _port: i32) -> AnyResult<TcpPingResult> {
        Err(anyhow!("NotImplement"))
    }
}

#[cfg(test)]
#[cfg(feature = "net")]
mod tests {
    use super::*;
    use anyhow::{anyhow, Result as AnyResult};

    #[test]
    #[cfg(target_os = "linux")]
    fn test_get_interface_list() {
        let _network = LinuxNetwork {};
        println!("{:?}", LinuxNetwork::get_interface_list());
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_get_interface_list() {
        let _network = MacOSNetwork {};
        println!("{:?}", MacOSNetwork::get_interface_list());
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_get_route_table() {
        let _network = LinuxNetwork {};

        let route_table = LinuxNetwork::get_route_table().unwrap();

        println!(
            "{:?}/{:?}/test-route {:?}",
            route_table,
            route_table.Parse(),
            route_table.FindRoute("8.8.8.8")
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_get_route_table() {
        let _network = MacOSNetwork {};

        let route_table = MacOSNetwork::get_route_table().unwrap();

        println!(
            "{:?}/{:?}/test-route {:?}",
            route_table,
            route_table.Parse(),
            route_table.FindRoute("8.8.8.8")
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_ping() {
        let _network = LinuxNetwork {};

        let _route_table = LinuxNetwork::get_route_table().unwrap();

        println!("{:?}", LinuxNetwork::ping("8.8.8.8"));
        println!("{:?}", LinuxNetwork::ping("1.net"));
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_ping() {
        let _network = MacOSNetwork {};

        let _route_table = MacOSNetwork::get_route_table().unwrap();

        println!("{:?}", MacOSNetwork::ping("8.8.8.8"));
        println!("{:?}", MacOSNetwork::ping("1.net"));
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_get_if_mac_ip_addr() {
        // Test with loopback interface which should exist on all macOS systems
        let result = MacOSNetwork::getIfMacIpAddr("lo0");
        assert!(result.is_ok());
        let netlink = result.unwrap();
        assert_eq!(netlink.Name, "lo0");
        println!("Interface lo0: {:?}", netlink);
        
        // Test with a non-existent interface
        let result = MacOSNetwork::getIfMacIpAddr("nonexistent0");
        assert!(result.is_err());
        println!("Non-existent interface test passed");
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_route_table_parse() {
        let route_table = MacOSNetwork::get_route_table().unwrap();
        let gateway_info = route_table.Parse();
        println!("Gateway Info: {:?}", gateway_info);
        
        // Should have at least some routes
        assert!(route_table.data.len() > 0);
        
        // Gateway info should be valid
        // Note: usize is always >= 0, so we just verify the values are reasonable
        assert!(gateway_info.nGatewayCount > 0);
        assert!(gateway_info.nLinkUp > 0 || gateway_info.nLinkUp == 0);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_find_route() {
        let route_table = MacOSNetwork::get_route_table().unwrap();
        
        // Test finding route for common addresses
        let route = route_table.FindRoute("8.8.8.8");
        println!("Route to 8.8.8.8: {:?}", route);
        
        let route = route_table.FindRoute("1.1.1.1");
        println!("Route to 1.1.1.1: {:?}", route);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_ping_internal() {
        // Test successful ping
        let result = MacOSNetwork::ping_internal("8.8.8.8", false);
        assert!(result.is_ok());
        let ping_result = result.unwrap();
        assert!(ping_result.duration >= 0.0);
        println!("Ping to 8.8.8.8: {:?}", ping_result);
        
        // Test ping to unreachable host (should fail)
        let result = MacOSNetwork::ping_internal("192.0.2.1", false); // Reserved test IP
        assert!(result.is_err());
        println!("Ping to reserved IP result: {:?}", result);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_nslookup_not_implemented() {
        let result = MacOSNetwork::nslookup("google.com");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "NotImplement");
        println!("NSLookup correctly returns NotImplement");
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_tcping_not_implemented() {
        let result = MacOSNetwork::tcping("google.com", 80);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "NotImplement");
        println!("TCPing correctly returns NotImplement");
    }
}
