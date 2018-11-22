use super::glm::Vector2;
use entities::*;

#[derive(Debug)]
pub struct GameState {
    ecs: EntityComponentStore,
    name: String,
}

impl GameState {
    pub fn new(name: &str) -> GameState {
        GameState {
            ecs: EntityComponentStore::new(),
            name: name.to_string(),
        }
    }

    pub fn new_entity(&mut self, name: &str) -> EntityRef {
        self.ecs
            .add_entity(Entity::new(name, Vector2::new(0.0, 0.0)))
    }

    pub fn new_entity_with_pos(&mut self, name: &str, pos: Vector2<f32>) -> EntityRef {
        self.ecs.add_entity(Entity::new(name, pos))
    }

    pub fn get_entity(&self, er: &EntityRef) -> Option<&Entity> {
        self.ecs.get_entity(er)
    }
}
