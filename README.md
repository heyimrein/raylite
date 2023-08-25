# raylite
Lightweight, 0-dependency raycasting in Rust

🔧**WIP: *Very* early in development, feel free to post issues/contribute!**

[Crates.io](https://crates.io/crates/raylite/0.1.1)

## 🚀 Installation
Install using cargo: `cargo add raylite`

## 🪶 quickstart
```rust
use raylite::{cast, Barrier, Ray};

fn main() {
    // Positions are differentiated here because emission direction matters
    let ray = Ray {
        position: (0., 0.),     // Emission origin position
        end_position: (2., 0.), // Emission end position
    };
    let mut bar = Barrier {
        positions: ((1., -1.), (1., 1.)), // Direction does not matter for Barriers
    };

    let result = cast(&ray, &bar); // Returns a Result<RayHit, RayFail>

    assert!(result.is_ok()); // Result is an Ok<RayHit> containing hit info

    bar = Barrier {
        positions: ((-1., -1.), (-1., 1.)), // Place barrier behind the Ray
    };

    let result = cast(&ray, &bar);
    assert!(result.is_err()); // Result is an Err<RayFail::NoHit>
}
```

## 📃 Development Todo
### Urgent
- Write explanatory docs/quickstart guide

### Maybe?
- Transfer main workflow to trait usage for extensibility
