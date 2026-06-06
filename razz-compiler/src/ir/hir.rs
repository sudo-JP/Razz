use crate::ast::TypeKind;


pub struct HIRFunctionParam {
    pub name: String, 
    pub ty: TypeKind,
}
