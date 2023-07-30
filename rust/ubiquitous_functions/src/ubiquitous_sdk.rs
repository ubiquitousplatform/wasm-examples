use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvocationContext {
    pub namespace: String,
    pub action: String,
    pub payload: serde_json::Value,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OkResponse {
    pub ok: bool,
    pub payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    pub id: String,
    pub level: String,
    pub message: String,
    pub trace_parent: String,
    pub span_id: String,
    pub props: serde_json::Value,
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailSendRequest {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub source_template: String,
    pub props: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailSendResponse {
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailSendError {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailSendResult {
    pub success: bool,
    pub response: Option<EmailSendResponse>,
    pub error: Option<EmailSendError>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExampleUserProps {
    pub name: String,
    pub age: i32,
    pub address: EmailSendRequestPropsAddress,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailSendRequestPropsAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}


pub fn invoke_json (log: Log) -> Result<OkResponse, Box<dyn std::error::Error>> {
let json_string =  serde_json::to_string(&log)?;


let size = json_string.len() as i32;
let ptr = json_string.as_ptr();
std::mem::forget(ptr);

unsafe {
    invoke_json(ptr as i32, size);
}

println!("Invoke_json called!");

let mem_size = unsafe { get_response_size() };

let mut buf: Vec<u8> = Vec::with_capacity(mem_size as usize);
let ptr = buf.as_mut_ptr();
std::mem::forget(ptr);

let response_buffer = unsafe {
    get_response(ptr as i32);
    Vec::from_raw_parts(ptr, mem_size as usize, mem_size as usize)
};

println!("response_buffer = {:?}", response_buffer);

let response: OkResponse = serde_json::from_slice(&response_buffer).map_err(|e| {
    eprintln!("ser: {e}");
    e
})?;
}


/* 
{ 
    "namespace": "log",
    "action": "info",
    "payload": {
        "message": "Hello World",
        "correlationId": ""
        "props": {
            "name": "Bob",
            "age": 42,
            "address": {
                "street": "123 Main St",
                "city": "Anytown",
                "state": "CA",
                "zip": "12345"
            }
        }
    }
    
}*/


/* 
{ 
    "namespace": "email",
    "action": "send",
    "payload": {
        "to": "example@email.com",
        "from": "bob@loblaw.com",
        "subject": "Hello World",
        "sourceTemplate": "email-template-1",
        "props": {
            "name": "Bob",
            "age": 42,
            "address": {
                "street": "123 Main St",
                "city": "Anytown",
                "state": "CA",
                "zip": "12345"
            }
        }
    }
}
*/