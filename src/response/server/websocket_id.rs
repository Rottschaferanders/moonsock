// {
//     "jsonrpc": "2.0",
//     "result": {
//         "websocket_id": 1730367696
//     },
//     "id": 345
// }
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebsocketIdResponse {
    pub websocket_id: u32
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::{MoonResponse, JsonRpcVersion, response::MoonResultData};

    #[test]
    fn test_serialize_websocket_id() {
        let response = MoonResponse::MoonResult {
            jsonrpc: JsonRpcVersion::V2,
            result: MoonResultData::WebsocketIdResponse(WebsocketIdResponse {
                websocket_id: 1730367696
            }),
            id: 345
        };

        let expected = r#"{"jsonrpc":"2.0","result":{"websocket_id":1730367696},"id":345}"#;
        assert_eq!(serde_json::to_string(&response).unwrap(), expected);
    }

    #[test]
    fn test_deserialize_websocket_id() {
        let input = r#"{"jsonrpc":"2.0","result":{"websocket_id":1730367696},"id":345}"#;
        let response: MoonResponse = serde_json::from_str(input).unwrap();

        let expected = MoonResponse::MoonResult {
            jsonrpc: JsonRpcVersion::V2,
            result: MoonResultData::WebsocketIdResponse(WebsocketIdResponse {
                websocket_id: 1730367696
            }),
            id: 345
        };

        assert_eq!(response, expected);
    }
}