use bevy::{prelude::*, render::texture};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{prelude::*, action_state};
fn main() {
    App::new().insert_resource(WindowDescriptor {
        title: "Rolling Game".into(),
        ..Default::default()
    })
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(movement)
        .run();

}
#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());

    let player0_sprite = "ball_blue_large.png";
    let player1_sprite = "ball_red_large.png";
    spawn_player(0, Vec2::new(-150.0, 0.0), player0_sprite,&mut commands, &asset_server);
    spawn_player(1, Vec2::new(150.0, 0.0), player1_sprite, &mut commands, &asset_server);

    // Spawn a triangle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(150.0, 200.0, 1.0), 
            texture: asset_server.load("block_corner.png"),
            ..Default::default()
        }) 
        .insert(Restitution::coefficient(1.0))
        .insert(Collider::triangle(
            Vec2::new(-30.0, -30.0),
            Vec2::new(-30.0, 32.0),
            Vec2::new(32.0, -30.0),
        ));
}

fn spawn_player(id: usize, location: Vec2, sprite_path: &str, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Spawn Player
    let z: f32 = 1.0;
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(location.extend(z)), 
            texture: asset_server.load(sprite_path),
            ..Default::default()
        })
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .insert(
                    if id == 0 {VirtualDPad::wasd()} else {VirtualDPad::arrow_keys()}, Action::Move)
                .set_gamepad(Gamepad { id })
                .build(),
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(32.0))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 5.0,
        })
        .insert(Restitution::coefficient(1.0))
        .insert(Player);
}

const MOVE_FORCE: f32 = 1500.0;

fn movement (
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>,
) {
    for (action_state, mut external_force) in &mut query {
        let axis_vector = action_state.clamped_axis_pair(Action::Move).unwrap().xy();
        external_force.force = axis_vector * MOVE_FORCE * time.delta_seconds();

    }
}