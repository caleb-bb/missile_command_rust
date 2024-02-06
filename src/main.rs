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
    missiles: Vec<Missile>,
}

impl Silo {
    fn new(x: i32, y: i32) -> Self {
        let missiles: Vec<Missile> = vec![
            Missile {
                x,
                y,
                destination_x: 0,
                destination_y: 0
            };
            9
        ];
        Silo { x, y, missiles }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // ctx.set_active_console(1);
        // ctx.cls();
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('S'));
        ctx.set(
            self.x,
            self.y + 1,
            GREEN,
            BLACK,
            to_cp437(self.missiles.len().to_string().chars().nth(0).unwrap()),
        )
    }
}

#[derive(Clone)]
struct Missile {
    x: i32,
    y: i32,
    destination_x: i32,
    destination_y: i32,
}

impl Missile {
    fn new(x: i32, y: i32, destination_x: i32, destination_y: i32) -> Self {
        Missile {
            x,
            y,
            destination_x,
            destination_y,
        }
    }

    fn spawn_enemy_missile(rng: &mut RandomNumberGenerator, enemy_missiles: &mut Vec<Missile>) {
        let x = rng.range(0, SCREEN_WIDTH);
        let missile = Missile {
            x,
            y: 0,
            destination_x: SCREEN_WIDTH / 2,
            destination_y: SCREEN_HEIGHT,
        };
        enemy_missiles.push(missile);
    }

    fn spawn_friendly_missile(
        destination_x: i32,
        destination_y: i32,
        friendly_missiles: &mut Vec<Missile>,
    ) {
        let missile = Missile {
            x: SCREEN_WIDTH / 2,
            y: SCREEN_HEIGHT,
            destination_x,
            destination_y,
        };
        friendly_missiles.push(missile)
    }

    fn fly(&mut self) {
        self.y += 1;
    }

    fn fly_to_point(&mut self) {
        let x_distance = self.destination_x - self.x;
        let y_distance = self.destination_y - self.y;

        if x_distance > y_distance {
            self.y -= 1;
            self.x -= (x_distance / y_distance)
        }
        if y_distance >= x_distance {
            self.x -= 1;
            self.y -= (y_distance / x_distance)
        }
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
    friendly_missiles: Vec<Missile>,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let silos = vec![Silo::new(15, 48), Silo::new(45, 48), Silo::new(75, 48)];
        let mut enemy_missiles: Vec<Missile> = Vec::new();
        let mut friendly_missiles: Vec<Missile> = Vec::new();
        let mut rng = RandomNumberGenerator::new();
        Self {
            ecs,
            silos,
            rng,
            enemy_missiles,
            friendly_missiles,
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
        for missile in &mut (self.friendly_missiles) {
            missile.render(ctx);
            missile.fly_to_point();
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
            Missile::spawn_friendly_missile(mouse.x, mouse.y, &mut self.friendly_missiles)
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
