use bevy::math::{IVec3, Vec3A};

// (position, normal)
#[derive(Debug)]
pub struct Hit {
    pub position: Vec3A,
    pub normal: Vec3A,
}

pub fn raytrace_dda<F>(
    ray_pos: Vec3A,
    ray_dir: Vec3A,
    raytrace_length: i32,
    mut hit_evaluator: F,
) -> Option<Hit>
where
    F: FnMut(IVec3) -> bool,
{
    let mut hit = Hit {
        position: Vec3A::ZERO,
        normal: Vec3A::ZERO,
    };

    let step_sizes = 1.0 / ray_dir.abs();
    let step_dir = ray_dir.signum();
    let mut next_dist = (step_dir * 0.5 + 0.5 - ray_pos.fract()) / ray_dir;

    let mut voxel_pos = ray_pos.floor();
    let mut current_pos = ray_pos;

    for _ in 0..raytrace_length {
        let closest_dist = next_dist.min_element();

        current_pos += ray_dir * closest_dist;

        let step_axis = Vec3A::new(
            if next_dist.x <= closest_dist {
                1.0
            } else {
                0.0
            },
            if next_dist.y <= closest_dist {
                1.0
            } else {
                0.0
            },
            if next_dist.z <= closest_dist {
                1.0
            } else {
                0.0
            },
        );

        voxel_pos += step_axis * step_dir;

        next_dist -= closest_dist;
        next_dist += step_sizes * step_axis;

        hit.normal = step_axis;

        if hit_evaluator(voxel_pos.floor().as_ivec3() + 1) {
            hit.position = current_pos;
            hit.normal *= -step_dir;

            return Some(hit);
        }
    }

    None
}

pub fn simple_stupid_raytrace<F>(
    ray_pos: Vec3A,
    ray_dir: Vec3A,
    max_dist: f32,
    num_steps: usize,
    mut hit_evaluator: F,
) -> Option<Hit>
where
    F: FnMut(IVec3) -> bool,
{
    let start_pos = ray_pos;
    let end_pos = ray_pos + ray_dir * max_dist;

    let ray_step = (end_pos - start_pos) / num_steps as f32;

    for i in 0..num_steps {
        let current_pos = start_pos + ray_step * i as f32;

        if hit_evaluator(current_pos.as_ivec3()) {
            return Some(Hit {
                position: current_pos,
                normal: Vec3A::Y,
            });
        }
    }

    None
}
