use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::components::Blueprint;

#[derive(Event)]
pub struct BlueprintFinished(pub Entity);

pub fn blueprint(
    mut commands: Commands,
    mut event_blueprint_finished: EventReader<BlueprintFinished>,
    mut color_query: Query<&mut TileColor, With<Blueprint>>,
) {
    for event in event_blueprint_finished.read() {
        let mut color = color_query
            .get_mut(event.0)
            .expect("No blueprint entity transform found");
        *color = TileColor(Color::WHITE);
        commands.entity(event.0).remove::<Blueprint>();
    }
}
