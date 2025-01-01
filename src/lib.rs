use utils::distance;

mod utils;

/// Cast a ray for collision detection, with only the consideration of a single [Barrier].
pub fn cast(ray: &Ray, bar: &Barrier) -> Result<RayHit, RayFail> {
    let den = (ray.position.0 - ray.end_position.0) * (bar.positions.0 .1 - bar.positions.1 .1)
        - (ray.position.1 - ray.end_position.1) * (bar.positions.0 .0 - bar.positions.1 .0);
    if den == 0. {
        return Err(RayFail::Parallel);
    }

    let t_num = (ray.position.0 - bar.positions.0 .0) * (bar.positions.0 .1 - bar.positions.1 .1)
        - (ray.position.1 - bar.positions.0 .1) * (bar.positions.0 .0 - bar.positions.1 .0);
    let u_num = (ray.position.0 - bar.positions.0 .0) * (ray.position.1 - ray.end_position.1)
        - (ray.position.1 - bar.positions.0 .1) * (ray.position.0 - ray.end_position.0);

    let t = t_num / den;
    let u = u_num / den;

    if (t >= 0. && t <= 1.) && (u >= 0. && u <= 1.) {
        let point = (
            ray.position.0 + t * (ray.end_position.0 - ray.position.0),
            ray.position.1 + t * (ray.end_position.1 - ray.position.1),
        );

        return Ok(RayHit {
            position: point,
            distance: distance(ray.position, point),
        });
    }
    return Err(RayFail::NoHit);
}

/// Cast a Ray for collision detection, with the consideration of several [Barrier]'s.
///
/// `bars` must have at least 1 element.
///
/// The (possibly) returned hit information will represent the closest barrier to `ray`'s
/// origin point that was hit.
pub fn cast_wide(ray: &Ray, bars: &[Barrier]) -> Result<RayHit, RayFail> {
    if bars.len() <= 0 {
        panic!("Barrier vector cannot be empty!");
    }

    let mut ray_clone = ray.to_owned();

    let mut hit: Option<RayHit> = None;
    for bar in bars {
        match cast(&ray_clone, bar) {
            Ok(v) => {
                ray_clone.end_position = v.position;
                hit = Some(v);
            }
            Err(_) => continue,
        }
    }

    if hit.is_some() {
        return Ok(hit.unwrap());
    }

    return Err(RayFail::NoHit);
}

/// Raycast failure states.
pub enum RayFail {
    /// Did not hit any colliders.
    ///
    /// *Universal*
    NoHit,
    /// Ray and Barrier are parallel; cannot collide.
    ///
    /// *Exclusive to isolated checks* -> [cast]
    Parallel,
}

/// Raycast collision data.
pub struct RayHit {
    /// Position of collision point.
    pub position: (f32, f32),
    /// Distance of collision point from Ray emission origin.
    pub distance: f32,
}

/// Raycast collision unit, the basis for all raycast collision detection.
/// Determines the conditions under which collision will be detected.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// Origin position the Ray will emit from.
    pub position: (f32, f32),
    /// Position representing the end of the Ray.
    pub end_position: (f32, f32),
}

/// 1-dimensional collision subject; Solid line.
/// Simplest building block for collider objects.
#[derive(Debug, Clone, Copy)]
pub struct Barrier {
    pub positions: ((f32, f32), (f32, f32)),
}
