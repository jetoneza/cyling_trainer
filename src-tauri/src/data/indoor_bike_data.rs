#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct IndoorBikeData {
    pub cadence: u16,
    pub speed: u16,
    pub distance: u32,
}

enum IndoorBikeDataType {
    Speed,
    Cadence,
    Distance,
}

// Size in octets
const FLAGS_SIZE: usize = 2;
const INSTANTANEOUS_SPEED_SIZE: usize = 2;
const AVERAGE_SPEED_SIZE: usize = 2;
const INSTANTANEOUS_CADENCE_SIZE: usize = 2;
const AVERAGE_CADENCE_SIZE: usize = 2;
const TOTAL_DISTANCE_SIZE: usize = 3;

// Resource:
// https://www.bluetooth.com/specifications/specs/gatt-specification-supplement/
// Check for Indoor Bike Data
pub fn parse_indoor_bike_data(data: &Vec<u8>) -> IndoorBikeData {
    let speed = get_speed(data);
    let cadence = get_cadence(data);
    let distance = get_distance(data);

    IndoorBikeData {
        cadence,
        speed,
        distance,
    }
}

// Instantaneous Speed
// Data type: u16
// Size (octets): 0 or 2
fn get_speed(data: &Vec<u8>) -> u16 {
    // Present if bit 0 of Flags field is set to 0
    if !is_speed_present(data) {
        return 0;
    }

    let data_index = get_data_index(data, IndoorBikeDataType::Speed);

    let raw_speed = combine_u8_to_u16(data[data_index], data[data_index + 1]);

    // Unit is 1/100 of a kilometer per hour
    raw_speed / 100
}

// Instantaneous Cadence
// Data type: u16
// Size (octets): 0 or 2
fn get_cadence(data: &Vec<u8>) -> u16 {
    // Present if bit 2 of Flags field is set to 1
    if !is_cadence_present(data) {
        return 0;
    }

    let data_index = get_data_index(data, IndoorBikeDataType::Cadence);

    let cadence = combine_u8_to_u16(data[data_index], data[data_index + 1]);

    // Unit is 1/2 of a revolution per minute
    cadence / 2
}

// Total Distance since the beginning of the training session
// Data type: u24
// Size (octets): 0 or 3
fn get_distance(data: &Vec<u8>) -> u32 {
    // Present if bit 4 of Flags field is set to 1
    if !is_distance_present(data) {
        return 0;
    }

    let data_index = get_data_index(data, IndoorBikeDataType::Distance);

    let distance = combine_u8_to_u32(data[data_index], data[data_index + 1], data[data_index + 2]);

    return distance;
}

fn get_data_index(data: &Vec<u8>, data_type: IndoorBikeDataType) -> usize {
    match data_type {
        IndoorBikeDataType::Speed => FLAGS_SIZE,
        IndoorBikeDataType::Cadence => {
            let mut index = FLAGS_SIZE;

            if is_speed_present(data) {
                index += INSTANTANEOUS_SPEED_SIZE;
            }

            if is_ave_speed_present(data) {
                index += AVERAGE_SPEED_SIZE;
            }

            index
        }
        IndoorBikeDataType::Distance => {
            let mut index = get_data_index(data, data_type);

            if is_cadence_present(data) {
                index += INSTANTANEOUS_CADENCE_SIZE;
            }

            if is_ave_cadence_present(data) {
                index += AVERAGE_CADENCE_SIZE;
            }

            index
        }
    }
}

fn is_speed_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 1 == 0
}

fn is_ave_speed_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 0b10 == 0b10
}

fn is_cadence_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 0b100 == 0b100
}

fn is_ave_cadence_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 0b1000 == 0b1000
}

fn is_distance_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 0b10000 == 0b10000
}

// Flags field
// 0 - More data
// 1 - Average speed present
// 2 - Instantaneous cadence present
fn get_flags(data: &Vec<u8>) -> u16 {
    combine_u8_to_u16(data[0], data[1])
}

fn combine_u8_to_u16(first: u8, second: u8) -> u16 {
    (first as u16) | (second as u16) << 8
}

fn combine_u8_to_u32(first: u8, second: u8, third: u8) -> u32 {
    (first as u32) | (second as u32) << 8 | (third as u32) << 16
}
