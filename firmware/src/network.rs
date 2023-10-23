use esp_idf_svc::eventloop::{EspEventLoop, System};
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::nvs::{EspNvsPartition, NvsDefault};
use esp_idf_svc::sntp::EspSntp;
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use std::{thread, time::Duration};

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

const RECONNECT_COOLDOWN: Duration = Duration::from_secs(5);

pub fn setup_wireless(
    modem: Modem,
    sysloop: EspEventLoop<System>,
    nvs: EspNvsPartition<NvsDefault>,
) -> anyhow::Result<()> {
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sysloop.clone(), Some(nvs.clone()))?,
        sysloop,
    )?;

    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.into(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.into(),
        channel: None,
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

    Ok(())
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
