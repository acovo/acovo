// Conditional compilation for the proto feature
#[cfg(feature = "proto")]
use anyhow::{anyhow, Result};
#[cfg(feature = "proto")]
use serde_derive::{Deserialize, Serialize};

// RequestHeader represents the header structure for API requests
// It contains metadata about the request such as version, action, signature, timestamp and sender
#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RequestHeader {
    // Version of the API protocol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    // Action to be performed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    // Signature for request authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
    // Timestamp of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    // Sender identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
}
// Implementation of RequestHeader methods
#[cfg(feature = "proto")]
impl RequestHeader {
    // Creates a new RequestHeader with only the signature field set
    pub fn new_with_sign(sign: String) -> Self {
        RequestHeader {
            version: None,
            action: None,
            sign: Some(sign),
            timestamp: None,
            sender: None,
        }
    }

    // Creates a new RequestHeader with both signature and action fields set
    pub fn new_with_sign_action(sign: String, action: String) -> Self {
        RequestHeader {
            version: None,
            action: Some(action),
            sign: Some(sign),
            timestamp: None,
            sender: None,
        }
    }
}

// Generic Request structure that wraps a header and a body
// T represents the type of the request body
#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Request<T> {
    // Optional header containing request metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<RequestHeader>,
    // Optional body containing the actual request data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,
}

// Implementation of Request methods
#[cfg(feature = "proto")]
impl<T> Request<T> {
    // Creates a new Request with the given body value
    pub fn new(value: Option<T>) -> Self {
        Request {
            head: None,
            body: value,
        }
    }

    // Validates the request by ensuring it has a header with a signature
    pub fn validate(&self) -> Result<()> {
        if *&self.head.is_none() || *(&self.head.as_ref().unwrap().sign.is_none()) {
            return Err(anyhow!("sign data is required！"));
        } else {
            return Ok(());
        }
    }
}

// State represents the status of a response with a code and message
#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct State {
    // Status code (typically follows HTTP status codes)
    pub ret_code: u32,
    // Human-readable status message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_message: Option<String>,
}

// ResponseHeader represents the header structure for API responses
// It contains metadata about the response such as version, action, signature, timestamp and host
#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResponseHeader {
    // Version of the API protocol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    // Action that was performed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    // Signature for response authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
    // Timestamp of the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    // Host that generated the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

// Implementation of ResponseHeader methods
#[cfg(feature = "proto")]
impl ResponseHeader {
    // Creates a new ResponseHeader with only the signature field set
    pub fn new_with_sign(sign: String) -> Self {
        ResponseHeader {
            version: None,
            action: None,
            sign: Some(sign),
            timestamp: None,
            host: None,
        }
    }
}

// Generic Response structure that wraps a state, header, and body
// T represents the type of the response body
#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Response<T> {
    // Optional state containing response status information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,

    // Optional header containing response metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<ResponseHeader>,

    // Optional body containing the actual response data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,
}

// Implementation of Response methods
#[cfg(feature = "proto")]
impl<T> Response<T> {
    // Creates a new Response with the given body value
    pub fn new(value: Option<T>) -> Self {
        Response {
            state: None,
            head: None,
            body: value,
        }
    }

    // Creates a new Response with a state containing the provided code and message
    pub fn new_with_state(code: u32, msg: &str) -> Self {
        let state = State {
            ret_code: code,
            ret_message: Some(msg.to_string()),
        };

        Response {
            state: Some(state),
            head: None,
            body: None,
        }
    }

    // Sets the response state to indicate a request error with the provided code and message
    pub fn raiseRequestError(&mut self, code: u32, msg: &str) {
        self.state = Some(State {
            ret_code: code,
            ret_message: Some(msg.to_string()),
        });
    }
}

// Unit tests for the proto module
#[cfg(test)]
#[cfg(feature = "proto")]
mod tests {
    use super::*;

    // Test serialization of a Request with a numeric body
    #[test]
    fn test_request_json() {
        let req = Request::<u32>::new(Some(1));
        assert_eq!(serde_json::to_string(&req).is_ok(), true);
    }

    // Test validation of a Request with a header containing a signature
    #[test]
    fn test_request_validate() {
        let mut req = Request::<u32>::new(Some(1));
        req.head = Some(RequestHeader::new_with_sign("1".to_string()));
        println!("serialize:{}", serde_json::to_string(&req).unwrap());
        assert_eq!(req.validate().is_ok(), true);
    }

    // Test deserialization of various Request formats
    #[test]
    fn test_deserialize() {
        use serde_derive::Deserialize;

        let data = r###"{}"###;
        let obj = serde_json::from_str::<Request<String>>(data);
        println!("obj1->{:?}", obj);

        let data = r###"{"body":"1234"}"###;
        let obj = serde_json::from_str::<Request<String>>(data);
        println!("obj2->{:?}", obj);

        #[derive(Deserialize, Debug)]
        struct Item {
            AppId: String,
        }

        let data = r###"{"body":{"AppId":"a1"}}"###;
        let obj3 = serde_json::from_str::<Request<Item>>(data);
        println!("obj3->{:?}", obj3);
    }

    // Test RequestHeader::new_with_sign method
    #[test]
    fn test_request_header_new_with_sign() {
        let sign = "test_signature".to_string();
        let header = RequestHeader::new_with_sign(sign.clone());
        
        assert_eq!(header.version, None);
        assert_eq!(header.action, None);
        assert_eq!(header.sign, Some(sign));
        assert_eq!(header.timestamp, None);
        assert_eq!(header.sender, None);
    }

    // Test RequestHeader::new_with_sign_action method
    #[test]
    fn test_request_header_new_with_sign_action() {
        let sign = "test_signature".to_string();
        let action = "test_action".to_string();
        let header = RequestHeader::new_with_sign_action(sign.clone(), action.clone());
        
        assert_eq!(header.version, None);
        assert_eq!(header.action, Some(action));
        assert_eq!(header.sign, Some(sign));
        assert_eq!(header.timestamp, None);
        assert_eq!(header.sender, None);
    }

    // Test Request::new method
    #[test]
    fn test_request_new() {
        let req = Request::<String>::new(Some("test_body".to_string()));
        assert_eq!(req.head, None);
        assert_eq!(req.body, Some("test_body".to_string()));
        
        let req_empty = Request::<String>::new(None);
        assert_eq!(req_empty.head, None);
        assert_eq!(req_empty.body, None);
    }

    // Test Request::validate with valid header and signature
    #[test]
    fn test_request_validate_success() {
        let mut req = Request::<u32>::new(Some(1));
        req.head = Some(RequestHeader::new_with_sign("valid_signature".to_string()));
        assert_eq!(req.validate().is_ok(), true);
    }

    // Test Request::validate with missing header
    #[test]
    fn test_request_validate_missing_header() {
        let req = Request::<u32>::new(Some(1));
        // head is None
        assert_eq!(req.validate().is_err(), true);
        assert_eq!(req.validate().unwrap_err().to_string(), "sign data is required！");
    }

    // Test Request::validate with missing signature
    #[test]
    fn test_request_validate_missing_signature() {
        let mut req = Request::<u32>::new(Some(1));
        req.head = Some(RequestHeader {
            version: None,
            action: None,
            sign: None,  // Missing signature
            timestamp: None,
            sender: None,
        });
        assert_eq!(req.validate().is_err(), true);
        assert_eq!(req.validate().unwrap_err().to_string(), "sign data is required！");
    }

    // Test Request::validate with empty signature
    #[test]
    fn test_request_validate_empty_signature() {
        let mut req = Request::<u32>::new(Some(1));
        req.head = Some(RequestHeader {
            version: None,
            action: None,
            sign: Some("".to_string()),  // Empty signature
            timestamp: None,
            sender: None,
        });
        assert_eq!(req.validate().is_ok(), true);  // Empty string is still Some(String)
    }

    // Test Request::validate with whitespace-only signature
    #[test]
    fn test_request_validate_whitespace_signature() {
        let mut req = Request::<u32>::new(Some(1));
        req.head = Some(RequestHeader {
            version: None,
            action: None,
            sign: Some("   ".to_string()),  // Whitespace-only signature
            timestamp: None,
            sender: None,
        });
        assert_eq!(req.validate().is_ok(), true);  // Whitespace string is still Some(String)
    }

    // Test Request::validate error message formatting
    #[test]
    fn test_request_validate_error_message() {
        let req = Request::<u32>::new(Some(1));
        let result = req.validate();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "sign data is required！");
    }

    // Test Response::new method
    #[test]
    fn test_response_new() {
        let resp = Response::<String>::new(Some("test_body".to_string()));
        assert_eq!(resp.state, None);
        assert_eq!(resp.head, None);
        assert_eq!(resp.body, Some("test_body".to_string()));
        
        let resp_empty = Response::<String>::new(None);
        assert_eq!(resp_empty.state, None);
        assert_eq!(resp_empty.head, None);
        assert_eq!(resp_empty.body, None);
    }

    // Test Response::new_with_state method
    #[test]
    fn test_response_new_with_state() {
        let resp = Response::<String>::new_with_state(200, "Success");
        assert!(resp.state.is_some());
        assert_eq!(resp.state.unwrap().ret_code, 200);
        assert_eq!(resp.head, None);
        assert_eq!(resp.body, None);
    }

    // Test Response::new_with_state with empty message
    #[test]
    fn test_response_new_with_state_empty_message() {
        let resp = Response::<String>::new_with_state(404, "");
        assert!(resp.state.is_some());
        let state = resp.state.unwrap();
        assert_eq!(state.ret_code, 404);
        assert_eq!(state.ret_message, Some("".to_string()));
        assert_eq!(resp.head, None);
        assert_eq!(resp.body, None);
    }

    // Test Response::new_with_state with special characters in message
    #[test]
    fn test_response_new_with_state_special_message() {
        let special_msg = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
        let resp = Response::<String>::new_with_state(500, special_msg);
        assert!(resp.state.is_some());
        let state = resp.state.unwrap();
        assert_eq!(state.ret_code, 500);
        assert_eq!(state.ret_message, Some(special_msg.to_string()));
        assert_eq!(resp.head, None);
        assert_eq!(resp.body, None);
    }

    // Test Response::raiseRequestError method
    #[test]
    fn test_response_raise_request_error() {
        let mut resp = Response::<String>::new(Some("test_body".to_string()));
        resp.raiseRequestError(404, "Not Found");
        
        assert!(resp.state.is_some());
        let state = resp.state.unwrap();
        assert_eq!(state.ret_code, 404);
        assert_eq!(state.ret_message, Some("Not Found".to_string()));
    }

    // Test Response::raiseRequestError with empty message
    #[test]
    fn test_response_raise_request_error_empty() {
        let mut resp = Response::<String>::new(Some("test_body".to_string()));
        resp.raiseRequestError(400, "");
        
        assert!(resp.state.is_some());
        let state = resp.state.unwrap();
        assert_eq!(state.ret_code, 400);
        assert_eq!(state.ret_message, Some("".to_string()));
    }

    // Test Response::raiseRequestError with special characters
    #[test]
    fn test_response_raise_request_error_special_chars() {
        let special_msg = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
        let mut resp = Response::<String>::new(Some("test_body".to_string()));
        resp.raiseRequestError(500, special_msg);
        
        assert!(resp.state.is_some());
        let state = resp.state.unwrap();
        assert_eq!(state.ret_code, 500);
        assert_eq!(state.ret_message, Some(special_msg.to_string()));
    }

    // Test State serialization and deserialization
    #[test]
    fn test_state_serialization() {
        let state = State {
            ret_code: 200,
            ret_message: Some("OK".to_string()),
        };
        
        let json_str = serde_json::to_string(&state).unwrap();
        let deserialized_state: State = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(state.ret_code, deserialized_state.ret_code);
        assert_eq!(state.ret_message, deserialized_state.ret_message);
    }

    // Test State serialization with None message
    #[test]
    fn test_state_serialization_none_message() {
        let state = State {
            ret_code: 404,
            ret_message: None,
        };
        
        let json_str = serde_json::to_string(&state).unwrap();
        // When ret_message is None, it should be omitted from JSON due to skip_serializing_if
        assert!(!json_str.contains("ret_message"));
        assert!(json_str.contains("ret_code"));
        
        let deserialized_state: State = serde_json::from_str(&json_str).unwrap();
        assert_eq!(state.ret_code, deserialized_state.ret_code);
        assert_eq!(state.ret_message, deserialized_state.ret_message);
    }

    // Test State serialization with empty message
    #[test]
    fn test_state_serialization_empty_message() {
        let state = State {
            ret_code: 500,
            ret_message: Some("".to_string()),
        };
        
        let json_str = serde_json::to_string(&state).unwrap();
        // When ret_message is Some("") it should be included in JSON
        assert!(json_str.contains("ret_message"));
        assert!(json_str.contains("\"\""));
        assert!(json_str.contains("ret_code"));
        
        let deserialized_state: State = serde_json::from_str(&json_str).unwrap();
        assert_eq!(state.ret_code, deserialized_state.ret_code);
        assert_eq!(state.ret_message, deserialized_state.ret_message);
    }

    // Test ResponseHeader::new_with_sign method
    #[test]
    fn test_response_header_new_with_sign() {
        let sign = "test_signature".to_string();
        let header = ResponseHeader::new_with_sign(sign.clone());
        
        assert_eq!(header.version, None);
        assert_eq!(header.action, None);
        assert_eq!(header.sign, Some(sign));
        assert_eq!(header.timestamp, None);
        assert_eq!(header.host, None);
    }

    // Test Request serialization with complex nested structures
    #[test]
    fn test_request_complex_serialization() {
        use serde_derive::Deserialize;

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct ComplexBody {
            id: u32,
            name: String,
            tags: Vec<String>,
        }

        let complex_body = ComplexBody {
            id: 123,
            name: "test".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        };

        let mut req = Request::new(Some(complex_body));
        req.head = Some(RequestHeader::new_with_sign_action(
            "signature".to_string(),
            "action".to_string()
        ));

        let json_str = serde_json::to_string(&req).unwrap();
        let deserialized_req: Request<ComplexBody> = serde_json::from_str(&json_str).unwrap();

        assert_eq!(req.body, deserialized_req.body);
        assert_eq!(req.head.as_ref().unwrap().sign, deserialized_req.head.as_ref().unwrap().sign);
    }

    // Test with empty strings and special characters
    #[test]
    fn test_request_with_special_characters() {
        let special_sign = "!@#$%^&*()_+-=[]{}|;':\",./<>?".to_string();
        let header = RequestHeader::new_with_sign(special_sign.clone());
        assert_eq!(header.sign, Some(special_sign));

        let unicode_string = "🚀🌟🦀".to_string();
        let req = Request::<String>::new(Some(unicode_string.clone()));
        assert_eq!(req.body, Some(unicode_string));
    }

    // Test Response serialization with state
    #[test]
    fn test_response_with_state_serialization() {
        let mut resp = Response::<String>::new(Some("response_body".to_string()));
        resp.state = Some(State {
            ret_code: 418,
            ret_message: Some("I'm a teapot".to_string()),
        });

        let json_str = serde_json::to_string(&resp).unwrap();
        let deserialized_resp: Response<String> = serde_json::from_str(&json_str).unwrap();

        assert_eq!(resp.body, deserialized_resp.body);
        assert_eq!(resp.state.as_ref().unwrap().ret_code, deserialized_resp.state.as_ref().unwrap().ret_code);
        assert_eq!(resp.state.as_ref().unwrap().ret_message, deserialized_resp.state.as_ref().unwrap().ret_message);
    }

    // Test Request with None body serialization
    #[test]
    fn test_request_none_body_serialization() {
        let req = Request::<String>::new(None);
        let json_str = serde_json::to_string(&req).unwrap();
        assert!(json_str.contains("\"body\":null") || !json_str.contains("body"));
        
        let deserialized_req: Request<String> = serde_json::from_str(&json_str).unwrap();
        assert_eq!(req.body, deserialized_req.body);
    }

    // Test Response with None body serialization
    #[test]
    fn test_response_none_body_serialization() {
        let resp = Response::<String>::new(None);
        let json_str = serde_json::to_string(&resp).unwrap();
        assert!(json_str.contains("\"body\":null") || !json_str.contains("body"));
        
        let deserialized_resp: Response<String> = serde_json::from_str(&json_str).unwrap();
        assert_eq!(resp.body, deserialized_resp.body);
    }

    // Test RequestHeader with all fields None except sign
    #[test]
    fn test_request_header_minimal_fields() {
        let header = RequestHeader {
            version: None,
            action: None,
            sign: Some("test_sign".to_string()),
            timestamp: None,
            sender: None,
        };
        
        assert_eq!(header.sign, Some("test_sign".to_string()));
        assert_eq!(header.version, None);
        assert_eq!(header.action, None);
        assert_eq!(header.timestamp, None);
        assert_eq!(header.sender, None);
    }

    // Test ResponseHeader with all fields None except sign
    #[test]
    fn test_response_header_minimal_fields() {
        let header = ResponseHeader {
            version: None,
            action: None,
            sign: Some("test_sign".to_string()),
            timestamp: None,
            host: None,
        };
        
        assert_eq!(header.sign, Some("test_sign".to_string()));
        assert_eq!(header.version, None);
        assert_eq!(header.action, None);
        assert_eq!(header.timestamp, None);
        assert_eq!(header.host, None);
    }

    // Test State with high ret_code values
    #[test]
    fn test_state_high_ret_code() {
        let state = State {
            ret_code: 999999,
            ret_message: Some("High code test".to_string()),
        };
        
        assert_eq!(state.ret_code, 999999);
        assert_eq!(state.ret_message, Some("High code test".to_string()));
    }
}
