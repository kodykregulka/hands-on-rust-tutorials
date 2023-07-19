#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map){
	let mut movers = <(&mut Point, &MovingRandomly)>::query();
	movers.
		.iter_mut(ecs)
		.for_each(|(pos, _)| {
			
		})
}