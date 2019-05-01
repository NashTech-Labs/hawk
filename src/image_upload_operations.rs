use std::{env, ffi::OsStr, fs, path::Path};

use futures::{future::ok, Future};
use log::error;
use s3::{bucket::Bucket, credentials::Credentials, error::S3Error};

use crate::app_config::UploadCredentials;
use crate::aws_lambda;

static BUCKET_NAME: &str = "Clicked_Image_Bucket";
static HTTP_RESPONSE_CODE: &str = "200";
const IMAGE_FORMATS: [&str; 2] = ["jpg", "png"];
static INVALID_IMAGE_FORMAT: &str = "Invalid Image Format";
static IO_ERROR: &str = "No such file or directory found";
static REGION: &str = "Region";
const SUCCESS_STATUS_CODE: u32 = 200;
const S3ERROR: u32 = 400;

/// This method establishes connection with S3 bucket and returns the bucket instance
fn connection_bucket() -> Bucket {
    Bucket::new(
        env::var(BUCKET_NAME).unwrap().as_str(),
        env::var(REGION).unwrap().parse().unwrap(),
        Credentials::default(),
    )
}

/// Get the mime type for a clicked image
///
/// # Arguments
///
/// * `file` - This is the path of clicked image
///
/// # Return
///
/// This function returns mime type of clicked image
fn extract_clicked_image_extension(file: &str) -> Result<&str, &'static str> {
    let clicked_image_format: &str = Path::new(file)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("invalid extension");
    for image_format in IMAGE_FORMATS.iter() {
        if *image_format == clicked_image_format {
            return Ok(clicked_image_format);
        }
    }
    Err(INVALID_IMAGE_FORMAT)
}

/// The method upload_image is used to get clicked image of the user and upload that image to S3
/// bucket returning response code from the bucket
///
/// # Arguments
///
/// * `clicked_image` - This is the byte array of clicked image
///
/// * `content_type` - This is the extension of clicked image
///
/// * `employee_code` - This is the id of reference image
///
/// * `bucket` - This is the instance of S3 bucket
///
/// # Return
///
/// This function returns response code of S3 operation(either success code or error code)
///
/// # ERROR
///
/// If this function encounters an S3Error, and print Error in loggers and returns S3Error code
fn upload_image(
    clicked_image: &[u8],
    content_type: &str,
    employee_code: String,
    bucket: Bucket,
) -> impl Future<Item = u32, Error = ()> {
    ok(bucket.put(employee_code.as_str(), clicked_image, content_type)).map(
        |response: Result<(Vec<u8>, u32), S3Error>| match response {
            Ok(s3_result) => s3_result.1,
            Err(s3_error) => {
                error!("{:?}", s3_error);
                S3ERROR
            }
        },
    )
}

/// This method sends request to AWS Recognition for face recognition and upload the same image on
/// s3 bucket asynchronously, returning a string response message.
///
/// # Arguments
///
/// * `upload_credentials` - This is the instance of UploadCredentials
///
/// # Return
///
/// This function returns face match response of the clicked image with stored image in s3 bucket.
pub fn compare_faces(upload_credentials: UploadCredentials) -> &'static str {
    let bucket: Bucket = connection_bucket();
    match extract_clicked_image_extension(upload_credentials.clicked_image.trim()) {
        Ok(content_type) => match fs::read(upload_credentials.clicked_image.trim()) {
            Ok(clicked_image) => upload_image(
                clicked_image.as_slice(),
                content_type,
                upload_credentials.employee_code.clone(),
                bucket,
            )
            .join(aws_lambda::trigger_lambda(
                clicked_image,
                upload_credentials.employee_code,
            ))
            .map(|(s3_response, recognition_response)| {
                if recognition_response.as_str() != HTTP_RESPONSE_CODE {
                    error!("{}", "500 Internal Server Error");
                }
                match s3_response {
                    SUCCESS_STATUS_CODE => "Image uploaded successfully on S3 bucket",
                    _ => "Unable to upload image on S3 bucket",
                }
            })
            .wait()
            .unwrap(),
            Err(_) => IO_ERROR,
        },
        Err(file_extension_error) => file_extension_error,
    }
}

#[cfg(test)]
pub mod tests {
    use s3::region::Region;

    use crate::app_config::UploadCredentials;

    use super::*;

    pub const TEST_EMPLOYEE_ID: &str = "2007";
    const TEST_BUCKET_NAME: &str = "hawk04";
    pub const TEST_IMAGE_FILE: &str = "tests/resources/test.jpg";
    const TEST_TEXT_FILE: &str = "tests/resources/test.txt";
    const REGION: Region = Region::ApSouth1;

    fn get_utilities(bucket_name: &str, credentials: Credentials) -> (Bucket, Vec<u8>, &str) {
        let bucket: Bucket = Bucket::new(bucket_name, REGION, credentials);
        let image: Vec<u8> = fs::read(TEST_IMAGE_FILE).unwrap();
        let content_type: &str =
            extract_clicked_image_extension(TEST_IMAGE_FILE).unwrap_or_default();
        (bucket, image, content_type)
    }

    #[test]
    fn check_connection_for_success() {
        let credentials: Credentials = Credentials::default();
        let bucket: Bucket = Bucket::new(TEST_BUCKET_NAME, REGION, credentials);
        assert_eq!(bucket, connection_bucket());
    }

    #[test]
    fn check_connection_for_failure() {
        let credentials: Credentials = Credentials::default();
        let bucket: Bucket = Bucket::new("invalid_bucket", REGION, credentials);
        assert_ne!(bucket, connection_bucket())
    }

    #[test]
    fn check_image_upload_for_success() {
        let credentials: Credentials = Credentials::default();
        let utilities: (Bucket, Vec<u8>, &str) = get_utilities(TEST_BUCKET_NAME, credentials);
        assert_eq!(
            upload_image(
                &utilities.1,
                utilities.2,
                TEST_EMPLOYEE_ID.to_string(),
                utilities.0,
            )
            .wait()
            .unwrap(),
            SUCCESS_STATUS_CODE
        )
    }

    #[test]
    fn check_image_upload_invalid_bucket() {
        let credentials: Credentials = Credentials::default();
        let utilities: (Bucket, Vec<u8>, &str) = get_utilities("hawk-image", credentials);
        assert_eq!(
            upload_image(
                &utilities.1,
                utilities.2,
                TEST_EMPLOYEE_ID.to_string(),
                utilities.0,
            )
            .wait()
            .unwrap(),
            S3ERROR
        )
    }

    #[test]
    fn check_image_upload_invalid_credentials() {
        let bad_access_key: Option<String> = Some("access".to_owned());
        let bad_secret_key: Option<String> = Some("secret".to_owned());
        let credentials: Credentials = Credentials::new(bad_access_key, bad_secret_key, None, None);
        let utilities: (Bucket, Vec<u8>, &str) = get_utilities(TEST_BUCKET_NAME, credentials);
        assert_eq!(
            upload_image(
                &utilities.1,
                utilities.2,
                TEST_EMPLOYEE_ID.to_string(),
                utilities.0,
            )
            .wait()
            .unwrap(),
            S3ERROR
        )
    }

    #[test]
    fn test_local_file_get_mime_image() {
        assert_eq!(
            extract_clicked_image_extension(TEST_IMAGE_FILE).unwrap_or_default(),
            "jpg"
        )
    }

    #[test]
    fn test_local_file_get_mime_other_file() {
        assert_eq!(
            extract_clicked_image_extension(TEST_TEXT_FILE).unwrap_err(),
            INVALID_IMAGE_FORMAT
        );
    }

    #[test]
    fn test_compare_faces() {
        let clicked_image: String = TEST_IMAGE_FILE.to_string();
        let credentials: UploadCredentials =
            UploadCredentials::new(clicked_image, TEST_EMPLOYEE_ID.to_string());
        assert_eq!(
            compare_faces(credentials),
            "Image uploaded successfully on S3 bucket"
        );
    }

    #[test]
    fn test_compare_faces_read_file_failure() {
        let clicked_image: String = "tests/wrong_path.jpg".to_string();
        let employee_code: String = TEST_EMPLOYEE_ID.to_string();
        let credentials: UploadCredentials = UploadCredentials::new(clicked_image, employee_code);
        assert_eq!(compare_faces(credentials), IO_ERROR);
    }

    #[test]
    fn test_compare_faces_get_file_format_failure() {
        let clicked_image: String = TEST_TEXT_FILE.to_string();
        let employee_code: String = TEST_EMPLOYEE_ID.to_string();
        let credentials: UploadCredentials = UploadCredentials::new(clicked_image, employee_code);
        assert_eq!(compare_faces(credentials), INVALID_IMAGE_FORMAT);
    }
}
