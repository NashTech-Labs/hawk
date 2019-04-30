/// Required user input as UploadCredentials
#[derive(Debug, PartialEq, Clone)]
pub struct UploadCredentials {
    /// This is the path of clicked image
    pub clicked_image: String,
    /// This is the id of reference image
    pub employee_code: String,
}

impl UploadCredentials {
    /// Instantiates UploadCredentials
    ///
    /// # Arguments
    ///
    /// * `clicked_image` - This is the path of clicked image
    ///
    /// * `employee_code` - This is the id of reference image
    ///
    /// # Return
    ///
    /// This function returns instance of UploadCredentials
    pub fn new(clicked_image: String, employee_code: String) -> UploadCredentials {
        UploadCredentials {
            clicked_image,
            employee_code,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::image_upload_operations::tests::{TEST_EMPLOYEE_ID, TEST_IMAGE_FILE};

    use super::*;

    const INVALID_TEST_IMAGE_FILE: &str = "tests/resources/test.txt";
    const INVALID_TEST_EMPLOYEE_ID: &str = "2009";

    #[test]
    fn test_upload_credentials_success() {
        let security_credentials: UploadCredentials =
            UploadCredentials::new(TEST_IMAGE_FILE.to_string(), TEST_EMPLOYEE_ID.to_string());
        let test_credentials: UploadCredentials = UploadCredentials {
            clicked_image: TEST_IMAGE_FILE.to_string(),
            employee_code: TEST_EMPLOYEE_ID.to_string(),
        };
        assert_eq!(security_credentials, test_credentials);
    }

    #[test]
    fn test_upload_credentials_failure() {
        let security_credentials: UploadCredentials =
            UploadCredentials::new(TEST_IMAGE_FILE.to_string(), TEST_EMPLOYEE_ID.to_string());
        let test_credentials: UploadCredentials = UploadCredentials {
            clicked_image: INVALID_TEST_IMAGE_FILE.to_string(),
            employee_code: INVALID_TEST_EMPLOYEE_ID.to_string(),
        };
        assert_ne!(security_credentials, test_credentials);
    }
}
