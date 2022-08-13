pub mod error;
pub mod model;

#[derive(Debug)]
struct ApexClient<'a> {
    token: &'a str,
}

#[async_trait]
impl<'a> ApexClient<'a> {
    fn new(token: &'a str) -> Self {
        ApexClient { token }
    }

    async fn get_map_rotations(
        &self,
    ) -> Result<crate::model::MapRotations, crate::error::ApexError> {
    }
}
