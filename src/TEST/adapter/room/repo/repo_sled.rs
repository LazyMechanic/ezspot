use crate::TEST::port::room::repo::*;
use crate::TEST::port::RepoResult;

pub struct RoomRepoSled {
    tree: sled::Tree,
}

impl RoomRepoSled {
    pub fn new(tree: sled::Tree) -> Self {
        Self { tree }
    }
}

#[async_trait::async_trait]
impl RoomRepo for RoomRepoSled {
    async fn create_room_credentials(
        &self,
        req: CreateRoomCredentialsRequest,
    ) -> RepoResult<CreateRoomCredentialsResponse> {
        todo!()
    }
}
