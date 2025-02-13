//! Used to interact with the [Collection Endpoints](https://replicate.com/docs/reference/http#collections.get).
//!
//! The Collection struct is initialized with a Config struct.
//!
//! # Example
//!
//! ```
//! use replicate_rust::{Replicate, config::Config};
//!
//! let config = Config::default();
//! let replicate = Replicate::new(config);
//!
//! let collections = replicate.collections.get("audio-generation")?;
//! println!("Collection : {:?}", collections);
//!
//! # Ok::<(), replicate_rust::errors::ReplicateError>(())
//! ```
//!

use crate::{
    api_definitions::{GetCollectionModels, ListCollectionModels},
    errors::ReplicateError, prediction::default_client,
};

/// Used to interact with the [Collection Endpoints](https://replicate.com/docs/reference/http#collections.get).
#[derive(Clone, Debug)]
pub struct Collection {
    /// Holds a reference to a Config struct, which contains the base url,  auth token among other settings.
    pub parent: crate::config::Config,
}

impl Collection {
    /// Create a new Collection struct.
    pub fn new(rep: crate::config::Config) -> Self {
        Self { parent: rep }
    }

    /// Get a collection by slug.
    ///
    /// # Example
    ///
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// let collections = replicate.collections.get("audio-generation")?;
    /// println!("Collections : {:?}", collections);
    ///
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    pub fn get(&self, collection_slug: &str) -> Result<GetCollectionModels, ReplicateError> {
        let client = default_client();

        let response = client
            .get(format!(
                "{}/collections/{}",
                self.parent.base_url, collection_slug
            ))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        if !response.status().is_success() {
            return Err(ReplicateError::ResponseError(response.text()?));
        }

        let response_string = response.text()?;
        let response_struct: GetCollectionModels = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }

    /// List all collections present in Replicate.
    ///
    /// # Example
    ///
    /// ```
    /// use replicate_rust::{Replicate, config::Config};
    ///
    /// let config = Config::default();
    /// let replicate = Replicate::new(config);
    ///
    /// let collections = replicate.collections.list()?;
    /// println!("Collections : {:?}", collections);
    ///
    /// # Ok::<(), replicate_rust::errors::ReplicateError>(())
    /// ```
    pub fn list(&self) -> Result<ListCollectionModels, ReplicateError> {
        let client = default_client();

        let response = client
            .get(format!("{}/collections", self.parent.base_url))
            .header("Authorization", format!("Token {}", self.parent.auth))
            .header("User-Agent", &self.parent.user_agent)
            .send()?;

        if !response.status().is_success() {
            return Err(ReplicateError::ResponseError(response.text()?));
        }

        let response_string = response.text()?;
        let response_struct: ListCollectionModels = serde_json::from_str(&response_string)?;

        Ok(response_struct)
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, errors::ReplicateError, Replicate};

    use httpmock::{Method::GET, MockServer};
    use serde_json::json;

    #[test]
    fn test_get() -> Result<(), ReplicateError> {
        let server = MockServer::start();

        let get_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/collections/super-resolution");
            then.status(200).json_body_obj(&json!( {
                "name": "Super resolution",
                "slug": "super-resolution",
                "description": "Upscaling models that create high-quality images from low-quality images.",
                "models": [],
              }));
        });

        let config = Config {
            auth: String::from("test"),
            base_url: server.base_url(),
            ..Config::default()
        };
        let replicate = Replicate::new(config);

        let result = replicate.collections.get("super-resolution");

        // Assert that the returned value is correct
        assert_eq!(result?.name, "Super resolution");

        // Ensure the mocks were called as expected
        get_mock.assert();

        Ok(())
    }

    #[test]
    fn test_list() -> Result<(), ReplicateError> {
        let server = MockServer::start();

        let get_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/collections");
            then.status(200).json_body_obj(&json!( {
                "results": [
                  {
                    "name": "Super resolution",
                    "slug": "super-resolution",
                    "description": "Upscaling models that create high-quality images from low-quality images.",
                  },
                  {
                    "name": "Image classification",
                    "slug": "image-classification",
                    "description": "Models that classify images.",
                  },
                ],
                "next": None::<String>,
                "previous": None::<String>,
              }));
        });

        let config: Config = Config {
            auth: String::from("test"),
            base_url: server.base_url(),
            ..Config::default()
        };
        let replicate = Replicate::new(config);

        let result = replicate.collections.list()?;

        // Assert that the returned value is correct
        assert_eq!(result.results.len(), 2);

        // Ensure the mocks were called as expected
        get_mock.assert();

        Ok(())
    }
}
