use http::Http;

pub mod error;
mod http;
pub mod model;

const MAP_ROTATIONS_LOCATION: &'static str = "maprotation";
const ALL_ROTATIONS: (&'static str, &'static str) = ("version", "2");

#[derive(Debug)]
pub struct ApexClient<'a> {
    token: &'a str,
}

impl<'a> ApexClient<'a> {
    pub fn new(token: &'a str) -> Self {
        ApexClient { token }
    }

    pub async fn get_map_rotations(
        &self,
    ) -> Result<crate::model::MapRotations, crate::error::ApexError> {
        let http = Http::new_with_auth(self.token);
        let body = http
            .request(MAP_ROTATIONS_LOCATION, &[ALL_ROTATIONS])
            .await?;
        Ok(serde_json::from_str(&body)?)
    }
}
