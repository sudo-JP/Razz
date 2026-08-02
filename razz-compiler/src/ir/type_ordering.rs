use std::collections::HashMap;
use std::sync::LazyLock;

use crate::ast::SpecificTypeKind;

pub static SPECIFIC_TYPE_ORDERING: LazyLock<HashMap<SpecificTypeKind, Vec<&'static str>>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();

        map.insert(SpecificTypeKind::Vec3, vec!["x", "y", "z"]);
        map.insert(SpecificTypeKind::Point3, vec!["x", "y", "z"]);
        map.insert(SpecificTypeKind::Color, vec!["r", "g", "b"]);

        map.insert(SpecificTypeKind::Dielectric, vec!["refractionIdx"]);
        map.insert(SpecificTypeKind::Lambertian, vec!["albedo"]);
        map.insert(SpecificTypeKind::Metal, vec!["albedo", "fuzz"]);

        map.insert(SpecificTypeKind::Background, vec!["top", "bottom"]);
        map.insert(
            SpecificTypeKind::Camera,
            vec![
                "lookfrom",
                "lookat",
                "vfov",
                "vup",
                "focus_dist",
                "defocus_angle",
            ],
        );
        map.insert(SpecificTypeKind::Sphere, vec!["coord", "radius", "material"]);
        map.insert(SpecificTypeKind::Image, vec!["width", "height"]);
        map.insert(SpecificTypeKind::Output, vec!["type", "file"]);

        map.insert(SpecificTypeKind::Arduino, vec![]);
        map.insert(SpecificTypeKind::PPM, vec![]);
        map.insert(SpecificTypeKind::OutputType, vec![]);
        map.insert(SpecificTypeKind::Material, vec![]);

        map
    }
);
