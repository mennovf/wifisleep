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
    },
};
    
use esp_hal::rtc_cntl::sleep::*;
use esp_hal::rtc_cntl::*;

#[esp_hal::main]
fn main() -> ! {
    esp_alloc::heap_allocator!(size: 72 * 1024);
    
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut rtc = Rtc::new(peripherals.LPWR);

    let timer_group0 = TimerGroup::new(peripherals.TIMG0);

    let rng = Rng::new(peripherals.RNG);

    let esp_wifi_control = esp_wifi::init(
        timer_group0.timer0,
        rng,
        peripherals.RADIO_CLK,
    ).unwrap();

    let (mut wifi_controller, _interfaces) = esp_wifi::wifi::new(
        &esp_wifi_control,
        peripherals.WIFI
    ).unwrap();

    let client_config = Configuration::Client(ClientConfiguration {
        ssid: "SSID".try_into().unwrap(),
        password: "PASSWORD".try_into().unwrap(),
        ..Default::default()
    });
    
    wifi_controller.set_configuration(&client_config).unwrap();

    /* Doesn't get stuck here */

    wifi_controller.start().unwrap();
    
    /* Gets stuck */
    
    println!("Sleep!");
    let mut wakeup_source = TimerWakeupSource::new(core::time::Duration::from_millis(1000));
    rtc.sleep_deep(&[&mut wakeup_source]);
}
