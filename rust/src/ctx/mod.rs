use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Ctx {
    pub user_id: Uuid,
}

impl Ctx {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

impl Ctx {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}