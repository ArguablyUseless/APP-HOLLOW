
//use std::thread;
//use surrealdb::sql::Value;

use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::{Serialize, Deserialize};

use surrealdb::sql::Value;
use std::time::SystemTime;
use chrono;

//use geocoding::{Opencage, Reverse, Point};

use crate::database::{get_database};
//use crate::get_address;

#[get("/all")]
pub async fn all_request() -> Json<Vec<Value>> {
	let db = get_database();

	let res = db.query("SELECT * FROM payload, device", None).await.unwrap();

	Json(res)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                    py-saxion sensor request                                    //
////////////////////////////////////////////////////////////////////////////////////////////////////
#[get("/py-saxion")]
pub async fn py_saxion_request() -> Json<Data> {
    let db = get_database(); // The get_database() function can be used to get a reference to the database

    let timestamp_data = db
        .query("SELECT * FROM time::now() LIMIT 1;", None)
        .await
        .unwrap();

    let timestamp_data_temp = json!(db.query("SELECT timestamp FROM device:`py-saxion`;", None).await.unwrap());

    println!("{}", timestamp_data_temp);

    let timestamp_data = timestamp_data_temp
	.as_array()
	.unwrap()[0]
	.as_array()
	.unwrap()[0]
	.get("timestamp")
	.unwrap()
	.as_str()
	.unwrap();

    let location_data = json!(db
        .query("SELECT latitude, longitude FROM location WHERE device = 'device:⟨py-saxion⟩';", None)
        .await
        .unwrap());

    let py_saxion_latitude = (location_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("latitude")
        .unwrap()
        .as_str()
        .unwrap();

    let py_saxion_longitude = (location_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("longitude")
        .unwrap()
        .as_str()
        .unwrap();

    let location = [py_saxion_latitude.to_string(), py_saxion_longitude.to_string()];

    let temperature_past_data = json!(db
        .query("SELECT indoor_temperature FROM payload WHERE device = 'device:⟨py-saxion⟩' LIMIT 3;", None)
        .await
        .unwrap());
    println!("{:?}",temperature_past_data);

    let temperature_past_json = (temperature_past_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap();
                
	let mut temperature_past = [String::new(), String::new()];
	let mut i = 0;
	for temp in temperature_past_json.iter().rev() {
		temperature_past[i] = temp.get("indoor_temperature").unwrap().as_f64().unwrap().to_string();
		i += 1;	
		if i == 2 { break; }
	}

    /*let temperature_past_data_2 = json!(db
        .query("SELECT indoor_temperature FROM payload WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_past_2 = (temperature_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("indoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();

    let temperature_past = [temperature_past_2.to_string(), temperature_past_1.to_string()];
	*/

    let pressure_past_data_1 = json!(db
        .query("SELECT pressure FROM payload WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let pressure_past_1 = (pressure_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("pressure")
        .unwrap()
        .as_f64()
        .unwrap();

    let pressure_past_data_2 = json!(db
        .query("SELECT pressure FROM payload WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let pressure_past_2 = (pressure_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("pressure")
        .unwrap()
        .as_f64()
        .unwrap();

    let pressure_past = [pressure_past_2.to_string(), pressure_past_1.to_string()];


    let light_past_data_1 = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_past_1 = (light_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();

    let light_past_data_2 = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_past_2 = (light_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();

    let light_past = [light_past_2.to_string(), light_past_1.to_string()];


    let temperature_current_data = json!(db
        .query("SELECT indoor_temperature FROM payload WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_current = (temperature_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("indoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();


    let pressure_current_data = json!(db
        .query("SELECT pressure FROM payload WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let pressure_current = (pressure_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("pressure")
        .unwrap()
        .as_f64()
        .unwrap();


    let light_current_data = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_current = (light_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();


    let bandwidth_data = json!(db
        .query("SELECT bandwidth FROM settings WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let bandwidth = (bandwidth_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("bandwidth")
        .unwrap()
        .as_f64()
        .unwrap();


    let spreading_factor_data = json!(db
        .query("SELECT spreading_factor FROM settings WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let spreading_factor = (spreading_factor_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("spreading_factor")
        .unwrap()
        .as_str()
        .unwrap();


    let coding_rate_data = json!(db
        .query("SELECT coding_rate FROM settings WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let coding_rate = (coding_rate_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("coding_rate")
        .unwrap()
        .as_str()
        .unwrap();


    let frequency_data = json!(db
        .query("SELECT frequency FROM settings WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let frequency = (frequency_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("frequency")
        .unwrap()
        .as_str()
        .unwrap();


    let rssi_data = json!(db
        .query("SELECT rssi FROM settings WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let rssi = (rssi_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("rssi")
        .unwrap()
        .as_str()
        .unwrap();


    let channel_rssi_data = json!(db
        .query("SELECT channel_rssi FROM settings WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let channel_rssi = (channel_rssi_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("channel_rssi")
        .unwrap()
        .as_str()
        .unwrap();


    let snr_data = json!(db
        .query("SELECT snr FROM settings WHERE device = 'device:⟨py-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let snr = (snr_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("snr")
        .unwrap()
        .as_str()
        .unwrap();


    let data = Data{
        sensor_name: "py-saxion".to_string(),
        timestamp: timestamp_data.to_string(),
        color: "#FF965D".to_string(),
        location: LocationInfo{
            latitude_longitude: location,
            //address: address
        }, weather_past: PastWeatherInfo{
            temperature_past_2: temperature_past,
            pressure_past_2: pressure_past,
            humidity_past_2: ["0".to_string(),"0".to_string()],
            light_past_2: light_past,
        }, weather_current: CurrentWeatherInfo{
            temperature_current: temperature_current.to_string(),
            pressure_current: pressure_current.to_string(),
            humidity_current: "0".to_string(),
            light_current: light_current.to_string()
        },sensor: SensorInfo{
            source_id: "py-saxion".to_string(),
            bandwidth: bandwidth.to_string(),
            spreading_factor: spreading_factor.to_string(),
            coding_rate: coding_rate.to_string(),
            frequency: frequency.to_string(),
            rssi: rssi.to_string(),
            channel_rssi: channel_rssi.to_string(),
            snr: snr.to_string(),
            battery_voltage: "0".to_string()
        }
    };

    Json(data)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                    py-wierden sensor request                                    //
////////////////////////////////////////////////////////////////////////////////////////////////////
#[get("/py-wierden")]
pub async fn py_wierden_request() -> Json<Data> {
    let db = get_database(); // The get_database() function can be used to get a reference to the database

    let timestamp_data = db
        .query("SELECT * FROM time::now() LIMIT 1;", None)
        .await
        .unwrap();


    let location_data = json!(db
        .query("SELECT latitude, longitude FROM location WHERE device = 'device:⟨py-wierden⟩';", None)
        .await
        .unwrap());

    let latitude = (location_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("latitude")
        .unwrap()
        .as_str()
        .unwrap();

    let longitude = (location_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("longitude")
        .unwrap()
        .as_str()
        .unwrap();

    let location = [latitude.to_string(), longitude.to_string()];


    let temperature_past_data_1 = json!(db
        .query("SELECT indoor_temperature FROM payload WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_past_1 = (temperature_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("indoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();

    let temperature_past_data_2 = json!(db
        .query("SELECT indoor_temperature FROM payload WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());
    println!("{:?}",temperature_past_data_2);

    let temperature_past_2 = (temperature_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("indoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();

    let temperature_past = [temperature_past_1.to_string(), temperature_past_2.to_string()];


    let pressure_past_data_1 = json!(db
        .query("SELECT pressure FROM payload WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let pressure_past_1 = (pressure_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("pressure")
        .unwrap()
        .as_f64()
        .unwrap();

    let pressure_past_data_2 = json!(db
        .query("SELECT pressure FROM payload WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let pressure_past_2 = (pressure_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("pressure")
        .unwrap()
        .as_f64()
        .unwrap();

    let pressure_past = [pressure_past_2.to_string(), pressure_past_1.to_string()];


    let light_past_data_1 = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_past_1 = (light_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();

    let light_past_data_2 = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_past_2 = (light_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();

    let light_past = [light_past_2.to_string(), light_past_1.to_string()];


    let temperature_current_data = json!(db
        .query("SELECT indoor_temperature FROM payload WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_current = (temperature_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("indoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();


    let pressure_current_data = json!(db
        .query("SELECT pressure FROM payload WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let pressure_current = (pressure_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("pressure")
        .unwrap()
        .as_f64()
        .unwrap();


    let light_current_data = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_current = (light_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();


    let bandwidth_data = json!(db
        .query("SELECT bandwidth FROM settings WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let bandwidth = (bandwidth_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("bandwidth")
        .unwrap()
        .as_f64()
        .unwrap();


    let spreading_factor_data = json!(db
        .query("SELECT spreading_factor FROM settings WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let spreading_factor = (spreading_factor_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("spreading_factor")
        .unwrap()
        .as_str()
        .unwrap();


    let coding_rate_data = json!(db
        .query("SELECT coding_rate FROM settings WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let coding_rate = (coding_rate_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("coding_rate")
        .unwrap()
        .as_str()
        .unwrap();


    let frequency_data = json!(db
        .query("SELECT frequency FROM settings WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let frequency = (frequency_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("frequency")
        .unwrap()
        .as_str()
        .unwrap();


    let rssi_data = json!(db
        .query("SELECT rssi FROM settings WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let rssi = (rssi_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("rssi")
        .unwrap()
        .as_str()
        .unwrap();


    let channel_rssi_data = json!(db
        .query("SELECT channel_rssi FROM settings WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let channel_rssi = (channel_rssi_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("channel_rssi")
        .unwrap()
        .as_str()
        .unwrap();


    let snr_data = json!(db
        .query("SELECT snr FROM settings WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let snr = (snr_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("snr")
        .unwrap()
        .as_str()
        .unwrap();


    let data = Data{
        sensor_name: "py-wierden".to_string(),
        timestamp: timestamp_data[0].to_string(),
        color: "#FFE75D".to_string(),
        location: LocationInfo{
            latitude_longitude: location,
            //address: address
        }, weather_past: PastWeatherInfo{
            temperature_past_2: temperature_past,
            pressure_past_2: pressure_past,
            humidity_past_2: ["0".to_string(),"0".to_string()],
            light_past_2: light_past,
        }, weather_current: CurrentWeatherInfo{
            temperature_current: temperature_current.to_string(),
            pressure_current: pressure_current.to_string(),
            humidity_current: "0".to_string(),
            light_current: light_current.to_string()
        },sensor: SensorInfo{
            source_id: "py-wierden".to_string(),
            bandwidth: bandwidth.to_string(),
            spreading_factor: spreading_factor.to_string(),
            coding_rate: coding_rate.to_string(),
            frequency: frequency.to_string(),
            rssi: rssi.to_string(),
            channel_rssi: channel_rssi.to_string(),
            snr: snr.to_string(),
            battery_voltage: "0".to_string()
        }
    };

    Json(data)
}


////////////////////////////////////////////////////////////////////////////////////////////////////
//                                    lht-wierden sensor request                                  //
////////////////////////////////////////////////////////////////////////////////////////////////////
#[get("/lht-wierden")]
pub async fn lht_wierden_request() -> Json<Data> {
    let db = get_database(); // The get_database() function can be used to get a reference to the database

    let timestamp_data = db
        .query("SELECT * FROM time::now() LIMIT 1;", None)
        .await
        .unwrap();


    let location_data = json!(db
        .query("SELECT latitude, longitude FROM location WHERE device = 'device:⟨lht-wierden⟩';", None)
        .await
        .unwrap());

    let latitude = (location_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("latitude")
        .unwrap()
        .as_str()
        .unwrap();

    let longitude = (location_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("longitude")
        .unwrap()
        .as_str()
        .unwrap();

    let location = [latitude.to_string(), longitude.to_string()];


    let temperature_past_data_1 = json!(db
        .query("SELECT outdoor_temperature FROM payload WHERE device = 'device:⟨lht-wierden⟩' AND (time::now() - 5m < timestamp < time::now()) LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_past_1 = (temperature_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("outdoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();

    let temperature_past_data_2 = json!(db
        .query("SELECT outdoor_temperature FROM payload WHERE device = 'device:⟨lht-wierden⟩' AND (time::now() - 10m < timestamp < time::now() - 5m) LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_past_2 = (temperature_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("outdoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();

    let temperature_past = [temperature_past_2.to_string(), temperature_past_1.to_string()];


    let humidity_past_data_1 = json!(db
        .query("SELECT humidity FROM payload WHERE device = 'device:⟨lht-wierden⟩' AND (time::now() - 1h < timestamp < time::now()) LIMIT 1;", None)
        .await
        .unwrap());

    let humidity_past_1 = (humidity_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("humidity")
        .unwrap()
        .as_f64()
        .unwrap();

    let humidity_past_data_2 = json!(db
        .query("SELECT humidity FROM payload WHERE device = 'device:⟨lht-wierden⟩' AND (time::now() - 1h < timestamp < time::now()) LIMIT 1;", None)
        .await
        .unwrap());

    let humidity_past_2 = (humidity_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("humidity")
        .unwrap()
        .as_f64()
        .unwrap();

    let humidity_past = [humidity_past_2.to_string(), humidity_past_1.to_string()];


    let light_past_data_1 = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_past_1 = (light_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();

    let light_past_data_2 = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_past_2 = (light_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();

    let light_past = [light_past_2.to_string(), light_past_1.to_string()];


    let temperature_current_data = json!(db
        .query("SELECT outdoor_temperature FROM payload WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_current = (temperature_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("outdoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();


    let humidity_current_data = json!(db
        .query("SELECT humidity FROM payload WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let humidity_current = (humidity_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("humidity")
        .unwrap()
        .as_f64()
        .unwrap();


    let light_current_data = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨py-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_current = (light_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();

    let bandwidth_data = json!(db
        .query("SELECT bandwidth FROM settings WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let bandwidth = (bandwidth_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("bandwidth")
        .unwrap()
        .as_f64()
        .unwrap();


    let spreading_factor_data = json!(db
        .query("SELECT spreading_factor FROM settings WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let spreading_factor = (spreading_factor_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("spreading_factor")
        .unwrap()
        .as_str()
        .unwrap();


    let coding_rate_data = json!(db
        .query("SELECT coding_rate FROM settings WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let coding_rate = (coding_rate_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("coding_rate")
        .unwrap()
        .as_str()
        .unwrap();


    let frequency_data = json!(db
        .query("SELECT frequency FROM settings WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let frequency = (frequency_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("frequency")
        .unwrap()
        .as_str()
        .unwrap();


    let rssi_data = json!(db
        .query("SELECT rssi FROM settings WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let rssi = (rssi_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("rssi")
        .unwrap()
        .as_str()
        .unwrap();


    let channel_rssi_data = json!(db
        .query("SELECT channel_rssi FROM settings WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let channel_rssi = (channel_rssi_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("channel_rssi")
        .unwrap()
        .as_str()
        .unwrap();


    let snr_data = json!(db
        .query("SELECT snr FROM settings WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let snr = (snr_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("snr")
        .unwrap()
        .as_str()
        .unwrap();

    let battery_voltage_data = json!(db
        .query("SELECT battery_voltage FROM settings WHERE device = 'device:⟨lht-wierden⟩' LIMIT 1;", None)
        .await
        .unwrap());

    println!("{}", battery_voltage_data);

    let battery_voltage = (battery_voltage_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("battery_voltage")
        .unwrap()
        .as_f64()
        .unwrap_or(0.0);


    let data = Data{
        sensor_name: "lht-wierden".to_string(),
        timestamp: timestamp_data[0].to_string(),
        color: "#C6FF5D".to_string(),
        location: LocationInfo{
            latitude_longitude: location,
            //address: address
        }, weather_past: PastWeatherInfo{
            temperature_past_2: temperature_past,
            pressure_past_2: ["0".to_string(),"0".to_string()],
            humidity_past_2: humidity_past,
            light_past_2: light_past,
        }, weather_current: CurrentWeatherInfo{
            temperature_current: temperature_current.to_string(),
            pressure_current: "0".to_string(),
            humidity_current: humidity_current.to_string(),
            light_current: light_current.to_string()
        },sensor: SensorInfo{
            source_id: "lht-wierden".to_string(),
            bandwidth: bandwidth.to_string(),
            spreading_factor: spreading_factor.to_string(),
            coding_rate: coding_rate.to_string(),
            frequency: frequency.to_string(),
            rssi: rssi.to_string(),
            channel_rssi: channel_rssi.to_string(),
            snr: snr.to_string(),
            battery_voltage: battery_voltage.to_string()
        }
    };

    Json(data)
}


////////////////////////////////////////////////////////////////////////////////////////////////////
//                                    lht-gronau sensor request                                   //
////////////////////////////////////////////////////////////////////////////////////////////////////
#[get("/lht-gronau")]
pub async fn lht_gronau_request() -> Json<Data> {
    let db = get_database(); // The get_database() function can be used to get a reference to the database

    let timestamp_data = db
        .query("SELECT * FROM time::now() LIMIT 1;", None)
        .await
        .unwrap();


    let location_data = json!(db
        .query("SELECT latitude, longitude FROM location WHERE device = 'device:⟨lht-gronau⟩';", None)
        .await
        .unwrap());

    let latitude = (location_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("latitude")
        .unwrap()
        .as_str()
        .unwrap();

    let longitude = (location_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("longitude")
        .unwrap()
        .as_str()
        .unwrap();

    let location = [latitude.to_string(), longitude.to_string()];


    let temperature_past_data_1 = json!(db
        .query("SELECT outdoor_temperature FROM payload WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_past_1 = (temperature_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("outdoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();

    let temperature_past_data_2 = json!(db
        .query("SELECT outdoor_temperature FROM payload WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_past_2 = (temperature_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("outdoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();

    let temperature_past = [temperature_past_1.to_string(), temperature_past_2.to_string()];


    let humidity_past_data_1 = json!(db
        .query("SELECT humidity FROM payload WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let humidity_past_1 = (humidity_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("humidity")
        .unwrap()
        .as_f64()
        .unwrap();

    let humidity_past_data_2 = json!(db
        .query("SELECT humidity FROM payload WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let humidity_past_2 = (humidity_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("humidity")
        .unwrap()
        .as_f64()
        .unwrap();

    let humidity_past = [humidity_past_2.to_string(), humidity_past_1.to_string()];


    let light_past_data_1 = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_past_1 = (light_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();

    let light_past_data_2 = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_past_2 = (light_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();

    let light_past = [light_past_2.to_string(), light_past_1.to_string()];


    let temperature_current_data = json!(db
        .query("SELECT outdoor_temperature FROM payload WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_current = (temperature_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("outdoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();


    let humidity_current_data = json!(db
        .query("SELECT humidity FROM payload WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let humidity_current = (humidity_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("humidity")
        .unwrap()
        .as_f64()
        .unwrap();


    let light_current_data = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

	println!("{}", light_current_data);

    let light_current = (light_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap();


    let bandwidth_data = json!(db
        .query("SELECT bandwidth FROM settings WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let bandwidth = (bandwidth_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("bandwidth")
        .unwrap()
        .as_f64()
        .unwrap();


    let spreading_factor_data = json!(db
        .query("SELECT spreading_factor FROM settings WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let spreading_factor = (spreading_factor_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("spreading_factor")
        .unwrap()
        .as_str()
        .unwrap();


    let coding_rate_data = json!(db
        .query("SELECT coding_rate FROM settings WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let coding_rate = (coding_rate_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("coding_rate")
        .unwrap()
        .as_str()
        .unwrap();


    let frequency_data = json!(db
        .query("SELECT frequency FROM settings WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let frequency = (frequency_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("frequency")
        .unwrap()
        .as_str()
        .unwrap();


    let rssi_data = json!(db
        .query("SELECT rssi FROM settings WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let rssi = (rssi_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("rssi")
        .unwrap()
        .as_str()
        .unwrap();


    let channel_rssi_data = json!(db
        .query("SELECT channel_rssi FROM settings WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let channel_rssi = (channel_rssi_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("channel_rssi")
        .unwrap()
        .as_str()
        .unwrap();


    let snr_data = json!(db
        .query("SELECT snr FROM settings WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let snr = (snr_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("snr")
        .unwrap()
        .as_str()
        .unwrap();

    let battery_voltage_data = json!(db
        .query("SELECT battery_voltage FROM settings WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let battery_voltage = (battery_voltage_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("battery_voltage")
        .unwrap()
        .as_f64()
        .unwrap();


    let data = Data{
        sensor_name: "lht-gronau".to_string(),
        timestamp: timestamp_data[0].to_string(),
        color: "#5DFFE7".to_string(),
        location: LocationInfo{
            latitude_longitude: location,
            //address: address
        }, weather_past: PastWeatherInfo{
            temperature_past_2: temperature_past,
            pressure_past_2: ["0".to_string(),"0".to_string()],
            humidity_past_2: humidity_past,
            light_past_2: light_past,
        }, weather_current: CurrentWeatherInfo{
            temperature_current: temperature_current.to_string(),
            pressure_current: "0".to_string(),
            humidity_current: humidity_current.to_string(),
            light_current: light_current.to_string()
        },sensor: SensorInfo{
            source_id: "lht-gronau".to_string(),
            bandwidth: bandwidth.to_string(),
            spreading_factor: spreading_factor.to_string(),
            coding_rate: coding_rate.to_string(),
            frequency: frequency.to_string(),
            rssi: rssi.to_string(),
            channel_rssi: channel_rssi.to_string(),
            snr: snr.to_string(),
            battery_voltage: battery_voltage.to_string()
        }
    };

    Json(data)
}


////////////////////////////////////////////////////////////////////////////////////////////////////
//                                    lht-saxion sensor request                                   //
////////////////////////////////////////////////////////////////////////////////////////////////////
#[get("/lht-saxion")]
pub async fn lht_saxion_request() -> Json<Data> {
    let db = get_database(); // The get_database() function can be used to get a reference to the database

    let timestamp_data = db
        .query("SELECT * FROM time::now() LIMIT 1;", None)
        .await
        .unwrap();


    let location_data = json!(db
        .query("SELECT latitude, longitude FROM location WHERE device = 'device:⟨lht-saxion⟩';", None)
        .await
        .unwrap());

    let latitude = (location_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("latitude")
        .unwrap()
        .as_str()
        .unwrap();

    let longitude = (location_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("longitude")
        .unwrap()
        .as_str()
        .unwrap();

    let location = [latitude.to_string(), longitude.to_string()];

    //let address = get_address(latitude.parse::<f64>().unwrap(), longitude.parse::<f64>().unwrap());


    let temperature_past_data_1 = json!(db
        .query("SELECT outdoor_temperature FROM payload WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());
    println!("{:?}",temperature_past_data_1);

    let temperature_past_1 = (temperature_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("outdoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();

    let temperature_past_data_2 = json!(db
        .query("SELECT outdoor_temperature FROM payload WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_past_2 = (temperature_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("outdoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();

    let temperature_past = [temperature_past_1.to_string(), temperature_past_2.to_string()];


    let humidity_past_data_1 = json!(db
        .query("SELECT humidity FROM payload WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let humidity_past_1 = (humidity_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("humidity")
        .unwrap()
        .as_f64()
        .unwrap();

    let humidity_past_data_2 = json!(db
        .query("SELECT humidity FROM payload WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let humidity_past_2 = (humidity_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("humidity")
        .unwrap()
        .as_f64()
        .unwrap();

    let humidity_past = [humidity_past_2.to_string(), humidity_past_1.to_string()];


    let light_past_data_1 = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

	println!("{}", light_past_data_1);

    let light_past_1 = (light_past_data_1)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap_or(0.0);

    let light_past_data_2 = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let light_past_2 = (light_past_data_2)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap_or(0.0);

    let light_past = [light_past_2.to_string(), light_past_1.to_string()];


    let temperature_current_data = json!(db
        .query("SELECT outdoor_temperature FROM payload WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let temperature_current = (temperature_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("outdoor_temperature")
        .unwrap()
        .as_f64()
        .unwrap();


    let humidity_current_data = json!(db
        .query("SELECT humidity FROM payload WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let humidity_current = (humidity_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("humidity")
        .unwrap()
        .as_f64()
        .unwrap();


    let light_current_data = json!(db
        .query("SELECT light FROM payload WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

	println!("{}", light_current_data);

    let light_current = (light_current_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("light")
        .unwrap()
        .as_f64()
        .unwrap_or(0.0);

    let bandwidth_data = json!(db
        .query("SELECT bandwidth FROM settings WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let bandwidth = (bandwidth_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("bandwidth")
        .unwrap()
        .as_f64()
        .unwrap();


    let spreading_factor_data = json!(db
        .query("SELECT spreading_factor FROM settings WHERE device = 'device:⟨lht-gronau⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let spreading_factor = (spreading_factor_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("spreading_factor")
        .unwrap()
        .as_str()
        .unwrap();


    let coding_rate_data = json!(db
        .query("SELECT coding_rate FROM settings WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let coding_rate = (coding_rate_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("coding_rate")
        .unwrap()
        .as_str()
        .unwrap();


    let frequency_data = json!(db
        .query("SELECT frequency FROM settings WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let frequency = (frequency_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("frequency")
        .unwrap()
        .as_str()
        .unwrap();


    let rssi_data = json!(db
        .query("SELECT rssi FROM settings WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let rssi = (rssi_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("rssi")
        .unwrap()
        .as_str()
        .unwrap();


    let channel_rssi_data = json!(db
        .query("SELECT channel_rssi FROM settings WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let channel_rssi = (channel_rssi_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("channel_rssi")
        .unwrap()
        .as_str()
        .unwrap();


    let snr_data = json!(db
        .query("SELECT snr FROM settings WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let snr = (snr_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("snr")
        .unwrap()
        .as_str()
        .unwrap();

    let battery_voltage_data = json!(db
        .query("SELECT battery_voltage FROM settings WHERE device = 'device:⟨lht-saxion⟩' LIMIT 1;", None)
        .await
        .unwrap());

    let battery_voltage = (battery_voltage_data)
        .as_array()
        .unwrap()[0]
        .as_array()
        .unwrap()[0]
        .get("battery_voltage")
        .unwrap()
        .as_f64()
        .unwrap();


    let data = Data{
        sensor_name: "lht-saxion".to_string(),
        timestamp: timestamp_data[0].to_string(),
        color: "#C25DFF".to_string(),
        location: LocationInfo{
            latitude_longitude: location,
            //address: address
        }, weather_past: PastWeatherInfo{
            temperature_past_2: temperature_past,
            pressure_past_2: ["0".to_string(),"0".to_string()],
            humidity_past_2: humidity_past,
            light_past_2: light_past,
        }, weather_current: CurrentWeatherInfo{
            temperature_current: temperature_current.to_string(),
            pressure_current: "0".to_string(),
            humidity_current: humidity_current.to_string(),
            light_current: light_current.to_string()
        },sensor: SensorInfo{
            source_id: "lht-saxion".to_string(),
            bandwidth: bandwidth.to_string(),
            spreading_factor: spreading_factor.to_string(),
            coding_rate: coding_rate.to_string(),
            frequency: frequency.to_string(),
            rssi: rssi.to_string(),
            channel_rssi: channel_rssi.to_string(),
            snr: snr.to_string(),
            battery_voltage: battery_voltage.to_string()
        }

    };

    Json(data)
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Data {
    pub sensor_name : String,
    pub timestamp : String,
    pub color: String,
    pub location: LocationInfo,
    pub weather_past: PastWeatherInfo,
    pub weather_current: CurrentWeatherInfo,
    pub sensor:SensorInfo
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LocationInfo{
    pub latitude_longitude: [String; 2],
    //pub address: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CurrentWeatherInfo {
    pub temperature_current: String,
    pub pressure_current: String,
    pub humidity_current: String,
    pub light_current: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PastWeatherInfo {
    pub temperature_past_2: [String; 2],
    pub pressure_past_2: [String; 2],
    pub humidity_past_2: [String; 2],
    pub light_past_2: [String; 2]
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SensorInfo {
    pub source_id: String,
    pub bandwidth: String,
    pub spreading_factor: String,
    pub coding_rate: String,
    pub frequency: String,
    pub rssi: String,
    pub channel_rssi: String,
    pub snr: String,
    pub battery_voltage: String
}
