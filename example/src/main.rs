mod test;

use oss_rust_sdk::oss::*;
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use std::str;
use base64::engine::general_purpose;
use tokio::runtime::Runtime;
use oss_rust_sdk::async_object::AsyncObjectAPI;

#[tokio::main]
async fn main() {
    // get_object();
    // async_get_object();
    // list_object();
    // async_list_object();
    // put_object().await;
    append_object().await;
}


async fn append_object() {
    let oss_instance = OSS::new(
        "your_AccessKeyId",
        "your_AccessKeySecret",
        "your_Endpoint",
        "your_Bucket",
    );

    let mut headers = HashMap::new();
    // headers.insert("content-type", "application/json");
    headers.insert("content-type", "text/plain");
    // headers.insert("Date", "Sun, 29 Sep 2024 09:01:00 GMT");

    let mut resources = HashMap::new();
    resources.insert("position", Some("0"));
    resources.insert("append", None);


    let result = oss_instance.append_object("123".as_bytes(),
                                         "segment/f.txt", headers, resources).await;
    println!("text = {:?}", result);
}


async fn put_object() {
    // use your own oss config
    let oss_instance = OSS::new(
        "your_AccessKeyId",
        "your_AccessKeySecret",
        "your_Endpoint",
        "your_Bucket",
    );

    let mut headers = HashMap::new();
    headers.insert("content-type", "application/json");

    let result = oss_instance.put_object("agc".as_bytes(),
                                         "segment/a.txt", headers, None).await;
    println!("text = {:?}", result.unwrap());
}

fn get_object() {
    use oss_rust_sdk::object::*;

    // use your own oss config
    let oss_instance = OSS::new(
        "your_AccessKeyId",
        "your_AccessKeySecret",
        "your_Endpoint",
        "your_Bucket",
    );

    let result = oss_rust_sdk::async_object::AsyncObjectAPI::get_object(&oss_instance, "objectName", None::<HashMap<&str, &str>>, None);
    // println!("text = {:?}", String::from_utf8(result.unwrap()));
}


pub fn base64_decoder(base64_string: String) -> String {
    let mut wrapped_reader = Cursor::new(base64_string);
    let mut decoder = base64::read::DecoderReader::new(
        &mut wrapped_reader,
        &general_purpose::STANDARD);

    // handle errors as you normally would
    let mut result = Vec::new();
    decoder.read_to_end(&mut result).unwrap();

    String::from_utf8_lossy(&*result).to_string()
}

fn async_get_object() {
    use oss_rust_sdk::async_object::*;

    // use your own oss config
    let oss_instance = OSS::new(
        "your_AccessKeyId",
        "your_AccessKeySecret",
        "your_Endpoint",
        "your_Bucket",
    );

    let mut rt = Runtime::new().expect("failed to start runtime");

    rt.block_on(async move {
        let buf = oss_instance
            .get_object("objectName", None::<HashMap<&str, &str>>, None)
            .await
            .unwrap();
        println!("buffer = {:?}", String::from_utf8(buf.to_vec()));
    });
}

fn async_list_object() {
    use oss_rust_sdk::async_object::*;

    // use your own oss config
    let oss_instance = OSS::new(
        "your_AccessKeyId",
        "your_AccessKeySecret",
        "your_Endpoint",
        "your_Bucket",
    );
    let mut params = HashMap::new();
    params.insert("max-keys", Some("5"));

    let mut rt = Runtime::new().expect("failed to start runtime");
    rt.block_on(async move {
        let result = oss_instance.list_object(None, params).await.unwrap();

        for object in result.contents() {
            dbg!(&object.key());
        }
    });
}

fn list_object() {
    use oss_rust_sdk::object::*;

    // use your own oss config
    let oss_instance = OSS::new(
        "your_AccessKeyId",
        "your_AccessKeySecret",
        "your_Endpoint",
        "your_Bucket",
    );

    let mut params = HashMap::new();
    params.insert("max-keys", Some("5"));
    let result = oss_rust_sdk::object::ObjectAPI::list_object(&oss_instance, None, params).unwrap();

    for object in result.contents() {
        dbg!(&object.key());
    }
}
