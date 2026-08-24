use crate::parser::{
    parse_tree::{expr::Expr, ty::Ty},
    source::Spanned,
};

pub struct SliceTy<'a> {
    pub base: Box<Spanned<'a, Ty<'a>>>,
    pub len: Option<Spanned<'a, Expr<'a>>>,
}
