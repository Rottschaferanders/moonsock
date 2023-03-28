use moonsock::{MoonMSG, MoonMethod, MoonParam};

#[test]
fn notify_gcode_response() {
    let msg_struct = MoonMSG::MethodParamVec {
        jsonrpc: moonsock::JsonRpcVersion::V2_0,
        method: MoonMethod::NotifyGcodeResponse,
        params: vec![MoonParam::NotifyGcodeResponse("!! Must home axis first: 160.200 210.000 50.022 [7013.719]".to_string())],
    };
    let msg_struct_string = serde_json::to_string(&msg_struct).unwrap();
    println!("{}", msg_struct_string);
    let message = r#"{
        "jsonrpc": "2.0", 
        "method": "notify_gcode_response", 
        "params": [
            "!! Must home axis first: 160.200 210.000 50.022 [7013.719]"
        ]
    }"#;
    let msg: MoonMSG = serde_json::from_str(message).unwrap();
    println!("{:?}", msg);
    let meg_string = serde_json::to_string(&msg).unwrap();
    println!("{}", meg_string);
    match msg.params()[0].clone() {
        MoonParam::NotifyGcodeResponse(response) => {
            assert_eq!(response.clone(), "!! Must home axis first: 160.200 210.000 50.022 [7013.719]");
        }
        _ => panic!("Wrong message type"),
    }
}

#[test]
fn notify_proc_stat_update() {
    use moonsock::{MoonMSG, MoonMethod, MoonParam};
    let message = r##"{
        "jsonrpc": "2.0", 
        "method": "notify_proc_stat_update", 
        "params": 
            [
                {
                    "moonraker_stats": {
                        "time": 1679112330.7287357, 
                        "cpu_usage": 2.37, 
                        "memory": 18156, 
                        "mem_units": "kB"
                    }, 
                    "cpu_temp": 54.53, 
                    "network": {
                        "lo": {
                            "rx_bytes": 6394808114, 
                            "tx_bytes": 6394808114, 
                            "rx_packets": 21933659, 
                            "tx_packets": 21933659, 
                            "rx_errs": 0, 
                            "tx_errs": 0, 
                            "rx_drop": 0, 
                            "tx_drop": 0, 
                            "bandwidth": 8630.21
                        }, 
                        "eth0": {
                            "rx_bytes": 0, 
                            "tx_bytes": 0, 
                            "rx_packets": 0, 
                            "tx_packets": 0, 
                            "rx_errs": 0,
                            "tx_errs": 0, 
                            "rx_drop": 0, 
                            "tx_drop": 0, 
                            "bandwidth": 0.0
                        }, 
                        "wlan0": {
                            "rx_bytes": 2637034553, 
                            "tx_bytes": 3243733993, 
                            "rx_packets": 14919504, 
                            "tx_packets": 13199378, 
                            "rx_errs": 0, 
                            "tx_errs": 0, 
                            "rx_drop": 0, 
                            "tx_drop": 0, 
                            "bandwidth": 4659.79
                        }
                    }, 
                    "system_cpu_usage": {
                        "cpu": 8.79, 
                        "cpu0": 2.02, 
                        "cpu1": 1.03, 
                        "cpu2": 2.97, 
                        "cpu3": 29.0
                    }, 
                    "system_memory": {
                        "total": 3748168, 
                        "available": 901140, 
                        "used": 2847028
                    }, 
                    "websocket_connections": 2
                }
            ]
    }"##;
    // let example = example_notify_proc_stat_update();
    // let example_string = serde_json::to_string_pretty(&example).unwrap();
    // println!("{}", example_string);
    let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
    // println!("{:?}", &serialized);
    assert_eq!(serialized.method(), MoonMethod::NotifyProcStatUpdate);
    match serialized.params()[0].clone() {
        MoonParam::NotifyProcStatUpdate {
            moonraker_stats,
            cpu_temp,
            network,
            system_cpu_usage,
            system_memory,
            websocket_connections,
        } => {
            assert_eq!(moonraker_stats.time, 1679112330.7287357);
            assert_eq!(moonraker_stats.cpu_usage, 2.37);
            assert_eq!(moonraker_stats.memory, 18156);
            assert_eq!(moonraker_stats.mem_units, "kB");
            assert_eq!(cpu_temp.clone(), 54.53);
            assert_eq!(network.lo.rx_bytes, 6394808114);
            assert_eq!(network.lo.tx_bytes, 6394808114);
            assert_eq!(network.lo.rx_packets, 21933659);
            assert_eq!(network.lo.tx_packets, 21933659);
            assert_eq!(network.lo.rx_errs, 0);
            assert_eq!(network.lo.tx_errs, 0);
            assert_eq!(network.lo.rx_drop, 0);
            assert_eq!(network.lo.tx_drop, 0);
            assert_eq!(network.lo.bandwidth, 8630.21);
            assert_eq!(network.eth0.rx_bytes, 0);
            assert_eq!(network.eth0.tx_bytes, 0);
            assert_eq!(network.eth0.rx_packets, 0);
            assert_eq!(network.eth0.tx_packets, 0);
            assert_eq!(network.eth0.rx_errs, 0);
            assert_eq!(network.eth0.tx_errs, 0);
            assert_eq!(network.eth0.rx_drop, 0);
            assert_eq!(network.eth0.tx_drop, 0);
            assert_eq!(network.eth0.bandwidth, 0.0);
            assert_eq!(network.wlan0.rx_bytes, 2637034553);
            assert_eq!(network.wlan0.tx_bytes, 3243733993);
            assert_eq!(network.wlan0.rx_packets, 14919504);
            assert_eq!(network.wlan0.tx_packets, 13199378);
            assert_eq!(network.wlan0.rx_errs, 0);
            assert_eq!(network.wlan0.tx_errs, 0);
            assert_eq!(network.wlan0.rx_drop, 0);
            assert_eq!(network.wlan0.tx_drop, 0);
            assert_eq!(network.wlan0.bandwidth, 4659.79);
            assert_eq!(system_cpu_usage.cpu, 8.79);
            assert_eq!(system_cpu_usage.cpu0, 2.02);
            assert_eq!(system_cpu_usage.cpu1, 1.03);
            assert_eq!(system_cpu_usage.cpu2, 2.97);
            assert_eq!(system_cpu_usage.cpu3, 29.0);
            assert_eq!(system_memory.total, 3748168);
            assert_eq!(system_memory.available, 901140);
            assert_eq!(system_memory.used, 2847028);
            assert_eq!(websocket_connections, 2);
        }
        _ => assert!(
            false,
            "ERROR: Did not parse params as MoonParam::NotifyProcStatUpdate"
        ),
    }
}

#[test]
fn notify_proc_stat_update_two() {
    use moonsock::{MoonMSG, MoonMethod};
    let message = r##"{
        "jsonrpc": "2.0", 
        "method": "notify_proc_stat_update", 
        "params": [
            {"moonraker_stats": 
                {"time": 1679112330.7287357, "cpu_usage": 2.37, "memory": 18156, "mem_units": "kB"}, 
            "cpu_temp": 54.53, 
            "network": {
                "lo": 
                    {"rx_bytes": 6394808114, "tx_bytes": 6394808114, "rx_packets": 21933659, "tx_packets": 21933659, "rx_errs": 0, "tx_errs": 0, "rx_drop": 0, "tx_drop": 0, "bandwidth": 8630.21}, 
                "eth0": {"rx_bytes": 0, "tx_bytes": 0, "rx_packets": 0, "tx_packets": 0, "rx_errs": 0, "tx_errs": 0, "rx_drop": 0, "tx_drop": 0, "bandwidth": 0.0}, 
                "wlan0": {"rx_bytes": 2637034553, "tx_bytes": 3243733993, "rx_packets": 14919504, "tx_packets": 13199378, "rx_errs": 0, "tx_errs": 0, "rx_drop": 0, "tx_drop": 0, "bandwidth": 4659.79}}, 
                "system_cpu_usage": {"cpu": 8.79, "cpu0": 2.02, "cpu1": 1.03, "cpu2": 2.97, "cpu3": 29.0}, 
                "system_memory": {"total": 3748168, "available": 901140, "used": 2847028}, 
                "websocket_connections": 2
            }
        ]
    }"##;
    let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
    assert_eq!(serialized.method(), MoonMethod::NotifyProcStatUpdate);
}
use moonsock::{moon_param::{MoonrakerStats, Network, NetworkData, SystemCpuUsage, SystemMemory}};
fn example_notify_proc_stat_update() -> MoonMSG {
    MoonMSG::MethodParam {
        jsonrpc: moonsock::JsonRpcVersion::V2_0,
        method: MoonMethod::NotifyProcStatUpdate,
        params: MoonParam::NotifyProcStatUpdate {
            moonraker_stats: MoonrakerStats {
                time: 1679112330.7287357,
                cpu_usage: 2.37,
                memory: 18156,
                mem_units: "kB".to_string(),
            },
            cpu_temp: 54.53,
            network: Network {
                lo: NetworkData {
                    rx_bytes: 6394808114,
                    tx_bytes: 6394808114,
                    rx_packets: 21933659,
                    tx_packets: 21933659,
                    rx_errs: 0,
                    tx_errs: 0,
                    rx_drop: 0,
                    tx_drop: 0,
                    bandwidth: 8630.21,
                },
                eth0: NetworkData {
                    rx_bytes: 0,
                    tx_bytes: 0,
                    rx_packets: 0,
                    tx_packets: 0,
                    rx_errs: 0,
                    tx_errs: 0,
                    rx_drop: 0,
                    tx_drop: 0,
                    bandwidth: 0.0,
                },
                wlan0: NetworkData {
                    rx_bytes: 2637034553,
                    tx_bytes: 3243733993,
                    rx_packets: 14919504,
                    tx_packets: 13199378,
                    rx_errs: 0,
                    tx_errs: 0,
                    rx_drop: 0,
                    tx_drop: 0,
                    bandwidth: 4659.79,
                },
            },
            system_cpu_usage: SystemCpuUsage {
                cpu: 8.79,
                cpu0: 2.02,
                cpu1: 1.03,
                cpu2: 2.97,
                cpu3: 29.0,
            },
            system_memory: SystemMemory {
                total: 3748168,
                available: 901140,
                used: 2847028,
            },
            websocket_connections: 2,
        }
    }
}


#[test]
fn notify_cpu_throttled() {
    let message = r##"{
        "jsonrpc": "2.0", 
        "method": "notify_cpu_throttled", 
        "params": [
            {
                "bits": 327680, 
                "flags": [
                    "Previously Under-Volted", 
                    "Previously Throttled"
                ]
            }
        ]
    }"##;
    let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
    match serialized.params()[0].clone() {
        MoonParam::NotifyCpuThrottled { bits, flags } => {
            assert_eq!(bits, 327680);
            assert_eq!(flags[0], "Previously Under-Volted");
            assert_eq!(flags[1], "Previously Throttled");
        }
        _ => assert!(false, "ERROR: Did not parse params as MoonParam::NotifyCpuThrottled"),
    }
    assert_eq!(serialized.method(), MoonMethod::NotifyCpuThrottled);
}
