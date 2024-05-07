use serde::{Serialize, Deserialize};

mod printer_state;
pub use printer_state::*;

mod temperature;
pub use temperature::*;

mod gcode;
pub use gcode::*;

mod printer_object;
pub use printer_object::*;

mod printer_objects_list;
pub use printer_objects_list::*;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MoonOk {
    #[serde(rename = "ok")]
    Ok,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MoonResultData {
    #[serde(alias = "ok")]
    Ok(MoonOk),
    TemperatureStore(TemperatureStore),
    PrinterInfoResponse(PrinterInfoResponse),
    QueryPrinterObjectsResponse(QueryPrinterObjectsResponse),
    PrinterObjectsListResponse(PrinterObjectsListResponse),
}