
extern crate reqwest;

extern crate chrono;
use chrono::Utc;

extern crate sha2;

extern crate ring;
extern crate data_encoding;
extern crate hex;

use ring::{digest, hmac};
use data_encoding::BASE64;

extern crate base64;

use std::str;
use base64::decode;

extern crate serde;
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[allow(dead_code)] //get rid of this if needed
pub struct Log{
    input: String,
    table: String,
    check_type: String, //type of check being done by the rack manager
    customer: String,
    key: String,
}

struct Test{
    test1: String,
    test2: String,
}

impl Serialize for Log {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 5 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Log", 4)?;
        state.serialize_field("table", &self.table)?;
        state.serialize_field("check_type", &self.check_type)?;
        state.serialize_field("customer", &self.customer)?;
        state.serialize_field("key", &self.key)?;
        state.end()
    }
}

impl Serialize for Test {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 5 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Tester", 2)?;
        state.serialize_field("test1", &self.test1)?;
        state.serialize_field("test2", &self.test2)?;
        state.end()
    }
}

fn main() {

    let tester = Test{
        test1: String::from("testing"),
        test2: String::from("testing2"),
    };

    let serialized = serde_json::to_string(&tester).unwrap();
    println!("Serialized: {}", serialized); 
    
    //sample input used for 
    let payload = Log {
        input: serialized,
        table: String::from("rust"),
        check_type: String::from("new"),
        customer: String::from("20a9a17e-baf7-4b8c-a8fc-f2efd151a240"),
        key: String::from(env::var("SHARED_KEY_LOGANALYTICS").unwrap()),
    };
    
    let event_count = 4;

    send_to_log_analytics(payload, event_count);

}


fn send_to_log_analytics(l: Log, event_count: usize) -> bool{

    let now = Utc::now(); //gets current time in ISO 8601
    let date = now.format("%a, %e %b %Y %T GMT"); //format to RFC1123
    let date_string = format!("{}", date);

    let json_bytes = l.input.as_bytes(); //use String.into_bytes() to consume a String object
    let string_to_hash = format!("POST\n{}\napplication/json\nx-ms-date:{}\n/api/logs", json_bytes.len(), date_string);
    
    let hashed_string = build_signature(string_to_hash, l.key);
    let signature = format!("SharedKey {}:{}", l.customer, hashed_string);
    
    let ret = post_data(signature, date_string, l.input, l.customer, l.table);

    match ret{
        Err(e) =>{
                match e.status(){
                    Some(t) => println!("Error: {}", t),
                    None => (),
                }
            },       
        _ => (),
    }

    println!{"EventsSent: {}", event_count};

    true
}


fn build_signature(message: String, secret: String) -> String{

    let key_bytes = decode(&secret).unwrap(); //decode to base-64
    let key_bytes = &key_bytes[..]; //take the bytes of the base-64 secret
    let message_bytes  = message.as_bytes(); 

    let signed_key = hmac::SigningKey::new(&digest::SHA256, key_bytes);
    let signature = hmac::sign(&signed_key, message_bytes);
    let b64_encoded_sig = BASE64.encode(signature.as_ref());

    b64_encoded_sig

}

fn post_data(signature: String, date: String, we_json: String, customer_id: String, log_name: String) -> Result<(), reqwest::Error>{

    let url: &str = &format!("https://{}.ods.opinsights.azure.com/api/logs?api-version=2016-04-01", customer_id);

    // let client = reqwest::Client::new();

    // for use with fiddler proxy
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://localhost:8888")?)
        .build()?;

    let mut res = client.post(url)
            .header("Accept", "application/json")
            .header("Content-Type","application/json")
            .header("Log-Type", log_name)
            .header("Authorization", signature)  
            .header("x-ms-date", date)  
            .body(we_json).send()?;
    
    let body = res.text()?;
    println!("{}", body);
    Ok(())

}
