use std::{str, thread, time::Duration};

use kafka::consumer::Consumer;
use kafka::Error;
use log::{info, warn};

use hawk::app_config::UploadCredentials;
use hawk::image_upload_operations::compare_faces;
use hawk::user_data::{subscribe, trigger_camera};

/// This project is intended to recognize the clicked image of the user from user's reference image
/// and upload the clicked image to s3 bucket as well
#[cfg_attr(tarpaulin, skip)]
fn main() {
    env_logger::init();
    loop {
        let consumer: Result<Consumer, Error> = subscribe();

        match consumer {
            Ok(mut consumer) => loop {
                for message_service in consumer.poll().unwrap().iter() {
                    for message in message_service.messages() {
                        let employee_id: String =
                            str::from_utf8(&message.value).unwrap().to_string();
                        match trigger_camera() {
                            Ok(clicked_image) => {
                                let upload_credentials: UploadCredentials =
                                    UploadCredentials::new(clicked_image, employee_id);
                                info!("{}", compare_faces(upload_credentials));
                            }
                            Err(camera_error) => info!("{}", camera_error),
                        }
                    }
                    consumer.consume_messageset(message_service).unwrap();
                }
                consumer.commit_consumed().unwrap();
            },
            Err(no_topics_assigned) => {
                warn!("{}", no_topics_assigned);
                thread::sleep(Duration::from_millis(10000));
            }
        }
    }
}
