

use moonsock::{
    response::{GcodeMove, PrinterObjectStatus, Toolhead}, JsonRpcVersion, 
    MoonResponse, NotificationMethod, NotificationParam
};

// #[test]
// fn notify_status_update() {
//     let message = r##"{
//         "jsonrpc": "2.0", 
//         "method": "notify_status_update", 
//         "params": 
//             [
//                 {
//                     "gcode_move": {
//                         "absolute_coordinates": true,
//                         "absolute_extrude": true,
//                         "extrude_factor": 1,
//                         "gcode_position": [0, 0, 0, 0],
//                         "homing_origin": [0, 0, 0, 0],
//                         "position": [0, 0, 0, 0],
//                         "speed": 1500,
//                         "speed_factor": 1
//                     },
//                     "toolhead": {
//                         "position": [0, 0, 0, 0]
//                     }
//                 },
//                 32423.234411232
//             ]
//     }"##;
//     let serialized: MoonResponse = serde_json::from_str(&message).unwrap();

//     match serialized {
//         MoonResponse::Notification { method, params, .. } => {
//             assert_eq!(method, NotificationMethod::NotifyStatusUpdate);

//             // match params.unwrap()[0].clone() {
//             match params.unwrap().clone() {
//                 NotificationParam::StatusUpdate(printer_object_status, eventtime) => {
//                     // GcodeMove
//                     assert_eq!(printer_object_status.gcode_move.clone().unwrap().absolute_coordinates.unwrap(), true);
//                     assert_eq!(printer_object_status.gcode_move.clone().unwrap().absolute_extrude.unwrap(), true);
//                     assert_eq!(printer_object_status.gcode_move.clone().unwrap().extrude_factor.unwrap(), 1.0);
//                     assert_eq!(printer_object_status.gcode_move.clone().unwrap().gcode_position.unwrap(), vec![0.0, 0.0, 0.0, 0.0]);
//                     assert_eq!(printer_object_status.gcode_move.clone().unwrap().homing_origin.unwrap(), vec![0.0, 0.0, 0.0, 0.0]);
//                     assert_eq!(printer_object_status.gcode_move.clone().unwrap().position.unwrap(), vec![0.0, 0.0, 0.0, 0.0]);
//                     assert_eq!(printer_object_status.gcode_move.clone().unwrap().speed.unwrap(), 1500.0);
//                     assert_eq!(printer_object_status.gcode_move.clone().unwrap().speed_factor.unwrap(), 1.0);
//                     // Toolhead
//                     assert_eq!(printer_object_status.toolhead.clone().unwrap().position.unwrap(), vec![0.0, 0.0, 0.0, 0.0]);
//                     // Eventtime
//                     assert_eq!(eventtime, 32423.234411232);
//                 },
//                 _ => assert!(
//                     false,
//                     "ERROR: Did not parse params as MoonParam::NotifyProcStatUpdate"
//                 ),
//             }
//         },
//         _ => assert!(false),
//     }
// }

#[test]
fn test_serialize_notify_status_update() {
    let data = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifyStatusUpdate,
        params: Some(NotificationParam::StatusUpdate(
            PrinterObjectStatus {
                gcode_move: Some(GcodeMove {
                    speed_factor: Some(1.0),
                    speed: Some(1500.0),
                    extrude_factor: Some(1.0),
                    absolute_coordinates: Some(true),
                    absolute_extrude: Some(true),
                    homing_origin: Some(vec![
                        // 0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001, 
                        0.0,
                        0.0, 
                        0.0, 
                        0.0,
                    ]),
                    position: Some(vec![0.0, 0.0, 0.0, 0.0]),
                    gcode_position: Some(vec![0.0, 0.0, 0.0, 0.0])
                }),
                toolhead: Some(Toolhead {
                    homed_axes: None,
                    print_time: None,
                    estimated_print_time: None,
                    extruder: None,
                    position: Some(vec![0.0, 0.0, 0.0, 0.0]),
                    max_velocity: None,
                    max_accel: None,
                    max_accel_to_decel: None,
                    square_corner_velocity: None,
                }),
                configfile: None,
                extruder: None,
                heater_bed: None,
                fan: None,
                idle_timeout: None,
                virtual_sdcard: None,
                print_stats: None,
                display_status: None,
                z_tilt: None,
            },
            32423.234411232
        )),
    };
    // Next 4 lines: Not Relevent, just some random stuff I was doing, can delete, but kind of want to keep it here in rememberance.
    // let testing_max_float_dp = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string();
    // println!("Max Float Decimal Places: {}", testing_max_float_dp.len());
    // let count = testing_max_float_dp.chars().count();
    // println!("Max Float Decimal Places (by `chars().count()`: {}", count);
    
    let expected = r##"{"jsonrpc":"2.0","method":"notify_status_update","params":[{"gcode_move":{"absolute_coordinates":true,"absolute_extrude":true,"extrude_factor":1.0,"gcode_position":[0.0,0.0,0.0,0.0],"homing_origin":[0.0,0.0,0.0,0.0],"position":[0.0,0.0,0.0,0.0],"speed":1500.0,"speed_factor":1.0},"toolhead":{"position":[0.0,0.0,0.0,0.0]}},32423.234411232]}"##;
    let actual = serde_json::to_string(&data).unwrap();
    println!("The Actual:\n{actual}");
    println!("The expected:\n{expected}");
    assert_eq!(actual, expected);
}

#[test]
fn test_deserialize_notify_status_update() {
    let json = r##"{
        "jsonrpc": "2.0",
        "method": "notify_status_update",
        "params": [
            {
                "gcode_move": {
                    "absolute_coordinates": true,
                    "absolute_extrude": true,
                    "extrude_factor": 1.0,
                    "gcode_position": [0.0, 0.0, 0.0, 0.0],
                    "homing_origin": [0.0, 0.0, 0.0, 0.0],
                    "position": [0.0, 0.0, 0.0, 0.0],
                    "speed": 1500.0,
                    "speed_factor": 1.0
                },
                "toolhead": {
                    "position": [0.0, 0.0, 0.0, 0.0]
                }
            },
            32423.234411232
        ]
    }"##;

    let expected = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifyStatusUpdate,
        params: Some(NotificationParam::StatusUpdate(
            PrinterObjectStatus {
                gcode_move: Some(GcodeMove {
                    speed_factor: Some(1.0),
                    speed: Some(1500.0),
                    extrude_factor: Some(1.0),
                    absolute_coordinates: Some(true),
                    absolute_extrude: Some(true),
                    homing_origin: Some(vec![0.0, 0.0, 0.0, 0.0]),
                    position: Some(vec![0.0, 0.0, 0.0, 0.0]),
                    gcode_position: Some(vec![0.0, 0.0, 0.0, 0.0])
                }),
                toolhead: Some(Toolhead {
                    homed_axes: None,
                    print_time: None,
                    estimated_print_time: None,
                    extruder: None,
                    position: Some(vec![0.0, 0.0, 0.0, 0.0]),
                    max_velocity: None,
                    max_accel: None,
                    max_accel_to_decel: None,
                    square_corner_velocity: None,
                }),
                configfile: None,
                extruder: None,
                heater_bed: None,
                fan: None,
                idle_timeout: None,
                virtual_sdcard: None,
                print_stats: None,
                display_status: None,
                z_tilt: None,
            },
            32423.234411232
        )),
    };

    let actual: MoonResponse = serde_json::from_str(json).unwrap();
    assert_eq!(actual, expected);
}