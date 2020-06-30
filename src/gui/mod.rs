extern crate lyon;
use lyon::math::{point, Point};
use lyon::path::Path;
use lyon::path::builder::*;
use lyon::tessellation::*;
pub fn draw() {
    // Build a Path.
    let mut builder = Path::builder().with_svg();
    builder.move_to(point(0.0, 0.0));
    builder.line_to(point(1.0, 0.0));
    builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
    builder.cubic_bezier_to(point(1.0, 1.0), point(0.0, 1.0), point(0.0, 0.0));
    builder.close();//maybe wrong
    let path = builder.build();
    // use custom vertex type instead of the default one.
    #[derive(Copy, Clone, Debug)]
    struct MyVertex { position: [f32; 2] };
    // Will contain the result of the tessellation.
    let mut geometry: VertexBuffers<MyVertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();
    {
        // Compute the tessellation.
        tessellator.tessellate_path(
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut geometry, |pos: Point, _: FillAttributes| {
                MyVertex {
                    position: pos.to_array(),
                }
            }),
        ).unwrap();
    }
    // The tessellated geometry is ready to be uploaded to the GPU.
    println!(" -- {} vertices {} indices",
        geometry.vertices.len(),
        geometry.indices.len()
    );
}