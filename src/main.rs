#![warn(clippy::pedantic)]

// Once we reach a point where ECS is desired, look at the part of Hands On Rust
// where the change-over is made to ECS.
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 90;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct Silo {
    x: i32,
    y: i32,
}

impl Silo {
    fn new(x: i32, y: i32) -> Self {
        Silo { x, y }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // ctx.set_active_console(1);
        // ctx.cls();
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('S'));
    }
}

struct Missile {
    x: i32,
    y: i32,
}

impl Missile {
    fn new(x: i32, y: i32) -> Self {
        Missile { x, y }
    }

    fn spawn_enemy_missile(rng: &mut RandomNumberGenerator, enemy_missiles: &mut Vec<Missile>) {
        let x = rng.range(0, SCREEN_WIDTH);
        let missile = Missile { x, y: 0 };
        enemy_missiles.push(missile);
    }

    fn fly(&mut self) {
        self.y += 1;
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // ctx.set_active_console(1);
        // ctx.cls();
        ctx.set(self.x, self.y, RED, BLACK, to_cp437('M'));
    }
}

struct State {
    ecs: World,
    silos: Vec<Silo>,
    rng: RandomNumberGenerator,
    enemy_missiles: Vec<Missile>,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let silos = vec![Silo::new(15, 48), Silo::new(45, 48), Silo::new(75, 48)];
        let mut enemy_missiles: Vec<Missile> = Vec::new();
        let mut rng = RandomNumberGenerator::new();
        Self {
            ecs,
            silos,
            rng,
            enemy_missiles,
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLACK);
        for silo in &mut self.silos {
            silo.render(ctx);
        }
        for missile in &mut self.enemy_missiles {
            missile.render(ctx);
            missile.fly();
        }
        self.enemy_missiles
            .retain(|missile| missile.y < SCREEN_HEIGHT);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        let mouse = ctx.mouse_point();
        if ctx.left_click {
            self.enemy_missiles
                .retain(|missile| missile.x != mouse.x || missile.y != mouse.y);
        }
        render_draw_buffer(ctx).expect("Render error, muthafugga");
        if self.enemy_missiles.len() < 1 {
            Missile::spawn_enemy_missile(&mut self.rng, &mut self.enemy_missiles);
        }
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
