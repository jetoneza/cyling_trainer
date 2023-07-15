use btleplug::api::Manager as _;
use btleplug::platform::{Adapter, Manager};
use log::{error, warn};
use uuid::Uuid;

use super::bluetooth::DeviceType;
use super::constants::{
    CYCLING_POWER_MEASUREMENT_UUID, FITNESS_MACHINE_CONTROL_POINT_UUID,
    FITNESS_MACHINE_SERVICE_UUID, HEART_RATE_MEASUREMENT_UUID, HEART_RATE_SERVICE_UUID,
    INDOOR_BIKE_DATA_UUID,
};
use super::event_handlers::Characteristic;

const LOGGER_NAME: &str = "ble::utils";

pub async fn get_central(manager: &Option<Manager>) -> Option<Adapter> {
    let Some(manager) = manager.as_ref() else {
        warn!("{}::get_central: No manager found", LOGGER_NAME);
        return None;
    };

    let Ok(adapters) = manager.adapters().await else {
        warn!("{}::get_central: No adapters found", LOGGER_NAME);
        return None;
    };

    adapters.into_iter().next()
}

pub async fn get_manager() -> Option<Manager> {
    match Manager::new().await {
        Ok(manager) => Some(manager),
        Err(e) => {
            error!(
                "{}::get_manager: Could not initialize bluetooth manager: {}",
                LOGGER_NAME, e
            );
            None
        }
    }
}

pub fn get_device_type(services: Vec<Uuid>) -> DeviceType {
    let is_heart_rate = services.contains(&HEART_RATE_SERVICE_UUID);
    let is_smart_trainer = services.contains(&FITNESS_MACHINE_SERVICE_UUID);

    match (is_heart_rate, is_smart_trainer) {
        (true, false) => DeviceType::HeartRate,
        (false, true) => DeviceType::SmartTrainer,
        _ => DeviceType::Generic,
    }
}

pub fn get_uuid_characteristic(uuid: Uuid) -> Characteristic {
    match uuid {
        CYCLING_POWER_MEASUREMENT_UUID => Characteristic::CyclingPowerMeasurement,
        HEART_RATE_MEASUREMENT_UUID => Characteristic::HeartRateMeasurement,
        INDOOR_BIKE_DATA_UUID => Characteristic::IndoorBikeData,
        FITNESS_MACHINE_CONTROL_POINT_UUID => Characteristic::FitnessMachineControlPoint,
        _ => Characteristic::Unknown,
    }
}

pub fn convert_i16_to_u8(num: i16) -> [u8; 2] {
    let i16_num = num as i16;

    [(i16_num & 0xFF) as u8, ((i16_num >> 8) & 0xFF) as u8]
}
