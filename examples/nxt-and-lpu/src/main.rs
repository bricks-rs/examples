use lego_powered_up::{consts::named_port, iodevice::motor::EncoderMotor};
use nxtusb::sensor::{InPort, SensorMode, SensorType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to NXT over bluetooth...");
    let nxt = nxtusb::Bluetooth::wait_for_nxt().await?;

    println!("Connecting to Powered-Up hub over BLE...");
    let lpu = lego_powered_up::setup::single_hub().await?;

    println!("All devices connected, starting main loop");
    nxt.set_input_mode(InPort::S1, SensorType::Switch, SensorMode::Bool)
        .await?;
    nxt.set_input_mode(InPort::S2, SensorType::Switch, SensorMode::Bool)
        .await?;

    let (mot_a, mot_b) = {
        let lock = lpu.mutex.lock().await;
        (
            lock.io_from_port(named_port::A)?,
            lock.io_from_port(named_port::B)?,
        )
    };

    loop {
        let btn1 = nxt.get_input_values(InPort::S1).await?;
        let btn2 = nxt.get_input_values(InPort::S2).await?;

        let mot_a_pwr = if btn1.scaled_value == 0 { 0 } else { 50 };
        let mot_b_pwr = if btn2.scaled_value == 0 { 0 } else { 50 };

        mot_a.start_speed(mot_a_pwr, 100).await?;
        mot_b.start_speed(mot_b_pwr, 100).await?;

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}
