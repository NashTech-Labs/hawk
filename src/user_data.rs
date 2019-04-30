use std::env;
use std::process::Command;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

static CAMERA_SHUTTER_SPEED: &str = "1000";
static IMAGE_WIDTH: &str = "180";
static IMAGE_HEIGHT: &str = "200";
static INVALID_CLICKED_IMAGE: &str = "Unable to capture image";
static HOST: &str = "Host";
static RFID_CATEGORY: &str = "Topic_Name";
static RFID_CONSUMER: &str = "Consumer_Group";
static RASPISTILL: &str = "raspistill";
static CLICKED_IMAGE_PATH: &str = "Clicked_Image_Path";

/// This method get_consumer returns kafka consumer
///
/// # Return
///
/// This function returns Result with kafka Consumer and kafka Error
pub fn subscribe() -> Result<kafka::consumer::Consumer, kafka::error::Error> {
    Consumer::from_hosts(vec![env::var(HOST).unwrap()])
        .with_topic(env::var(RFID_CATEGORY).unwrap())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group(env::var(RFID_CONSUMER).unwrap())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
}

/// This function triggers camera
///
/// # Return
///
/// This function returns camera image
pub fn trigger_camera() -> Result<String, &'static str> {
    let clicked_image_path: String = env::var(CLICKED_IMAGE_PATH).unwrap();
    match Command::new(RASPISTILL)
        .args(&[
            "-t",
            CAMERA_SHUTTER_SPEED,
            "-w",
            IMAGE_WIDTH,
            "-h",
            IMAGE_HEIGHT,
            "-o",
            clicked_image_path.as_str(),
        ])
        .output()
    {
        Ok(_) => Ok(clicked_image_path),
        Err(_) => Err(INVALID_CLICKED_IMAGE),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_subscribe() {
        let mut consumer: Consumer = subscribe().unwrap();
        assert_eq!(consumer.poll().unwrap().is_empty(), true)
    }

    #[test]
    fn test_camera_invalid_image() {
        assert_eq!(trigger_camera().unwrap_err(), INVALID_CLICKED_IMAGE)
    }
}
