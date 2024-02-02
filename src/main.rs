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
    y: i32,
    frame: usize,
}

impl Silo {
    fn new(x: i32, y: i32) -> Self {
        Silo { x, y, frame: 0 }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // ctx.set_active_console(1);
        // ctx.cls();
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('S'));
    }
}

struct State {
    ecs: World,
    silos: Vec<Silo>,
    frame_time: f32,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let silos = vec![Silo::new(10, 49), Silo::new(40, 49), Silo::new(70, 49)];
        Self {
            ecs: ecs,
            silos: silos,
            frame_time: 0.0,
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLACK);
        for silo in &mut self.silos {
            silo.render(ctx);
        }
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
