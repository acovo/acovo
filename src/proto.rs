// Conditional compilation for the proto feature
#[cfg(feature = "proto")]
use anyhow::{anyhow, Result};
#[cfg(feature = "proto")]
use serde_derive::{Deserialize, Serialize};

// RequestHeader represents the header structure for API requests
// It contains metadata about the request such as version, action, signature, timestamp and sender
#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
}
