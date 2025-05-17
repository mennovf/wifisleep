#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    timer::timg::TimerGroup, rng::Rng,
    clock::CpuClock
};
use esp_println::println;
use esp_wifi::{
    wifi::{
        ClientConfiguration, Configuration,
        WifiState
    },
};
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

#[esp_hal::main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer_group0 = TimerGroup::new(peripherals.TIMG0);

    let rng = Rng::new(peripherals.RNG);
    let delay = esp_hal::delay::Delay::new();

    let esp_wifi_control = esp_wifi::init(
        timer_group0.timer0,
        rng,
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let (mut wifi_controller, _interfaces) = esp_wifi::wifi::new(
        &esp_wifi_control,
        peripherals.WIFI
    ).unwrap();
    
    println!("About to initialize WiFi");
    let client_config = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        password: PASSWORD.try_into().unwrap(),
        ..Default::default()
    });

    wifi_controller.set_configuration(&client_config).unwrap();

    println!("Starting WiFi");
    wifi_controller.start().unwrap();

    println!("Connecting to WiFi network '{}'...", SSID);
    wifi_controller.connect().unwrap();

    println!("Waiting for connection...");

    loop {
        if esp_wifi::wifi::wifi_state() == WifiState::StaConnected {
            break;
        }
        delay.delay_millis(100);
    }

    println!("WiFi connection established!");
    loop {
        println!(
            "WiFi still connected: {:?}",
            wifi_controller.is_connected().unwrap()
        );
        delay.delay_millis(5000);
    }
}
