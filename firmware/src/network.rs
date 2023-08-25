use embedded_svc::mqtt::client::{Details, Event, QoS};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal as _;
use esp_idf_hal::modem::Modem;
use esp_idf_svc::eventloop::{EspEventLoop, System};
use esp_idf_svc::mqtt::client::{EspMqttClient, EspMqttMessage, MqttClientConfiguration};
use esp_idf_svc::nvs::{EspNvs, EspNvsPartition, NvsDefault};
use esp_idf_svc::sntp::EspSntp;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

const MQTT_URL: &str = env!("MQTT_URL");
const MQTT_USER: &str = env!("MQTT_USER");
const MQTT_PASS: &str = env!("MQTT_PASS");

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

pub fn setup_mqtt(
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    door_tx: Sender<()>,
) -> anyhow::Result<EspMqttClient> {
    let mqtt_config = MqttClientConfiguration {
        client_id: Some("doorsys"),
        username: Some(MQTT_USER),
        password: Some(MQTT_PASS),
        disable_clean_session: true,
        ..Default::default()
    };

    let mut client = EspMqttClient::new(MQTT_URL, &mqtt_config, move |res| match res {
        Ok(Event::Received(msg)) => route_message(msg, &nvs, &door_tx),
        Ok(event) => log::info!("mqtt event: {:?}", event),
        Err(error) => log::warn!("from mqtt: {:?} {:?}", res, error),
    })?;

    client.subscribe("doorsys/user/+", QoS::AtLeastOnce)?;
    client.subscribe("doorsys/open", QoS::AtMostOnce)?;

    Ok(client)
}

fn route_message(msg: &EspMqttMessage, nvs: &Mutex<EspNvs<NvsDefault>>, door_tx: &Sender<()>) {
    match msg.topic() {
        Some("doorsys/user/add") => process_user_message(msg, nvs, true),
        Some("doorsys/user/del") => process_user_message(msg, nvs, false),
        Some("doorsys/open") => proccess_open_message(msg, door_tx),
        Some(topic) => log::warn!("unknown topic {}", topic),
        None => log::error!("no topic provided"),
    }
}

fn proccess_open_message(msg: &EspMqttMessage, door_tx: &Sender<()>) {
    log::info!("open door from mqtt {:?}", msg);
    if let Err(e) = door_tx.send(()) {
        log::error!("error sending door message {}", e);
    }
}

fn process_user_message(msg: &EspMqttMessage, nvs: &Mutex<EspNvs<NvsDefault>>, add: bool) {
    if msg.details() != &Details::Complete {
        log::error!("incomplete message, dropping {:?}", msg);
        return;
    }

    log::info!("msg: {:?}, data: {:?}", msg, msg.data());
    if let Ok(key) = std::str::from_utf8(msg.data()) {
        let mut nvs = nvs.lock().unwrap();
        if add {
            if let Err(e) = nvs.set_u8(key, 1) {
                log::error!("nvs error: {}", e);
            }
        } else if let Err(e) = nvs.remove(key) {
            log::error!("nvs error: {}", e);
        }
    }
}
