pub mod material;
pub mod hitable;
pub mod hitable_list;
pub mod sphere;
pub mod hitable_library;

pub use self::hitable::{HitRecord, Hitable, RayCastResult, HitableID};
pub use self::material::{Material, MaterialID, MaterialLibrary, Lambertian, Metal, Deilectric, ScatterHit, Sky, CheckerBoard};
pub use self::hitable_list::HitableList;
pub use self::hitable_library::HitableLibrary;
pub use self::sphere::Sphere;