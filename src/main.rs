#![warn(clippy::pedantic)]

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

const DRAGON_FRAMES: [u16; 6] = [64, 1, 2, 3, 2, 1];
use prelude::*;

struct Silo {
    x: i32,
    y: f32,
    frame: usize,
}

impl Silo {
    fn new(x: i32, y: f32) -> Self {
        Silo { x, y, frame: 0 }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // ctx.set_active_console(1);
        // ctx.cls();
        ctx.set(1, 1, YELLOW, BLACK, to_cp437('@'));
    }
}

struct State {
    ecs: World,
    silo: Silo,
    frame_time: f32,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        Self {
            ecs: ecs,
            silo: Silo::new(0, 0.0),
            frame_time: 0.0,
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.silo.render(ctx);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        render_draw_buffer(ctx).expect("Render error, muthafugga");
        self.play(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Missile Command")
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fps_cap(30.0)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()?;

    main_loop(context, State::new())
}
