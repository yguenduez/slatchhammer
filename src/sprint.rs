use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        system::Query,
    },
};

#[derive(Component)]
pub struct SprintState {
    duration: f32,
}

const SPRINT_DURATION: f32 = 10.0;

impl Default for SprintState {
    fn default() -> Self {
        SprintState {
            duration: SPRINT_DURATION,
        }
    }
}

#[derive(Component)]
pub struct ShowBars;

impl SprintState {
    pub fn reset(&mut self) {
        self.duration = SPRINT_DURATION;
    }

    pub fn resupply(&mut self, value: f32) {
        if self.duration + value > SPRINT_DURATION {
            self.duration = SPRINT_DURATION;
        }
        self.duration += value;
    }

    pub fn reduce(&mut self, value: f32) {
        if value > self.duration {
            self.duration = 0.0;
        } else {
            self.duration -= value;
        }
    }

    pub fn is_available(&self) -> bool {
        self.duration > 0.0
    }
}

#[derive(Event)]
pub struct ApplySprintEvent {
    pub amount: f32,
    pub target: Entity,
}

fn apply_sprint_events(
    mut event_reader: EventReader<ApplySprintEvent>,
    mut query: Query<&mut SprintState>,
) {
    for ev in event_reader.read() {
        let Ok(mut state) = query.get_mut(ev.target) else {
            continue;
        };
        match ev.amount > 0. {
            true => state.resupply(ev.amount),
            false => state.reduce(ev.amount * -1.0),
        }
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ApplySprintEvent>()
            .add_systems(Update, apply_sprint_events);
    }
}

#[cfg(test)]
mod tests {
    use crate::sprint::SprintState;

    #[test]
    fn is_available_when_called_then_is_true() {
        assert!(SprintState::default().is_available())
    }

    #[test]
    fn resupply_when_called_adds_duration() {
        // given
        let mut sprint = SprintState::default();
        sprint.duration = 0.0;

        // when
        sprint.resupply(1.0);

        // then
        assert_eq!(sprint.duration as i32, 1);
    }

    #[test]
    fn is_avaiable_when_called_after_depletion_then_false() {
        // given
        let mut sprint = SprintState::default();
        sprint.duration = 0.0;

        // when // then
        assert!(!sprint.is_available());
    }

    #[test]
    fn reduce_when_called_reduces_duration() {
        let mut sprint = SprintState::default();
        sprint.reduce(2.0);

        assert_eq!(sprint.duration as i32, 1);
    }
}
