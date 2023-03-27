use std::collections::HashMap;

use serde::Deserialize;
use url::Url;

use crate::error::ShodanError;
use crate::error::ShodanError::ShodanClientError;
use crate::response::ShodanClientResponse;

pub mod account;
pub mod api_status;
pub mod builders;
pub mod directory;
pub mod dns;
pub mod error;
pub mod response;
pub mod scanning;
pub mod search;
pub mod utility;

const BASE_API_URL: &str = "https://api.shodan.io";

#[derive(Clone)]
pub struct ShodanClient {
    api_key: String,
}

impl ShodanClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    fn build_request_url(
        &self,
        endpoint: &str,
        parameters: Option<HashMap<String, String>>,
    ) -> String {
        let mut url = Url::parse(BASE_API_URL).unwrap();
        url.set_path(endpoint);

        url.query_pairs_mut()
            .append_pair("key", self.api_key.as_str());

        if let Some(url_parameters) = parameters {
            url.query_pairs_mut()
                .extend_pairs(url_parameters.into_iter());
        }

        url.to_string()
    }

    async fn fetch<T: for<'a> Deserialize<'a>>(url: String) -> Result<T, ShodanError> {
        let response = reqwest::get(url)
            .await
            .map_err(ShodanError::ReqwestError)?
            .json::<ShodanClientResponse<T>>()
            .await
            .map_err(ShodanError::ReqwestError)?;

        match response {
            ShodanClientResponse::Error(e) => {
                Err(ShodanClientError(format!("Error response: {}", e.error)))
            }
            ShodanClientResponse::Response(r) => Ok(r),
        }
    }
}

pub fn add_optional_parameter(
    name: &str,
    param: Option<impl ToString>,
    map: &mut HashMap<String, String>,
) {
    if let Some(unwrapped) = param {
        map.insert(name.into(), unwrapped.to_string());
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    pub fn get_test_api_key() -> String {
        let api_key = env::var("SHODAN_TEST_KEY");
        match api_key {
            Ok(key) => key,
            Err(_) => panic!("Did not specify a shodan API key for testing"),
        }
    }
}
