#[cfg(feature = "proto")]
use anyhow::{anyhow, Result};
#[cfg(feature = "proto")]
use serde_derive::{Deserialize, Serialize};


#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestHeader{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
}
#[cfg(feature = "proto")]
impl RequestHeader {
    pub fn new_with_sign(sign:String) -> Self {
        RequestHeader {
            version:None,
            action:None,
            sign:Some(sign),
            timestamp:None,
            sender:None
        }
    }
}

#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Request<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head:Option<RequestHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,
}

#[cfg(feature = "proto")]
impl<T> Request<T> {
    pub fn new(value: Option<T>) -> Self {
        Request {
            head:None,
            body: value,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if *&self.head.is_none() || *(&self.head.as_ref().unwrap().sign.is_none()){
            return Err(anyhow!("sign data is required！"));
        }else {
            return Ok(());
        }
    }
}


#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub ret_code: u32,
    pub ret_message: String,
}

#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseHeader{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

#[cfg(feature = "proto")]
impl ResponseHeader {
    pub fn new_with_sign(sign:String) -> Self {
        ResponseHeader {
            version:None,
            action:None,
            sign:Some(sign),
            timestamp:None,
            host:None,
        }
    }
}

#[cfg(feature = "proto")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<ResponseHeader>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<T>,
}

#[cfg(feature = "proto")]
impl<T> Response<T> {
    pub fn new(value: Option<T>) -> Self {
        Response {
            state: None,
            head:None,
            body: value,
        }
    }
    pub fn raiseRequestError(&mut self, code: u32, msg: &str) {
        self.state = Some(State{
            ret_code:code,
            ret_message:msg.to_string()
        });
    }
}



#[cfg(test)]
#[cfg(feature = "proto")]
mod tests {
    use super::*;
    use serde_json::*;

    #[test]
    fn test_request_json() {
        let req= Request::<u32>::new(Some(1));
        assert_eq!(serde_json::to_string(&req).is_ok(),true);
    }

    #[test]
    fn test_request_validate() {
        let mut req= Request::<u32>::new(Some(1));
        req.head = Some(RequestHeader::new_with_sign("1".to_string()));
        println!("serialize:{}",serde_json::to_string(&req).unwrap());
        assert_eq!(req.validate().is_ok(),true);
    }
}