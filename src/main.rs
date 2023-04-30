use std::{error::Error, thread, time::Duration};

use serde_json::json;

#[derive(Debug, PartialEq, Eq)]
pub enum DeviceState {
    On,
    Off,
}

fn main() {
    let mut lights_on: Option<DeviceState> = None;
    loop {
        if let Some(state) = media_state::camera_state() {
            match state {
                media_state::State::On => lights_on = set_lights(lights_on, DeviceState::On).ok(),
                media_state::State::Off => lights_on = set_lights(lights_on, DeviceState::Off).ok(),
            }
        } else {
            println!("unable to determine camera state");
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn set_lights(
    current_state: Option<DeviceState>,
    target_state: DeviceState,
) -> Result<DeviceState, Box<dyn Error>> {
    if let Some(current_state) = current_state {
        if current_state == target_state {
            return Ok(current_state);
        }
    }

    let body = json!(
    {
        "lights":[
            {
                "brightness":50,
                "temperature":165,
                "on":target_state == DeviceState::On,
            }],
        "numberOfLights":1
    });

    send_request_to_lights(body)?;
    Ok(target_state)
}

fn send_request_to_lights(body: serde_json::Value) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    client
        .put("http://192.168.1.234:9123/elgato/lights")
        .json(&body)
        .send()?;
    client
        .put("http://192.168.1.237:9123/elgato/lights")
        .json(&body)
        .send()?;
    Ok(())
}
