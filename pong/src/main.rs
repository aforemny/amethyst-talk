use amethyst::{
    core::transform::{Transform, TransformBundle},
    core::timing::Time,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        Camera,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
        ImageFormat,
        SpriteRender,
    },
    utils::application_root_dir,
    ecs::{Component, DenseVecStorage, Read, ReadStorage, WriteStorage, System, Join},
    assets::{AssetStorage, Loader, Handle},
    input::{InputBundle, StringBindings, InputHandler},
};

struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
        let sprite_sheet = load_sprite_sheet(world);
        initialise_paddles(world, sprite_sheet.clone());
        initialise_ball(world, sprite_sheet.clone());
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let binding_path = config_dir.join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(PaddleSystem, "paddle_system", &["input_system"])
        .with(BallSystem, "ball_system", &[])
        .with(BounceSystem, "bounce_system", &["ball_system"]);

    let mut game = Application::new(app_root, MyState, game_data)?;
    game.run();

    Ok(())
}

const ARENA_WIDTH: f32 = 100.;
const ARENA_HEIGHT: f32 = 100.;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.);

    world.create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}


pub enum Side {
    Left,
    Right,
}


pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}


const PADDLE_WIDTH: f32 = 16.;
const PADDLE_HEIGHT: f32 = 4.;


impl Paddle {
    fn new(side: Side) -> Self {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}


fn initialise_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT * 0.5;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };

    world.create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    world.create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .with(sprite_render.clone())
        .build();
}


fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_storage
    )
}


pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input, ): Self::SystemData) {
        for ( paddle, transform, ) in ( &paddles, &mut transforms, ).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    transform.prepend_translation_y(scaled_amount);
                }
            }
        }
    }
}


const BALL_VELOCITY_X: f32 = 75.;
const BALL_VELOCITY_Y: f32 = 50.;
const BALL_RADIUS: f32 = 2.;


pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}


impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}


fn initialise_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 1,
    };

    world.create_entity()
        .with(transform)
        .with(sprite_render)
        .with(Ball {
            velocity: [ BALL_VELOCITY_X, BALL_VELOCITY_Y, ],
            radius: BALL_RADIUS,
        })
        .build();
}


pub struct BallSystem;


impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, Time>,
    );

    fn run(&mut self, ( mut transforms, balls, time, ): Self::SystemData) {
        let delta_seconds = time.delta_seconds();
        for ( transform, ball, ) in ( &mut transforms, &balls, ).join() {
            transform.prepend_translation_x(ball.velocity[0] * delta_seconds);
            transform.prepend_translation_y(ball.velocity[1] * delta_seconds);
        }
    }
}


pub struct BounceSystem;


impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, ( mut balls, transforms, ): Self::SystemData) {
        for ( ball, transform, ) in ( &mut balls, &transforms, ).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if (ball_x < 0. && ball.velocity[0] < 0.)
                || (ball_x + ball.radius > ARENA_WIDTH && ball.velocity[0] > 0.)
            {
                ball.velocity[0] = -ball.velocity[0];
            }

            if (ball_y < 0. && ball.velocity[1] < 0.)
                || (ball_y + ball.radius > ARENA_HEIGHT && ball.velocity[1] > 0.)
            {
                ball.velocity[1] = -ball.velocity[1];
            }
        }
    }
}
