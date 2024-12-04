// Main1.rs : Test ThreeD
// Instanced cubes with colors
// Based on the example from the three-d crate
// Stress test : change the variable side_count 
// New allocation (mat4, euler, color, vec3, etc) for each cube every frame
// Naive implementation : update euler angles for each cube every frame, then compute the transformation matrix to apply to each instance

// https://github.com/asny/three-d/blob/master/examples/instanced_shapes/src/main.rs

// https://docs.rs/three-d/latest/three_d/


use three_d::*;

pub fn main() {

    // Fix this number for stress testing
    let side_count = 40;

    let window = Window::new(WindowSettings {
        title: "Test ThreeD".to_string(),
        max_size: Some((1440, 960)),
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

    // Instanced mesh creation
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

    // Create colors for each cube
    let count = side_count * side_count * side_count;
    let mut colors: Vec<Srgba> = Vec::new();
    for i in 0..count {
        let x = i % side_count;
        let y = (i / side_count) % side_count;
        let z = i / (side_count * side_count);
        let r = (x as f32 / side_count as f32 * 255.0) as u8 ;
        let g = (y as f32 / side_count as f32 * 255.0) as u8;
        let b = (z as f32 / side_count as f32 * 255.0) as u8;
        colors.push(Srgba::new(r, g, b, 255));
    }

        
    // Render loop
    window.render_loop(move | mut frame_input | {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);


        let time = (frame_input.accumulated_time * 0.001) as f32;
        let instances = compute_instances(count, side_count, Some(&colors), time);
        instanced_mesh.set_instances(&instances);


        let screen = frame_input.screen();
        screen.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0));
        screen.render(
                &camera,
                &instanced_mesh,
                &[&light0, &light1],
            );
        println!("frame duration = {} ms", frame_input.elapsed_time);
        FrameOutput::default()
    });
    
}


// Returns a new Instances object every frame
fn compute_instances(count: i32, side_count: i32, colors_option: Option<&Vec<Srgba>>, time: f32) -> Instances {
    let shift = 3.5;
    let half_shift = shift  * 0.5;
    let mut transformations: Vec<Mat4> = Vec::new();
    
    
    for i in 0..count {
        let x = i % side_count;
        let y = (i / side_count) % side_count;
        let z = i / (side_count * side_count);
        let translation = Mat4::from_translation(vec3(shift * x as f32 - half_shift * side_count as f32, shift * y as f32 - half_shift * side_count as f32, shift * z as f32 - half_shift * side_count as f32));

        let euler_angle = cgmath::Euler {
            x: cgmath::Rad(time * x as f32 * 0.3),
            y: cgmath::Rad(time * y as f32 * 0.2),
            z: cgmath::Rad(time * z as f32 * 0.1),
        };

        let rotation = Mat4::from(euler_angle);
        let transformation = translation * rotation;
        transformations.push(transformation);
    }
 
    Instances {
        transformations,
        colors: colors_option.cloned(),
        ..Instances::default()
    }
 
}  
