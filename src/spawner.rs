pub fn spawn_missile(ecs: &mut World, pos: Point) {
    ecs.push((Missile, pos))
}
