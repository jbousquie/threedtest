// Main2.rs
// Instanced cubes with colors
// Based on the example from the three-d crate
// Stress test : change the variable side_count 
// Reuse of once allocated objects : mat4, vectors, etc for some intensive calculations
// Naive implementation : update an euler angle for each cube every frame, then compute the transformation matrix to apply to each instance


// https://github.com/asny/three-d/blob/master/examples/instanced_shapes/src/main.rs

// https://docs.rs/three-d/latest/three_d/


use cgmath::Euler;
use three_d::*;

pub fn main() {

    let side_count = 40;

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

    let instances = Instances::default();
    let mut instanced_mesh = Gm::new(
        InstancedMesh::new(&context, &instances, &CpuMesh::cube()),
        PhysicalMaterial::new(
            &context,
            &CpuMaterial {
                albedo: Srgba { r: 128, g: 128, b: 128, a: 255 },
                ..Default::default()
            },
        ),
    );


    let count = side_count * side_count * side_count;
    // Create colors for each cube
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
    

    // Reuse of once allocated objects
    let mut instances = Instances::default();
    instances.transformations = vec![Mat4::identity(); count as usize];
    let mut translation = Mat4::identity();
    let mut rotation = Mat4::identity();
    instances.colors = Some(colors);

    // Render loop
    window.render_loop(move | mut frame_input | {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        let time = (frame_input.accumulated_time * 0.001) as f32;
        
        // inline compute_instances to simplify the borrow management from inside the closure
        for i in 0..count {
            let x = i % side_count;
            let y = (i / side_count) % side_count;
            let z = i / (side_count * side_count);
            matrix_from_translation_to_ref(
                3.0 * x as f32 - 1.5 * side_count as f32, 3.0 * y as f32 - 1.5 * side_count as f32, 3.0 * z as f32 - 1.5 * side_count as f32, 
                &mut translation
            );
    
            let euler_angle = cgmath::Euler {
                x: cgmath::Rad(time * x as f32 * 0.3),
                y: cgmath::Rad(time * y as f32 * 0.2),
                z: cgmath::Rad(time * z as f32 * 0.1),
            };
    
            matrix_from_euler_to_ref(&euler_angle, &mut rotation);
            matrix_mul_to_ref(translation, rotation, &mut instances.transformations[i as usize]);
        }
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


// Updates the passed rotation matrix with the euler angles
fn matrix_from_euler_to_ref(src: &Euler<Rad<f32>>, dest: &mut Mat4)  {

    // Page A-2: http://ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19770024290.pdf
    let (sx, cx) = Rad::sin_cos(src.x.into());
    let (sy, cy) = Rad::sin_cos(src.y.into());
    let (sz, cz) = Rad::sin_cos(src.z.into());

    dest.x[0] = cy * cz;
    dest.x[1] = cx * sz + sx * sy * cz;
    dest.x[2] = sx * sz - cx * sy * cz;
    dest.x[3] = 0.0;
    dest.y[0] = -cy * sz;
    dest.y[1] = cx * cz - sx * sy * sz;
    dest.y[2] = sx * cz + cx * sy * sz;
    dest.y[3] = 0.0;
    dest.z[0] = sy;
    dest.z[1] = -sx * cy;
    dest.z[2] = cx * cy;
    dest.z[3] = 0.0;
    dest.w[0] = 0.0;
    dest.w[1] = 0.0;
    dest.w[2] = 0.0;
    dest.w[3] = 1.0;

}

// Updates the passed identity matrix with the translation vector coordinates
fn matrix_from_translation_to_ref(x: f32, y: f32, z: f32, dest: &mut Mat4) {
    dest.w[0] = x;
    dest.w[1] = y;
    dest.w[2] = z;
}

// Updates the passed matrix dest with the multiplication of two matrices l and r
fn matrix_mul_to_ref(l: Mat4, r: Mat4, dest: &mut Mat4) {

    let l00 = l.x[0];
    let l01 = l.x[1];
    let l02 = l.x[2];
    let l03 = l.x[3];
    let l10 = l.y[0];
    let l11 = l.y[1];
    let l12 = l.y[2];
    let l13 = l.y[3];
    let l20 = l.z[0];
    let l21 = l.z[1];
    let l22 = l.z[2];
    let l23 = l.z[3];
    let l30 = l.w[0];
    let l31 = l.w[1];
    let l32 = l.w[2];
    let l33 = l.w[3];
    
    let r00 = r.x[0];
    let r01 = r.x[1];
    let r02 = r.x[2];
    let r03 = r.x[3];
    let r10 = r.y[0];
    let r11 = r.y[1];
    let r12 = r.y[2];
    let r13 = r.y[3];
    let r20 = r.z[0];
    let r21 = r.z[1];
    let r22 = r.z[2];
    let r23 = r.z[3];
    let r30 = r.w[0];
    let r31 = r.w[1];
    let r32 = r.w[2];
    let r33 = r.w[3];
    
    dest.x[0] = l00 * r00 + l10 * r01 + l20 * r02 + l30 * r03;
    dest.x[1] = l01 * r00 + l11 * r01 + l21 * r02 + l31 * r03;
    dest.x[2] = l02 * r00 + l12 * r01 + l22 * r02 + l32 * r03;
    dest.x[3] = l03 * r00 + l13 * r01 + l23 * r02 + l33 * r03;
    dest.y[0] = l00 * r10 + l10 * r11 + l20 * r12 + l30 * r13;  
    dest.y[1] = l01 * r10 + l11 * r11 + l21 * r12 + l31 * r13;
    dest.y[2] = l02 * r10 + l12 * r11 + l22 * r12 + l32 * r13;
    dest.y[3] = l03 * r10 + l13 * r11 + l23 * r12 + l33 * r13;
    dest.z[0] = l00 * r20 + l10 * r21 + l20 * r22 + l30 * r23;
    dest.z[1] = l01 * r20 + l11 * r21 + l21 * r22 + l31 * r23;
    dest.z[2] = l02 * r20 + l12 * r21 + l22 * r22 + l32 * r23;
    dest.z[3] = l03 * r20 + l13 * r21 + l23 * r22 + l33 * r23;
    dest.w[0] = l00 * r30 + l10 * r31 + l20 * r32 + l30 * r33;
    dest.w[1] = l01 * r30 + l11 * r31 + l21 * r32 + l31 * r33;
    dest.w[2] = l02 * r30 + l12 * r31 + l22 * r32 + l32 * r33;
    dest.w[3] = l03 * r30 + l13 * r31 + l23 * r32 + l33 * r33;

}

