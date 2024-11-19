use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Deserialize, Debug)]
pub struct Todo {
    #[serde(default = "random_uuid")]
    pub id: Uuid,
    pub text: String,
    #[serde(default = "default_completed")]
    pub completed: bool,
}

fn random_uuid() -> Uuid {
    Uuid::new_v4()
}

fn default_completed() -> bool {
    false
}