use std::collections::HashMap;
use std::env;

use futures::{future::ok, Future};
use log::info;
use reqwest as request;

static CLICKED_IMAGE: &str = "clicked_image";
static EMPLOYEE_CODE: &str = "image_id";
static LAMBDA_END_POINT: &str = "Lambda_End_Point";
const HTTP_RESPONSE_CODE: &str = "200";

/// This method trigger_lambda sends request to aws lambda
///
/// # Arguments
///
/// * `clicked_image` - This is the byte array of clicked image
///
/// * `employee_code` - This is the employee code of the user
///
/// # Return
///
///  This function returns status code of aws lambda asynchronously
pub fn trigger_lambda(
    clicked_image: Vec<u8>,
    employee_code: String,
) -> impl Future<Item = request::StatusCode, Error = ()> {
    ok(request::Client::new())
        .map(move |client| {
            let mut employee_detail: HashMap<&str, &[u8]> = HashMap::new();
            employee_detail.insert(CLICKED_IMAGE, clicked_image.as_slice());
            employee_detail.insert(EMPLOYEE_CODE, employee_code.trim().as_bytes());
            client
                .post(env::var(LAMBDA_END_POINT).unwrap().as_str())
                .json(&employee_detail)
                .send()
                .unwrap()
        })
        .map(|mut response| match response.status().as_str() {
            HTTP_RESPONSE_CODE => {
                let mut response_message: Vec<u8> = vec![];
                response.copy_to(&mut response_message).unwrap();
                info!("{}", String::from_utf8(response_message).unwrap());
                response.status()
            }
            _ => response.status(),
        })
}

#[cfg(test)]
mod test {
    use std::fs;

    use futures::future::Future;

    use crate::aws_lambda;
    use crate::image_upload_operations::tests::TEST_IMAGE_FILE;

    #[test]
    fn test_trigger_lambda_success() {
        let image: Vec<u8> = fs::read(TEST_IMAGE_FILE).unwrap();
        assert_eq!(
            aws_lambda::trigger_lambda(image, "2007".to_string())
                .wait()
                .unwrap()
                .to_string(),
            "200 OK"
        );
    }
}
