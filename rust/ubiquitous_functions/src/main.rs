use std::error::Error;

use ubiquitous_sdk::{Log,OkResponse};

use serde_json;

mod wire;

mod ubiquitous_sdk;

#[link(wasm_import_module = "ubiquitous_functions")]
extern "C" {
    fn get_response_size() -> i32;// fn get_input_size() -> i32;
    fn get_response(ptr: i32);// fn get_input(ptr: i32);
    fn invoke_json(ptr: i32, size: i32);// fn set_output(ptr: i32, size: i32);
    fn invoke_msgpack(ptr: i32, size: i32);// fn set_output(ptr: i32, size: i32);
}

fn main() -> Result<(), Box<dyn Error>> {

    println!("Hello from Rust WASM! (stdout)");

    println!("Calling log function with JSON...");

    // TODO: pass version of API expected? 
    let log = Log {
        level: "info".to_string(),
        message: "Hello, world!".to_string(),
        trace_parent: "trace_parent".to_string(),
        span_id: "span_id".to_string(),
        props: serde_json::json!({
            "name": "John Doe",
            "age": 43,
            "address": {
                "street": "123 Main Street",
                "city": "Anytown",
                "state": "CA",
                "zip": "12345"
            }
        }),
        id:"test".to_string(),
    };

   

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

    println!("response = {:?}", response);


/*
    let mem_size = unsafe { get_input_size() };

    let mut buf: Vec<u8> = Vec::with_capacity(mem_size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(ptr);

    let input_buf = unsafe {
        get_input(ptr as i32);
        Vec::from_raw_parts(ptr, mem_size as usize, mem_size as usize)
    };

    println!("input_buf = {:?}", input_buf);

    let input: Input = serde_json::from_slice(&input_buf).map_err(|e| {
        eprintln!("ser: {e}");
        e
    })?;

    println!("input = {:?}", input);

    let names: Vec<String> = (0..input.num).map(|_idx| input.name.clone()).collect();

    let output = Output { names };
    let serialized = serde_json::to_vec(&output).map_err(|e| {
        eprintln!("de: {e}");
        e
    })?;
    let size = serialized.len() as i32;
    let ptr = serialized.as_ptr();
    std::mem::forget(ptr);

    unsafe {
        set_output(ptr as i32, size);
    } */

    Ok(())
}
