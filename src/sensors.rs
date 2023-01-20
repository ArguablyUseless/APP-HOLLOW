use std::{process, time::Duration};

use paho_mqtt as mqtt;
use rocket::{futures::{executor::block_on, StreamExt}, serde::json::{Value, self}};

use crate::database::get_database;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                       MQTT Configuration                                       //
////////////////////////////////////////////////////////////////////////////////////////////////////

const HOST: &str = "eu1.cloud.thethings.network:1883";
const CLIENT_ID: &str = "client";

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Topics                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

const PY_SAXION_TOPIC: &str = "v3/project-software-engineering@ttn/devices/py-saxion/up";
const LHT_SAXION_TOPIC: &str = "v3/project-software-engineering@ttn/devices/lht-saxion/up";
const PY_WIERDEN_TOPIC: &str = "v3/project-software-engineering@ttn/devices/py-wierden/up";
const LHT_WIERDEN_TOPIC: &str = "v3/project-software-engineering@ttn/devices/lht-wierden/up";
const LHT_GRONAU_TOPIC: &str = "v3/project-software-engineering@ttn/devices/lht-gronau/up";

const TOPICS: &[&str] = &[
    PY_SAXION_TOPIC,
    LHT_SAXION_TOPIC,

    PY_WIERDEN_TOPIC,
    LHT_WIERDEN_TOPIC,

    LHT_GRONAU_TOPIC
];
const QOS: &[i32] = &[1, 1, 1, 1, 1];

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                       Start MQTT Listener                                      //
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Start the MQTT Listener and subscribe to the defined topics.
/// This function is blocking and should be called on the different state
pub fn start_mqtt() {
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .server_uri(HOST)
        .client_id(CLIENT_ID)
        .finalize();

    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        eprintln!("[Error]: Failed to create client '{:?}'", e);
        process::exit(1);
    });

    if let Err(err) = block_on(async {
        let mut strm = cli.get_stream(25);

        let lwt = mqtt::Message::new("test", "Async subscriber lost connection", mqtt::QOS_1);

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(30))
            .clean_session(false)
            .will_message(lwt)
            .password("NNSXS.DTT4HTNBXEQDZ4QYU6SG73Q2OXCERCZ6574RVXI.CQE6IG6FYNJOO2MOFMXZVWZE4GXTCC2YXNQNFDLQL4APZMWU6ZGA")
            .user_name("project-software-engineering@ttn")
            .finalize();

        // Make the connection to the broker
        println!("[Info]: Connecting to the MQTT server...");
        cli.connect(conn_opts).await?;

        println!("[Info]: Subscribing to topics: {:?}", TOPICS);
        cli.subscribe_many(TOPICS, QOS).await?;

        // Just loop on incoming messages.
        println!("[Info]: Connected successfully, waiting for messages...");

        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                match msg.topic() {
                    PY_SAXION_TOPIC => {
                        if let Some(res) = decode_py_saxion(&*msg.payload_str()) {
                            push_to_db(res).await;
                        } else {
                            eprintln!("[Error]: Failed to decode py-saxion's data");
                        }
                    },
                    LHT_SAXION_TOPIC => {
                        if let Some(res) = decode_lht_saxion(&*msg.payload_str()) {
                            push_to_db(res).await;
                        } else {
                            eprintln!("[Error]: Failed to decode lht-saxion's data");
                        }
                    },
                    PY_WIERDEN_TOPIC => {
                        if let Some(res) = decode_py_wierden(&*msg.payload_str()) {
                            push_to_db(res).await;
                        } else {
                            eprintln!("[Error]: Failed to decode py-wierden's data");
                        }
                    },
                    LHT_WIERDEN_TOPIC => {
                        if let Some(res) = decode_lht_wierden(&*msg.payload_str()) {
                            push_to_db(res).await;
                        } else {
                            eprintln!("[Error]: Failed to decode lht-wierden's data");
                        }
                    },
                    LHT_GRONAU_TOPIC => {
                        if let Some(res) = decode_lht_gronau(&*msg.payload_str()) {
                            push_to_db(res).await;
                        } else {
                            eprintln!("[Error]: Failed to decode lht-gronau's data");
                        }
                    },
                    topic => println!("[Warning]: Unexpected message from '{}'", topic)
                }
            } else {
                eprintln!("[Error]: Lost connection. Attempting reconnect.");
                while let Err(err) = cli.reconnect().await {
                    eprintln!("[Error]: Failed reconnection: {}", err);
                    async_std::task::sleep(Duration::from_millis(1000)).await;
                }
            }
        }

        // Explicit return for the async block
        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    };

}

/// Push the [`DBMessage`] to the Database
async fn push_to_db(msg: DBMessage) {
    let db = get_database();

    // Update the device table if needed
    db.query(format!(
        "UPDATE device:`{}` SET name = {:?}, description = {:?}, timestamp = {:?}",
        msg.device.id,
        msg.device.name,
        msg.device.description,
        msg.device.timestamp
    ).as_str(), None).await.unwrap();

    // Add the payload to the payload table
    db.query(format!(
        "CREATE payload SET device = device:`{}`, light = {}, pressure = {}, humidity = {}, indoor_temperature = {}, outdoor_temperature = {}, timestamp = '{}'",
        msg.device.id,
        msg.payload.light.map_or("null".to_string(), |x| format!("<float> {}", x)),
        msg.payload.pressure.map_or("null".to_string(), |x| format!("<float> {}", x)),
        msg.payload.humidity.map_or("null".to_string(), |x| format!("<float> {}", x)),
        msg.payload.in_temperature.map_or("null".to_string(), |x| format!("<float> {}", x)),
        msg.payload.out_temperature.map_or("null".to_string(), |x| format!("<float> {}", x)),
        msg.payload.timestamp
    ).as_str(), None).await.unwrap();

    // Add the location to the location table
    db.query(format!(
        "CREATE location SET device = device:`{}`, latitude = {}, longitude = {}, altitude = {}",
        msg.device.id,
        msg.location.latitude,
        msg.location.longitude,
        msg.location.altitude
    ).as_str(), None).await.unwrap();

    // Add the settings to the settings table
    db.query(format!(
        "CREATE settings SET device = device:`{}`, bandwidth = {}, spreading_factor = '{}', coding_rate = '{}', frequency = '{}', rssi = '{}', channel_rssi = '{}', battery_voltage = {}, snr = '{}'",
        msg.device.id,
        msg.settings.bandwidth,
        msg.settings.spreading_factor,
        msg.settings.coding_rate,
        msg.settings.frequency,
        msg.settings.rssi,
        msg.settings.channel_rssi,
        msg.settings.battery_voltage.map_or("null".to_string(), |x| format!("<float> {}", x)),
        msg.settings.snr
    ).as_str(), None).await.unwrap();
}

/// Decode the data as [`Json`] for the py-saxion sensor
/// If the function fails to decode the message, it'll return None,
/// otherwise it'll just return the decoded data as a [`DBMessage`]
fn decode_py_saxion(msg: &str) -> Option<DBMessage> {
    let json: Value = json::from_str(msg).ok()?;

    let device_id = json.get("end_device_ids")?.get("device_id")?.as_str()?;

    let timestamp = json.get("received_at")?.as_str()?;

    // WARNING: I'm not sure if this value is already in '%'
    let light = json.get("uplink_message")?.get("decoded_payload")?.get("light")?.as_f64()?;
    let pressure = json.get("uplink_message")?.get("decoded_payload")?.get("pressure")?.as_f64()?;
    let temperature = json.get("uplink_message")?.get("decoded_payload")?.get("temperature")?.as_f64()?;

    let rx_metadata = json.get("uplink_message")?.get("rx_metadata")?.as_array()?.first()?;
    let rssi = rx_metadata.get("rssi")?.as_i64()?;
    let channel_rssi = rx_metadata.get("channel_rssi")?.as_i64()?;
    let snr = rx_metadata.get("snr")?.as_f64()?;

    let latitude = rx_metadata.get("location")?.get("latitude")?.as_f64()?;
    let longitude = rx_metadata.get("location")?.get("longitude")?.as_f64()?;
    let altitude = rx_metadata.get("location")?.get("altitude")?.as_f64()?;

    let bandwidth = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("bandwidth")?.as_i64()?;
    let spreading_factor = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("spreading_factor")?.as_i64()?;
    let coding_rate = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("coding_rate")?.as_str()?;

    let frequency = json.get("uplink_message")?.get("settings")?.get("frequency")?.as_str()?;

    Some(DBMessage {
        device: DBDevice {
            id: device_id.to_string(),
            name: "py saxion".to_string(),
            description: "saxion's py sensor".to_string(),
            timestamp: timestamp.to_string()
        }, settings: DBSettings {
            bandwidth,
            spreading_factor: spreading_factor.to_string(),
            coding_rate: coding_rate.to_string(),
            frequency: frequency.to_string(),
            rssi: rssi.to_string(),
            channel_rssi: channel_rssi.to_string(),
            snr: snr.to_string(),
            battery_voltage: None
        }, payload: DBPayload {
            in_temperature: Some(temperature),
            out_temperature: None,
            light: Some(light),
            pressure: Some(pressure),
            humidity: None,
            timestamp: timestamp.to_string()
        }, location: DBLocation {
            latitude: latitude.to_string(),
            longitude: longitude.to_string(),
            altitude: altitude
        }
    })
}

/// Decode the data as [`Json`] for the py-wierden sensor
/// If the function fails to decode the message, it'll return None,
/// otherwise it'll just return the decoded data as a [`DBMessage`]
fn decode_py_wierden(msg: &str) -> Option<DBMessage> {
    let json: Value = json::from_str(msg).ok()?;

    let device_id = json.get("end_device_ids")?.get("device_id")?.as_str()?;

    let timestamp = json.get("received_at")?.as_str()?;

    let light = json.get("uplink_message")?.get("decoded_payload")?.get("light")?.as_f64()?;
    let pressure = json.get("uplink_message")?.get("decoded_payload")?.get("pressure")?.as_f64()?;
    let temperature = json.get("uplink_message")?.get("decoded_payload")?.get("temperature")?.as_f64()?;

    let rx_metadata = json.get("uplink_message")?.get("rx_metadata")?.as_array()?.first()?;
    let rssi = rx_metadata.get("rssi")?.as_i64()?;
    let channel_rssi = rx_metadata.get("channel_rssi")?.as_i64()?;
    let snr = rx_metadata.get("snr")?.as_f64()?;

    let latitude = rx_metadata.get("location")?.get("latitude")?.as_f64()?;
    let longitude = rx_metadata.get("location")?.get("longitude")?.as_f64()?;
    let altitude = rx_metadata.get("location")?.get("altitude")?.as_f64()?;

    let bandwidth = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("bandwidth")?.as_i64()?;
    let spreading_factor = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("spreading_factor")?.as_i64()?;
    let coding_rate = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("coding_rate")?.as_str()?;

    let frequency = json.get("uplink_message")?.get("settings")?.get("frequency")?.as_str()?;

    Some(DBMessage {
        device: DBDevice {
            id: device_id.to_string(),
            name: "py wierden".to_string(),
            description: "wierden's py sensor".to_string(),
            timestamp: timestamp.to_string()
        }, settings: DBSettings {
            bandwidth,
            spreading_factor: spreading_factor.to_string(),
            coding_rate: coding_rate.to_string(),
            frequency: frequency.to_string(),
            rssi: rssi.to_string(),
            channel_rssi: channel_rssi.to_string(),
            snr: snr.to_string(),
            battery_voltage: None
        }, payload: DBPayload {
            in_temperature: Some(temperature),
            out_temperature: None,
            light: Some(light),
            pressure: Some(pressure),
            humidity: None,
            timestamp: timestamp.to_string()
        }, location: DBLocation {
            latitude: latitude.to_string(),
            longitude: longitude.to_string(),
            altitude: altitude
        }
    })
}


/// Convert the logarithm scale of the lux value into a linear scale [0; 255]
fn lux_to_linear(value: f64) -> f64 {
    if value < 123.0 {
        value
    } else {
        value.log(1.04).round().max(255.0)
    }
}


/// Decode the data as [`Json`] for the lht-wierden sensor
/// If the function fails to decode the message, it'll return None,
/// otherwise it'll just return the decoded data as a [`DBMessage`]
fn decode_lht_wierden(msg: &str) -> Option<DBMessage> {
    let json: Value = json::from_str(msg).ok()?;

    let device_id = json.get("end_device_ids")?.get("device_id")?.as_str()?;

    let timestamp = json.get("received_at")?.as_str()?;

    let light = lux_to_linear(json.get("uplink_message")?.get("decoded_payload")?.get("ILL_lx")?.as_f64()?) / 255.0;
    let humidity = json.get("uplink_message")?.get("decoded_payload")?.get("Hum_SHT")?.as_f64()?;
    let temperature = json.get("uplink_message")?.get("decoded_payload")?.get("TempC_SHT")?.as_f64()?;
    let battery_voltage = json.get("uplink_message")?.get("decoded_payload")?.get("BatV")?.as_f64()?;

    let rx_metadata = json.get("uplink_message")?.get("rx_metadata")?.as_array()?.first()?;
    let rssi = rx_metadata.get("rssi")?.as_i64()?;
    let channel_rssi = rx_metadata.get("channel_rssi")?.as_i64()?;
    let snr = rx_metadata.get("snr")?.as_f64()?;

    let latitude = rx_metadata.get("location")?.get("latitude")?.as_f64()?;
    let longitude = rx_metadata.get("location")?.get("longitude")?.as_f64()?;
    let altitude = rx_metadata.get("location")?.get("altitude")?.as_f64()?;

    let bandwidth = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("bandwidth")?.as_i64()?;
    let spreading_factor = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("spreading_factor")?.as_i64()?;
    let coding_rate = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("coding_rate")?.as_str()?;

    let frequency = json.get("uplink_message")?.get("settings")?.get("frequency")?.as_str()?;

    Some(DBMessage {
        device: DBDevice {
            id: device_id.to_string(),
            name: "lht wierden".to_string(),
            description: "wierden's lht sensor".to_string(),
            timestamp: timestamp.to_string()
        }, settings: DBSettings {
            bandwidth,
            spreading_factor: spreading_factor.to_string(),
            coding_rate: coding_rate.to_string(),
            frequency: frequency.to_string(),
            rssi: rssi.to_string(),
            channel_rssi: channel_rssi.to_string(),
            snr: snr.to_string(),
            battery_voltage: Some(battery_voltage)
        }, payload: DBPayload {
            in_temperature: None,
            out_temperature: Some(temperature),
            light: Some(light),
            pressure: None,
            humidity: Some(humidity),
            timestamp: timestamp.to_string()
        }, location: DBLocation {
            latitude: latitude.to_string(),
            longitude: longitude.to_string(),
            altitude: altitude
        }
    })
}

/// Decode the data as [`Json`] for the lht-gronau sensor
/// If the function fails to decode the message, it'll return None,
/// otherwise it'll just return the decoded data as a [`DBMessage`]
fn decode_lht_gronau(msg: &str) -> Option<DBMessage> {
    let json: Value = json::from_str(msg).ok()?;

    let device_id = json.get("end_device_ids")?.get("device_id")?.as_str()?;

    let timestamp = json.get("received_at")?.as_str()?;

    let light = lux_to_linear(json.get("uplink_message")?.get("decoded_payload")?.get("ILL_lx")?.as_f64()?) / 255.0;
    let humidity = json.get("uplink_message")?.get("decoded_payload")?.get("Hum_SHT")?.as_f64()?;
    let temperature = json.get("uplink_message")?.get("decoded_payload")?.get("TempC_SHT")?.as_f64()?;
    let battery_voltage = json.get("uplink_message")?.get("decoded_payload")?.get("BatV")?.as_f64()?;

    let rx_metadata = json.get("uplink_message")?.get("rx_metadata")?.as_array()?.first()?;
    let rssi = rx_metadata.get("rssi")?.as_i64()?;
    let channel_rssi = rx_metadata.get("channel_rssi")?.as_i64()?;
    let snr = rx_metadata.get("snr")?.as_f64()?;

    let latitude = rx_metadata.get("location")?.get("latitude")?.as_f64()?;
    let longitude = rx_metadata.get("location")?.get("longitude")?.as_f64()?;

    let bandwidth = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("bandwidth")?.as_i64()?;
    let spreading_factor = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("spreading_factor")?.as_i64()?;
    let coding_rate = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("coding_rate")?.as_str()?;

    let frequency = json.get("uplink_message")?.get("settings")?.get("frequency")?.as_str()?;

    Some(DBMessage {
        device: DBDevice {
            id: device_id.to_string(),
            name: "lht gronau".to_string(),
            description: "gronau's lht sensor".to_string(),
            timestamp: timestamp.to_string()
        }, settings: DBSettings {
            bandwidth,
            spreading_factor: spreading_factor.to_string(),
            coding_rate: coding_rate.to_string(),
            frequency: frequency.to_string(),
            rssi: rssi.to_string(),
            channel_rssi: channel_rssi.to_string(),
            snr: snr.to_string(),
            battery_voltage: Some(battery_voltage)
        }, payload: DBPayload {
            in_temperature: None,
            out_temperature: Some(temperature),
            light: Some(light),
            pressure: None,
            humidity: Some(humidity),
            timestamp: timestamp.to_string()
        }, location: DBLocation {
            latitude: latitude.to_string(),
            longitude: longitude.to_string(),
            altitude: 0.0
        }
    })
}


/// Decode the data as [`Json`] for the lht-saxion sensor
/// If the function fails to decode the message, it'll return None,
/// otherwise it'll just return the decoded data as a [`DBMessage`]
fn decode_lht_saxion(msg: &str) -> Option<DBMessage> {
    let json: Value = json::from_str(msg).ok()?;

    let device_id = json.get("end_device_ids")?.get("device_id")?.as_str()?;

    let timestamp = json.get("received_at")?.as_str()?;

    let humidity = json.get("uplink_message")?.get("decoded_payload")?.get("Hum_SHT")?.as_f64()?;
    let in_temperature = json.get("uplink_message")?.get("decoded_payload")?.get("TempC_SHT")?.as_f64()?;
    let out_temperature = json.get("uplink_message")?.get("decoded_payload")?.get("TempC_DS")?.as_f64()?;
    let battery_voltage = json.get("uplink_message")?.get("decoded_payload")?.get("BatV")?.as_f64()?;

    let rx_metadata = json.get("uplink_message")?.get("rx_metadata")?.as_array()?.first()?;
    let rssi = rx_metadata.get("rssi")?.as_i64()?;
    let channel_rssi = rx_metadata.get("channel_rssi")?.as_i64()?;
    let snr = rx_metadata.get("snr")?.as_f64()?;

    let latitude = rx_metadata.get("location")?.get("latitude")?.as_f64()?;
    let longitude = rx_metadata.get("location")?.get("longitude")?.as_f64()?;
    let altitude = rx_metadata.get("location")?.get("altitude")?.as_f64()?;

    let bandwidth = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("bandwidth")?.as_i64()?;
    let spreading_factor = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("spreading_factor")?.as_i64()?;
    let coding_rate = json.get("uplink_message")?.get("settings")?.get("data_rate")?.get("lora")?.get("coding_rate")?.as_str()?;

    let frequency = json.get("uplink_message")?.get("settings")?.get("frequency")?.as_str()?;

    Some(DBMessage {
        device: DBDevice {
            id: device_id.to_string(),
            name: "lht saxion".to_string(),
            description: "saxion's lht sensor".to_string(),
            timestamp: timestamp.to_string()
        }, settings: DBSettings {
            bandwidth,
            spreading_factor: spreading_factor.to_string(),
            coding_rate: coding_rate.to_string(),
            frequency: frequency.to_string(),
            rssi: rssi.to_string(),
            channel_rssi: channel_rssi.to_string(),
            snr: snr.to_string(),
            battery_voltage: Some(battery_voltage)
        }, payload: DBPayload {
            in_temperature: Some(in_temperature),
            out_temperature: Some(out_temperature),
            light: None,
            pressure: None,
            humidity: Some(humidity),
            timestamp: timestamp.to_string()
        }, location: DBLocation {
            latitude: latitude.to_string(),
            longitude: longitude.to_string(),
            altitude: altitude
        }
    })
}

#[derive(Debug)]
struct DBMessage {
    pub device: DBDevice,
    pub settings: DBSettings,
    pub payload: DBPayload,
    pub location: DBLocation
}

#[derive(Debug)]
struct DBDevice {
    pub id: String,
    pub name: String,
    pub description: String,
    pub timestamp: String
}

#[derive(Debug)]
struct DBLocation {
    pub latitude: String,
    pub longitude: String,
    pub altitude: f64,
}

#[derive(Debug)]
struct DBSettings {
    pub bandwidth: i64,
    pub spreading_factor: String,
    pub coding_rate: String,
    pub frequency: String,
    pub rssi: String,
    pub channel_rssi: String,
    pub snr: String,
    pub battery_voltage: Option<f64>,
}

#[derive(Debug)]
struct DBPayload {
    pub in_temperature: Option<f64>,
    pub out_temperature: Option<f64>,
    pub light: Option<f64>,
    pub pressure: Option<f64>,
    pub humidity: Option<f64>,
    pub timestamp: String
}
