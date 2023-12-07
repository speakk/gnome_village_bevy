use bevy::prelude::*;

use crate::components::Blueprint;

#[derive(Event)]
pub struct BlueprintFinished(pub Entity);

pub fn blueprint(
    mut commands: Commands,
    mut event_blueprint_finished: EventReader<BlueprintFinished>,
) {
    for event in event_blueprint_finished.read() {
        commands.entity(event.0).remove::<Blueprint>();
    }
}
