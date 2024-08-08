use std::ffi::CStr;
use std::{thread, time::Duration};

use esp_idf_svc::eventloop::{EspEventLoop, System};
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::nvs::{EspNvsPartition, NvsDefault};
use esp_idf_svc::sntp::EspSntp;
use esp_idf_svc::sys::CONFIG_LWIP_LOCAL_HOSTNAME;
use esp_idf_svc::wifi::{
    AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi, WifiDeviceId,
};

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

const RECONNECT_COOLDOWN: Duration = Duration::from_secs(5);

pub fn setup_wireless(
    modem: Modem,
    sysloop: EspEventLoop<System>,
    nvs: EspNvsPartition<NvsDefault>,
) -> anyhow::Result<String> {
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sysloop.clone(), Some(nvs.clone()))?,
        sysloop,
    )?;

    let net_id = create_net_id(&wifi)?;
    log::info!("Device net_id: {net_id}");

    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        password: PASSWORD.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    log::info!("Wifi started");

    connect_wifi_loop(&mut wifi);

    // Wifi reconnect thread
    thread::spawn(move || {
        let sntp = EspSntp::new_default();
        if let Err(e) = sntp {
            log::warn!("error creating sntp: {}", e);
        }
        loop {
            wifi.wifi_wait_while(|| wifi.is_connected(), None).unwrap();
            log::warn!("Lost wifi connection, reconnecting...");
            connect_wifi_loop(&mut wifi);
        }
    });

    Ok(net_id)
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi>) -> anyhow::Result<()> {
    wifi.connect()?;
    log::info!("Wifi connected");

    wifi.wait_netif_up()?;
    log::info!("Wifi netif up");

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    log::info!("Wifi DHCP info: {:?}", ip_info);

    Ok(())
}

fn connect_wifi_loop(wifi: &mut BlockingWifi<EspWifi>) {
    let mut count = 0;
    while connect_wifi(wifi).is_err() {
        count += 1;
        log::error!("error connecting to wifi, retrying... [{}]", count);
        thread::sleep(RECONNECT_COOLDOWN);
    }
}

/// Creates a unique identifier for this device based on local hostname
/// plus last 3 octets of the mac address
fn create_net_id(wifi: &BlockingWifi<EspWifi>) -> anyhow::Result<String> {
    let mac = wifi.wifi().get_mac(WifiDeviceId::Sta)?;
    let mac_id = mac
        .iter()
        .skip(3)
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + ((x as u32) << (i * 8)));
    let hostname = CStr::from_bytes_with_nul(CONFIG_LWIP_LOCAL_HOSTNAME)?;
    Ok(format!("{}-{:x}", hostname.to_string_lossy(), mac_id))
}
