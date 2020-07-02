mod audio;
mod renderer;
mod utils;
mod gui;

#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: [f32; 4],
    color: [f32; 4],
}

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}

fn main() {
    println!("Hello, world!");
    let _ = std::thread::spawn(move || {
        audio::test_audio();
    });
    use raqote::{DrawTarget, Point};
    let mut dt = DrawTarget::new(400, 400);

    gui::test_gui::draw_button(&mut dt, "test btn", (200., 100.), Point::new(50., 50.), 10.)
}

// use specs::*;

// #[derive(Debug)]
// struct Position {
//     x: f32,
//     y: f32,
// }

// impl Component for Position {
//     type Storage = VecStorage<Self>;
// }

// #[derive(Debug)]
// struct Velocity {
//     x: f32,
//     y: f32,
// }

// impl Component for Velocity {
//     type Storage = VecStorage<Self>;
// }

// struct HelloWorld;

// impl<'a> System<'a> for HelloWorld {
//     type SystemData = ReadStorage<'a, Position>;

//     fn run(&mut self, position: Self::SystemData) {
//         use rayon::prelude::*;
//         use specs::ParJoin;

//         position.par_join()
//         .for_each(|position|{
//             println!("Hello, {:?}", &position);
//         });
//     }
// }

// fn main() {
//     let mut world = World::new();
//     world.register::<Position>();
//     world.register::<Velocity>();

//     world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
//     world.create_entity().with(Position{ x:1.0, y:2.0}).build();

//     let mut hello_world = HelloWorld;
//     hello_world.run_now(&world);
//     world.maintain();
// }