use moonsock::{MoonMSG, moon_result::MoonResultData, moon_result::{TemperatureStore, TempStoreData, HeaterNames}};
use std::{
    string::String,
    fs::File,
    io::Read,
};

#[test]
fn temperature_store_parse() {
    let bed = TempStoreData::TempTgtsPowers {
        temperatures: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
        targets: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
        powers: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
    };
    let extruder = TempStoreData::TempTgtsPowers {
        temperatures: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
        targets: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
        powers: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
    };
    // let mut temp_store = HashMap::new();
    let mut temp_store = TemperatureStore::new();
    temp_store.add_to_hashmap(HeaterNames::HeaterBed, bed);
    temp_store.add_to_hashmap(HeaterNames::Extruder, extruder);
    // let msg_two = MoonMSG::new_result(MoonResultData::TemperatureStore(temp_store), 1); 

    // let msg_string = serde_json::to_string(&msg_two).unwrap();
    // println!("{}", msg_string);

    let message = r##"{
        "jsonrpc": "2.0", 
        "result": {
            "heater_bed": {
                "temperatures": [80.04, 80.05, 80.04, 80.03, 80.04], 
                "targets": [ 80.0, 80.0, 80.0, 80.0, 80.0, 80.0], 
                "powers": [0.6396684663343952, 0.45537880698021727]
            },
            "extruder": {
                "temperatures": [229.99, 229.94, 229.89, 229.95, 229.96, 230.07], 
                "targets": [0.0, 230.0, 230.0, 230.0, 230.0, 230.0, 230.0, 230.0], 
                "powers": [0.07110582186476708, 0.1597490186929997, 0.079685480023094]
            }
        }, 
        "id": 2043
    }"##;

    let msg: MoonMSG = serde_json::from_str(message).unwrap();
    // println!("Parsed: {:#?}", msg);
    match msg {
        MoonMSG::MoonResult { result, .. } => {
            // println!("Received Result: {}", id);
            // println!("Result: {:?}", result);
            match result {
                MoonResultData::TemperatureStore ( temp_store ) => {
                    let bed_contents = temp_store.items.get(&HeaterNames::HeaterBed).unwrap();
                    let extruder_contents = temp_store.items.get(&HeaterNames::Extruder).unwrap();
                    assert_eq!(
                        bed_contents, 
                        &TempStoreData::TempTgtsPowers {
                            temperatures: vec![80.04, 80.05, 80.04, 80.03, 80.04], 
                            targets: vec![80.0, 80.0, 80.0, 80.0, 80.0, 80.0], 
                            powers: vec![0.6396684663343952, 0.45537880698021727]
                        }
                    );
                    assert_eq!(
                        extruder_contents, 
                        &TempStoreData::TempTgtsPowers {
                            temperatures: vec![229.99, 229.94, 229.89, 229.95, 229.96, 230.07],
                            targets: vec![0.0, 230.0, 230.0, 230.0, 230.0, 230.0, 230.0, 230.0],
                            powers: vec![0.07110582186476708, 0.1597490186929997, 0.079685480023094]
                        }
                    );
                },
                _ => panic!("Message is MoonResult, but it's result is not ServerTemperatureStore: {:?}", result),
            }
        },
        _ => panic!("Unexpected Message: {:?}", msg),
    }
}


const TEMP_STORE_JSON_PATH: &str = "tests/temp_store.json";
#[test]
fn temp_store_serialize() {
    let bed = TempStoreData::TempTgtsPowers {
        temperatures: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
        targets: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
        powers: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
    };
    let extruder = TempStoreData::TempTgtsPowers {
        temperatures: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
        targets: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
        powers: vec![22.66, 22.65, 22.58, 22.6, 22.6, 22.59, 22.63, 22.62, 22.62, 22.59, 22.57, 22.55, 22.62],
    };
    // let mut temp_store = HashMap::new();
    let mut temp_store = TemperatureStore::new();
    temp_store.add_to_hashmap(HeaterNames::Extruder, extruder);
    temp_store.add_to_hashmap(HeaterNames::HeaterBed, bed);
    let msg_two = MoonMSG::new_result(MoonResultData::TemperatureStore(temp_store), 1); 

    let mut file = File::open(TEMP_STORE_JSON_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let file_parsed = serde_json::from_str::<MoonMSG>(&contents).unwrap();
    
    assert_eq!(msg_two, file_parsed);
}