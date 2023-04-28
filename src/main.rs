use std::error::Error;

use serde_json::json;

fn main() {
    let mut lights_on: Option<bool> = None;
    loop {
        if let Some(state) = media_state::camera_state() {
            match state {
                media_state::State::On => {
                    lights_on = set_lights(lights_on, true).ok();
                },
                media_state::State::Off => {
                    lights_on = set_lights(lights_on, false).ok();
                }
            }
        } else {
            println!("unable to determine camera state");
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn set_lights(current_state: Option<bool>, target_state: bool) -> Result<bool, Box<dyn Error>>{
    if let Some(current_state) = current_state {
        if current_state == target_state {
            return Ok(current_state)
        }
    }

    let body = json!(
        {
            "lights":[
                {
                    "brightness":50,
                    "temperature":165,
                    "on":target_state
                }],
            "numberOfLights":1
        });

    send_request_to_lights(body)?;
    Ok(target_state)
}


fn send_request_to_lights(body: serde_json::Value) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    client.put("http://192.168.1.234:9123/elgato/lights").json(&body).send()?;
    client.put("http://192.168.1.237:9123/elgato/lights").json(&body).send()?;
    Ok(())
}
