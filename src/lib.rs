mod utils;

/// Cast a ray for collision detection, with only the consideration of a single `Barrier`.
pub fn cast(ray: &Ray, barrier: &Barrier) -> Result<RayHit, RayFailResult> {
    let ray_end = (
        ray.direction.0 * ray.distance + ray.position.0,
        ray.direction.1 * ray.distance + ray.position.1,
    );

    let den = (ray.position.0 - ray_end.0) * (barrier.positions.0 .1 - barrier.positions.1 .1)
        - (ray.position.1 - ray_end.1) * (barrier.positions.0 .0 - barrier.positions.1 .0);
    if den == 0. {
        return Err(RayFailResult::Parallel);
    }

    let t_num = (ray.position.0 - barrier.positions.0 .0)
        * (barrier.positions.0 .1 - barrier.positions.1 .1)
        - (ray.position.1 - barrier.positions.0 .1)
            * (barrier.positions.0 .0 - barrier.positions.1 .0);
    let u_num = (ray.position.0 - barrier.positions.0 .0) * (ray.position.1 - ray_end.1)
        - (ray.position.1 - barrier.positions.0 .1) * (ray.position.0 - ray_end.1);

    let t = t_num / den;
    let u = u_num / den;

    if (t >= 0. && t <= 1.) && (u >= 0. && u <= 1.) {
        let point = (
            ray.position.0 + t * (ray_end.0 - ray.position.0),
            ray.position.1 + t * (ray_end.1 - ray.position.1),
        );

        return Ok(RayHit { position: point });
    }
    return Err(RayFailResult::NoHit);
}

/// Cast a Ray for collision detection, with the consideration of several `Barrier`'s.
///
/// `barriers` must have at least 1 element.
pub fn cast_wide(ray: &Ray, barriers: &Vec<Barrier>) -> Result<RayHit, RayFailResult> {
    if barriers.len() <= 0 {
        panic!("Barrier array cannot be empty!");
    }

    let mut min_dist: Option<f32> = None;
    let mut hit: Option<RayHit> = None;
    for bar in barriers {
        let new_hit: Option<RayHit>;
        match cast(ray, bar) {
            Ok(v) => new_hit = Some(v),
            Err(_) => continue,
        }

        let dist = utils::distance(new_hit.as_ref().unwrap().position, ray.position);
        if min_dist.is_none() || dist < min_dist.unwrap() {
            min_dist = Some(dist);
            hit = new_hit;
        }
    }

    if hit.is_some() {
        return Ok(hit.unwrap());
    }

    return Err(RayFailResult::NoHit);
}

/// Raycast failure states.
pub enum RayFailResult {
    /// *Universal*
    /// Did not hit any colliders.
    NoHit,
    /// *Exclusive to isolated checks* -> `cast()`
    /// Ray and Barrier are parallel; cannot collide.
    Parallel,
}

/// Raycast collision data.
pub struct RayHit {
    position: (f32, f32),
}

/// Raycast collision unit, the basis for all raycast collision detection.
/// Determines the conditions under which collision will be detected.
pub struct Ray {
    /// Origin position the Ray will emit from.
    position: (f32, f32),
    /// Relative emission direction from origin.
    direction: (f32, f32),
    /// Maximum emission distance.
    distance: f32,
}

/// 1-dimensional collision subject; Solid line.
/// Simplest building block for collider objects.
pub struct Barrier {
    positions: ((f32, f32), (f32, f32)),
}
