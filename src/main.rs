// https://github.com/asny/three-d/blob/master/examples/instanced_shapes/src/main.rs

use three_d::*;

pub fn main() {

    let side_count = 10;

    let window = Window::new(WindowSettings {
        title: "Test ThreeD".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(60.0, 50.0, 60.0),
         vec3(0.0, 0.0, -0.5),
         vec3(0.0, 1.0, 0.0),
         degrees(45.0),
         0.1,
         1000.0,
    );

    let mut control = OrbitControl::new(camera.target(), 1.0, 1000.0);

    let light0: DirectionalLight = DirectionalLight::new(&context, 1.0, Srgba::WHITE, vec3(0.0, -0.5, -0.5));
    let light1: DirectionalLight = DirectionalLight::new(&context, 2.0, Srgba::WHITE, vec3(0.0, 0.5, 0.5));

    let mut instanced_mesh = Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &CpuMesh::cube()),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Srgba { r: 128, g: 128, b: 128, a: 255 },
                ..Default::default()
            },
        ),
    );


    let count = side_count * side_count * side_count;
    instanced_mesh.set_instances(&Instances {
        transformations: (0..count)
            .map(|i| {
                let x = i % side_count;
                let y = (i / side_count) % side_count;
                let z = i / (side_count * side_count);
            
                Mat4::from_translation(vec3(3.0 * x as f32 - 1.5 * side_count as f32, 3.0 * y as f32 - 1.5 * side_count as f32, 3.0 * z as f32 - 1.5 * side_count as f32))
            })
            .collect(),
        colors: Some(
            (0..count)
            .map(|i| {
                let x = i % side_count;
                let y = (i / side_count) % side_count;
                let z = i / (side_count * side_count);
                let r = (x as f32 / side_count as f32 * 255.0) as u8 + 26;
                let g = (y as f32 / side_count as f32 * 255.0) as u8 + 26;
                let b = (z as f32 / side_count as f32 * 255.0) as u8 + 26;
                Srgba::new(r, g, b, 255)
            })
            .collect(),
        ),
        ..Instances::default()
    });
        

    window.render_loop(move | mut frame_input | {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);




        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                &instanced_mesh,
                &[&light0, &light1],
            );

        FrameOutput::default()
    });
    

}