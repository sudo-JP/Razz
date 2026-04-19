use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use crate::ast::expression::BinOpKind;
use crate::ast::TypeKind;

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

    // ====== CONDITIONALS ====== 
    let mut allowed_cond = HashSet::new();
    allowed_cond.insert(TypeKind::Int);
    allowed_cond.insert(TypeKind::Float);
    allowed_cond.insert(TypeKind::Bool);

    let mut allowed_bool = HashSet::new();
    allowed_bool.insert(TypeKind::Bool);

    // Finallize
    m.insert(BinOpKind::Add, allowed_add);
    m.insert(BinOpKind::Sub, allowed_sub);
    m.insert(BinOpKind::Mult, allowed_mult);
    m.insert(BinOpKind::Div, allowed_div);
    
    // Its fine to clone here, its const 
    // + I already clone on basically other parts
    m.insert(BinOpKind::Eq, allowed_cond.clone());
    m.insert(BinOpKind::Neq, allowed_cond.clone());
    m.insert(BinOpKind::Lt, allowed_cond.clone());
    m.insert(BinOpKind::Le, allowed_cond.clone());
    m.insert(BinOpKind::Gt, allowed_cond.clone());
    m.insert(BinOpKind::Ge, allowed_cond.clone());

    m.insert(BinOpKind::And, allowed_bool.clone());
    m.insert(BinOpKind::Or, allowed_bool.clone());
    m
});
