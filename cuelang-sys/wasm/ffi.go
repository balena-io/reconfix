package main

import _ "cuelang.org/go/pkg"

import (
	cue "cuelang.org/go/cue"
	ast "cuelang.org/go/cue/ast"

	astutil "cuelang.org/go/cue/ast/astutil"

	build "cuelang.org/go/cue/build"

	errors "cuelang.org/go/cue/errors"

	parser "cuelang.org/go/cue/parser"

	token "cuelang.org/go/cue/token"

	fmt "fmt"

	big "math/big"

	reflect "reflect"

	sync "sync"

	js "syscall/js"
)

func defaultcue_Attribute() uint32 {
	var tmp cue.Attribute
	return register(&tmp)
}
func defaultcue_FieldInfo() uint32 {
	var tmp cue.FieldInfo
	return register(&tmp)
}
func get_cue_FieldInfo_IsDefinition(object uint32) bool {
	IsDefinition := (*get(object).(*cue.FieldInfo)).IsDefinition
	return IsDefinition
}
func set_cue_FieldInfo_IsDefinition(object uint32, IsDefinition bool) {
	(*get(object).(*cue.FieldInfo)).IsDefinition = IsDefinition
}
func get_cue_FieldInfo_IsHidden(object uint32) bool {
	IsHidden := (*get(object).(*cue.FieldInfo)).IsHidden
	return IsHidden
}
func set_cue_FieldInfo_IsHidden(object uint32, IsHidden bool) {
	(*get(object).(*cue.FieldInfo)).IsHidden = IsHidden
}
func get_cue_FieldInfo_IsOptional(object uint32) bool {
	IsOptional := (*get(object).(*cue.FieldInfo)).IsOptional
	return IsOptional
}
func set_cue_FieldInfo_IsOptional(object uint32, IsOptional bool) {
	(*get(object).(*cue.FieldInfo)).IsOptional = IsOptional
}
func get_cue_FieldInfo_Name(object uint32) string {
	Name := (*get(object).(*cue.FieldInfo)).Name
	return Name
}
func set_cue_FieldInfo_Name(object uint32, Name string) {
	(*get(object).(*cue.FieldInfo)).Name = Name
}
func get_cue_FieldInfo_Pos(object uint32) int {
	Pos := (*get(object).(*cue.FieldInfo)).Pos
	return Pos
}
func set_cue_FieldInfo_Pos(object uint32, Pos int) {
	(*get(object).(*cue.FieldInfo)).Pos = Pos
}
func get_cue_FieldInfo_Selector(object uint32) string {
	Selector := (*get(object).(*cue.FieldInfo)).Selector
	return Selector
}
func set_cue_FieldInfo_Selector(object uint32, Selector string) {
	(*get(object).(*cue.FieldInfo)).Selector = Selector
}
func get_cue_FieldInfo_Value(object uint32) uint32 {
	Value := (*get(object).(*cue.FieldInfo)).Value
	return register(&Value)
}
func set_cue_FieldInfo_Value(object uint32, Value uint32) {
	(*get(object).(*cue.FieldInfo)).Value = *get(Value).(*cue.Value)
}
func defaultcue_Instance() uint32 {
	var tmp cue.Instance
	return register(&tmp)
}
func get_cue_Instance_Dir(object uint32) string {
	Dir := (*get(object).(*cue.Instance)).Dir
	return Dir
}
func set_cue_Instance_Dir(object uint32, Dir string) {
	(*get(object).(*cue.Instance)).Dir = Dir
}
func get_cue_Instance_DisplayName(object uint32) string {
	DisplayName := (*get(object).(*cue.Instance)).DisplayName
	return DisplayName
}
func set_cue_Instance_DisplayName(object uint32, DisplayName string) {
	(*get(object).(*cue.Instance)).DisplayName = DisplayName
}
func get_cue_Instance_Err(object uint32) uint32 {
	Err := (*get(object).(*cue.Instance)).Err
	return register(&Err)
}
func set_cue_Instance_Err(object uint32, Err uint32) {
	(*get(object).(*cue.Instance)).Err = *get(Err).(*errors.Error)
}
func get_cue_Instance_ImportPath(object uint32) string {
	ImportPath := (*get(object).(*cue.Instance)).ImportPath
	return ImportPath
}
func set_cue_Instance_ImportPath(object uint32, ImportPath string) {
	(*get(object).(*cue.Instance)).ImportPath = ImportPath
}
func get_cue_Instance_Incomplete(object uint32) bool {
	Incomplete := (*get(object).(*cue.Instance)).Incomplete
	return Incomplete
}
func set_cue_Instance_Incomplete(object uint32, Incomplete bool) {
	(*get(object).(*cue.Instance)).Incomplete = Incomplete
}
func get_cue_Instance_PkgName(object uint32) string {
	PkgName := (*get(object).(*cue.Instance)).PkgName
	return PkgName
}
func set_cue_Instance_PkgName(object uint32, PkgName string) {
	(*get(object).(*cue.Instance)).PkgName = PkgName
}
func defaultcue_Iterator() uint32 {
	var tmp cue.Iterator
	return register(&tmp)
}
func defaultcue_Kind() uint32 {
	var tmp cue.Kind
	return register(&tmp)
}
func defaultcue_Op() uint32 {
	var tmp cue.Op
	return register(&tmp)
}
func defaultcue_Path() uint32 {
	var tmp cue.Path
	return register(&tmp)
}
func defaultcue_Runtime() uint32 {
	var tmp cue.Runtime
	return register(&tmp)
}
func defaultcue_Selector() uint32 {
	var tmp cue.Selector
	return register(&tmp)
}
func defaultcue_Struct() uint32 {
	var tmp cue.Struct
	return register(&tmp)
}
func defaultcue_Value() uint32 {
	var tmp cue.Value
	return register(&tmp)
}
func defaultast_Alias() uint32 {
	var tmp ast.Alias
	return register(&tmp)
}
func get_ast_Alias_Equal(object uint32) uint32 {
	Equal := (*get(object).(*ast.Alias)).Equal
	return register(&Equal)
}
func set_ast_Alias_Equal(object uint32, Equal uint32) {
	(*get(object).(*ast.Alias)).Equal = *get(Equal).(*token.Pos)
}
func get_ast_Alias_Expr(object uint32) uint32 {
	Expr := (*get(object).(*ast.Alias)).Expr
	return register(&Expr)
}
func set_ast_Alias_Expr(object uint32, Expr uint32) {
	(*get(object).(*ast.Alias)).Expr = *get(Expr).(*ast.Expr)
}
func get_ast_Alias_Ident(object uint32) uint32 {
	Ident := (*get(object).(*ast.Alias)).Ident
	return register(&Ident)
}
func set_ast_Alias_Ident(object uint32, Ident uint32) {
	(*get(object).(*ast.Alias)).Ident = *get(Ident).(**ast.Ident)
}
func defaultast_Attribute() uint32 {
	var tmp ast.Attribute
	return register(&tmp)
}
func get_ast_Attribute_At(object uint32) uint32 {
	At := (*get(object).(*ast.Attribute)).At
	return register(&At)
}
func set_ast_Attribute_At(object uint32, At uint32) {
	(*get(object).(*ast.Attribute)).At = *get(At).(*token.Pos)
}
func get_ast_Attribute_Text(object uint32) string {
	Text := (*get(object).(*ast.Attribute)).Text
	return Text
}
func set_ast_Attribute_Text(object uint32, Text string) {
	(*get(object).(*ast.Attribute)).Text = Text
}
func defaultast_BadDecl() uint32 {
	var tmp ast.BadDecl
	return register(&tmp)
}
func get_ast_BadDecl_From(object uint32) uint32 {
	From := (*get(object).(*ast.BadDecl)).From
	return register(&From)
}
func set_ast_BadDecl_From(object uint32, From uint32) {
	(*get(object).(*ast.BadDecl)).From = *get(From).(*token.Pos)
}
func get_ast_BadDecl_To(object uint32) uint32 {
	To := (*get(object).(*ast.BadDecl)).To
	return register(&To)
}
func set_ast_BadDecl_To(object uint32, To uint32) {
	(*get(object).(*ast.BadDecl)).To = *get(To).(*token.Pos)
}
func defaultast_BadExpr() uint32 {
	var tmp ast.BadExpr
	return register(&tmp)
}
func get_ast_BadExpr_From(object uint32) uint32 {
	From := (*get(object).(*ast.BadExpr)).From
	return register(&From)
}
func set_ast_BadExpr_From(object uint32, From uint32) {
	(*get(object).(*ast.BadExpr)).From = *get(From).(*token.Pos)
}
func get_ast_BadExpr_To(object uint32) uint32 {
	To := (*get(object).(*ast.BadExpr)).To
	return register(&To)
}
func set_ast_BadExpr_To(object uint32, To uint32) {
	(*get(object).(*ast.BadExpr)).To = *get(To).(*token.Pos)
}
func defaultast_BasicLit() uint32 {
	var tmp ast.BasicLit
	return register(&tmp)
}
func get_ast_BasicLit_Kind(object uint32) uint32 {
	Kind := (*get(object).(*ast.BasicLit)).Kind
	return register(&Kind)
}
func set_ast_BasicLit_Kind(object uint32, Kind uint32) {
	(*get(object).(*ast.BasicLit)).Kind = *get(Kind).(*token.Token)
}
func get_ast_BasicLit_Value(object uint32) string {
	Value := (*get(object).(*ast.BasicLit)).Value
	return Value
}
func set_ast_BasicLit_Value(object uint32, Value string) {
	(*get(object).(*ast.BasicLit)).Value = Value
}
func get_ast_BasicLit_ValuePos(object uint32) uint32 {
	ValuePos := (*get(object).(*ast.BasicLit)).ValuePos
	return register(&ValuePos)
}
func set_ast_BasicLit_ValuePos(object uint32, ValuePos uint32) {
	(*get(object).(*ast.BasicLit)).ValuePos = *get(ValuePos).(*token.Pos)
}
func defaultast_BinaryExpr() uint32 {
	var tmp ast.BinaryExpr
	return register(&tmp)
}
func get_ast_BinaryExpr_Op(object uint32) uint32 {
	Op := (*get(object).(*ast.BinaryExpr)).Op
	return register(&Op)
}
func set_ast_BinaryExpr_Op(object uint32, Op uint32) {
	(*get(object).(*ast.BinaryExpr)).Op = *get(Op).(*token.Token)
}
func get_ast_BinaryExpr_OpPos(object uint32) uint32 {
	OpPos := (*get(object).(*ast.BinaryExpr)).OpPos
	return register(&OpPos)
}
func set_ast_BinaryExpr_OpPos(object uint32, OpPos uint32) {
	(*get(object).(*ast.BinaryExpr)).OpPos = *get(OpPos).(*token.Pos)
}
func get_ast_BinaryExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.BinaryExpr)).X
	return register(&X)
}
func set_ast_BinaryExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.BinaryExpr)).X = *get(X).(*ast.Expr)
}
func get_ast_BinaryExpr_Y(object uint32) uint32 {
	Y := (*get(object).(*ast.BinaryExpr)).Y
	return register(&Y)
}
func set_ast_BinaryExpr_Y(object uint32, Y uint32) {
	(*get(object).(*ast.BinaryExpr)).Y = *get(Y).(*ast.Expr)
}
func defaultast_BottomLit() uint32 {
	var tmp ast.BottomLit
	return register(&tmp)
}
func get_ast_BottomLit_Bottom(object uint32) uint32 {
	Bottom := (*get(object).(*ast.BottomLit)).Bottom
	return register(&Bottom)
}
func set_ast_BottomLit_Bottom(object uint32, Bottom uint32) {
	(*get(object).(*ast.BottomLit)).Bottom = *get(Bottom).(*token.Pos)
}
func defaultast_CallExpr() uint32 {
	var tmp ast.CallExpr
	return register(&tmp)
}
func get_ast_CallExpr_Args(object uint32) uint32 {
	Args := (*get(object).(*ast.CallExpr)).Args
	return register(&Args)
}
func set_ast_CallExpr_Args(object uint32, Args uint32) {
	(*get(object).(*ast.CallExpr)).Args = *get(Args).(*[]ast.Expr)
}
func get_ast_CallExpr_Fun(object uint32) uint32 {
	Fun := (*get(object).(*ast.CallExpr)).Fun
	return register(&Fun)
}
func set_ast_CallExpr_Fun(object uint32, Fun uint32) {
	(*get(object).(*ast.CallExpr)).Fun = *get(Fun).(*ast.Expr)
}
func get_ast_CallExpr_Lparen(object uint32) uint32 {
	Lparen := (*get(object).(*ast.CallExpr)).Lparen
	return register(&Lparen)
}
func set_ast_CallExpr_Lparen(object uint32, Lparen uint32) {
	(*get(object).(*ast.CallExpr)).Lparen = *get(Lparen).(*token.Pos)
}
func get_ast_CallExpr_Rparen(object uint32) uint32 {
	Rparen := (*get(object).(*ast.CallExpr)).Rparen
	return register(&Rparen)
}
func set_ast_CallExpr_Rparen(object uint32, Rparen uint32) {
	(*get(object).(*ast.CallExpr)).Rparen = *get(Rparen).(*token.Pos)
}
func as_ast_Clause(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Clause)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}
func defaultast_Comment() uint32 {
	var tmp ast.Comment
	return register(&tmp)
}
func get_ast_Comment_Slash(object uint32) uint32 {
	Slash := (*get(object).(*ast.Comment)).Slash
	return register(&Slash)
}
func set_ast_Comment_Slash(object uint32, Slash uint32) {
	(*get(object).(*ast.Comment)).Slash = *get(Slash).(*token.Pos)
}
func get_ast_Comment_Text(object uint32) string {
	Text := (*get(object).(*ast.Comment)).Text
	return Text
}
func set_ast_Comment_Text(object uint32, Text string) {
	(*get(object).(*ast.Comment)).Text = Text
}
func defaultast_CommentGroup() uint32 {
	var tmp ast.CommentGroup
	return register(&tmp)
}
func get_ast_CommentGroup_Doc(object uint32) bool {
	Doc := (*get(object).(*ast.CommentGroup)).Doc
	return Doc
}
func set_ast_CommentGroup_Doc(object uint32, Doc bool) {
	(*get(object).(*ast.CommentGroup)).Doc = Doc
}
func get_ast_CommentGroup_Line(object uint32) bool {
	Line := (*get(object).(*ast.CommentGroup)).Line
	return Line
}
func set_ast_CommentGroup_Line(object uint32, Line bool) {
	(*get(object).(*ast.CommentGroup)).Line = Line
}
func get_ast_CommentGroup_List(object uint32) uint32 {
	List := (*get(object).(*ast.CommentGroup)).List
	return register(&List)
}
func set_ast_CommentGroup_List(object uint32, List uint32) {
	(*get(object).(*ast.CommentGroup)).List = *get(List).(*[]*ast.Comment)
}
func get_ast_CommentGroup_Position(object uint32) int8 {
	Position := (*get(object).(*ast.CommentGroup)).Position
	return Position
}
func set_ast_CommentGroup_Position(object uint32, Position int8) {
	(*get(object).(*ast.CommentGroup)).Position = Position
}
func defaultast_Comprehension() uint32 {
	var tmp ast.Comprehension
	return register(&tmp)
}
func get_ast_Comprehension_Clauses(object uint32) uint32 {
	Clauses := (*get(object).(*ast.Comprehension)).Clauses
	return register(&Clauses)
}
func set_ast_Comprehension_Clauses(object uint32, Clauses uint32) {
	(*get(object).(*ast.Comprehension)).Clauses = *get(Clauses).(*[]ast.Clause)
}
func get_ast_Comprehension_Value(object uint32) uint32 {
	Value := (*get(object).(*ast.Comprehension)).Value
	return register(&Value)
}
func set_ast_Comprehension_Value(object uint32, Value uint32) {
	(*get(object).(*ast.Comprehension)).Value = *get(Value).(*ast.Expr)
}
func as_ast_Decl(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Decl)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}
func defaultast_Ellipsis() uint32 {
	var tmp ast.Ellipsis
	return register(&tmp)
}
func get_ast_Ellipsis_Ellipsis(object uint32) uint32 {
	Ellipsis := (*get(object).(*ast.Ellipsis)).Ellipsis
	return register(&Ellipsis)
}
func set_ast_Ellipsis_Ellipsis(object uint32, Ellipsis uint32) {
	(*get(object).(*ast.Ellipsis)).Ellipsis = *get(Ellipsis).(*token.Pos)
}
func get_ast_Ellipsis_Type(object uint32) uint32 {
	Type := (*get(object).(*ast.Ellipsis)).Type
	return register(&Type)
}
func set_ast_Ellipsis_Type(object uint32, Type uint32) {
	(*get(object).(*ast.Ellipsis)).Type = *get(Type).(*ast.Expr)
}
func defaultast_EmbedDecl() uint32 {
	var tmp ast.EmbedDecl
	return register(&tmp)
}
func get_ast_EmbedDecl_Expr(object uint32) uint32 {
	Expr := (*get(object).(*ast.EmbedDecl)).Expr
	return register(&Expr)
}
func set_ast_EmbedDecl_Expr(object uint32, Expr uint32) {
	(*get(object).(*ast.EmbedDecl)).Expr = *get(Expr).(*ast.Expr)
}
func as_ast_Expr(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Expr)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}
func defaultast_Field() uint32 {
	var tmp ast.Field
	return register(&tmp)
}
func get_ast_Field_Attrs(object uint32) uint32 {
	Attrs := (*get(object).(*ast.Field)).Attrs
	return register(&Attrs)
}
func set_ast_Field_Attrs(object uint32, Attrs uint32) {
	(*get(object).(*ast.Field)).Attrs = *get(Attrs).(*[]*ast.Attribute)
}
func get_ast_Field_Label(object uint32) uint32 {
	Label := (*get(object).(*ast.Field)).Label
	return register(&Label)
}
func set_ast_Field_Label(object uint32, Label uint32) {
	(*get(object).(*ast.Field)).Label = *get(Label).(*ast.Label)
}
func get_ast_Field_Optional(object uint32) uint32 {
	Optional := (*get(object).(*ast.Field)).Optional
	return register(&Optional)
}
func set_ast_Field_Optional(object uint32, Optional uint32) {
	(*get(object).(*ast.Field)).Optional = *get(Optional).(*token.Pos)
}
func get_ast_Field_Token(object uint32) uint32 {
	Token := (*get(object).(*ast.Field)).Token
	return register(&Token)
}
func set_ast_Field_Token(object uint32, Token uint32) {
	(*get(object).(*ast.Field)).Token = *get(Token).(*token.Token)
}
func get_ast_Field_TokenPos(object uint32) uint32 {
	TokenPos := (*get(object).(*ast.Field)).TokenPos
	return register(&TokenPos)
}
func set_ast_Field_TokenPos(object uint32, TokenPos uint32) {
	(*get(object).(*ast.Field)).TokenPos = *get(TokenPos).(*token.Pos)
}
func get_ast_Field_Value(object uint32) uint32 {
	Value := (*get(object).(*ast.Field)).Value
	return register(&Value)
}
func set_ast_Field_Value(object uint32, Value uint32) {
	(*get(object).(*ast.Field)).Value = *get(Value).(*ast.Expr)
}
func defaultast_File() uint32 {
	var tmp ast.File
	return register(&tmp)
}
func get_ast_File_Decls(object uint32) uint32 {
	Decls := (*get(object).(*ast.File)).Decls
	return register(&Decls)
}
func set_ast_File_Decls(object uint32, Decls uint32) {
	(*get(object).(*ast.File)).Decls = *get(Decls).(*[]ast.Decl)
}
func get_ast_File_Filename(object uint32) string {
	Filename := (*get(object).(*ast.File)).Filename
	return Filename
}
func set_ast_File_Filename(object uint32, Filename string) {
	(*get(object).(*ast.File)).Filename = Filename
}
func get_ast_File_Imports(object uint32) uint32 {
	Imports := (*get(object).(*ast.File)).Imports
	return register(&Imports)
}
func set_ast_File_Imports(object uint32, Imports uint32) {
	(*get(object).(*ast.File)).Imports = *get(Imports).(*[]*ast.ImportSpec)
}
func get_ast_File_Unresolved(object uint32) uint32 {
	Unresolved := (*get(object).(*ast.File)).Unresolved
	return register(&Unresolved)
}
func set_ast_File_Unresolved(object uint32, Unresolved uint32) {
	(*get(object).(*ast.File)).Unresolved = *get(Unresolved).(*[]*ast.Ident)
}
func defaultast_ForClause() uint32 {
	var tmp ast.ForClause
	return register(&tmp)
}
func get_ast_ForClause_Colon(object uint32) uint32 {
	Colon := (*get(object).(*ast.ForClause)).Colon
	return register(&Colon)
}
func set_ast_ForClause_Colon(object uint32, Colon uint32) {
	(*get(object).(*ast.ForClause)).Colon = *get(Colon).(*token.Pos)
}
func get_ast_ForClause_For(object uint32) uint32 {
	For := (*get(object).(*ast.ForClause)).For
	return register(&For)
}
func set_ast_ForClause_For(object uint32, For uint32) {
	(*get(object).(*ast.ForClause)).For = *get(For).(*token.Pos)
}
func get_ast_ForClause_In(object uint32) uint32 {
	In := (*get(object).(*ast.ForClause)).In
	return register(&In)
}
func set_ast_ForClause_In(object uint32, In uint32) {
	(*get(object).(*ast.ForClause)).In = *get(In).(*token.Pos)
}
func get_ast_ForClause_Key(object uint32) uint32 {
	Key := (*get(object).(*ast.ForClause)).Key
	return register(&Key)
}
func set_ast_ForClause_Key(object uint32, Key uint32) {
	(*get(object).(*ast.ForClause)).Key = *get(Key).(**ast.Ident)
}
func get_ast_ForClause_Source(object uint32) uint32 {
	Source := (*get(object).(*ast.ForClause)).Source
	return register(&Source)
}
func set_ast_ForClause_Source(object uint32, Source uint32) {
	(*get(object).(*ast.ForClause)).Source = *get(Source).(*ast.Expr)
}
func get_ast_ForClause_Value(object uint32) uint32 {
	Value := (*get(object).(*ast.ForClause)).Value
	return register(&Value)
}
func set_ast_ForClause_Value(object uint32, Value uint32) {
	(*get(object).(*ast.ForClause)).Value = *get(Value).(**ast.Ident)
}
func defaultast_Ident() uint32 {
	var tmp ast.Ident
	return register(&tmp)
}
func get_ast_Ident_Name(object uint32) string {
	Name := (*get(object).(*ast.Ident)).Name
	return Name
}
func set_ast_Ident_Name(object uint32, Name string) {
	(*get(object).(*ast.Ident)).Name = Name
}
func get_ast_Ident_NamePos(object uint32) uint32 {
	NamePos := (*get(object).(*ast.Ident)).NamePos
	return register(&NamePos)
}
func set_ast_Ident_NamePos(object uint32, NamePos uint32) {
	(*get(object).(*ast.Ident)).NamePos = *get(NamePos).(*token.Pos)
}
func get_ast_Ident_Node(object uint32) uint32 {
	Node := (*get(object).(*ast.Ident)).Node
	return register(&Node)
}
func set_ast_Ident_Node(object uint32, Node uint32) {
	(*get(object).(*ast.Ident)).Node = *get(Node).(*ast.Node)
}
func get_ast_Ident_Scope(object uint32) uint32 {
	Scope := (*get(object).(*ast.Ident)).Scope
	return register(&Scope)
}
func set_ast_Ident_Scope(object uint32, Scope uint32) {
	(*get(object).(*ast.Ident)).Scope = *get(Scope).(*ast.Node)
}
func defaultast_IfClause() uint32 {
	var tmp ast.IfClause
	return register(&tmp)
}
func get_ast_IfClause_Condition(object uint32) uint32 {
	Condition := (*get(object).(*ast.IfClause)).Condition
	return register(&Condition)
}
func set_ast_IfClause_Condition(object uint32, Condition uint32) {
	(*get(object).(*ast.IfClause)).Condition = *get(Condition).(*ast.Expr)
}
func get_ast_IfClause_If(object uint32) uint32 {
	If := (*get(object).(*ast.IfClause)).If
	return register(&If)
}
func set_ast_IfClause_If(object uint32, If uint32) {
	(*get(object).(*ast.IfClause)).If = *get(If).(*token.Pos)
}
func defaultast_ImportDecl() uint32 {
	var tmp ast.ImportDecl
	return register(&tmp)
}
func get_ast_ImportDecl_Import(object uint32) uint32 {
	Import := (*get(object).(*ast.ImportDecl)).Import
	return register(&Import)
}
func set_ast_ImportDecl_Import(object uint32, Import uint32) {
	(*get(object).(*ast.ImportDecl)).Import = *get(Import).(*token.Pos)
}
func get_ast_ImportDecl_Lparen(object uint32) uint32 {
	Lparen := (*get(object).(*ast.ImportDecl)).Lparen
	return register(&Lparen)
}
func set_ast_ImportDecl_Lparen(object uint32, Lparen uint32) {
	(*get(object).(*ast.ImportDecl)).Lparen = *get(Lparen).(*token.Pos)
}
func get_ast_ImportDecl_Rparen(object uint32) uint32 {
	Rparen := (*get(object).(*ast.ImportDecl)).Rparen
	return register(&Rparen)
}
func set_ast_ImportDecl_Rparen(object uint32, Rparen uint32) {
	(*get(object).(*ast.ImportDecl)).Rparen = *get(Rparen).(*token.Pos)
}
func get_ast_ImportDecl_Specs(object uint32) uint32 {
	Specs := (*get(object).(*ast.ImportDecl)).Specs
	return register(&Specs)
}
func set_ast_ImportDecl_Specs(object uint32, Specs uint32) {
	(*get(object).(*ast.ImportDecl)).Specs = *get(Specs).(*[]*ast.ImportSpec)
}
func defaultast_ImportSpec() uint32 {
	var tmp ast.ImportSpec
	return register(&tmp)
}
func get_ast_ImportSpec_EndPos(object uint32) uint32 {
	EndPos := (*get(object).(*ast.ImportSpec)).EndPos
	return register(&EndPos)
}
func set_ast_ImportSpec_EndPos(object uint32, EndPos uint32) {
	(*get(object).(*ast.ImportSpec)).EndPos = *get(EndPos).(*token.Pos)
}
func get_ast_ImportSpec_Name(object uint32) uint32 {
	Name := (*get(object).(*ast.ImportSpec)).Name
	return register(&Name)
}
func set_ast_ImportSpec_Name(object uint32, Name uint32) {
	(*get(object).(*ast.ImportSpec)).Name = *get(Name).(**ast.Ident)
}
func get_ast_ImportSpec_Path(object uint32) uint32 {
	Path := (*get(object).(*ast.ImportSpec)).Path
	return register(&Path)
}
func set_ast_ImportSpec_Path(object uint32, Path uint32) {
	(*get(object).(*ast.ImportSpec)).Path = *get(Path).(**ast.BasicLit)
}
func defaultast_IndexExpr() uint32 {
	var tmp ast.IndexExpr
	return register(&tmp)
}
func get_ast_IndexExpr_Index(object uint32) uint32 {
	Index := (*get(object).(*ast.IndexExpr)).Index
	return register(&Index)
}
func set_ast_IndexExpr_Index(object uint32, Index uint32) {
	(*get(object).(*ast.IndexExpr)).Index = *get(Index).(*ast.Expr)
}
func get_ast_IndexExpr_Lbrack(object uint32) uint32 {
	Lbrack := (*get(object).(*ast.IndexExpr)).Lbrack
	return register(&Lbrack)
}
func set_ast_IndexExpr_Lbrack(object uint32, Lbrack uint32) {
	(*get(object).(*ast.IndexExpr)).Lbrack = *get(Lbrack).(*token.Pos)
}
func get_ast_IndexExpr_Rbrack(object uint32) uint32 {
	Rbrack := (*get(object).(*ast.IndexExpr)).Rbrack
	return register(&Rbrack)
}
func set_ast_IndexExpr_Rbrack(object uint32, Rbrack uint32) {
	(*get(object).(*ast.IndexExpr)).Rbrack = *get(Rbrack).(*token.Pos)
}
func get_ast_IndexExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.IndexExpr)).X
	return register(&X)
}
func set_ast_IndexExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.IndexExpr)).X = *get(X).(*ast.Expr)
}
func defaultast_Interpolation() uint32 {
	var tmp ast.Interpolation
	return register(&tmp)
}
func get_ast_Interpolation_Elts(object uint32) uint32 {
	Elts := (*get(object).(*ast.Interpolation)).Elts
	return register(&Elts)
}
func set_ast_Interpolation_Elts(object uint32, Elts uint32) {
	(*get(object).(*ast.Interpolation)).Elts = *get(Elts).(*[]ast.Expr)
}
func as_ast_Label(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Label)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}
func defaultast_LetClause() uint32 {
	var tmp ast.LetClause
	return register(&tmp)
}
func get_ast_LetClause_Equal(object uint32) uint32 {
	Equal := (*get(object).(*ast.LetClause)).Equal
	return register(&Equal)
}
func set_ast_LetClause_Equal(object uint32, Equal uint32) {
	(*get(object).(*ast.LetClause)).Equal = *get(Equal).(*token.Pos)
}
func get_ast_LetClause_Expr(object uint32) uint32 {
	Expr := (*get(object).(*ast.LetClause)).Expr
	return register(&Expr)
}
func set_ast_LetClause_Expr(object uint32, Expr uint32) {
	(*get(object).(*ast.LetClause)).Expr = *get(Expr).(*ast.Expr)
}
func get_ast_LetClause_Ident(object uint32) uint32 {
	Ident := (*get(object).(*ast.LetClause)).Ident
	return register(&Ident)
}
func set_ast_LetClause_Ident(object uint32, Ident uint32) {
	(*get(object).(*ast.LetClause)).Ident = *get(Ident).(**ast.Ident)
}
func get_ast_LetClause_Let(object uint32) uint32 {
	Let := (*get(object).(*ast.LetClause)).Let
	return register(&Let)
}
func set_ast_LetClause_Let(object uint32, Let uint32) {
	(*get(object).(*ast.LetClause)).Let = *get(Let).(*token.Pos)
}
func defaultast_ListComprehension() uint32 {
	var tmp ast.ListComprehension
	return register(&tmp)
}
func get_ast_ListComprehension_Clauses(object uint32) uint32 {
	Clauses := (*get(object).(*ast.ListComprehension)).Clauses
	return register(&Clauses)
}
func set_ast_ListComprehension_Clauses(object uint32, Clauses uint32) {
	(*get(object).(*ast.ListComprehension)).Clauses = *get(Clauses).(*[]ast.Clause)
}
func get_ast_ListComprehension_Expr(object uint32) uint32 {
	Expr := (*get(object).(*ast.ListComprehension)).Expr
	return register(&Expr)
}
func set_ast_ListComprehension_Expr(object uint32, Expr uint32) {
	(*get(object).(*ast.ListComprehension)).Expr = *get(Expr).(*ast.Expr)
}
func get_ast_ListComprehension_Lbrack(object uint32) uint32 {
	Lbrack := (*get(object).(*ast.ListComprehension)).Lbrack
	return register(&Lbrack)
}
func set_ast_ListComprehension_Lbrack(object uint32, Lbrack uint32) {
	(*get(object).(*ast.ListComprehension)).Lbrack = *get(Lbrack).(*token.Pos)
}
func get_ast_ListComprehension_Rbrack(object uint32) uint32 {
	Rbrack := (*get(object).(*ast.ListComprehension)).Rbrack
	return register(&Rbrack)
}
func set_ast_ListComprehension_Rbrack(object uint32, Rbrack uint32) {
	(*get(object).(*ast.ListComprehension)).Rbrack = *get(Rbrack).(*token.Pos)
}
func defaultast_ListLit() uint32 {
	var tmp ast.ListLit
	return register(&tmp)
}
func get_ast_ListLit_Elts(object uint32) uint32 {
	Elts := (*get(object).(*ast.ListLit)).Elts
	return register(&Elts)
}
func set_ast_ListLit_Elts(object uint32, Elts uint32) {
	(*get(object).(*ast.ListLit)).Elts = *get(Elts).(*[]ast.Expr)
}
func get_ast_ListLit_Lbrack(object uint32) uint32 {
	Lbrack := (*get(object).(*ast.ListLit)).Lbrack
	return register(&Lbrack)
}
func set_ast_ListLit_Lbrack(object uint32, Lbrack uint32) {
	(*get(object).(*ast.ListLit)).Lbrack = *get(Lbrack).(*token.Pos)
}
func get_ast_ListLit_Rbrack(object uint32) uint32 {
	Rbrack := (*get(object).(*ast.ListLit)).Rbrack
	return register(&Rbrack)
}
func set_ast_ListLit_Rbrack(object uint32, Rbrack uint32) {
	(*get(object).(*ast.ListLit)).Rbrack = *get(Rbrack).(*token.Pos)
}
func as_ast_Node(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Node)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}
func defaultast_Package() uint32 {
	var tmp ast.Package
	return register(&tmp)
}
func get_ast_Package_Name(object uint32) uint32 {
	Name := (*get(object).(*ast.Package)).Name
	return register(&Name)
}
func set_ast_Package_Name(object uint32, Name uint32) {
	(*get(object).(*ast.Package)).Name = *get(Name).(**ast.Ident)
}
func get_ast_Package_PackagePos(object uint32) uint32 {
	PackagePos := (*get(object).(*ast.Package)).PackagePos
	return register(&PackagePos)
}
func set_ast_Package_PackagePos(object uint32, PackagePos uint32) {
	(*get(object).(*ast.Package)).PackagePos = *get(PackagePos).(*token.Pos)
}
func defaultast_ParenExpr() uint32 {
	var tmp ast.ParenExpr
	return register(&tmp)
}
func get_ast_ParenExpr_Lparen(object uint32) uint32 {
	Lparen := (*get(object).(*ast.ParenExpr)).Lparen
	return register(&Lparen)
}
func set_ast_ParenExpr_Lparen(object uint32, Lparen uint32) {
	(*get(object).(*ast.ParenExpr)).Lparen = *get(Lparen).(*token.Pos)
}
func get_ast_ParenExpr_Rparen(object uint32) uint32 {
	Rparen := (*get(object).(*ast.ParenExpr)).Rparen
	return register(&Rparen)
}
func set_ast_ParenExpr_Rparen(object uint32, Rparen uint32) {
	(*get(object).(*ast.ParenExpr)).Rparen = *get(Rparen).(*token.Pos)
}
func get_ast_ParenExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.ParenExpr)).X
	return register(&X)
}
func set_ast_ParenExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.ParenExpr)).X = *get(X).(*ast.Expr)
}
func defaultast_SelectorExpr() uint32 {
	var tmp ast.SelectorExpr
	return register(&tmp)
}
func get_ast_SelectorExpr_Sel(object uint32) uint32 {
	Sel := (*get(object).(*ast.SelectorExpr)).Sel
	return register(&Sel)
}
func set_ast_SelectorExpr_Sel(object uint32, Sel uint32) {
	(*get(object).(*ast.SelectorExpr)).Sel = *get(Sel).(*ast.Label)
}
func get_ast_SelectorExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.SelectorExpr)).X
	return register(&X)
}
func set_ast_SelectorExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.SelectorExpr)).X = *get(X).(*ast.Expr)
}
func defaultast_SliceExpr() uint32 {
	var tmp ast.SliceExpr
	return register(&tmp)
}
func get_ast_SliceExpr_High(object uint32) uint32 {
	High := (*get(object).(*ast.SliceExpr)).High
	return register(&High)
}
func set_ast_SliceExpr_High(object uint32, High uint32) {
	(*get(object).(*ast.SliceExpr)).High = *get(High).(*ast.Expr)
}
func get_ast_SliceExpr_Lbrack(object uint32) uint32 {
	Lbrack := (*get(object).(*ast.SliceExpr)).Lbrack
	return register(&Lbrack)
}
func set_ast_SliceExpr_Lbrack(object uint32, Lbrack uint32) {
	(*get(object).(*ast.SliceExpr)).Lbrack = *get(Lbrack).(*token.Pos)
}
func get_ast_SliceExpr_Low(object uint32) uint32 {
	Low := (*get(object).(*ast.SliceExpr)).Low
	return register(&Low)
}
func set_ast_SliceExpr_Low(object uint32, Low uint32) {
	(*get(object).(*ast.SliceExpr)).Low = *get(Low).(*ast.Expr)
}
func get_ast_SliceExpr_Rbrack(object uint32) uint32 {
	Rbrack := (*get(object).(*ast.SliceExpr)).Rbrack
	return register(&Rbrack)
}
func set_ast_SliceExpr_Rbrack(object uint32, Rbrack uint32) {
	(*get(object).(*ast.SliceExpr)).Rbrack = *get(Rbrack).(*token.Pos)
}
func get_ast_SliceExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.SliceExpr)).X
	return register(&X)
}
func set_ast_SliceExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.SliceExpr)).X = *get(X).(*ast.Expr)
}
func as_ast_Spec(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Spec)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}
func defaultast_StructLit() uint32 {
	var tmp ast.StructLit
	return register(&tmp)
}
func get_ast_StructLit_Elts(object uint32) uint32 {
	Elts := (*get(object).(*ast.StructLit)).Elts
	return register(&Elts)
}
func set_ast_StructLit_Elts(object uint32, Elts uint32) {
	(*get(object).(*ast.StructLit)).Elts = *get(Elts).(*[]ast.Decl)
}
func get_ast_StructLit_Lbrace(object uint32) uint32 {
	Lbrace := (*get(object).(*ast.StructLit)).Lbrace
	return register(&Lbrace)
}
func set_ast_StructLit_Lbrace(object uint32, Lbrace uint32) {
	(*get(object).(*ast.StructLit)).Lbrace = *get(Lbrace).(*token.Pos)
}
func get_ast_StructLit_Rbrace(object uint32) uint32 {
	Rbrace := (*get(object).(*ast.StructLit)).Rbrace
	return register(&Rbrace)
}
func set_ast_StructLit_Rbrace(object uint32, Rbrace uint32) {
	(*get(object).(*ast.StructLit)).Rbrace = *get(Rbrace).(*token.Pos)
}
func defaultast_TemplateLabel() uint32 {
	var tmp ast.TemplateLabel
	return register(&tmp)
}
func get_ast_TemplateLabel_Ident(object uint32) uint32 {
	Ident := (*get(object).(*ast.TemplateLabel)).Ident
	return register(&Ident)
}
func set_ast_TemplateLabel_Ident(object uint32, Ident uint32) {
	(*get(object).(*ast.TemplateLabel)).Ident = *get(Ident).(**ast.Ident)
}
func get_ast_TemplateLabel_Langle(object uint32) uint32 {
	Langle := (*get(object).(*ast.TemplateLabel)).Langle
	return register(&Langle)
}
func set_ast_TemplateLabel_Langle(object uint32, Langle uint32) {
	(*get(object).(*ast.TemplateLabel)).Langle = *get(Langle).(*token.Pos)
}
func get_ast_TemplateLabel_Rangle(object uint32) uint32 {
	Rangle := (*get(object).(*ast.TemplateLabel)).Rangle
	return register(&Rangle)
}
func set_ast_TemplateLabel_Rangle(object uint32, Rangle uint32) {
	(*get(object).(*ast.TemplateLabel)).Rangle = *get(Rangle).(*token.Pos)
}
func defaultast_UnaryExpr() uint32 {
	var tmp ast.UnaryExpr
	return register(&tmp)
}
func get_ast_UnaryExpr_Op(object uint32) uint32 {
	Op := (*get(object).(*ast.UnaryExpr)).Op
	return register(&Op)
}
func set_ast_UnaryExpr_Op(object uint32, Op uint32) {
	(*get(object).(*ast.UnaryExpr)).Op = *get(Op).(*token.Token)
}
func get_ast_UnaryExpr_OpPos(object uint32) uint32 {
	OpPos := (*get(object).(*ast.UnaryExpr)).OpPos
	return register(&OpPos)
}
func set_ast_UnaryExpr_OpPos(object uint32, OpPos uint32) {
	(*get(object).(*ast.UnaryExpr)).OpPos = *get(OpPos).(*token.Pos)
}
func get_ast_UnaryExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.UnaryExpr)).X
	return register(&X)
}
func set_ast_UnaryExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.UnaryExpr)).X = *get(X).(*ast.Expr)
}
func as_astutil_Cursor(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(astutil.Cursor)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}
func defaultastutil_ImportInfo() uint32 {
	var tmp astutil.ImportInfo
	return register(&tmp)
}
func get_astutil_ImportInfo_Dir(object uint32) string {
	Dir := (*get(object).(*astutil.ImportInfo)).Dir
	return Dir
}
func set_astutil_ImportInfo_Dir(object uint32, Dir string) {
	(*get(object).(*astutil.ImportInfo)).Dir = Dir
}
func get_astutil_ImportInfo_ID(object uint32) string {
	ID := (*get(object).(*astutil.ImportInfo)).ID
	return ID
}
func set_astutil_ImportInfo_ID(object uint32, ID string) {
	(*get(object).(*astutil.ImportInfo)).ID = ID
}
func get_astutil_ImportInfo_Ident(object uint32) string {
	Ident := (*get(object).(*astutil.ImportInfo)).Ident
	return Ident
}
func set_astutil_ImportInfo_Ident(object uint32, Ident string) {
	(*get(object).(*astutil.ImportInfo)).Ident = Ident
}
func get_astutil_ImportInfo_PkgName(object uint32) string {
	PkgName := (*get(object).(*astutil.ImportInfo)).PkgName
	return PkgName
}
func set_astutil_ImportInfo_PkgName(object uint32, PkgName string) {
	(*get(object).(*astutil.ImportInfo)).PkgName = PkgName
}
func defaultbuild_Context() uint32 {
	var tmp build.Context
	return register(&tmp)
}
func defaultbuild_Encoding() uint32 {
	var tmp build.Encoding
	return register(&tmp)
}
func defaultbuild_File() uint32 {
	var tmp build.File
	return register(&tmp)
}
func get_build_File_Encoding(object uint32) uint32 {
	Encoding := (*get(object).(*build.File)).Encoding
	return register(&Encoding)
}
func set_build_File_Encoding(object uint32, Encoding uint32) {
	(*get(object).(*build.File)).Encoding = *get(Encoding).(*build.Encoding)
}
func get_build_File_Filename(object uint32) string {
	Filename := (*get(object).(*build.File)).Filename
	return Filename
}
func set_build_File_Filename(object uint32, Filename string) {
	(*get(object).(*build.File)).Filename = Filename
}
func get_build_File_Form(object uint32) uint32 {
	Form := (*get(object).(*build.File)).Form
	return register(&Form)
}
func set_build_File_Form(object uint32, Form uint32) {
	(*get(object).(*build.File)).Form = *get(Form).(*build.Form)
}
func get_build_File_Interpretation(object uint32) uint32 {
	Interpretation := (*get(object).(*build.File)).Interpretation
	return register(&Interpretation)
}
func set_build_File_Interpretation(object uint32, Interpretation uint32) {
	(*get(object).(*build.File)).Interpretation = *get(Interpretation).(*build.Interpretation)
}
func get_build_File_Source(object uint32) uint32 {
	Source := (*get(object).(*build.File)).Source
	return register(&Source)
}
func set_build_File_Source(object uint32, Source uint32) {
	(*get(object).(*build.File)).Source = reflect.ValueOf(get(Source)).Elem().Interface()
}
func get_build_File_Tags(object uint32) uint32 {
	Tags := (*get(object).(*build.File)).Tags
	return register(&Tags)
}
func set_build_File_Tags(object uint32, Tags uint32) {
	(*get(object).(*build.File)).Tags = *get(Tags).(*map[string]string)
}
func defaultbuild_Form() uint32 {
	var tmp build.Form
	return register(&tmp)
}
func defaultbuild_Instance() uint32 {
	var tmp build.Instance
	return register(&tmp)
}
func get_build_Instance_AllTags(object uint32) uint32 {
	AllTags := (*get(object).(*build.Instance)).AllTags
	return register(&AllTags)
}
func set_build_Instance_AllTags(object uint32, AllTags uint32) {
	(*get(object).(*build.Instance)).AllTags = *get(AllTags).(*[]string)
}
func get_build_Instance_BuildFiles(object uint32) uint32 {
	BuildFiles := (*get(object).(*build.Instance)).BuildFiles
	return register(&BuildFiles)
}
func set_build_Instance_BuildFiles(object uint32, BuildFiles uint32) {
	(*get(object).(*build.Instance)).BuildFiles = *get(BuildFiles).(*[]*build.File)
}
func get_build_Instance_CUEFiles(object uint32) uint32 {
	CUEFiles := (*get(object).(*build.Instance)).CUEFiles
	return register(&CUEFiles)
}
func set_build_Instance_CUEFiles(object uint32, CUEFiles uint32) {
	(*get(object).(*build.Instance)).CUEFiles = *get(CUEFiles).(*[]string)
}
func get_build_Instance_DataFiles(object uint32) uint32 {
	DataFiles := (*get(object).(*build.Instance)).DataFiles
	return register(&DataFiles)
}
func set_build_Instance_DataFiles(object uint32, DataFiles uint32) {
	(*get(object).(*build.Instance)).DataFiles = *get(DataFiles).(*[]string)
}
func get_build_Instance_Deps(object uint32) uint32 {
	Deps := (*get(object).(*build.Instance)).Deps
	return register(&Deps)
}
func set_build_Instance_Deps(object uint32, Deps uint32) {
	(*get(object).(*build.Instance)).Deps = *get(Deps).(*[]string)
}
func get_build_Instance_DepsErrors(object uint32) uint32 {
	DepsErrors := (*get(object).(*build.Instance)).DepsErrors
	return register(&DepsErrors)
}
func set_build_Instance_DepsErrors(object uint32, DepsErrors uint32) {
	(*get(object).(*build.Instance)).DepsErrors = *get(DepsErrors).(*[]error)
}
func get_build_Instance_Dir(object uint32) string {
	Dir := (*get(object).(*build.Instance)).Dir
	return Dir
}
func set_build_Instance_Dir(object uint32, Dir string) {
	(*get(object).(*build.Instance)).Dir = Dir
}
func get_build_Instance_DisplayPath(object uint32) string {
	DisplayPath := (*get(object).(*build.Instance)).DisplayPath
	return DisplayPath
}
func set_build_Instance_DisplayPath(object uint32, DisplayPath string) {
	(*get(object).(*build.Instance)).DisplayPath = DisplayPath
}
func get_build_Instance_Err(object uint32) uint32 {
	Err := (*get(object).(*build.Instance)).Err
	return register(&Err)
}
func set_build_Instance_Err(object uint32, Err uint32) {
	(*get(object).(*build.Instance)).Err = *get(Err).(*errors.Error)
}
func get_build_Instance_Files(object uint32) uint32 {
	Files := (*get(object).(*build.Instance)).Files
	return register(&Files)
}
func set_build_Instance_Files(object uint32, Files uint32) {
	(*get(object).(*build.Instance)).Files = *get(Files).(*[]*ast.File)
}
func get_build_Instance_IgnoredCUEFiles(object uint32) uint32 {
	IgnoredCUEFiles := (*get(object).(*build.Instance)).IgnoredCUEFiles
	return register(&IgnoredCUEFiles)
}
func set_build_Instance_IgnoredCUEFiles(object uint32, IgnoredCUEFiles uint32) {
	(*get(object).(*build.Instance)).IgnoredCUEFiles = *get(IgnoredCUEFiles).(*[]string)
}
func get_build_Instance_IgnoredFiles(object uint32) uint32 {
	IgnoredFiles := (*get(object).(*build.Instance)).IgnoredFiles
	return register(&IgnoredFiles)
}
func set_build_Instance_IgnoredFiles(object uint32, IgnoredFiles uint32) {
	(*get(object).(*build.Instance)).IgnoredFiles = *get(IgnoredFiles).(*[]*build.File)
}
func get_build_Instance_ImportComment(object uint32) string {
	ImportComment := (*get(object).(*build.Instance)).ImportComment
	return ImportComment
}
func set_build_Instance_ImportComment(object uint32, ImportComment string) {
	(*get(object).(*build.Instance)).ImportComment = ImportComment
}
func get_build_Instance_ImportPath(object uint32) string {
	ImportPath := (*get(object).(*build.Instance)).ImportPath
	return ImportPath
}
func set_build_Instance_ImportPath(object uint32, ImportPath string) {
	(*get(object).(*build.Instance)).ImportPath = ImportPath
}
func get_build_Instance_ImportPaths(object uint32) uint32 {
	ImportPaths := (*get(object).(*build.Instance)).ImportPaths
	return register(&ImportPaths)
}
func set_build_Instance_ImportPaths(object uint32, ImportPaths uint32) {
	(*get(object).(*build.Instance)).ImportPaths = *get(ImportPaths).(*[]string)
}
func get_build_Instance_ImportPos(object uint32) uint32 {
	ImportPos := (*get(object).(*build.Instance)).ImportPos
	return register(&ImportPos)
}
func set_build_Instance_ImportPos(object uint32, ImportPos uint32) {
	(*get(object).(*build.Instance)).ImportPos = *get(ImportPos).(*map[string][]token.Pos)
}
func get_build_Instance_Imports(object uint32) uint32 {
	Imports := (*get(object).(*build.Instance)).Imports
	return register(&Imports)
}
func set_build_Instance_Imports(object uint32, Imports uint32) {
	(*get(object).(*build.Instance)).Imports = *get(Imports).(*[]*build.Instance)
}
func get_build_Instance_Incomplete(object uint32) bool {
	Incomplete := (*get(object).(*build.Instance)).Incomplete
	return Incomplete
}
func set_build_Instance_Incomplete(object uint32, Incomplete bool) {
	(*get(object).(*build.Instance)).Incomplete = Incomplete
}
func get_build_Instance_InvalidCUEFiles(object uint32) uint32 {
	InvalidCUEFiles := (*get(object).(*build.Instance)).InvalidCUEFiles
	return register(&InvalidCUEFiles)
}
func set_build_Instance_InvalidCUEFiles(object uint32, InvalidCUEFiles uint32) {
	(*get(object).(*build.Instance)).InvalidCUEFiles = *get(InvalidCUEFiles).(*[]string)
}
func get_build_Instance_InvalidFiles(object uint32) uint32 {
	InvalidFiles := (*get(object).(*build.Instance)).InvalidFiles
	return register(&InvalidFiles)
}
func set_build_Instance_InvalidFiles(object uint32, InvalidFiles uint32) {
	(*get(object).(*build.Instance)).InvalidFiles = *get(InvalidFiles).(*[]*build.File)
}
func get_build_Instance_Match(object uint32) uint32 {
	Match := (*get(object).(*build.Instance)).Match
	return register(&Match)
}
func set_build_Instance_Match(object uint32, Match uint32) {
	(*get(object).(*build.Instance)).Match = *get(Match).(*[]string)
}
func get_build_Instance_Module(object uint32) string {
	Module := (*get(object).(*build.Instance)).Module
	return Module
}
func set_build_Instance_Module(object uint32, Module string) {
	(*get(object).(*build.Instance)).Module = Module
}
func get_build_Instance_OrphanedFiles(object uint32) uint32 {
	OrphanedFiles := (*get(object).(*build.Instance)).OrphanedFiles
	return register(&OrphanedFiles)
}
func set_build_Instance_OrphanedFiles(object uint32, OrphanedFiles uint32) {
	(*get(object).(*build.Instance)).OrphanedFiles = *get(OrphanedFiles).(*[]*build.File)
}
func get_build_Instance_PkgName(object uint32) string {
	PkgName := (*get(object).(*build.Instance)).PkgName
	return PkgName
}
func set_build_Instance_PkgName(object uint32, PkgName string) {
	(*get(object).(*build.Instance)).PkgName = PkgName
}
func get_build_Instance_Root(object uint32) string {
	Root := (*get(object).(*build.Instance)).Root
	return Root
}
func set_build_Instance_Root(object uint32, Root string) {
	(*get(object).(*build.Instance)).Root = Root
}
func get_build_Instance_Scope(object uint32) uint32 {
	Scope := (*get(object).(*build.Instance)).Scope
	return register(&Scope)
}
func set_build_Instance_Scope(object uint32, Scope uint32) {
	(*get(object).(*build.Instance)).Scope = *get(Scope).(**build.Instance)
}
func get_build_Instance_Standard(object uint32) bool {
	Standard := (*get(object).(*build.Instance)).Standard
	return Standard
}
func set_build_Instance_Standard(object uint32, Standard bool) {
	(*get(object).(*build.Instance)).Standard = Standard
}
func get_build_Instance_TestCUEFiles(object uint32) uint32 {
	TestCUEFiles := (*get(object).(*build.Instance)).TestCUEFiles
	return register(&TestCUEFiles)
}
func set_build_Instance_TestCUEFiles(object uint32, TestCUEFiles uint32) {
	(*get(object).(*build.Instance)).TestCUEFiles = *get(TestCUEFiles).(*[]string)
}
func get_build_Instance_ToolCUEFiles(object uint32) uint32 {
	ToolCUEFiles := (*get(object).(*build.Instance)).ToolCUEFiles
	return register(&ToolCUEFiles)
}
func set_build_Instance_ToolCUEFiles(object uint32, ToolCUEFiles uint32) {
	(*get(object).(*build.Instance)).ToolCUEFiles = *get(ToolCUEFiles).(*[]string)
}
func get_build_Instance_UnknownFiles(object uint32) uint32 {
	UnknownFiles := (*get(object).(*build.Instance)).UnknownFiles
	return register(&UnknownFiles)
}
func set_build_Instance_UnknownFiles(object uint32, UnknownFiles uint32) {
	(*get(object).(*build.Instance)).UnknownFiles = *get(UnknownFiles).(*[]*build.File)
}
func get_build_Instance_User(object uint32) bool {
	User := (*get(object).(*build.Instance)).User
	return User
}
func set_build_Instance_User(object uint32, User bool) {
	(*get(object).(*build.Instance)).User = User
}
func defaultbuild_Interpretation() uint32 {
	var tmp build.Interpretation
	return register(&tmp)
}
func defaultparser_DeprecationError() uint32 {
	var tmp parser.DeprecationError
	return register(&tmp)
}
func get_parser_DeprecationError_Version(object uint32) int {
	Version := (*get(object).(*parser.DeprecationError)).Version
	return Version
}
func set_parser_DeprecationError_Version(object uint32, Version int) {
	(*get(object).(*parser.DeprecationError)).Version = Version
}
func f_cue_0_All() uint32 {
	r0 := cue.All()
	return register(&r0)
}
func f_cue_0_AppendFloat(o uint32, a0 uint32, a1 byte, a2 int) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).AppendFloat(*get(a0).(*[]byte), a1, a2)
	return register(&r0), exportError(r1)
}
func f_cue_0_AppendInt(o uint32, a0 uint32, a1 int) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).AppendInt(*get(a0).(*[]byte), a1)
	return register(&r0), exportError(r1)
}
func f_cue_0_Attribute(o uint32, a0 string) uint32 {
	r0 := (*get(o).(*cue.Value)).Attribute(a0)
	return register(&r0)
}
func f_cue_0_Attributes(a0 bool) uint32 {
	r0 := cue.Attributes(a0)
	return register(&r0)
}
func f_cue_0_Bool(o uint32) (bool, string) {
	r0, r1 := (*get(o).(*cue.Value)).Bool()
	return r0, exportError(r1)
}
func f_cue_0_Build(a0 uint32) uint32 {
	r0 := cue.Build(*get(a0).(*[]*build.Instance))
	return register(&r0)
}
func f_cue_1_Build(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(**cue.Instance)).Build(*get(a0).(**build.Instance))
	return register(&r0)
}
func f_cue_2_Build(o uint32, a0 uint32) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Runtime)).Build(*get(a0).(**build.Instance))
	return register(&r0), exportError(r1)
}
func f_cue_0_Bytes(o uint32) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).Bytes()
	return register(&r0), exportError(r1)
}
func f_cue_0_CanString(o uint32) bool {
	r0 := (*get(o).(*cue.Kind)).CanString()
	return r0
}
func f_cue_0_Compile(o uint32, a0 string, a1 uint32) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Runtime)).Compile(a0, reflect.ValueOf(get(a1)).Elem().Interface())
	return register(&r0), exportError(r1)
}
func f_cue_0_CompileExpr(o uint32, a0 uint32) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Runtime)).CompileExpr(*get(a0).(*ast.Expr))
	return register(&r0), exportError(r1)
}
func f_cue_0_CompileFile(o uint32, a0 uint32) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Runtime)).CompileFile(*get(a0).(**ast.File))
	return register(&r0), exportError(r1)
}
func f_cue_0_Concrete(a0 bool) uint32 {
	r0 := cue.Concrete(a0)
	return register(&r0)
}
func f_cue_0_Decimal(o uint32) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).Decimal()
	return register(&r0), exportError(r1)
}
func f_cue_0_Decode(o uint32, a0 uint32) string {
	r0 := (*get(o).(*cue.Value)).Decode(reflect.ValueOf(get(a0)).Elem().Interface())
	return exportError(r0)
}
func f_cue_0_Def(a0 string) uint32 {
	r0 := cue.Def(a0)
	return register(&r0)
}
func f_cue_0_Default(o uint32) (uint32, bool) {
	r0, r1 := (*get(o).(*cue.Value)).Default()
	return register(&r0), r1
}
func f_cue_0_Definitions(a0 bool) uint32 {
	r0 := cue.Definitions(a0)
	return register(&r0)
}
func f_cue_0_Dereference(a0 uint32) uint32 {
	r0 := cue.Dereference(*get(a0).(*cue.Value))
	return register(&r0)
}
func f_cue_0_DisallowCycles(a0 bool) uint32 {
	r0 := cue.DisallowCycles(a0)
	return register(&r0)
}
func f_cue_0_Doc(o uint32) uint32 {
	r0 := (*get(o).(**cue.Instance)).Doc()
	return register(&r0)
}
func f_cue_1_Doc(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Doc()
	return register(&r0)
}
func f_cue_0_Docs(a0 bool) uint32 {
	r0 := cue.Docs(a0)
	return register(&r0)
}
func f_cue_0_Elem(o uint32) (uint32, bool) {
	r0, r1 := (*get(o).(*cue.Value)).Elem()
	return register(&r0), r1
}
func f_cue_0_Equals(o uint32, a0 uint32) bool {
	r0 := (*get(o).(*cue.Value)).Equals(*get(a0).(*cue.Value))
	return r0
}
func f_cue_0_Err(o uint32) string {
	r0 := (*get(o).(**cue.Attribute)).Err()
	return exportError(r0)
}
func f_cue_1_Err(o uint32) string {
	r0 := (*get(o).(*cue.Path)).Err()
	return exportError(r0)
}
func f_cue_2_Err(o uint32) string {
	r0 := (*get(o).(*cue.Value)).Err()
	return exportError(r0)
}
func f_cue_0_Eval(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(**cue.Instance)).Eval(*get(a0).(*ast.Expr))
	return register(&r0)
}
func f_cue_1_Eval(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Eval()
	return register(&r0)
}
func f_cue_0_Exists(o uint32) bool {
	r0 := (*get(o).(*cue.Value)).Exists()
	return r0
}
func f_cue_0_Expr(o uint32) (uint32, uint32) {
	r0, r1 := (*get(o).(*cue.Value)).Expr()
	return register(&r0), register(&r1)
}
func f_cue_0_Field(o uint32, a0 int) uint32 {
	r0 := (*get(o).(**cue.Struct)).Field(a0)
	return register(&r0)
}
func f_cue_0_FieldByName(o uint32, a0 string, a1 bool) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Struct)).FieldByName(a0, a1)
	return register(&r0), exportError(r1)
}
func f_cue_1_FieldByName(o uint32, a0 string, a1 bool) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).FieldByName(a0, a1)
	return register(&r0), exportError(r1)
}
func f_cue_0_Fields(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(**cue.Struct)).Fields(*get(a0).(*[]cue.Option)...)
	return register(&r0)
}
func f_cue_1_Fields(o uint32, a0 uint32) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).Fields(*get(a0).(*[]cue.Option)...)
	return register(&r0), exportError(r1)
}
func f_cue_0_Fill(o uint32, a0 uint32, a1 uint32) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Instance)).Fill(reflect.ValueOf(get(a0)).Elem().Interface(), *get(a1).(*[]string)...)
	return register(&r0), exportError(r1)
}
func f_cue_1_Fill(o uint32, a0 uint32, a1 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Fill(reflect.ValueOf(get(a0)).Elem().Interface(), *get(a1).(*[]string)...)
	return register(&r0)
}
func f_cue_0_Final() uint32 {
	r0 := cue.Final()
	return register(&r0)
}
func f_cue_0_Flag(o uint32, a0 int, a1 string) (bool, string) {
	r0, r1 := (*get(o).(**cue.Attribute)).Flag(a0, a1)
	return r0, exportError(r1)
}
func f_cue_0_Float64(o uint32) (float64, string) {
	r0, r1 := (*get(o).(*cue.Value)).Float64()
	return r0, exportError(r1)
}
func f_cue_0_Format(o uint32, a0 uint32, a1 rune) {
	(*get(o).(*cue.Value)).Format(*get(a0).(*fmt.State), a1)
}
func f_cue_0_FromExpr(o uint32, a0 uint32) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Runtime)).FromExpr(*get(a0).(*ast.Expr))
	return register(&r0), exportError(r1)
}
func f_cue_0_Hidden(a0 bool) uint32 {
	r0 := cue.Hidden(a0)
	return register(&r0)
}
func f_cue_0_ID(o uint32) string {
	r0 := (*get(o).(**cue.Instance)).ID()
	return r0
}
func f_cue_0_IncompleteKind(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).IncompleteKind()
	return register(&r0)
}
func f_cue_0_Index(a0 int) uint32 {
	r0 := cue.Index(a0)
	return register(&r0)
}
func f_cue_0_Int(o uint32, a0 int) (int64, string) {
	r0, r1 := (*get(o).(**cue.Attribute)).Int(a0)
	return r0, exportError(r1)
}
func f_cue_1_Int(o uint32, a0 uint32) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).Int(*get(a0).(**big.Int))
	return register(&r0), exportError(r1)
}
func f_cue_0_Int64(o uint32) (int64, string) {
	r0, r1 := (*get(o).(*cue.Value)).Int64()
	return r0, exportError(r1)
}
func f_cue_0_IsAnyOf(o uint32, a0 uint32) bool {
	r0 := (*get(o).(*cue.Kind)).IsAnyOf(*get(a0).(*cue.Kind))
	return r0
}
func f_cue_0_IsClosed(o uint32) bool {
	r0 := (*get(o).(*cue.Value)).IsClosed()
	return r0
}
func f_cue_0_IsConcrete(o uint32) bool {
	r0 := (*get(o).(*cue.Value)).IsConcrete()
	return r0
}
func f_cue_0_IsDefinition(o uint32) bool {
	r0 := (*get(o).(**cue.Iterator)).IsDefinition()
	return r0
}
func f_cue_0_IsHidden(o uint32) bool {
	r0 := (*get(o).(**cue.Iterator)).IsHidden()
	return r0
}
func f_cue_0_IsOptional(o uint32) bool {
	r0 := (*get(o).(**cue.Iterator)).IsOptional()
	return r0
}
func f_cue_0_Kind(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Kind()
	return register(&r0)
}
func f_cue_0_Label(o uint32) string {
	r0 := (*get(o).(**cue.Iterator)).Label()
	return r0
}
func f_cue_1_Label(o uint32) (string, bool) {
	r0, r1 := (*get(o).(*cue.Value)).Label()
	return r0, r1
}
func f_cue_0_Len(o uint32) int {
	r0 := (*get(o).(**cue.Struct)).Len()
	return r0
}
func f_cue_1_Len(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Len()
	return register(&r0)
}
func f_cue_0_List(o uint32) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).List()
	return register(&r0), exportError(r1)
}
func f_cue_0_Lookup(o uint32, a0 int, a1 string) (string, bool, string) {
	r0, r1, r2 := (*get(o).(**cue.Attribute)).Lookup(a0, a1)
	return r0, r1, exportError(r2)
}
func f_cue_1_Lookup(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(**cue.Instance)).Lookup(*get(a0).(*[]string)...)
	return register(&r0)
}
func f_cue_2_Lookup(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Lookup(*get(a0).(*[]string)...)
	return register(&r0)
}
func f_cue_0_LookupDef(o uint32, a0 string) uint32 {
	r0 := (*get(o).(**cue.Instance)).LookupDef(a0)
	return register(&r0)
}
func f_cue_1_LookupDef(o uint32, a0 string) uint32 {
	r0 := (*get(o).(*cue.Value)).LookupDef(a0)
	return register(&r0)
}
func f_cue_0_LookupField(o uint32, a0 uint32) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Instance)).LookupField(*get(a0).(*[]string)...)
	return register(&r0), exportError(r1)
}
func f_cue_1_LookupField(o uint32, a0 string) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).LookupField(a0)
	return register(&r0), exportError(r1)
}
func f_cue_0_LookupPath(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).LookupPath(*get(a0).(*cue.Path))
	return register(&r0)
}
func f_cue_0_MakePath(a0 uint32) uint32 {
	r0 := cue.MakePath(*get(a0).(*[]cue.Selector)...)
	return register(&r0)
}
func f_cue_0_MantExp(o uint32, a0 uint32) (int, string) {
	r0, r1 := (*get(o).(*cue.Value)).MantExp(*get(a0).(**big.Int))
	return r0, exportError(r1)
}
func f_cue_0_Marshal(o uint32, a0 uint32) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Runtime)).Marshal(*get(a0).(*[]*cue.Instance)...)
	return register(&r0), exportError(r1)
}
func f_cue_0_MarshalJSON(o uint32) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).MarshalJSON()
	return register(&r0), exportError(r1)
}
func f_cue_0_Merge(a0 uint32) uint32 {
	r0 := cue.Merge(*get(a0).(*[]*cue.Instance)...)
	return register(&r0)
}
func f_cue_0_Next(o uint32) bool {
	r0 := (*get(o).(**cue.Iterator)).Next()
	return r0
}
func f_cue_0_Null(o uint32) string {
	r0 := (*get(o).(*cue.Value)).Null()
	return exportError(r0)
}
func f_cue_0_Optional(a0 bool) uint32 {
	r0 := cue.Optional(a0)
	return register(&r0)
}
func f_cue_0_Parse(o uint32, a0 string, a1 uint32) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Runtime)).Parse(a0, reflect.ValueOf(get(a1)).Elem().Interface())
	return register(&r0), exportError(r1)
}
func f_cue_0_ParsePath(a0 string) uint32 {
	r0 := cue.ParsePath(a0)
	return register(&r0)
}
func f_cue_0_Path(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Path()
	return register(&r0)
}
func f_cue_0_Pos(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Pos()
	return register(&r0)
}
func f_cue_0_Raw() uint32 {
	r0 := cue.Raw()
	return register(&r0)
}
func f_cue_0_Reader(o uint32) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).Reader()
	return register(&r0), exportError(r1)
}
func f_cue_0_Reference(o uint32) (uint32, uint32) {
	r0, r1 := (*get(o).(*cue.Value)).Reference()
	return register(&r0), register(&r1)
}
func f_cue_0_ResolveReferences(a0 bool) uint32 {
	r0 := cue.ResolveReferences(a0)
	return register(&r0)
}
func f_cue_0_Schema() uint32 {
	r0 := cue.Schema()
	return register(&r0)
}
func f_cue_0_Selectors(o uint32) uint32 {
	r0 := (*get(o).(*cue.Path)).Selectors()
	return register(&r0)
}
func f_cue_0_Source(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Source()
	return register(&r0)
}
func f_cue_0_Split(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Split()
	return register(&r0)
}
func f_cue_0_Str(a0 string) uint32 {
	r0 := cue.Str(a0)
	return register(&r0)
}
func f_cue_0_String(o uint32, a0 int) (string, string) {
	r0, r1 := (*get(o).(**cue.Attribute)).String(a0)
	return r0, exportError(r1)
}
func f_cue_1_String(o uint32) string {
	r0 := (*get(o).(*cue.Kind)).String()
	return r0
}
func f_cue_2_String(o uint32) string {
	r0 := (*get(o).(*cue.Op)).String()
	return r0
}
func f_cue_3_String(o uint32) string {
	r0 := (*get(o).(*cue.Path)).String()
	return r0
}
func f_cue_4_String(o uint32) string {
	r0 := (*get(o).(*cue.Selector)).String()
	return r0
}
func f_cue_5_String(o uint32) (string, string) {
	r0, r1 := (*get(o).(*cue.Value)).String()
	return r0, exportError(r1)
}
func f_cue_0_Struct(o uint32) (uint32, string) {
	r0, r1 := (*get(o).(*cue.Value)).Struct()
	return register(&r0), exportError(r1)
}
func f_cue_0_Subsume(o uint32, a0 uint32, a1 uint32) string {
	r0 := (*get(o).(*cue.Value)).Subsume(*get(a0).(*cue.Value), *get(a1).(*[]cue.Option)...)
	return exportError(r0)
}
func f_cue_0_Subsumes(o uint32, a0 uint32) bool {
	r0 := (*get(o).(*cue.Value)).Subsumes(*get(a0).(*cue.Value))
	return r0
}
func f_cue_0_Syntax(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Syntax(*get(a0).(*[]cue.Option)...)
	return register(&r0)
}
func f_cue_0_Token(o uint32) uint32 {
	r0 := (*get(o).(*cue.Op)).Token()
	return register(&r0)
}
func f_cue_0_TypeString(o uint32) string {
	r0 := (*get(o).(*cue.Kind)).TypeString()
	return r0
}
func f_cue_0_Uint64(o uint32) (uint64, string) {
	r0, r1 := (*get(o).(*cue.Value)).Uint64()
	return r0, exportError(r1)
}
func f_cue_0_Unify(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Unify(*get(a0).(*cue.Value))
	return register(&r0)
}
func f_cue_0_UnifyAccept(o uint32, a0 uint32, a1 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).UnifyAccept(*get(a0).(*cue.Value), *get(a1).(*cue.Value))
	return register(&r0)
}
func f_cue_0_Unmarshal(o uint32, a0 uint32) (uint32, string) {
	r0, r1 := (*get(o).(**cue.Runtime)).Unmarshal(*get(a0).(*[]byte))
	return register(&r0), exportError(r1)
}
func f_cue_0_Validate(o uint32, a0 uint32) string {
	r0 := (*get(o).(*cue.Value)).Validate(*get(a0).(*[]cue.Option)...)
	return exportError(r0)
}
func f_cue_0_Value(o uint32) uint32 {
	r0 := (*get(o).(**cue.Instance)).Value()
	return register(&r0)
}
func f_cue_1_Value(o uint32) uint32 {
	r0 := (*get(o).(**cue.Iterator)).Value()
	return register(&r0)
}
func f_ast_0_AddComment(a0 uint32, a1 uint32) {
	ast.AddComment(*get(a0).(*ast.Node), *get(a1).(**ast.CommentGroup))
}
func f_ast_1_AddComment(o uint32, a0 uint32) {
	(*get(o).(**ast.Comment)).AddComment(*get(a0).(**ast.CommentGroup))
}
func f_ast_2_AddComment(o uint32, a0 uint32) {
	(*get(o).(**ast.CommentGroup)).AddComment(*get(a0).(**ast.CommentGroup))
}
func f_ast_0_Comments(a0 uint32) uint32 {
	r0 := ast.Comments(*get(a0).(*ast.Node))
	return register(&r0)
}
func f_ast_1_Comments(o uint32) uint32 {
	r0 := (*get(o).(**ast.Comment)).Comments()
	return register(&r0)
}
func f_ast_2_Comments(o uint32) uint32 {
	r0 := (*get(o).(**ast.CommentGroup)).Comments()
	return register(&r0)
}
func f_ast_0_Embed(a0 uint32) uint32 {
	r0 := ast.Embed(*get(a0).(*ast.Expr))
	return register(&r0)
}
func f_ast_0_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Alias)).End()
	return register(&r0)
}
func f_ast_1_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Attribute)).End()
	return register(&r0)
}
func f_ast_2_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.BadDecl)).End()
	return register(&r0)
}
func f_ast_3_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.BadExpr)).End()
	return register(&r0)
}
func f_ast_4_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.BasicLit)).End()
	return register(&r0)
}
func f_ast_5_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.BinaryExpr)).End()
	return register(&r0)
}
func f_ast_6_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.BottomLit)).End()
	return register(&r0)
}
func f_ast_7_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.CallExpr)).End()
	return register(&r0)
}
func f_ast_8_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Comment)).End()
	return register(&r0)
}
func f_ast_9_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.CommentGroup)).End()
	return register(&r0)
}
func f_ast_10_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Comprehension)).End()
	return register(&r0)
}
func f_ast_11_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Ellipsis)).End()
	return register(&r0)
}
func f_ast_12_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.EmbedDecl)).End()
	return register(&r0)
}
func f_ast_13_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Field)).End()
	return register(&r0)
}
func f_ast_14_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.File)).End()
	return register(&r0)
}
func f_ast_15_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ForClause)).End()
	return register(&r0)
}
func f_ast_16_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Ident)).End()
	return register(&r0)
}
func f_ast_17_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.IfClause)).End()
	return register(&r0)
}
func f_ast_18_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ImportDecl)).End()
	return register(&r0)
}
func f_ast_19_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ImportSpec)).End()
	return register(&r0)
}
func f_ast_20_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.IndexExpr)).End()
	return register(&r0)
}
func f_ast_21_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Interpolation)).End()
	return register(&r0)
}
func f_ast_22_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.LetClause)).End()
	return register(&r0)
}
func f_ast_23_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ListComprehension)).End()
	return register(&r0)
}
func f_ast_24_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ListLit)).End()
	return register(&r0)
}
func f_ast_25_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Package)).End()
	return register(&r0)
}
func f_ast_26_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ParenExpr)).End()
	return register(&r0)
}
func f_ast_27_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.SelectorExpr)).End()
	return register(&r0)
}
func f_ast_28_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.SliceExpr)).End()
	return register(&r0)
}
func f_ast_29_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.StructLit)).End()
	return register(&r0)
}
func f_ast_30_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.TemplateLabel)).End()
	return register(&r0)
}
func f_ast_31_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.UnaryExpr)).End()
	return register(&r0)
}
func f_ast_0_IsValidIdent(a0 string) bool {
	r0 := ast.IsValidIdent(a0)
	return r0
}
func f_ast_0_LabelName(a0 uint32) (string, bool, string) {
	r0, r1, r2 := ast.LabelName(*get(a0).(*ast.Label))
	return r0, r1, exportError(r2)
}
func f_ast_0_Name(a0 uint32) string {
	r0 := ast.Name(*get(a0).(*ast.Node))
	return r0
}
func f_ast_0_NewBinExpr(a0 uint32, a1 uint32) uint32 {
	r0 := ast.NewBinExpr(*get(a0).(*token.Token), *get(a1).(*[]ast.Expr)...)
	return register(&r0)
}
func f_ast_0_NewBool(a0 bool) uint32 {
	r0 := ast.NewBool(a0)
	return register(&r0)
}
func f_ast_0_NewCall(a0 uint32, a1 uint32) uint32 {
	r0 := ast.NewCall(*get(a0).(*ast.Expr), *get(a1).(*[]ast.Expr)...)
	return register(&r0)
}
func f_ast_0_NewIdent(a0 string) uint32 {
	r0 := ast.NewIdent(a0)
	return register(&r0)
}
func f_ast_0_NewImport(a0 uint32, a1 string) uint32 {
	r0 := ast.NewImport(*get(a0).(**ast.Ident), a1)
	return register(&r0)
}
func f_ast_0_NewList(a0 uint32) uint32 {
	r0 := ast.NewList(*get(a0).(*[]ast.Expr)...)
	return register(&r0)
}
func f_ast_0_NewLit(a0 uint32, a1 string) uint32 {
	r0 := ast.NewLit(*get(a0).(*token.Token), a1)
	return register(&r0)
}
func f_ast_0_NewNull() uint32 {
	r0 := ast.NewNull()
	return register(&r0)
}
func f_ast_0_NewSel(a0 uint32, a1 uint32) uint32 {
	r0 := ast.NewSel(*get(a0).(*ast.Expr), *get(a1).(*[]string)...)
	return register(&r0)
}
func f_ast_0_NewString(a0 string) uint32 {
	r0 := ast.NewString(a0)
	return register(&r0)
}
func f_ast_0_NewStruct(a0 uint32) uint32 {
	r0 := ast.NewStruct(*get(a0).(*[]interface {
	})...)
	return register(&r0)
}
func f_ast_0_PackageName(o uint32) string {
	r0 := (*get(o).(**ast.File)).PackageName()
	return r0
}
func f_ast_0_ParseIdent(a0 uint32) (string, string) {
	r0, r1 := ast.ParseIdent(*get(a0).(**ast.Ident))
	return r0, exportError(r1)
}
func f_ast_0_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Alias)).Pos()
	return register(&r0)
}
func f_ast_1_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Attribute)).Pos()
	return register(&r0)
}
func f_ast_2_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.BadDecl)).Pos()
	return register(&r0)
}
func f_ast_3_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.BadExpr)).Pos()
	return register(&r0)
}
func f_ast_4_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.BasicLit)).Pos()
	return register(&r0)
}
func f_ast_5_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.BinaryExpr)).Pos()
	return register(&r0)
}
func f_ast_6_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.BottomLit)).Pos()
	return register(&r0)
}
func f_ast_7_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.CallExpr)).Pos()
	return register(&r0)
}
func f_ast_8_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Comment)).Pos()
	return register(&r0)
}
func f_ast_9_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.CommentGroup)).Pos()
	return register(&r0)
}
func f_ast_10_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Comprehension)).Pos()
	return register(&r0)
}
func f_ast_11_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Ellipsis)).Pos()
	return register(&r0)
}
func f_ast_12_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.EmbedDecl)).Pos()
	return register(&r0)
}
func f_ast_13_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Field)).Pos()
	return register(&r0)
}
func f_ast_14_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.File)).Pos()
	return register(&r0)
}
func f_ast_15_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ForClause)).Pos()
	return register(&r0)
}
func f_ast_16_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Ident)).Pos()
	return register(&r0)
}
func f_ast_17_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.IfClause)).Pos()
	return register(&r0)
}
func f_ast_18_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ImportDecl)).Pos()
	return register(&r0)
}
func f_ast_19_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ImportSpec)).Pos()
	return register(&r0)
}
func f_ast_20_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.IndexExpr)).Pos()
	return register(&r0)
}
func f_ast_21_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Interpolation)).Pos()
	return register(&r0)
}
func f_ast_22_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.LetClause)).Pos()
	return register(&r0)
}
func f_ast_23_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ListComprehension)).Pos()
	return register(&r0)
}
func f_ast_24_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ListLit)).Pos()
	return register(&r0)
}
func f_ast_25_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Package)).Pos()
	return register(&r0)
}
func f_ast_26_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ParenExpr)).Pos()
	return register(&r0)
}
func f_ast_27_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.SelectorExpr)).Pos()
	return register(&r0)
}
func f_ast_28_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.SliceExpr)).Pos()
	return register(&r0)
}
func f_ast_29_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.StructLit)).Pos()
	return register(&r0)
}
func f_ast_30_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.TemplateLabel)).Pos()
	return register(&r0)
}
func f_ast_31_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.UnaryExpr)).Pos()
	return register(&r0)
}
func f_ast_0_Preamble(o uint32) uint32 {
	r0 := (*get(o).(**ast.File)).Preamble()
	return register(&r0)
}
func f_ast_0_QuoteIdent(a0 string) (string, string) {
	r0, r1 := ast.QuoteIdent(a0)
	return r0, exportError(r1)
}
func f_ast_0_SetComments(a0 uint32, a1 uint32) {
	ast.SetComments(*get(a0).(*ast.Node), *get(a1).(*[]*ast.CommentGroup))
}
func f_ast_0_SetPos(a0 uint32, a1 uint32) {
	ast.SetPos(*get(a0).(*ast.Node), *get(a1).(*token.Pos))
}
func f_ast_0_SetRelPos(a0 uint32, a1 uint32) {
	ast.SetRelPos(*get(a0).(*ast.Node), *get(a1).(*token.RelPos))
}
func f_ast_0_Split(o uint32) (string, string) {
	r0, r1 := (*get(o).(**ast.Attribute)).Split()
	return r0, r1
}
func f_ast_0_String(o uint32) string {
	r0 := (*get(o).(**ast.Ident)).String()
	return r0
}
func f_ast_0_Text(o uint32) string {
	r0 := (*get(o).(**ast.CommentGroup)).Text()
	return r0
}
func f_astutil_0_ApplyRecursively(a0 uint32) uint32 {
	r0 := astutil.ApplyRecursively(*get(a0).(*ast.Node))
	return register(&r0)
}
func f_astutil_0_CopyComments(a0 uint32, a1 uint32) {
	astutil.CopyComments(*get(a0).(*ast.Node), *get(a1).(*ast.Node))
}
func f_astutil_0_CopyMeta(a0 uint32, a1 uint32) uint32 {
	r0 := astutil.CopyMeta(*get(a0).(*ast.Node), *get(a1).(*ast.Node))
	return register(&r0)
}
func f_astutil_0_CopyPosition(a0 uint32, a1 uint32) {
	astutil.CopyPosition(*get(a0).(*ast.Node), *get(a1).(*ast.Node))
}
func f_astutil_0_ParseImportSpec(a0 uint32) (uint32, string) {
	r0, r1 := astutil.ParseImportSpec(*get(a0).(**ast.ImportSpec))
	return register(&r0), exportError(r1)
}
func f_astutil_0_Resolve(a0 uint32, a1 uint32) {
	astutil.Resolve(*get(a0).(**ast.File), *get(a1).(*astutil.ErrFunc))
}
func f_astutil_0_ResolveExpr(a0 uint32, a1 uint32) {
	astutil.ResolveExpr(*get(a0).(*ast.Expr), *get(a1).(*astutil.ErrFunc))
}
func f_astutil_0_Sanitize(a0 uint32) string {
	r0 := astutil.Sanitize(*get(a0).(**ast.File))
	return exportError(r0)
}
func f_astutil_0_ToFile(a0 uint32) (uint32, string) {
	r0, r1 := astutil.ToFile(*get(a0).(*ast.Expr))
	return register(&r0), exportError(r1)
}
func f_build_0_Abs(o uint32, a0 string) string {
	r0 := (*get(o).(**build.Instance)).Abs(a0)
	return r0
}
func f_build_0_AddFile(o uint32, a0 string, a1 uint32) string {
	r0 := (*get(o).(**build.Instance)).AddFile(a0, reflect.ValueOf(get(a1)).Elem().Interface())
	return exportError(r0)
}
func f_build_0_AddSyntax(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(**build.Instance)).AddSyntax(*get(a0).(**ast.File))
	return register(&r0)
}
func f_build_0_Complete(o uint32) string {
	r0 := (*get(o).(**build.Instance)).Complete()
	return exportError(r0)
}
func f_build_0_Context(o uint32) uint32 {
	r0 := (*get(o).(**build.Instance)).Context()
	return register(&r0)
}
func f_build_0_Dependencies(o uint32) uint32 {
	r0 := (*get(o).(**build.Instance)).Dependencies()
	return register(&r0)
}
func f_build_0_ID(o uint32) string {
	r0 := (*get(o).(**build.Instance)).ID()
	return r0
}
func f_build_0_IsLocalImport(a0 string) bool {
	r0 := build.IsLocalImport(a0)
	return r0
}
func f_build_0_Loader(a0 uint32) uint32 {
	r0 := build.Loader(*get(a0).(*build.LoadFunc))
	return register(&r0)
}
func f_build_0_LookupImport(o uint32, a0 string) uint32 {
	r0 := (*get(o).(**build.Instance)).LookupImport(a0)
	return register(&r0)
}
func f_build_0_NewContext(a0 uint32) uint32 {
	r0 := build.NewContext(*get(a0).(*[]build.Option)...)
	return register(&r0)
}
func f_build_0_NewInstance(o uint32, a0 string, a1 uint32) uint32 {
	r0 := (*get(o).(**build.Context)).NewInstance(a0, *get(a1).(*build.LoadFunc))
	return register(&r0)
}
func f_build_0_ReportError(o uint32, a0 uint32) {
	(*get(o).(**build.Instance)).ReportError(*get(a0).(*errors.Error))
}
func f_parser_0_Error(o uint32) string {
	r0 := (*get(o).(**parser.DeprecationError)).Error()
	return r0
}
func f_parser_0_FileOffset(a0 int) uint32 {
	r0 := parser.FileOffset(a0)
	return register(&r0)
}
func f_parser_0_FromVersion(a0 int) uint32 {
	r0 := parser.FromVersion(a0)
	return register(&r0)
}
func f_parser_0_ParseExpr(a0 string, a1 uint32, a2 uint32) (uint32, string) {
	r0, r1 := parser.ParseExpr(a0, reflect.ValueOf(get(a1)).Elem().Interface(), *get(a2).(*[]parser.Option)...)
	return register(&r0), exportError(r1)
}
func f_parser_0_ParseFile(a0 string, a1 uint32, a2 uint32) (uint32, string) {
	r0, r1 := parser.ParseFile(a0, reflect.ValueOf(get(a1)).Elem().Interface(), *get(a2).(*[]parser.Option)...)
	return register(&r0), exportError(r1)
}
func exportError(object error) string {
	if object == nil {
		return ""
	}
	return object.Error()
}
func dereference(object uint32) (uint32, bool) {
	tmp := reflect.ValueOf(get(object)).Elem()
	ty := tmp.Kind()
	if ty != reflect.Ptr && ty != reflect.Interface {
		return 0, false
	}
	return register(tmp.Interface()), true
}
func isNil(object uint32) bool {
	return reflect.ValueOf(get(object)).Elem().IsNil()
}
func nilInterface() uint32 {
	var tmp interface {
	}
	return register(&tmp)
}
func cast(this uint32, object uint32) (uint32, bool) {
	tmp, ty := reflect.ValueOf(get(this)).Elem(), reflect.ValueOf(get(object)).Elem().Type()
	if tmp.Kind() == reflect.Interface {
		tmp = tmp.Elem()
	}
	if !tmp.Type().ConvertibleTo(ty) {
		return 0, false
	}
	tmp2 := reflect.New(ty)
	tmp2.Elem().Set(tmp.Convert(ty))
	return register(tmp2.Interface()), true
}
func makeSlice(object uint32, capacity int) uint32 {
	proxyMap.lock.RLock()
	ty := reflect.SliceOf(proxyMap.all[object].ty)
	proxyMap.lock.RUnlock()
	tmp := reflect.New(ty)
	tmp.Elem().Set(reflect.MakeSlice(ty, 0, capacity))
	return register(tmp.Interface())
}
func sliceLen(object uint32) int {
	return reflect.ValueOf(get(object)).Elem().Len()
}
func getObjectInSlice(object uint32, index int) (uint32, bool) {
	tmp := reflect.ValueOf(get(object)).Elem()
	if index >= tmp.Len() {
		return 0, false
	}
	return register(tmp.Index(index).Addr().Interface()), true
}
func push(this uint32, object uint32) {
	proxyMap.lock.Lock()
	defer proxyMap.lock.Unlock()
	tmp := proxyMap.all[this]
	reflect.ValueOf(tmp.object).Elem().Set(reflect.Append(reflect.ValueOf(tmp.object).Elem(), reflect.ValueOf(proxyMap.all[object].object).Elem()))
	proxyMap.all[this] = tmp
}
func makeMap(key uint32, value uint32, capacity int) uint32 {
	proxyMap.lock.RLock()
	ty := reflect.MapOf(proxyMap.all[key].ty, proxyMap.all[value].ty)
	proxyMap.lock.RUnlock()
	tmp := reflect.New(ty)
	tmp.Elem().Set(reflect.MakeMapWithSize(ty, capacity))
	return register(tmp.Interface())
}
func insert(this uint32, key uint32, value uint32) {
	reflect.ValueOf(get(this)).Elem().SetMapIndex(reflect.ValueOf(get(key)).Elem(), reflect.ValueOf(get(value)).Elem())
}

type proxyedObject struct {
	object interface {
	}
	ty reflect.Type
}
type proxyMapType struct {
	lock   sync.RWMutex
	all    map[uint32]proxyedObject
	nextID uint32
}

var proxyMap = proxyMapType{all: make(map[uint32]proxyedObject, 0)}

func register(object interface {
}) uint32 {
	proxyMap.lock.Lock()
	defer proxyMap.lock.Unlock()
	id := proxyMap.nextID
	proxyMap.nextID++
	proxyMap.all[id] = proxyedObject{object: object, ty: reflect.ValueOf(object).Elem().Type()}
	return id
}
func newPointer(id uint32) uint32 {
	proxyMap.lock.RLock()
	tmp, ok := proxyMap.all[id]
	proxyMap.lock.RUnlock()
	if !ok {
		panic("newPointer() called with an invalid handle")
	}
	object := reflect.New(reflect.PtrTo(tmp.ty))
	object.Elem().Set(reflect.ValueOf(tmp.object))
	return register(object.Interface())
}
func get(id uint32) interface {
} {
	proxyMap.lock.RLock()
	defer proxyMap.lock.RUnlock()
	tmp, ok := proxyMap.all[id]
	if !ok {
		panic("get() called with an invalid handle")
	}
	return tmp.object
}
func forget(id uint32) {
	proxyMap.lock.Lock()
	defer proxyMap.lock.Unlock()
	delete(proxyMap.all, id)
}
func proxyBool(object bool) uint32 {
	return register(&object)
}
func proxyFloat32(object float32) uint32 {
	return register(&object)
}
func proxyFloat64(object float64) uint32 {
	return register(&object)
}
func proxyInt16(object int16) uint32 {
	return register(&object)
}
func proxyInt32(object int32) uint32 {
	return register(&object)
}
func proxyInt64(object int64) uint32 {
	return register(&object)
}
func proxyInt8(object int8) uint32 {
	return register(&object)
}
func proxyUint16(object uint16) uint32 {
	return register(&object)
}
func proxyUint32(object uint32) uint32 {
	return register(&object)
}
func proxyUint64(object uint64) uint32 {
	return register(&object)
}
func proxyUint8(object uint8) uint32 {
	return register(&object)
}
func proxyString(object string) uint32 {
	tmp := object
	return register(&tmp)
}
func main() {
	namespace := js.ValueOf(make(map[string]interface {
	}, 0))
	namespace.Set("defaultcue_Attribute", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_Attribute()
		return r0
	}))
	namespace.Set("defaultcue_FieldInfo", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_FieldInfo()
		return r0
	}))
	namespace.Set("defaultcue_Instance", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_Instance()
		return r0
	}))
	namespace.Set("defaultcue_Iterator", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_Iterator()
		return r0
	}))
	namespace.Set("defaultcue_Kind", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_Kind()
		return r0
	}))
	namespace.Set("defaultcue_Op", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_Op()
		return r0
	}))
	namespace.Set("defaultcue_Path", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_Path()
		return r0
	}))
	namespace.Set("defaultcue_Runtime", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_Runtime()
		return r0
	}))
	namespace.Set("defaultcue_Selector", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_Selector()
		return r0
	}))
	namespace.Set("defaultcue_Struct", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_Struct()
		return r0
	}))
	namespace.Set("defaultcue_Value", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultcue_Value()
		return r0
	}))
	namespace.Set("defaultast_Alias", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_Alias()
		return r0
	}))
	namespace.Set("defaultast_Attribute", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_Attribute()
		return r0
	}))
	namespace.Set("defaultast_BadDecl", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_BadDecl()
		return r0
	}))
	namespace.Set("defaultast_BadExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_BadExpr()
		return r0
	}))
	namespace.Set("defaultast_BasicLit", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_BasicLit()
		return r0
	}))
	namespace.Set("defaultast_BinaryExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_BinaryExpr()
		return r0
	}))
	namespace.Set("defaultast_BottomLit", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_BottomLit()
		return r0
	}))
	namespace.Set("defaultast_CallExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_CallExpr()
		return r0
	}))
	namespace.Set("as_ast_Clause", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := as_ast_Clause(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("defaultast_Comment", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_Comment()
		return r0
	}))
	namespace.Set("defaultast_CommentGroup", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_CommentGroup()
		return r0
	}))
	namespace.Set("defaultast_Comprehension", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_Comprehension()
		return r0
	}))
	namespace.Set("as_ast_Decl", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := as_ast_Decl(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("defaultast_Ellipsis", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_Ellipsis()
		return r0
	}))
	namespace.Set("defaultast_EmbedDecl", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_EmbedDecl()
		return r0
	}))
	namespace.Set("as_ast_Expr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := as_ast_Expr(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("defaultast_Field", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_Field()
		return r0
	}))
	namespace.Set("defaultast_File", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_File()
		return r0
	}))
	namespace.Set("defaultast_ForClause", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_ForClause()
		return r0
	}))
	namespace.Set("defaultast_Ident", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_Ident()
		return r0
	}))
	namespace.Set("defaultast_IfClause", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_IfClause()
		return r0
	}))
	namespace.Set("defaultast_ImportDecl", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_ImportDecl()
		return r0
	}))
	namespace.Set("defaultast_ImportSpec", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_ImportSpec()
		return r0
	}))
	namespace.Set("defaultast_IndexExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_IndexExpr()
		return r0
	}))
	namespace.Set("defaultast_Interpolation", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_Interpolation()
		return r0
	}))
	namespace.Set("as_ast_Label", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := as_ast_Label(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("defaultast_LetClause", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_LetClause()
		return r0
	}))
	namespace.Set("defaultast_ListComprehension", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_ListComprehension()
		return r0
	}))
	namespace.Set("defaultast_ListLit", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_ListLit()
		return r0
	}))
	namespace.Set("as_ast_Node", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := as_ast_Node(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("defaultast_Package", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_Package()
		return r0
	}))
	namespace.Set("defaultast_ParenExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_ParenExpr()
		return r0
	}))
	namespace.Set("defaultast_SelectorExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_SelectorExpr()
		return r0
	}))
	namespace.Set("defaultast_SliceExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_SliceExpr()
		return r0
	}))
	namespace.Set("as_ast_Spec", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := as_ast_Spec(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("defaultast_StructLit", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_StructLit()
		return r0
	}))
	namespace.Set("defaultast_TemplateLabel", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_TemplateLabel()
		return r0
	}))
	namespace.Set("defaultast_UnaryExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultast_UnaryExpr()
		return r0
	}))
	namespace.Set("as_astutil_Cursor", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := as_astutil_Cursor(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("defaultastutil_ImportInfo", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultastutil_ImportInfo()
		return r0
	}))
	namespace.Set("defaultbuild_Context", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultbuild_Context()
		return r0
	}))
	namespace.Set("defaultbuild_Encoding", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultbuild_Encoding()
		return r0
	}))
	namespace.Set("defaultbuild_File", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultbuild_File()
		return r0
	}))
	namespace.Set("defaultbuild_Form", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultbuild_Form()
		return r0
	}))
	namespace.Set("defaultbuild_Instance", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultbuild_Instance()
		return r0
	}))
	namespace.Set("defaultbuild_Interpretation", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultbuild_Interpretation()
		return r0
	}))
	namespace.Set("defaultparser_DeprecationError", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := defaultparser_DeprecationError()
		return r0
	}))
	namespace.Set("f_cue_0_All", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_All()
		return r0
	}))
	namespace.Set("f_cue_0_AppendFloat", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 4 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_AppendFloat(uint32(args[0].Int()), uint32(args[1].Int()), byte(args[2].Int()), int(args[3].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_AppendInt", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_AppendInt(uint32(args[0].Int()), uint32(args[1].Int()), int(args[2].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Attribute", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Attribute(uint32(args[0].Int()), string(args[1].String()))
		return r0
	}))
	namespace.Set("f_cue_0_Attributes", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Attributes(bool(args[0].Truthy()))
		return r0
	}))
	namespace.Set("f_cue_0_Bool", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Bool(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Build", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Build(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_1_Build", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_1_Build(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_2_Build", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_2_Build(uint32(args[0].Int()), uint32(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Bytes", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Bytes(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_CanString", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_CanString(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Compile", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Compile(uint32(args[0].Int()), string(args[1].String()), uint32(args[2].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_CompileExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_CompileExpr(uint32(args[0].Int()), uint32(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_CompileFile", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_CompileFile(uint32(args[0].Int()), uint32(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Concrete", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Concrete(bool(args[0].Truthy()))
		return r0
	}))
	namespace.Set("f_cue_0_Decimal", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Decimal(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Decode", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Decode(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Def", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Def(string(args[0].String()))
		return r0
	}))
	namespace.Set("f_cue_0_Default", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Default(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Definitions", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Definitions(bool(args[0].Truthy()))
		return r0
	}))
	namespace.Set("f_cue_0_Dereference", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Dereference(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_DisallowCycles", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_DisallowCycles(bool(args[0].Truthy()))
		return r0
	}))
	namespace.Set("f_cue_0_Doc", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Doc(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_1_Doc", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_1_Doc(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Docs", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Docs(bool(args[0].Truthy()))
		return r0
	}))
	namespace.Set("f_cue_0_Elem", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Elem(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Equals", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Equals(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Err", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Err(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_1_Err", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_1_Err(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_2_Err", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_2_Err(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Eval", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Eval(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_1_Eval", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_1_Eval(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Exists", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Exists(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Expr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Expr(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Field", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Field(uint32(args[0].Int()), int(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_FieldByName", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_FieldByName(uint32(args[0].Int()), string(args[1].String()), bool(args[2].Truthy()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_1_FieldByName", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_1_FieldByName(uint32(args[0].Int()), string(args[1].String()), bool(args[2].Truthy()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Fields", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Fields(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_1_Fields", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_1_Fields(uint32(args[0].Int()), uint32(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Fill", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Fill(uint32(args[0].Int()), uint32(args[1].Int()), uint32(args[2].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_1_Fill", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_1_Fill(uint32(args[0].Int()), uint32(args[1].Int()), uint32(args[2].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Final", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Final()
		return r0
	}))
	namespace.Set("f_cue_0_Flag", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Flag(uint32(args[0].Int()), int(args[1].Int()), string(args[2].String()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Float64", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Float64(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Format", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_cue_0_Format(uint32(args[0].Int()), uint32(args[1].Int()), rune(args[2].Int()))
		return nil
	}))
	namespace.Set("f_cue_0_FromExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_FromExpr(uint32(args[0].Int()), uint32(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Hidden", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Hidden(bool(args[0].Truthy()))
		return r0
	}))
	namespace.Set("f_cue_0_ID", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_ID(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_IncompleteKind", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_IncompleteKind(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Index", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Index(int(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Int", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Int(uint32(args[0].Int()), int(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_1_Int", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_1_Int(uint32(args[0].Int()), uint32(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Int64", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Int64(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_IsAnyOf", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_IsAnyOf(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_IsClosed", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_IsClosed(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_IsConcrete", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_IsConcrete(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_IsDefinition", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_IsDefinition(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_IsHidden", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_IsHidden(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_IsOptional", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_IsOptional(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Kind", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Kind(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Label", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Label(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_1_Label", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_1_Label(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Len", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Len(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_1_Len", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_1_Len(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_List", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_List(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Lookup", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1, r2 := f_cue_0_Lookup(uint32(args[0].Int()), int(args[1].Int()), string(args[2].String()))
		return []interface {
		}{r0, r1, r2}
	}))
	namespace.Set("f_cue_1_Lookup", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_1_Lookup(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_2_Lookup", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_2_Lookup(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_LookupDef", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_LookupDef(uint32(args[0].Int()), string(args[1].String()))
		return r0
	}))
	namespace.Set("f_cue_1_LookupDef", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_1_LookupDef(uint32(args[0].Int()), string(args[1].String()))
		return r0
	}))
	namespace.Set("f_cue_0_LookupField", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_LookupField(uint32(args[0].Int()), uint32(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_1_LookupField", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_1_LookupField(uint32(args[0].Int()), string(args[1].String()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_LookupPath", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_LookupPath(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_MakePath", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_MakePath(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_MantExp", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_MantExp(uint32(args[0].Int()), uint32(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Marshal", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Marshal(uint32(args[0].Int()), uint32(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_MarshalJSON", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_MarshalJSON(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Merge", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Merge(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Next", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Next(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Null", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Null(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Optional", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Optional(bool(args[0].Truthy()))
		return r0
	}))
	namespace.Set("f_cue_0_Parse", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Parse(uint32(args[0].Int()), string(args[1].String()), uint32(args[2].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_ParsePath", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_ParsePath(string(args[0].String()))
		return r0
	}))
	namespace.Set("f_cue_0_Path", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Path(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Raw", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Raw()
		return r0
	}))
	namespace.Set("f_cue_0_Reader", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Reader(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Reference", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Reference(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_ResolveReferences", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_ResolveReferences(bool(args[0].Truthy()))
		return r0
	}))
	namespace.Set("f_cue_0_Schema", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Schema()
		return r0
	}))
	namespace.Set("f_cue_0_Selectors", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Selectors(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Source", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Source(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Split", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Split(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Str", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Str(string(args[0].String()))
		return r0
	}))
	namespace.Set("f_cue_0_String", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_String(uint32(args[0].Int()), int(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_1_String", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_1_String(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_2_String", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_2_String(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_3_String", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_3_String(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_4_String", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_4_String(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_5_String", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_5_String(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Struct", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Struct(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Subsume", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Subsume(uint32(args[0].Int()), uint32(args[1].Int()), uint32(args[2].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Subsumes", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Subsumes(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Syntax", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Syntax(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Token", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Token(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_TypeString", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_TypeString(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Uint64", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Uint64(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Unify", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Unify(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_UnifyAccept", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_UnifyAccept(uint32(args[0].Int()), uint32(args[1].Int()), uint32(args[2].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Unmarshal", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_cue_0_Unmarshal(uint32(args[0].Int()), uint32(args[1].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_cue_0_Validate", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Validate(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_cue_0_Value", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_0_Value(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_cue_1_Value", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_cue_1_Value(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_AddComment", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_ast_0_AddComment(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_ast_1_AddComment", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_ast_1_AddComment(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_ast_2_AddComment", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_ast_2_AddComment(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_ast_0_Comments", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_Comments(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_1_Comments", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_1_Comments(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_2_Comments", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_2_Comments(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_Embed", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_Embed(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_1_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_1_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_2_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_2_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_3_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_3_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_4_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_4_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_5_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_5_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_6_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_6_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_7_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_7_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_8_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_8_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_9_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_9_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_10_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_10_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_11_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_11_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_12_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_12_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_13_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_13_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_14_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_14_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_15_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_15_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_16_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_16_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_17_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_17_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_18_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_18_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_19_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_19_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_20_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_20_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_21_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_21_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_22_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_22_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_23_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_23_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_24_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_24_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_25_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_25_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_26_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_26_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_27_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_27_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_28_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_28_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_29_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_29_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_30_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_30_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_31_End", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_31_End(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_IsValidIdent", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_IsValidIdent(string(args[0].String()))
		return r0
	}))
	namespace.Set("f_ast_0_LabelName", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1, r2 := f_ast_0_LabelName(uint32(args[0].Int()))
		return []interface {
		}{r0, r1, r2}
	}))
	namespace.Set("f_ast_0_Name", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_Name(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_NewBinExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewBinExpr(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_NewBool", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewBool(bool(args[0].Truthy()))
		return r0
	}))
	namespace.Set("f_ast_0_NewCall", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewCall(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_NewIdent", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewIdent(string(args[0].String()))
		return r0
	}))
	namespace.Set("f_ast_0_NewImport", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewImport(uint32(args[0].Int()), string(args[1].String()))
		return r0
	}))
	namespace.Set("f_ast_0_NewList", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewList(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_NewLit", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewLit(uint32(args[0].Int()), string(args[1].String()))
		return r0
	}))
	namespace.Set("f_ast_0_NewNull", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 0 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewNull()
		return r0
	}))
	namespace.Set("f_ast_0_NewSel", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewSel(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_NewString", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewString(string(args[0].String()))
		return r0
	}))
	namespace.Set("f_ast_0_NewStruct", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_NewStruct(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_PackageName", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_PackageName(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_ParseIdent", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_ast_0_ParseIdent(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_ast_0_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_1_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_1_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_2_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_2_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_3_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_3_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_4_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_4_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_5_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_5_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_6_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_6_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_7_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_7_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_8_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_8_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_9_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_9_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_10_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_10_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_11_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_11_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_12_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_12_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_13_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_13_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_14_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_14_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_15_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_15_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_16_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_16_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_17_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_17_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_18_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_18_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_19_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_19_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_20_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_20_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_21_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_21_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_22_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_22_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_23_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_23_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_24_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_24_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_25_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_25_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_26_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_26_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_27_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_27_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_28_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_28_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_29_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_29_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_30_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_30_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_31_Pos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_31_Pos(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_Preamble", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_Preamble(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_QuoteIdent", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_ast_0_QuoteIdent(string(args[0].String()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_ast_0_SetComments", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_ast_0_SetComments(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_ast_0_SetPos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_ast_0_SetPos(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_ast_0_SetRelPos", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_ast_0_SetRelPos(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_ast_0_Split", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_ast_0_Split(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_ast_0_String", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_String(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_ast_0_Text", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_ast_0_Text(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_astutil_0_ApplyRecursively", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_astutil_0_ApplyRecursively(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_astutil_0_CopyComments", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_astutil_0_CopyComments(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_astutil_0_CopyMeta", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_astutil_0_CopyMeta(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_astutil_0_CopyPosition", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_astutil_0_CopyPosition(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_astutil_0_ParseImportSpec", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_astutil_0_ParseImportSpec(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_astutil_0_Resolve", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_astutil_0_Resolve(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_astutil_0_ResolveExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_astutil_0_ResolveExpr(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_astutil_0_Sanitize", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_astutil_0_Sanitize(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_astutil_0_ToFile", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_astutil_0_ToFile(uint32(args[0].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_build_0_Abs", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_Abs(uint32(args[0].Int()), string(args[1].String()))
		return r0
	}))
	namespace.Set("f_build_0_AddFile", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_AddFile(uint32(args[0].Int()), string(args[1].String()), uint32(args[2].Int()))
		return r0
	}))
	namespace.Set("f_build_0_AddSyntax", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_AddSyntax(uint32(args[0].Int()), uint32(args[1].Int()))
		return r0
	}))
	namespace.Set("f_build_0_Complete", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_Complete(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_build_0_Context", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_Context(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_build_0_Dependencies", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_Dependencies(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_build_0_ID", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_ID(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_build_0_IsLocalImport", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_IsLocalImport(string(args[0].String()))
		return r0
	}))
	namespace.Set("f_build_0_Loader", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_Loader(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_build_0_LookupImport", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_LookupImport(uint32(args[0].Int()), string(args[1].String()))
		return r0
	}))
	namespace.Set("f_build_0_NewContext", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_NewContext(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_build_0_NewInstance", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_build_0_NewInstance(uint32(args[0].Int()), string(args[1].String()), uint32(args[2].Int()))
		return r0
	}))
	namespace.Set("f_build_0_ReportError", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		f_build_0_ReportError(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("f_parser_0_Error", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_parser_0_Error(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("f_parser_0_FileOffset", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_parser_0_FileOffset(int(args[0].Int()))
		return r0
	}))
	namespace.Set("f_parser_0_FromVersion", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := f_parser_0_FromVersion(int(args[0].Int()))
		return r0
	}))
	namespace.Set("f_parser_0_ParseExpr", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_parser_0_ParseExpr(string(args[0].String()), uint32(args[1].Int()), uint32(args[2].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("f_parser_0_ParseFile", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) < 3 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0, r1 := f_parser_0_ParseFile(string(args[0].String()), uint32(args[1].Int()), uint32(args[2].Int()))
		return []interface {
		}{r0, r1}
	}))
	namespace.Set("makeSlice", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := makeSlice(uint32(args[0].Int()), int(args[1].Int()))
		return r0
	}))
	namespace.Set("push", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 2 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		push(uint32(args[0].Int()), uint32(args[1].Int()))
		return nil
	}))
	namespace.Set("newPointer", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := newPointer(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("forget", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		forget(uint32(args[0].Int()))
		return nil
	}))
	namespace.Set("proxyBool", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyBool(bool(args[0].Truthy()))
		return r0
	}))
	namespace.Set("proxyFloat32", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyFloat32(float32(args[0].Float()))
		return r0
	}))
	namespace.Set("proxyFloat64", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyFloat64(float64(args[0].Float()))
		return r0
	}))
	namespace.Set("proxyInt16", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyInt16(int16(args[0].Int()))
		return r0
	}))
	namespace.Set("proxyInt32", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyInt32(int32(args[0].Int()))
		return r0
	}))
	namespace.Set("proxyInt64", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyInt64(int64(args[0].Int()))
		return r0
	}))
	namespace.Set("proxyInt8", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyInt8(int8(args[0].Int()))
		return r0
	}))
	namespace.Set("proxyString", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyString(string(args[0].String()))
		return r0
	}))
	namespace.Set("proxyUint16", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyUint16(uint16(args[0].Int()))
		return r0
	}))
	namespace.Set("proxyUint32", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyUint32(uint32(args[0].Int()))
		return r0
	}))
	namespace.Set("proxyUint64", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyUint64(uint64(args[0].Int()))
		return r0
	}))
	namespace.Set("proxyUint8", js.FuncOf(func(this js.Value, args []js.Value) interface {
	} {
		if len(args) != 1 {
			return js.Error{js.ValueOf("Wrong number of arguments")}
		}
		r0 := proxyUint8(uint8(args[0].Int()))
		return r0
	}))
	js.Global().Get("Go").Set("cuelang-sys", namespace)
	select {}
}
