use geocoding::{Opencage, Reverse, Point};
//use reqwest::Error;


#[tokio::main]
pub async fn get_address(latitude: f64, longitude: f64) -> String{
    let p = Point::new(latitude, longitude);
    let oc = Opencage::new("10b0bf39aab44ce2b0b0ef0710a28321".to_string());
    let res = oc.reverse(&p);
    let mut address = String::new();
    address.push_str(res.unwrap().unwrap().as_str());
    address
}

