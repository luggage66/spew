#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
// #![forbid(missing_docs)]

use bevy::ecs::system::SystemState;
use bevy::prelude::*;

/// Everything you need to get started
pub mod prelude {
    pub use crate::{SpawnEvent, SpewApp, SpewPlugin};
}

#[derive(Debug)]
pub struct SpewPlugin<T: Eq + Send + Sync + 'static> {
    spawner_enum_type: std::marker::PhantomData<T>,
}

impl<T: Eq + Send + Sync + 'static> Default for SpewPlugin<T> {
    fn default() -> Self {
        Self {
            spawner_enum_type: std::marker::PhantomData,
        }
    }
}

impl<T: Eq + Send + Sync + 'static> Plugin for SpewPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent<T>>();
    }
}

pub trait SpewApp {
    fn add_spawner<T: Eq + Send + Sync + 'static>(
        &mut self,
        key: T,
        spawner: impl FnMut(Transform, &mut World) + Send + Sync + 'static,
    ) -> &mut App;
}

pub struct SpawnEvent<T: Eq + Send + Sync + 'static> {
    pub object: T,
    pub transform: Transform,
}

impl SpewApp for App {
    fn add_spawner<T: Eq + Send + Sync + 'static>(
        &mut self,
        key: T,
        mut spawner: impl FnMut(Transform, &mut World) + 'static + Send + Sync,
    ) -> &mut App {
        let system = move |world: &mut World| {
            let mut event_system_state = SystemState::<EventReader<SpawnEvent<T>>>::new(world);
            let mut events = event_system_state.get_mut(world);
            let transforms: Vec<_> = events
                .iter()
                .filter(|event| event.object == key)
                .map(|event| event.transform)
                .collect();
            for transform in transforms {
                spawner(transform, world);
            }
        };
        self.add_system(system);
        self
    }
}
