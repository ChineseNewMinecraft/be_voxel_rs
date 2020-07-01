mod audio;
mod renderer;
mod utils;

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
    use raqote::*;
    use raqote::{
        DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source, StrokeStyle, Transform,
    };
let mut dt = DrawTarget::new(400, 400);

    let mut pb = PathBuilder::new();
    pb.move_to(100., 100.);
    pb.line_to(200., 100.);
    pb.line_to(200., 150.);
    pb.line_to(100., 150.);
    pb.line_to(100., 100.);
    pb.close();

    let path = pb.finish();
    let gradient = Source::new_linear_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.8,
                    color: Color::new(0xff, 0x66, 0xcc, 0xff),
                },
                GradientStop {
                    position: 1.,
                    color: Color::new(0xff, 0x66, 0xff, 0xcc),
                },
            ],
        },
        Point::new(100., 100.),
        Point::new(100., 150.),
        Spread::Pad,
    );
    dt.fill(&path, &gradient, &DrawOptions::new());
    let image = dt.get_data_u8();
    utils::ui_debug::ui_debug(&image, (400,400));
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