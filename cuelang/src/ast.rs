use crate::{try_error, try_value, Error, Result};
use cuelang_sys::{
    ast_BadDecl, ast_BadExpr, ast_BasicLit, ast_BinaryExpr, ast_BottomLit,
    ast_CallExpr, ast_Decl, ast_Expr, ast_Field, ast_File, ast_Ident,
    ast_ImportDecl, ast_ImportSpec, ast_IndexExpr, ast_Label, ast_ListLit,
    ast_Package, ast_ParenExpr, ast_SelectorExpr, ast_SliceExpr, ast_StructLit,
    ast_UnaryExpr, ffiString, parser_Option, token_Token, GoAny, GoObject,
    GoPtr, GoSlice,
};
use lazy_static::lazy_static;
use std::{convert::TryFrom, result};

lazy_static! {
    static ref EMPTY_OPTION_SLICE: GoSlice<parser_Option> =
        GoSlice::make(cuelang_sys::FileOffset(0), 0);
}

pub struct Expression {
    inner: ast_Expr,
}

impl From<ast_Expr> for Expression {
    fn from(inner: ast_Expr) -> Self {
        Self { inner }
    }
}

impl From<BinaryExpression> for Expression {
    fn from(x: BinaryExpression) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<CallExpression> for Expression {
    fn from(x: CallExpression) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<IndexExpression> for Expression {
    fn from(x: IndexExpression) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<ParenExpression> for Expression {
    fn from(x: ParenExpression) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<SelectorExpression> for Expression {
    fn from(x: SelectorExpression) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<SliceExpression> for Expression {
    fn from(x: SliceExpression) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<UnaryExpression> for Expression {
    fn from(x: UnaryExpression) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<BasicLiteral> for Expression {
    fn from(x: BasicLiteral) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<BottomLiteral> for Expression {
    fn from(x: BottomLiteral) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<ListLiteral> for Expression {
    fn from(x: ListLiteral) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<StructLiteral> for Expression {
    fn from(x: StructLiteral) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<Identifier> for Expression {
    fn from(x: Identifier) -> Self {
        ast_Expr::from_goobject(&x.inner).unwrap().into()
    }
}

pub struct BinaryExpression {
    inner: GoPtr<ast_BinaryExpr>,
}

impl BinaryExpression {
    pub fn new<L, R>(
        left_operand: L,
        operator: BinaryOperator,
        right_operand: R,
    ) -> Self
    where
        L: Into<Expression>,
        R: Into<Expression>,
    {
        let mut this = Self {
            inner: GoPtr::default(),
        };
        this.set_left_operand(left_operand)
            .set_operator(operator)
            .set_right_operand(right_operand);

        this
    }

    pub fn left_operand(&self) -> Expression {
        self.inner.getX().into()
    }

    pub fn set_left_operand<L>(&mut self, left_operand: L) -> &mut Self
    where
        L: Into<Expression>,
    {
        self.inner.setX(left_operand.into().inner);

        self
    }

    pub fn operator(&self) -> BinaryOperator {
        BinaryOperator(self.inner.getOp())
    }

    pub fn set_operator(&mut self, operator: BinaryOperator) -> &mut Self {
        self.inner.setOp(operator.0);

        self
    }

    pub fn right_operand(&self) -> Expression {
        self.inner.getY().into()
    }

    pub fn set_right_operand<R>(&mut self, right_operand: R) -> &mut Self
    where
        R: Into<Expression>,
    {
        self.inner.setY(right_operand.into().inner);

        self
    }
}

impl From<GoPtr<ast_BinaryExpr>> for BinaryExpression {
    fn from(inner: GoPtr<ast_BinaryExpr>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for BinaryExpression {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_BinaryExpr>>()
            .map(Self::from)
            .ok_or(())
    }
}

/// **TODO: make this a proper enum**
pub struct BinaryOperator(token_Token);

pub struct CallExpression {
    inner: GoPtr<ast_CallExpr>,
}

impl CallExpression {
    pub fn new<O, I, T>(object: O, arguments: I) -> Self
    where
        O: Into<Expression>,
        I: IntoIterator<Item = T>,
        T: Into<Expression>,
    {
        let mut this = Self {
            inner: GoPtr::default(),
        };
        this.set_object(object).set_arguments(arguments);

        this
    }

    pub fn object(&self) -> Expression {
        self.inner.getFun().into()
    }

    pub fn set_object<O>(&mut self, object: O) -> &mut Self
    where
        O: Into<Expression>,
    {
        self.inner.setFun(object.into().inner);

        self
    }

    pub fn arguments(&self) -> impl Iterator<Item = Expression> {
        self.inner.getArgs().into_iter().map(Expression::from)
    }

    pub fn set_arguments<I, T>(&mut self, arguments: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Expression>,
    {
        let mut slice = GoSlice::make(
            ast_Expr::from_goobject(&GoPtr::<ast_BadExpr>::default()).unwrap(),
            0,
        );
        for argument in arguments {
            slice.push(argument.into().inner);
        }
        self.inner.setArgs(slice);

        self
    }
}

impl From<GoPtr<ast_CallExpr>> for CallExpression {
    fn from(inner: GoPtr<ast_CallExpr>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for CallExpression {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_CallExpr>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct IndexExpression {
    inner: GoPtr<ast_IndexExpr>,
}

impl IndexExpression {
    pub fn new<O, I>(object: O, index: I) -> Self
    where
        O: Into<Expression>,
        I: Into<Expression>,
    {
        let mut this = Self {
            inner: GoPtr::default(),
        };
        this.set_object(object).set_index(index);

        this
    }

    pub fn object(&self) -> Expression {
        self.inner.getX().into()
    }

    pub fn set_object<O>(&mut self, object: O) -> &mut Self
    where
        O: Into<Expression>,
    {
        self.inner.setX(object.into().inner);

        self
    }

    pub fn index(&self) -> Expression {
        self.inner.getIndex().into()
    }

    pub fn set_index<I>(&mut self, index: I) -> &mut Self
    where
        I: Into<Expression>,
    {
        self.inner.setIndex(index.into().inner);

        self
    }
}

impl From<GoPtr<ast_IndexExpr>> for IndexExpression {
    fn from(inner: GoPtr<ast_IndexExpr>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for IndexExpression {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_IndexExpr>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct ParenExpression {
    inner: GoPtr<ast_ParenExpr>,
}

impl ParenExpression {
    pub fn new<O>(object: O) -> Self
    where
        O: Into<Expression>,
    {
        let mut this = Self {
            inner: GoPtr::default(),
        };
        this.set_object(object);

        this
    }

    pub fn object(&self) -> Expression {
        self.inner.getX().into()
    }

    pub fn set_object<O>(&mut self, object: O) -> &mut Self
    where
        O: Into<Expression>,
    {
        self.inner.setX(object.into().inner);

        self
    }
}

impl From<GoPtr<ast_ParenExpr>> for ParenExpression {
    fn from(inner: GoPtr<ast_ParenExpr>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for ParenExpression {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_ParenExpr>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct SelectorExpression {
    inner: GoPtr<ast_SelectorExpr>,
}

impl SelectorExpression {
    pub fn new<O>(object: O, selector: &str) -> Self
    where
        O: Into<Expression>,
    {
        let mut this = Self {
            inner: GoPtr::default(),
        };
        this.set_object(object).set_selector(selector);

        this
    }

    pub fn object(&self) -> Expression {
        self.inner.getX().into()
    }

    pub fn set_object<O>(&mut self, object: O) -> &mut Self
    where
        O: Into<Expression>,
    {
        self.inner.setX(object.into().inner);

        self
    }

    pub fn selector(&self) -> ffiString {
        self.inner
            .getSel()
            .cast::<GoPtr<ast_Ident>>()
            .unwrap()
            .getName()
    }

    pub fn set_selector(&mut self, selector: &str) -> &mut Self {
        let mut selector_ident = GoPtr::<ast_Ident>::default();
        selector_ident.setName(selector);
        self.inner
            .setSel(ast_Label::from_goobject(&selector_ident).unwrap());

        self
    }
}

impl From<GoPtr<ast_SelectorExpr>> for SelectorExpression {
    fn from(inner: GoPtr<ast_SelectorExpr>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for SelectorExpression {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_SelectorExpr>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct SliceExpression {
    inner: GoPtr<ast_SliceExpr>,
}

impl SliceExpression {}

impl From<GoPtr<ast_SliceExpr>> for SliceExpression {
    fn from(inner: GoPtr<ast_SliceExpr>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for SliceExpression {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_SliceExpr>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct UnaryExpression {
    inner: GoPtr<ast_UnaryExpr>,
}

impl UnaryExpression {}

impl From<GoPtr<ast_UnaryExpr>> for UnaryExpression {
    fn from(inner: GoPtr<ast_UnaryExpr>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for UnaryExpression {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_UnaryExpr>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct BasicLiteral {
    inner: GoPtr<ast_BasicLit>,
}

impl BasicLiteral {}

impl From<GoPtr<ast_BasicLit>> for BasicLiteral {
    fn from(inner: GoPtr<ast_BasicLit>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for BasicLiteral {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_BasicLit>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct BottomLiteral {
    inner: GoPtr<ast_BottomLit>,
}

impl BottomLiteral {}

impl From<GoPtr<ast_BottomLit>> for BottomLiteral {
    fn from(inner: GoPtr<ast_BottomLit>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for BottomLiteral {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_BottomLit>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct ListLiteral {
    inner: GoPtr<ast_ListLit>,
}

impl ListLiteral {}

impl From<GoPtr<ast_ListLit>> for ListLiteral {
    fn from(inner: GoPtr<ast_ListLit>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for ListLiteral {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_ListLit>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct StructLiteral {
    inner: GoPtr<ast_StructLit>,
}

impl StructLiteral {}

impl From<GoPtr<ast_StructLit>> for StructLiteral {
    fn from(inner: GoPtr<ast_StructLit>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for StructLiteral {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_StructLit>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct Identifier {
    inner: GoPtr<ast_Ident>,
}

impl Identifier {
    pub fn new(object: &str) -> Self {
        let mut this = Self {
            inner: GoPtr::default(),
        };
        this.set_object(object);

        this
    }

    pub fn object(&self) -> ffiString {
        self.inner.getName()
    }

    pub fn set_object(&mut self, object: &str) -> &mut Self {
        self.inner.setName(object);

        self
    }
}

impl From<GoPtr<ast_Ident>> for Identifier {
    fn from(inner: GoPtr<ast_Ident>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Expression> for Identifier {
    type Error = ();

    fn try_from(x: &'a Expression) -> result::Result<Self, Self::Error> {
        x.inner.cast::<GoPtr<ast_Ident>>().map(Self::from).ok_or(())
    }
}

pub struct Declaration {
    inner: ast_Decl,
}

impl From<ast_Decl> for Declaration {
    fn from(inner: ast_Decl) -> Self {
        Self { inner }
    }
}

impl From<FieldDeclaration> for Declaration {
    fn from(x: FieldDeclaration) -> Self {
        ast_Decl::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<ImportDeclaration> for Declaration {
    fn from(x: ImportDeclaration) -> Self {
        ast_Decl::from_goobject(&x.inner).unwrap().into()
    }
}

impl From<PackageDeclaration> for Declaration {
    fn from(x: PackageDeclaration) -> Self {
        ast_Decl::from_goobject(&x.inner).unwrap().into()
    }
}

pub struct FieldDeclaration {
    inner: GoPtr<ast_Field>,
}

impl FieldDeclaration {
    pub fn new<V>(name: &str, value: V) -> Self
    where
        V: Into<Expression>,
    {
        let mut this = Self {
            inner: GoPtr::default(),
        };
        this.set_name(name).set_value(value);

        this
    }

    pub fn name(&self) -> ffiString {
        self.inner
            .getLabel()
            .cast::<GoPtr<ast_Ident>>()
            .unwrap()
            .getName()
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        let mut name_ident = GoPtr::<ast_Ident>::default();
        name_ident.setName(name);
        self.inner
            .setLabel(ast_Label::from_goobject(&name_ident).unwrap());

        self
    }

    pub fn value(&self) -> Expression {
        self.inner.getValue().into()
    }

    pub fn set_value<V>(&mut self, value: V) -> &mut Self
    where
        V: Into<Expression>,
    {
        self.inner.setValue(value.into().inner);

        self
    }
}

impl From<GoPtr<ast_Field>> for FieldDeclaration {
    fn from(inner: GoPtr<ast_Field>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Declaration> for FieldDeclaration {
    type Error = ();

    fn try_from(x: &'a Declaration) -> result::Result<Self, Self::Error> {
        x.inner.cast::<GoPtr<ast_Field>>().map(Self::from).ok_or(())
    }
}

pub struct ImportDeclaration {
    inner: GoPtr<ast_ImportDecl>,
}

impl ImportDeclaration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, alias: Option<&str>, path: &str) {
        let mut path_lit = GoPtr::<ast_BasicLit>::default();
        path_lit.setValue(path);
        let mut import_spec = GoPtr::<ast_ImportSpec>::default();
        import_spec.setPath(path_lit);
        if let Some(alias) = alias {
            let mut alias_ident = GoPtr::<ast_Ident>::default();
            alias_ident.setName(alias);
            import_spec.setName(alias_ident);
        }

        self.inner.getSpecs().push(import_spec);
    }

    pub fn iter(&self) -> impl Iterator<Item = (String, String)> {
        self.inner.getSpecs().into_iter().map(|x| {
            let path = x.getPath().getValue();
            let path = path[1..path.len() - 1].to_string();
            let alias = {
                let alias = x.getName();

                if alias.is_nil() {
                    path.split('/')
                        .filter(|x| !x.is_empty())
                        .last()
                        .expect("import path is empty")
                        .to_string()
                } else {
                    alias.getName().to_string()
                }
            };

            (alias, path)
        })
    }
}

impl Default for ImportDeclaration {
    fn default() -> Self {
        let mut inner = GoPtr::<ast_ImportDecl>::default();
        inner.setSpecs(GoSlice::new());

        Self { inner }
    }
}

impl From<GoPtr<ast_ImportDecl>> for ImportDeclaration {
    fn from(inner: GoPtr<ast_ImportDecl>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Declaration> for ImportDeclaration {
    type Error = ();

    fn try_from(x: &'a Declaration) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_ImportDecl>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct PackageDeclaration {
    inner: GoPtr<ast_Package>,
}

impl PackageDeclaration {
    pub fn new(name: &str) -> Self {
        let mut name_ident = GoPtr::<ast_Ident>::default();
        name_ident.setName(name);
        let mut inner = GoPtr::<ast_Package>::default();
        inner.setName(name_ident);

        Self { inner }
    }

    pub fn name(&self) -> ffiString {
        self.inner.getName().getName()
    }
}

impl From<GoPtr<ast_Package>> for PackageDeclaration {
    fn from(inner: GoPtr<ast_Package>) -> Self {
        Self { inner }
    }
}

impl<'a> TryFrom<&'a Declaration> for PackageDeclaration {
    type Error = ();

    fn try_from(x: &'a Declaration) -> result::Result<Self, Self::Error> {
        x.inner
            .cast::<GoPtr<ast_Package>>()
            .map(Self::from)
            .ok_or(())
    }
}

pub struct File {
    inner: GoPtr<ast_File>,
}

impl File {
    pub fn new<I, T>(declarations: I) -> Result<Self>
    where
        I: IntoIterator<Item = T>,
        T: Into<Declaration>,
    {
        let mut this = Self {
            inner: GoPtr::default(),
        };
        this.set_declarations(declarations)?;

        Ok(this)
    }

    pub fn parse(name: &str, source: &str) -> Result<Self> {
        try_value(
            cuelang_sys::ParseFile(
                name,
                &GoAny::from_str(source),
                &EMPTY_OPTION_SLICE,
            ),
            Error::Parse,
        )
        .map(|inner| Self { inner })
    }

    pub(crate) fn inner(&self) -> &GoPtr<ast_File> {
        &self.inner
    }

    pub fn declarations(&self) -> impl Iterator<Item = Declaration> {
        self.inner.getDecls().into_iter().map(Declaration::from)
    }

    pub fn set_declarations<I, T>(
        &mut self,
        declarations: I,
    ) -> Result<&mut Self>
    where
        I: IntoIterator<Item = T>,
        T: Into<Declaration>,
    {
        let mut slice = GoSlice::make(
            ast_Decl::from_goobject(&GoPtr::<ast_BadDecl>::default()).unwrap(),
            0,
        );
        for declaration in declarations {
            slice.push(declaration.into().inner)
        }
        self.inner.setDecls(slice);

        try_error(cuelang_sys::Sanitize(&self.inner), Error::Parse)?;

        Ok(self)
    }
}
