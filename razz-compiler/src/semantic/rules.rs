use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;


use crate::ast::expression::{BinOpKind, EndpointKind};
use crate::ast::statement::HTTPMethodKind;
use crate::ast::{SpecificTypeKind, TypeKind};

pub static ENDPOINT_MAP: LazyLock<HashMap<HTTPMethodKind, HashMap<EndpointKind, HashSet<SpecificTypeKind>>>> = LazyLock::new(|| {
    let mut m = HashMap::new(); 

    // POST 
    let mut post: HashMap<EndpointKind, HashSet<SpecificTypeKind>> = HashMap::new();
    let mut hittable_post: HashSet<SpecificTypeKind> = HashSet::new();
    hittable_post.insert(SpecificTypeKind::Sphere);
    post.insert(EndpointKind::Hittable, hittable_post);

    // PUT
    let mut put: HashMap<EndpointKind, HashSet<SpecificTypeKind>> = HashMap::new();

    let mut camera_put: HashSet<SpecificTypeKind> = HashSet::new();
    camera_put.insert(SpecificTypeKind::Camera);

    let mut bg_put: HashSet<SpecificTypeKind> = HashSet::new();
    bg_put.insert(SpecificTypeKind::Background);

    let mut img_put: HashSet<SpecificTypeKind> = HashSet::new();
    img_put.insert(SpecificTypeKind::Image);

    let mut out_put: HashSet<SpecificTypeKind> = HashSet::new();
    out_put.insert(SpecificTypeKind::Output);
    
    put.insert(EndpointKind::Camera, camera_put);
    put.insert(EndpointKind::Background, bg_put);
    put.insert(EndpointKind::Image, img_put);
    put.insert(EndpointKind::Output, out_put);

    // PATCH 
    let mut patch: HashMap<EndpointKind, HashSet<SpecificTypeKind>> = HashMap::new();

    let mut camera_patch: HashSet<SpecificTypeKind> = HashSet::new();
    camera_patch.insert(SpecificTypeKind::Camera);

    let mut bg_patch: HashSet<SpecificTypeKind> = HashSet::new();
    bg_patch.insert(SpecificTypeKind::Background);

    let mut img_patch: HashSet<SpecificTypeKind> = HashSet::new();
    img_patch.insert(SpecificTypeKind::Image);

    let mut out_patch: HashSet<SpecificTypeKind> = HashSet::new();
    out_patch.insert(SpecificTypeKind::Output);

    patch.insert(EndpointKind::Camera, camera_patch);
    patch.insert(EndpointKind::Background, bg_patch);
    patch.insert(EndpointKind::Image, img_patch);
    patch.insert(EndpointKind::Output, out_patch);

    m.insert(HTTPMethodKind::Post, post);
    m.insert(HTTPMethodKind::Put, put);
    m.insert(HTTPMethodKind::Patch, patch);
    m
});

pub static FIELD_ACCESS_MAP: LazyLock<HashMap<SpecificTypeKind, HashMap<&str, TypeKind>>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    let mut camera: HashMap<&str, TypeKind> = HashMap::new();
    camera.insert("lookfrom", TypeKind::SpecificType(SpecificTypeKind::Point3));
    camera.insert("lookat", TypeKind::SpecificType(SpecificTypeKind::Point3));
    camera.insert("vfov", TypeKind::Float);
    camera.insert("vup", TypeKind::SpecificType(SpecificTypeKind::Vec3));
    camera.insert("focus_dist", TypeKind::Float);
    camera.insert("defocus_angle", TypeKind::Float);
    
    let mut image: HashMap<&str, TypeKind> = HashMap::new();
    image.insert("width", TypeKind::Int);
    image.insert("height", TypeKind::Int);

    let mut background: HashMap<&str, TypeKind> = HashMap::new();
    background.insert("top", TypeKind::SpecificType(SpecificTypeKind::Vec3));
    background.insert("bottom", TypeKind::SpecificType(SpecificTypeKind::Vec3));

    let mut output: HashMap<&str, TypeKind> = HashMap::new();
    output.insert("type", TypeKind::SpecificType(SpecificTypeKind::OutputType));
    output.insert("file", TypeKind::String);

    let mut sphere: HashMap<&str, TypeKind> = HashMap::new();
    sphere.insert("coord", TypeKind::SpecificType(SpecificTypeKind::Vec3));
    sphere.insert("radius", TypeKind::Float);
    sphere.insert("material", TypeKind::SpecificType(SpecificTypeKind::Material));

    let mut vec3: HashMap<&str, TypeKind> = HashMap::new(); 
    vec3.insert("x", TypeKind::Float);
    vec3.insert("y", TypeKind::Float);
    vec3.insert("z", TypeKind::Float);

    let mut color: HashMap<&str, TypeKind> = HashMap::new();
    color.insert("r", TypeKind::Int);
    color.insert("g", TypeKind::Int);
    color.insert("b", TypeKind::Int);

    let mut lambertian: HashMap<&str, TypeKind> = HashMap::new();
    lambertian.insert("albedo", TypeKind::SpecificType(SpecificTypeKind::Color));

    let mut dielectric: HashMap<&str, TypeKind> = HashMap::new();
    dielectric.insert("refractionIdx", TypeKind::Float);

    let mut metal: HashMap<&str, TypeKind> = HashMap::new();
    metal.insert("albedo", TypeKind::SpecificType(SpecificTypeKind::Color));
    metal.insert("fuzz", TypeKind::Float);


    m.insert(SpecificTypeKind::Camera, camera);
    m.insert(SpecificTypeKind::Image, image);
    m.insert(SpecificTypeKind::Background, background);
    m.insert(SpecificTypeKind::Output, output);
    m.insert(SpecificTypeKind::Sphere, sphere);
    m.insert(SpecificTypeKind::Vec3, vec3.clone());
    m.insert(SpecificTypeKind::Point3, vec3);
    m.insert(SpecificTypeKind::Color, color);
    m.insert(SpecificTypeKind::Lambertian, lambertian);
    m.insert(SpecificTypeKind::Dielectric, dielectric);
    m.insert(SpecificTypeKind::Metal, metal);
    m
});

pub static BINOP_MAP: LazyLock<HashMap<BinOpKind, HashSet<TypeKind>>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // ====== ARITHMETIC ====== 
    // Allowed `+` operations
    let mut allowed_add = HashSet::new();
    allowed_add.insert(TypeKind::Int);
    allowed_add.insert(TypeKind::Float);
    allowed_add.insert(TypeKind::String); 

    // Allowed `-` operations
    let mut allowed_sub = HashSet::new();
    allowed_sub.insert(TypeKind::Int);
    allowed_sub.insert(TypeKind::Float);

    // Allowed `*` operations
    let mut allowed_mult = HashSet::new();
    allowed_mult.insert(TypeKind::Int);
    allowed_mult.insert(TypeKind::Float);

    // Allowed `/` operations
    let mut allowed_div = HashSet::new();
    allowed_div.insert(TypeKind::Int);
    allowed_div.insert(TypeKind::Float);


    // ====== INEQUALITY ====== 
    let mut allowed_ineq = HashSet::new();
    allowed_ineq.insert(TypeKind::Int);
    allowed_ineq.insert(TypeKind::Float);

    let mut allowed_bool = HashSet::new();
    allowed_bool.insert(TypeKind::Bool);

    let mut allowed_equality = HashSet::new();
    allowed_equality.insert(TypeKind::Bool);
    allowed_equality.insert(TypeKind::Int);
    allowed_equality.insert(TypeKind::Float);
    allowed_equality.insert(TypeKind::String);
    allowed_equality.insert(TypeKind::Null);
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Dielectric));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Lambertian));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Metal));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Vec3));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Point3));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Color));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Background));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Camera));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Image));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Sphere));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Output));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Arduino));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::PPM));
    allowed_equality.insert(TypeKind::SpecificType(SpecificTypeKind::Material));

    // Finallize
    m.insert(BinOpKind::Add, allowed_add);
    m.insert(BinOpKind::Sub, allowed_sub);
    m.insert(BinOpKind::Mult, allowed_mult);
    m.insert(BinOpKind::Div, allowed_div);
    
    // Its fine to clone here, its const 
    // + I already clone on basically other parts
    m.insert(BinOpKind::Lt, allowed_ineq.clone());
    m.insert(BinOpKind::Le, allowed_ineq.clone());
    m.insert(BinOpKind::Gt, allowed_ineq.clone());
    m.insert(BinOpKind::Ge, allowed_ineq.clone());

    m.insert(BinOpKind::And, allowed_bool.clone());
    m.insert(BinOpKind::Or, allowed_bool.clone());
    m.insert(BinOpKind::Eq, allowed_bool.clone());
    m.insert(BinOpKind::Neq, allowed_bool.clone());
    m
});
