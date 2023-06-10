use btleplug::api::bleuuid::uuid_from_u32;
use uuid::Uuid;

pub const HEART_RATE_SERVICE_UUID: Uuid = uuid_from_u32(0x180D);
pub const FITNESS_MACHINE_SERVICE_UUID: Uuid = uuid_from_u32(0x1826);
