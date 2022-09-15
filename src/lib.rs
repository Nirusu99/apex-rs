use http::Http;

pub mod error;
mod http;
pub mod model;

const MAP_ROTATIONS_LOCATION: &'static str = "maprotation";
const CRAFTER_ROTATIONS_LOCATION: &'static str = "crafting";
const ALL_ROTATIONS: (&'static str, &'static str) = ("version", "2");

#[derive(Debug)]
pub struct ApexClient<'a> {
    token: &'a str,
}

async fn request_all_rotations(
    token: &str,
) -> Result<crate::model::MapRotations, error::ApexError> {
    let http = Http::new_with_auth(token);
    let body = http
        .request(MAP_ROTATIONS_LOCATION, &[ALL_ROTATIONS])
        .await?;
    Ok(serde_json::from_str(&body)?)
}

impl<'a> ApexClient<'a> {
    pub fn new(token: &'a str) -> Self {
        ApexClient { token }
    }

    /// get the non ranked battle royal map rotation
    pub async fn battle_royal_rotation(
        &self,
    ) -> Result<crate::model::MapRotation, crate::error::ApexError> {
        request_all_rotations(self.token)
            .await?
            .battle_royal()
            .map_or(Err(error::ApexError::Unavailable), |rot| Ok(rot.clone()))
    }

    /// get all map rotations
    pub async fn all_rotations(
        &self,
    ) -> Result<crate::model::MapRotations, crate::error::ApexError> {
        request_all_rotations(self.token).await
    }

    pub async fn arena_rotation(
        &self,
    ) -> Result<crate::model::MapRotation, crate::error::ApexError> {
        request_all_rotations(self.token)
            .await?
            .arena()
            .map_or(Err(error::ApexError::Unavailable), |rot| Ok(rot.clone()))
    }

    pub async fn ranked_arena_rotation(
        &self,
    ) -> Result<crate::model::MapRotation, crate::error::ApexError> {
        request_all_rotations(self.token)
            .await?
            .arena_ranked()
            .map_or(Err(error::ApexError::Unavailable), |rot| Ok(rot.clone()))
    }

    pub async fn event_rotation(
        &self,
    ) -> Result<crate::model::MapRotation, crate::error::ApexError> {
        request_all_rotations(self.token)
            .await?
            .event()
            .map_or(Err(error::ApexError::Unavailable), |rot| Ok(rot.clone()))
    }

    /// get all crafter rotations
    pub async fn crafter_rotations(
        &self,
    ) -> Result<crate::model::Bundles, crate::error::ApexError> {
        let http = Http::new_with_auth(self.token);
        let body = http.request(CRAFTER_ROTATIONS_LOCATION, &[]).await?;
        Ok(serde_json::from_str(&body)?)
    }
}
