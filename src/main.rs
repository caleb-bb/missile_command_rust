#![warn(clippy::pedantic)]

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {
    ecs: World,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        Self { ecs }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        render_draw_buffer(ctx).expect("Render error, muthafugga")
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Missile Command")
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fps_cap(30.0)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()?;

    main_loop(context, State::new())
}
