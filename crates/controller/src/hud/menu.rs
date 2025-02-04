use core::fmt;

use bevy::prelude::*;
use de_core::{baseset::GameSet, gamestate::GameState, state::AppState};
use de_gui::{ButtonCommands, GuiCommands, OuterStyle};

use super::interaction::InteractionBlocker;

pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToggleGameMenu>()
            .add_system(setup.in_schedule(OnEnter(GameState::Playing)))
            .add_system(cleanup.in_schedule(OnExit(GameState::Playing)))
            .add_system(
                toggle_system
                    .in_base_set(GameSet::Input)
                    .run_if(in_state(GameState::Playing))
                    .in_set(GameMenuSet::Toggle),
            )
            .add_system(
                button_system
                    .in_base_set(GameSet::Input)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq, SystemSet)]
pub(crate) enum GameMenuSet {
    Toggle,
}

pub(crate) struct ToggleGameMenu;

#[derive(Component)]
struct PopUpMenu;

#[derive(Component, Clone, Copy)]
enum ButtonAction {
    Quit,
}

impl fmt::Display for ButtonAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Quit => write!(f, "Quit Game"),
        }
    }
}

fn setup(mut commands: GuiCommands) {
    let root_node = commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                position: UiRect::all(Val::Percent(0.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            visibility: Visibility::Hidden,
            z_index: ZIndex::Local(1000),
            ..default()
        })
        .insert((PopUpMenu, InteractionBlocker))
        .id();

    let menu_node = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(25.), Val::Percent(50.)),
                padding: UiRect::horizontal(Val::Percent(1.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .id();
    commands.entity(root_node).add_child(menu_node);

    button(&mut commands, menu_node, ButtonAction::Quit);
}

fn button(commands: &mut GuiCommands, parent: Entity, action: ButtonAction) {
    let button = commands
        .spawn_button(
            OuterStyle {
                size: Size::new(Val::Percent(100.), Val::Percent(16.)),
                margin: UiRect::new(
                    Val::Percent(0.),
                    Val::Percent(0.),
                    Val::Percent(2.),
                    Val::Percent(2.),
                ),
            },
            format!("{action}"),
        )
        .insert(action)
        .id();
    commands.entity(parent).add_child(button);
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<PopUpMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn toggle_system(
    mut events: EventReader<ToggleGameMenu>,
    mut query: Query<&mut Visibility, With<PopUpMenu>>,
) {
    if events.iter().count() % 2 == 0 {
        return;
    }

    *query.single_mut() = if query.single() == Visibility::Hidden {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
}

fn button_system(
    mut next_state: ResMut<NextState<AppState>>,
    interactions: Query<(&Interaction, &ButtonAction), Changed<Interaction>>,
) {
    for (&interaction, &action) in interactions.iter() {
        if let Interaction::Clicked = interaction {
            match action {
                ButtonAction::Quit => next_state.set(AppState::InMenu),
            }
        }
    }
}
