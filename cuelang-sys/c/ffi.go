package main

import _ "cuelang.org/go/pkg"

import "C"
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
)

//export defaultcue_Attribute
func defaultcue_Attribute() uint32 {
	var tmp cue.Attribute
	return register(&tmp)
}

//export defaultcue_FieldInfo
func defaultcue_FieldInfo() uint32 {
	var tmp cue.FieldInfo
	return register(&tmp)
}

//export get_cue_FieldInfo_IsDefinition
func get_cue_FieldInfo_IsDefinition(object uint32) bool {
	IsDefinition := (*get(object).(*cue.FieldInfo)).IsDefinition
	return IsDefinition
}

//export set_cue_FieldInfo_IsDefinition
func set_cue_FieldInfo_IsDefinition(object uint32, IsDefinition bool) {
	(*get(object).(*cue.FieldInfo)).IsDefinition = IsDefinition
}

//export get_cue_FieldInfo_IsHidden
func get_cue_FieldInfo_IsHidden(object uint32) bool {
	IsHidden := (*get(object).(*cue.FieldInfo)).IsHidden
	return IsHidden
}

//export set_cue_FieldInfo_IsHidden
func set_cue_FieldInfo_IsHidden(object uint32, IsHidden bool) {
	(*get(object).(*cue.FieldInfo)).IsHidden = IsHidden
}

//export get_cue_FieldInfo_IsOptional
func get_cue_FieldInfo_IsOptional(object uint32) bool {
	IsOptional := (*get(object).(*cue.FieldInfo)).IsOptional
	return IsOptional
}

//export set_cue_FieldInfo_IsOptional
func set_cue_FieldInfo_IsOptional(object uint32, IsOptional bool) {
	(*get(object).(*cue.FieldInfo)).IsOptional = IsOptional
}

//export get_cue_FieldInfo_Name
func get_cue_FieldInfo_Name(object uint32) *C.char {
	Name := (*get(object).(*cue.FieldInfo)).Name
	return C.CString(Name)
}

//export set_cue_FieldInfo_Name
func set_cue_FieldInfo_Name(object uint32, Name *C.char) {
	(*get(object).(*cue.FieldInfo)).Name = C.GoString(Name)
}

//export get_cue_FieldInfo_Pos
func get_cue_FieldInfo_Pos(object uint32) int {
	Pos := (*get(object).(*cue.FieldInfo)).Pos
	return Pos
}

//export set_cue_FieldInfo_Pos
func set_cue_FieldInfo_Pos(object uint32, Pos int) {
	(*get(object).(*cue.FieldInfo)).Pos = Pos
}

//export get_cue_FieldInfo_Selector
func get_cue_FieldInfo_Selector(object uint32) *C.char {
	Selector := (*get(object).(*cue.FieldInfo)).Selector
	return C.CString(Selector)
}

//export set_cue_FieldInfo_Selector
func set_cue_FieldInfo_Selector(object uint32, Selector *C.char) {
	(*get(object).(*cue.FieldInfo)).Selector = C.GoString(Selector)
}

//export get_cue_FieldInfo_Value
func get_cue_FieldInfo_Value(object uint32) uint32 {
	Value := (*get(object).(*cue.FieldInfo)).Value
	return register(&Value)
}

//export set_cue_FieldInfo_Value
func set_cue_FieldInfo_Value(object uint32, Value uint32) {
	(*get(object).(*cue.FieldInfo)).Value = *get(Value).(*cue.Value)
}

//export defaultcue_Instance
func defaultcue_Instance() uint32 {
	var tmp cue.Instance
	return register(&tmp)
}

//export get_cue_Instance_Dir
func get_cue_Instance_Dir(object uint32) *C.char {
	Dir := (*get(object).(*cue.Instance)).Dir
	return C.CString(Dir)
}

//export set_cue_Instance_Dir
func set_cue_Instance_Dir(object uint32, Dir *C.char) {
	(*get(object).(*cue.Instance)).Dir = C.GoString(Dir)
}

//export get_cue_Instance_DisplayName
func get_cue_Instance_DisplayName(object uint32) *C.char {
	DisplayName := (*get(object).(*cue.Instance)).DisplayName
	return C.CString(DisplayName)
}

//export set_cue_Instance_DisplayName
func set_cue_Instance_DisplayName(object uint32, DisplayName *C.char) {
	(*get(object).(*cue.Instance)).DisplayName = C.GoString(DisplayName)
}

//export get_cue_Instance_Err
func get_cue_Instance_Err(object uint32) uint32 {
	Err := (*get(object).(*cue.Instance)).Err
	return register(&Err)
}

//export set_cue_Instance_Err
func set_cue_Instance_Err(object uint32, Err uint32) {
	(*get(object).(*cue.Instance)).Err = *get(Err).(*errors.Error)
}

//export get_cue_Instance_ImportPath
func get_cue_Instance_ImportPath(object uint32) *C.char {
	ImportPath := (*get(object).(*cue.Instance)).ImportPath
	return C.CString(ImportPath)
}

//export set_cue_Instance_ImportPath
func set_cue_Instance_ImportPath(object uint32, ImportPath *C.char) {
	(*get(object).(*cue.Instance)).ImportPath = C.GoString(ImportPath)
}

//export get_cue_Instance_Incomplete
func get_cue_Instance_Incomplete(object uint32) bool {
	Incomplete := (*get(object).(*cue.Instance)).Incomplete
	return Incomplete
}

//export set_cue_Instance_Incomplete
func set_cue_Instance_Incomplete(object uint32, Incomplete bool) {
	(*get(object).(*cue.Instance)).Incomplete = Incomplete
}

//export get_cue_Instance_PkgName
func get_cue_Instance_PkgName(object uint32) *C.char {
	PkgName := (*get(object).(*cue.Instance)).PkgName
	return C.CString(PkgName)
}

//export set_cue_Instance_PkgName
func set_cue_Instance_PkgName(object uint32, PkgName *C.char) {
	(*get(object).(*cue.Instance)).PkgName = C.GoString(PkgName)
}

//export defaultcue_Iterator
func defaultcue_Iterator() uint32 {
	var tmp cue.Iterator
	return register(&tmp)
}

//export defaultcue_Kind
func defaultcue_Kind() uint32 {
	var tmp cue.Kind
	return register(&tmp)
}

//export defaultcue_Op
func defaultcue_Op() uint32 {
	var tmp cue.Op
	return register(&tmp)
}

//export defaultcue_Path
func defaultcue_Path() uint32 {
	var tmp cue.Path
	return register(&tmp)
}

//export defaultcue_Runtime
func defaultcue_Runtime() uint32 {
	var tmp cue.Runtime
	return register(&tmp)
}

//export defaultcue_Selector
func defaultcue_Selector() uint32 {
	var tmp cue.Selector
	return register(&tmp)
}

//export defaultcue_Struct
func defaultcue_Struct() uint32 {
	var tmp cue.Struct
	return register(&tmp)
}

//export defaultcue_Value
func defaultcue_Value() uint32 {
	var tmp cue.Value
	return register(&tmp)
}

//export defaultast_Alias
func defaultast_Alias() uint32 {
	var tmp ast.Alias
	return register(&tmp)
}

//export get_ast_Alias_Equal
func get_ast_Alias_Equal(object uint32) uint32 {
	Equal := (*get(object).(*ast.Alias)).Equal
	return register(&Equal)
}

//export set_ast_Alias_Equal
func set_ast_Alias_Equal(object uint32, Equal uint32) {
	(*get(object).(*ast.Alias)).Equal = *get(Equal).(*token.Pos)
}

//export get_ast_Alias_Expr
func get_ast_Alias_Expr(object uint32) uint32 {
	Expr := (*get(object).(*ast.Alias)).Expr
	return register(&Expr)
}

//export set_ast_Alias_Expr
func set_ast_Alias_Expr(object uint32, Expr uint32) {
	(*get(object).(*ast.Alias)).Expr = *get(Expr).(*ast.Expr)
}

//export get_ast_Alias_Ident
func get_ast_Alias_Ident(object uint32) uint32 {
	Ident := (*get(object).(*ast.Alias)).Ident
	return register(&Ident)
}

//export set_ast_Alias_Ident
func set_ast_Alias_Ident(object uint32, Ident uint32) {
	(*get(object).(*ast.Alias)).Ident = *get(Ident).(**ast.Ident)
}

//export defaultast_Attribute
func defaultast_Attribute() uint32 {
	var tmp ast.Attribute
	return register(&tmp)
}

//export get_ast_Attribute_At
func get_ast_Attribute_At(object uint32) uint32 {
	At := (*get(object).(*ast.Attribute)).At
	return register(&At)
}

//export set_ast_Attribute_At
func set_ast_Attribute_At(object uint32, At uint32) {
	(*get(object).(*ast.Attribute)).At = *get(At).(*token.Pos)
}

//export get_ast_Attribute_Text
func get_ast_Attribute_Text(object uint32) *C.char {
	Text := (*get(object).(*ast.Attribute)).Text
	return C.CString(Text)
}

//export set_ast_Attribute_Text
func set_ast_Attribute_Text(object uint32, Text *C.char) {
	(*get(object).(*ast.Attribute)).Text = C.GoString(Text)
}

//export defaultast_BadDecl
func defaultast_BadDecl() uint32 {
	var tmp ast.BadDecl
	return register(&tmp)
}

//export get_ast_BadDecl_From
func get_ast_BadDecl_From(object uint32) uint32 {
	From := (*get(object).(*ast.BadDecl)).From
	return register(&From)
}

//export set_ast_BadDecl_From
func set_ast_BadDecl_From(object uint32, From uint32) {
	(*get(object).(*ast.BadDecl)).From = *get(From).(*token.Pos)
}

//export get_ast_BadDecl_To
func get_ast_BadDecl_To(object uint32) uint32 {
	To := (*get(object).(*ast.BadDecl)).To
	return register(&To)
}

//export set_ast_BadDecl_To
func set_ast_BadDecl_To(object uint32, To uint32) {
	(*get(object).(*ast.BadDecl)).To = *get(To).(*token.Pos)
}

//export defaultast_BadExpr
func defaultast_BadExpr() uint32 {
	var tmp ast.BadExpr
	return register(&tmp)
}

//export get_ast_BadExpr_From
func get_ast_BadExpr_From(object uint32) uint32 {
	From := (*get(object).(*ast.BadExpr)).From
	return register(&From)
}

//export set_ast_BadExpr_From
func set_ast_BadExpr_From(object uint32, From uint32) {
	(*get(object).(*ast.BadExpr)).From = *get(From).(*token.Pos)
}

//export get_ast_BadExpr_To
func get_ast_BadExpr_To(object uint32) uint32 {
	To := (*get(object).(*ast.BadExpr)).To
	return register(&To)
}

//export set_ast_BadExpr_To
func set_ast_BadExpr_To(object uint32, To uint32) {
	(*get(object).(*ast.BadExpr)).To = *get(To).(*token.Pos)
}

//export defaultast_BasicLit
func defaultast_BasicLit() uint32 {
	var tmp ast.BasicLit
	return register(&tmp)
}

//export get_ast_BasicLit_Kind
func get_ast_BasicLit_Kind(object uint32) uint32 {
	Kind := (*get(object).(*ast.BasicLit)).Kind
	return register(&Kind)
}

//export set_ast_BasicLit_Kind
func set_ast_BasicLit_Kind(object uint32, Kind uint32) {
	(*get(object).(*ast.BasicLit)).Kind = *get(Kind).(*token.Token)
}

//export get_ast_BasicLit_Value
func get_ast_BasicLit_Value(object uint32) *C.char {
	Value := (*get(object).(*ast.BasicLit)).Value
	return C.CString(Value)
}

//export set_ast_BasicLit_Value
func set_ast_BasicLit_Value(object uint32, Value *C.char) {
	(*get(object).(*ast.BasicLit)).Value = C.GoString(Value)
}

//export get_ast_BasicLit_ValuePos
func get_ast_BasicLit_ValuePos(object uint32) uint32 {
	ValuePos := (*get(object).(*ast.BasicLit)).ValuePos
	return register(&ValuePos)
}

//export set_ast_BasicLit_ValuePos
func set_ast_BasicLit_ValuePos(object uint32, ValuePos uint32) {
	(*get(object).(*ast.BasicLit)).ValuePos = *get(ValuePos).(*token.Pos)
}

//export defaultast_BinaryExpr
func defaultast_BinaryExpr() uint32 {
	var tmp ast.BinaryExpr
	return register(&tmp)
}

//export get_ast_BinaryExpr_Op
func get_ast_BinaryExpr_Op(object uint32) uint32 {
	Op := (*get(object).(*ast.BinaryExpr)).Op
	return register(&Op)
}

//export set_ast_BinaryExpr_Op
func set_ast_BinaryExpr_Op(object uint32, Op uint32) {
	(*get(object).(*ast.BinaryExpr)).Op = *get(Op).(*token.Token)
}

//export get_ast_BinaryExpr_OpPos
func get_ast_BinaryExpr_OpPos(object uint32) uint32 {
	OpPos := (*get(object).(*ast.BinaryExpr)).OpPos
	return register(&OpPos)
}

//export set_ast_BinaryExpr_OpPos
func set_ast_BinaryExpr_OpPos(object uint32, OpPos uint32) {
	(*get(object).(*ast.BinaryExpr)).OpPos = *get(OpPos).(*token.Pos)
}

//export get_ast_BinaryExpr_X
func get_ast_BinaryExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.BinaryExpr)).X
	return register(&X)
}

//export set_ast_BinaryExpr_X
func set_ast_BinaryExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.BinaryExpr)).X = *get(X).(*ast.Expr)
}

//export get_ast_BinaryExpr_Y
func get_ast_BinaryExpr_Y(object uint32) uint32 {
	Y := (*get(object).(*ast.BinaryExpr)).Y
	return register(&Y)
}

//export set_ast_BinaryExpr_Y
func set_ast_BinaryExpr_Y(object uint32, Y uint32) {
	(*get(object).(*ast.BinaryExpr)).Y = *get(Y).(*ast.Expr)
}

//export defaultast_BottomLit
func defaultast_BottomLit() uint32 {
	var tmp ast.BottomLit
	return register(&tmp)
}

//export get_ast_BottomLit_Bottom
func get_ast_BottomLit_Bottom(object uint32) uint32 {
	Bottom := (*get(object).(*ast.BottomLit)).Bottom
	return register(&Bottom)
}

//export set_ast_BottomLit_Bottom
func set_ast_BottomLit_Bottom(object uint32, Bottom uint32) {
	(*get(object).(*ast.BottomLit)).Bottom = *get(Bottom).(*token.Pos)
}

//export defaultast_CallExpr
func defaultast_CallExpr() uint32 {
	var tmp ast.CallExpr
	return register(&tmp)
}

//export get_ast_CallExpr_Args
func get_ast_CallExpr_Args(object uint32) uint32 {
	Args := (*get(object).(*ast.CallExpr)).Args
	return register(&Args)
}

//export set_ast_CallExpr_Args
func set_ast_CallExpr_Args(object uint32, Args uint32) {
	(*get(object).(*ast.CallExpr)).Args = *get(Args).(*[]ast.Expr)
}

//export get_ast_CallExpr_Fun
func get_ast_CallExpr_Fun(object uint32) uint32 {
	Fun := (*get(object).(*ast.CallExpr)).Fun
	return register(&Fun)
}

//export set_ast_CallExpr_Fun
func set_ast_CallExpr_Fun(object uint32, Fun uint32) {
	(*get(object).(*ast.CallExpr)).Fun = *get(Fun).(*ast.Expr)
}

//export get_ast_CallExpr_Lparen
func get_ast_CallExpr_Lparen(object uint32) uint32 {
	Lparen := (*get(object).(*ast.CallExpr)).Lparen
	return register(&Lparen)
}

//export set_ast_CallExpr_Lparen
func set_ast_CallExpr_Lparen(object uint32, Lparen uint32) {
	(*get(object).(*ast.CallExpr)).Lparen = *get(Lparen).(*token.Pos)
}

//export get_ast_CallExpr_Rparen
func get_ast_CallExpr_Rparen(object uint32) uint32 {
	Rparen := (*get(object).(*ast.CallExpr)).Rparen
	return register(&Rparen)
}

//export set_ast_CallExpr_Rparen
func set_ast_CallExpr_Rparen(object uint32, Rparen uint32) {
	(*get(object).(*ast.CallExpr)).Rparen = *get(Rparen).(*token.Pos)
}

//export as_ast_Clause
func as_ast_Clause(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Clause)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}

//export defaultast_Comment
func defaultast_Comment() uint32 {
	var tmp ast.Comment
	return register(&tmp)
}

//export get_ast_Comment_Slash
func get_ast_Comment_Slash(object uint32) uint32 {
	Slash := (*get(object).(*ast.Comment)).Slash
	return register(&Slash)
}

//export set_ast_Comment_Slash
func set_ast_Comment_Slash(object uint32, Slash uint32) {
	(*get(object).(*ast.Comment)).Slash = *get(Slash).(*token.Pos)
}

//export get_ast_Comment_Text
func get_ast_Comment_Text(object uint32) *C.char {
	Text := (*get(object).(*ast.Comment)).Text
	return C.CString(Text)
}

//export set_ast_Comment_Text
func set_ast_Comment_Text(object uint32, Text *C.char) {
	(*get(object).(*ast.Comment)).Text = C.GoString(Text)
}

//export defaultast_CommentGroup
func defaultast_CommentGroup() uint32 {
	var tmp ast.CommentGroup
	return register(&tmp)
}

//export get_ast_CommentGroup_Doc
func get_ast_CommentGroup_Doc(object uint32) bool {
	Doc := (*get(object).(*ast.CommentGroup)).Doc
	return Doc
}

//export set_ast_CommentGroup_Doc
func set_ast_CommentGroup_Doc(object uint32, Doc bool) {
	(*get(object).(*ast.CommentGroup)).Doc = Doc
}

//export get_ast_CommentGroup_Line
func get_ast_CommentGroup_Line(object uint32) bool {
	Line := (*get(object).(*ast.CommentGroup)).Line
	return Line
}

//export set_ast_CommentGroup_Line
func set_ast_CommentGroup_Line(object uint32, Line bool) {
	(*get(object).(*ast.CommentGroup)).Line = Line
}

//export get_ast_CommentGroup_List
func get_ast_CommentGroup_List(object uint32) uint32 {
	List := (*get(object).(*ast.CommentGroup)).List
	return register(&List)
}

//export set_ast_CommentGroup_List
func set_ast_CommentGroup_List(object uint32, List uint32) {
	(*get(object).(*ast.CommentGroup)).List = *get(List).(*[]*ast.Comment)
}

//export get_ast_CommentGroup_Position
func get_ast_CommentGroup_Position(object uint32) int8 {
	Position := (*get(object).(*ast.CommentGroup)).Position
	return Position
}

//export set_ast_CommentGroup_Position
func set_ast_CommentGroup_Position(object uint32, Position int8) {
	(*get(object).(*ast.CommentGroup)).Position = Position
}

//export defaultast_Comprehension
func defaultast_Comprehension() uint32 {
	var tmp ast.Comprehension
	return register(&tmp)
}

//export get_ast_Comprehension_Clauses
func get_ast_Comprehension_Clauses(object uint32) uint32 {
	Clauses := (*get(object).(*ast.Comprehension)).Clauses
	return register(&Clauses)
}

//export set_ast_Comprehension_Clauses
func set_ast_Comprehension_Clauses(object uint32, Clauses uint32) {
	(*get(object).(*ast.Comprehension)).Clauses = *get(Clauses).(*[]ast.Clause)
}

//export get_ast_Comprehension_Value
func get_ast_Comprehension_Value(object uint32) uint32 {
	Value := (*get(object).(*ast.Comprehension)).Value
	return register(&Value)
}

//export set_ast_Comprehension_Value
func set_ast_Comprehension_Value(object uint32, Value uint32) {
	(*get(object).(*ast.Comprehension)).Value = *get(Value).(*ast.Expr)
}

//export as_ast_Decl
func as_ast_Decl(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Decl)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}

//export defaultast_Ellipsis
func defaultast_Ellipsis() uint32 {
	var tmp ast.Ellipsis
	return register(&tmp)
}

//export get_ast_Ellipsis_Ellipsis
func get_ast_Ellipsis_Ellipsis(object uint32) uint32 {
	Ellipsis := (*get(object).(*ast.Ellipsis)).Ellipsis
	return register(&Ellipsis)
}

//export set_ast_Ellipsis_Ellipsis
func set_ast_Ellipsis_Ellipsis(object uint32, Ellipsis uint32) {
	(*get(object).(*ast.Ellipsis)).Ellipsis = *get(Ellipsis).(*token.Pos)
}

//export get_ast_Ellipsis_Type
func get_ast_Ellipsis_Type(object uint32) uint32 {
	Type := (*get(object).(*ast.Ellipsis)).Type
	return register(&Type)
}

//export set_ast_Ellipsis_Type
func set_ast_Ellipsis_Type(object uint32, Type uint32) {
	(*get(object).(*ast.Ellipsis)).Type = *get(Type).(*ast.Expr)
}

//export defaultast_EmbedDecl
func defaultast_EmbedDecl() uint32 {
	var tmp ast.EmbedDecl
	return register(&tmp)
}

//export get_ast_EmbedDecl_Expr
func get_ast_EmbedDecl_Expr(object uint32) uint32 {
	Expr := (*get(object).(*ast.EmbedDecl)).Expr
	return register(&Expr)
}

//export set_ast_EmbedDecl_Expr
func set_ast_EmbedDecl_Expr(object uint32, Expr uint32) {
	(*get(object).(*ast.EmbedDecl)).Expr = *get(Expr).(*ast.Expr)
}

//export as_ast_Expr
func as_ast_Expr(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Expr)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}

//export defaultast_Field
func defaultast_Field() uint32 {
	var tmp ast.Field
	return register(&tmp)
}

//export get_ast_Field_Attrs
func get_ast_Field_Attrs(object uint32) uint32 {
	Attrs := (*get(object).(*ast.Field)).Attrs
	return register(&Attrs)
}

//export set_ast_Field_Attrs
func set_ast_Field_Attrs(object uint32, Attrs uint32) {
	(*get(object).(*ast.Field)).Attrs = *get(Attrs).(*[]*ast.Attribute)
}

//export get_ast_Field_Label
func get_ast_Field_Label(object uint32) uint32 {
	Label := (*get(object).(*ast.Field)).Label
	return register(&Label)
}

//export set_ast_Field_Label
func set_ast_Field_Label(object uint32, Label uint32) {
	(*get(object).(*ast.Field)).Label = *get(Label).(*ast.Label)
}

//export get_ast_Field_Optional
func get_ast_Field_Optional(object uint32) uint32 {
	Optional := (*get(object).(*ast.Field)).Optional
	return register(&Optional)
}

//export set_ast_Field_Optional
func set_ast_Field_Optional(object uint32, Optional uint32) {
	(*get(object).(*ast.Field)).Optional = *get(Optional).(*token.Pos)
}

//export get_ast_Field_Token
func get_ast_Field_Token(object uint32) uint32 {
	Token := (*get(object).(*ast.Field)).Token
	return register(&Token)
}

//export set_ast_Field_Token
func set_ast_Field_Token(object uint32, Token uint32) {
	(*get(object).(*ast.Field)).Token = *get(Token).(*token.Token)
}

//export get_ast_Field_TokenPos
func get_ast_Field_TokenPos(object uint32) uint32 {
	TokenPos := (*get(object).(*ast.Field)).TokenPos
	return register(&TokenPos)
}

//export set_ast_Field_TokenPos
func set_ast_Field_TokenPos(object uint32, TokenPos uint32) {
	(*get(object).(*ast.Field)).TokenPos = *get(TokenPos).(*token.Pos)
}

//export get_ast_Field_Value
func get_ast_Field_Value(object uint32) uint32 {
	Value := (*get(object).(*ast.Field)).Value
	return register(&Value)
}

//export set_ast_Field_Value
func set_ast_Field_Value(object uint32, Value uint32) {
	(*get(object).(*ast.Field)).Value = *get(Value).(*ast.Expr)
}

//export defaultast_File
func defaultast_File() uint32 {
	var tmp ast.File
	return register(&tmp)
}

//export get_ast_File_Decls
func get_ast_File_Decls(object uint32) uint32 {
	Decls := (*get(object).(*ast.File)).Decls
	return register(&Decls)
}

//export set_ast_File_Decls
func set_ast_File_Decls(object uint32, Decls uint32) {
	(*get(object).(*ast.File)).Decls = *get(Decls).(*[]ast.Decl)
}

//export get_ast_File_Filename
func get_ast_File_Filename(object uint32) *C.char {
	Filename := (*get(object).(*ast.File)).Filename
	return C.CString(Filename)
}

//export set_ast_File_Filename
func set_ast_File_Filename(object uint32, Filename *C.char) {
	(*get(object).(*ast.File)).Filename = C.GoString(Filename)
}

//export get_ast_File_Imports
func get_ast_File_Imports(object uint32) uint32 {
	Imports := (*get(object).(*ast.File)).Imports
	return register(&Imports)
}

//export set_ast_File_Imports
func set_ast_File_Imports(object uint32, Imports uint32) {
	(*get(object).(*ast.File)).Imports = *get(Imports).(*[]*ast.ImportSpec)
}

//export get_ast_File_Unresolved
func get_ast_File_Unresolved(object uint32) uint32 {
	Unresolved := (*get(object).(*ast.File)).Unresolved
	return register(&Unresolved)
}

//export set_ast_File_Unresolved
func set_ast_File_Unresolved(object uint32, Unresolved uint32) {
	(*get(object).(*ast.File)).Unresolved = *get(Unresolved).(*[]*ast.Ident)
}

//export defaultast_ForClause
func defaultast_ForClause() uint32 {
	var tmp ast.ForClause
	return register(&tmp)
}

//export get_ast_ForClause_Colon
func get_ast_ForClause_Colon(object uint32) uint32 {
	Colon := (*get(object).(*ast.ForClause)).Colon
	return register(&Colon)
}

//export set_ast_ForClause_Colon
func set_ast_ForClause_Colon(object uint32, Colon uint32) {
	(*get(object).(*ast.ForClause)).Colon = *get(Colon).(*token.Pos)
}

//export get_ast_ForClause_For
func get_ast_ForClause_For(object uint32) uint32 {
	For := (*get(object).(*ast.ForClause)).For
	return register(&For)
}

//export set_ast_ForClause_For
func set_ast_ForClause_For(object uint32, For uint32) {
	(*get(object).(*ast.ForClause)).For = *get(For).(*token.Pos)
}

//export get_ast_ForClause_In
func get_ast_ForClause_In(object uint32) uint32 {
	In := (*get(object).(*ast.ForClause)).In
	return register(&In)
}

//export set_ast_ForClause_In
func set_ast_ForClause_In(object uint32, In uint32) {
	(*get(object).(*ast.ForClause)).In = *get(In).(*token.Pos)
}

//export get_ast_ForClause_Key
func get_ast_ForClause_Key(object uint32) uint32 {
	Key := (*get(object).(*ast.ForClause)).Key
	return register(&Key)
}

//export set_ast_ForClause_Key
func set_ast_ForClause_Key(object uint32, Key uint32) {
	(*get(object).(*ast.ForClause)).Key = *get(Key).(**ast.Ident)
}

//export get_ast_ForClause_Source
func get_ast_ForClause_Source(object uint32) uint32 {
	Source := (*get(object).(*ast.ForClause)).Source
	return register(&Source)
}

//export set_ast_ForClause_Source
func set_ast_ForClause_Source(object uint32, Source uint32) {
	(*get(object).(*ast.ForClause)).Source = *get(Source).(*ast.Expr)
}

//export get_ast_ForClause_Value
func get_ast_ForClause_Value(object uint32) uint32 {
	Value := (*get(object).(*ast.ForClause)).Value
	return register(&Value)
}

//export set_ast_ForClause_Value
func set_ast_ForClause_Value(object uint32, Value uint32) {
	(*get(object).(*ast.ForClause)).Value = *get(Value).(**ast.Ident)
}

//export defaultast_Ident
func defaultast_Ident() uint32 {
	var tmp ast.Ident
	return register(&tmp)
}

//export get_ast_Ident_Name
func get_ast_Ident_Name(object uint32) *C.char {
	Name := (*get(object).(*ast.Ident)).Name
	return C.CString(Name)
}

//export set_ast_Ident_Name
func set_ast_Ident_Name(object uint32, Name *C.char) {
	(*get(object).(*ast.Ident)).Name = C.GoString(Name)
}

//export get_ast_Ident_NamePos
func get_ast_Ident_NamePos(object uint32) uint32 {
	NamePos := (*get(object).(*ast.Ident)).NamePos
	return register(&NamePos)
}

//export set_ast_Ident_NamePos
func set_ast_Ident_NamePos(object uint32, NamePos uint32) {
	(*get(object).(*ast.Ident)).NamePos = *get(NamePos).(*token.Pos)
}

//export get_ast_Ident_Node
func get_ast_Ident_Node(object uint32) uint32 {
	Node := (*get(object).(*ast.Ident)).Node
	return register(&Node)
}

//export set_ast_Ident_Node
func set_ast_Ident_Node(object uint32, Node uint32) {
	(*get(object).(*ast.Ident)).Node = *get(Node).(*ast.Node)
}

//export get_ast_Ident_Scope
func get_ast_Ident_Scope(object uint32) uint32 {
	Scope := (*get(object).(*ast.Ident)).Scope
	return register(&Scope)
}

//export set_ast_Ident_Scope
func set_ast_Ident_Scope(object uint32, Scope uint32) {
	(*get(object).(*ast.Ident)).Scope = *get(Scope).(*ast.Node)
}

//export defaultast_IfClause
func defaultast_IfClause() uint32 {
	var tmp ast.IfClause
	return register(&tmp)
}

//export get_ast_IfClause_Condition
func get_ast_IfClause_Condition(object uint32) uint32 {
	Condition := (*get(object).(*ast.IfClause)).Condition
	return register(&Condition)
}

//export set_ast_IfClause_Condition
func set_ast_IfClause_Condition(object uint32, Condition uint32) {
	(*get(object).(*ast.IfClause)).Condition = *get(Condition).(*ast.Expr)
}

//export get_ast_IfClause_If
func get_ast_IfClause_If(object uint32) uint32 {
	If := (*get(object).(*ast.IfClause)).If
	return register(&If)
}

//export set_ast_IfClause_If
func set_ast_IfClause_If(object uint32, If uint32) {
	(*get(object).(*ast.IfClause)).If = *get(If).(*token.Pos)
}

//export defaultast_ImportDecl
func defaultast_ImportDecl() uint32 {
	var tmp ast.ImportDecl
	return register(&tmp)
}

//export get_ast_ImportDecl_Import
func get_ast_ImportDecl_Import(object uint32) uint32 {
	Import := (*get(object).(*ast.ImportDecl)).Import
	return register(&Import)
}

//export set_ast_ImportDecl_Import
func set_ast_ImportDecl_Import(object uint32, Import uint32) {
	(*get(object).(*ast.ImportDecl)).Import = *get(Import).(*token.Pos)
}

//export get_ast_ImportDecl_Lparen
func get_ast_ImportDecl_Lparen(object uint32) uint32 {
	Lparen := (*get(object).(*ast.ImportDecl)).Lparen
	return register(&Lparen)
}

//export set_ast_ImportDecl_Lparen
func set_ast_ImportDecl_Lparen(object uint32, Lparen uint32) {
	(*get(object).(*ast.ImportDecl)).Lparen = *get(Lparen).(*token.Pos)
}

//export get_ast_ImportDecl_Rparen
func get_ast_ImportDecl_Rparen(object uint32) uint32 {
	Rparen := (*get(object).(*ast.ImportDecl)).Rparen
	return register(&Rparen)
}

//export set_ast_ImportDecl_Rparen
func set_ast_ImportDecl_Rparen(object uint32, Rparen uint32) {
	(*get(object).(*ast.ImportDecl)).Rparen = *get(Rparen).(*token.Pos)
}

//export get_ast_ImportDecl_Specs
func get_ast_ImportDecl_Specs(object uint32) uint32 {
	Specs := (*get(object).(*ast.ImportDecl)).Specs
	return register(&Specs)
}

//export set_ast_ImportDecl_Specs
func set_ast_ImportDecl_Specs(object uint32, Specs uint32) {
	(*get(object).(*ast.ImportDecl)).Specs = *get(Specs).(*[]*ast.ImportSpec)
}

//export defaultast_ImportSpec
func defaultast_ImportSpec() uint32 {
	var tmp ast.ImportSpec
	return register(&tmp)
}

//export get_ast_ImportSpec_EndPos
func get_ast_ImportSpec_EndPos(object uint32) uint32 {
	EndPos := (*get(object).(*ast.ImportSpec)).EndPos
	return register(&EndPos)
}

//export set_ast_ImportSpec_EndPos
func set_ast_ImportSpec_EndPos(object uint32, EndPos uint32) {
	(*get(object).(*ast.ImportSpec)).EndPos = *get(EndPos).(*token.Pos)
}

//export get_ast_ImportSpec_Name
func get_ast_ImportSpec_Name(object uint32) uint32 {
	Name := (*get(object).(*ast.ImportSpec)).Name
	return register(&Name)
}

//export set_ast_ImportSpec_Name
func set_ast_ImportSpec_Name(object uint32, Name uint32) {
	(*get(object).(*ast.ImportSpec)).Name = *get(Name).(**ast.Ident)
}

//export get_ast_ImportSpec_Path
func get_ast_ImportSpec_Path(object uint32) uint32 {
	Path := (*get(object).(*ast.ImportSpec)).Path
	return register(&Path)
}

//export set_ast_ImportSpec_Path
func set_ast_ImportSpec_Path(object uint32, Path uint32) {
	(*get(object).(*ast.ImportSpec)).Path = *get(Path).(**ast.BasicLit)
}

//export defaultast_IndexExpr
func defaultast_IndexExpr() uint32 {
	var tmp ast.IndexExpr
	return register(&tmp)
}

//export get_ast_IndexExpr_Index
func get_ast_IndexExpr_Index(object uint32) uint32 {
	Index := (*get(object).(*ast.IndexExpr)).Index
	return register(&Index)
}

//export set_ast_IndexExpr_Index
func set_ast_IndexExpr_Index(object uint32, Index uint32) {
	(*get(object).(*ast.IndexExpr)).Index = *get(Index).(*ast.Expr)
}

//export get_ast_IndexExpr_Lbrack
func get_ast_IndexExpr_Lbrack(object uint32) uint32 {
	Lbrack := (*get(object).(*ast.IndexExpr)).Lbrack
	return register(&Lbrack)
}

//export set_ast_IndexExpr_Lbrack
func set_ast_IndexExpr_Lbrack(object uint32, Lbrack uint32) {
	(*get(object).(*ast.IndexExpr)).Lbrack = *get(Lbrack).(*token.Pos)
}

//export get_ast_IndexExpr_Rbrack
func get_ast_IndexExpr_Rbrack(object uint32) uint32 {
	Rbrack := (*get(object).(*ast.IndexExpr)).Rbrack
	return register(&Rbrack)
}

//export set_ast_IndexExpr_Rbrack
func set_ast_IndexExpr_Rbrack(object uint32, Rbrack uint32) {
	(*get(object).(*ast.IndexExpr)).Rbrack = *get(Rbrack).(*token.Pos)
}

//export get_ast_IndexExpr_X
func get_ast_IndexExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.IndexExpr)).X
	return register(&X)
}

//export set_ast_IndexExpr_X
func set_ast_IndexExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.IndexExpr)).X = *get(X).(*ast.Expr)
}

//export defaultast_Interpolation
func defaultast_Interpolation() uint32 {
	var tmp ast.Interpolation
	return register(&tmp)
}

//export get_ast_Interpolation_Elts
func get_ast_Interpolation_Elts(object uint32) uint32 {
	Elts := (*get(object).(*ast.Interpolation)).Elts
	return register(&Elts)
}

//export set_ast_Interpolation_Elts
func set_ast_Interpolation_Elts(object uint32, Elts uint32) {
	(*get(object).(*ast.Interpolation)).Elts = *get(Elts).(*[]ast.Expr)
}

//export as_ast_Label
func as_ast_Label(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Label)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}

//export defaultast_LetClause
func defaultast_LetClause() uint32 {
	var tmp ast.LetClause
	return register(&tmp)
}

//export get_ast_LetClause_Equal
func get_ast_LetClause_Equal(object uint32) uint32 {
	Equal := (*get(object).(*ast.LetClause)).Equal
	return register(&Equal)
}

//export set_ast_LetClause_Equal
func set_ast_LetClause_Equal(object uint32, Equal uint32) {
	(*get(object).(*ast.LetClause)).Equal = *get(Equal).(*token.Pos)
}

//export get_ast_LetClause_Expr
func get_ast_LetClause_Expr(object uint32) uint32 {
	Expr := (*get(object).(*ast.LetClause)).Expr
	return register(&Expr)
}

//export set_ast_LetClause_Expr
func set_ast_LetClause_Expr(object uint32, Expr uint32) {
	(*get(object).(*ast.LetClause)).Expr = *get(Expr).(*ast.Expr)
}

//export get_ast_LetClause_Ident
func get_ast_LetClause_Ident(object uint32) uint32 {
	Ident := (*get(object).(*ast.LetClause)).Ident
	return register(&Ident)
}

//export set_ast_LetClause_Ident
func set_ast_LetClause_Ident(object uint32, Ident uint32) {
	(*get(object).(*ast.LetClause)).Ident = *get(Ident).(**ast.Ident)
}

//export get_ast_LetClause_Let
func get_ast_LetClause_Let(object uint32) uint32 {
	Let := (*get(object).(*ast.LetClause)).Let
	return register(&Let)
}

//export set_ast_LetClause_Let
func set_ast_LetClause_Let(object uint32, Let uint32) {
	(*get(object).(*ast.LetClause)).Let = *get(Let).(*token.Pos)
}

//export defaultast_ListComprehension
func defaultast_ListComprehension() uint32 {
	var tmp ast.ListComprehension
	return register(&tmp)
}

//export get_ast_ListComprehension_Clauses
func get_ast_ListComprehension_Clauses(object uint32) uint32 {
	Clauses := (*get(object).(*ast.ListComprehension)).Clauses
	return register(&Clauses)
}

//export set_ast_ListComprehension_Clauses
func set_ast_ListComprehension_Clauses(object uint32, Clauses uint32) {
	(*get(object).(*ast.ListComprehension)).Clauses = *get(Clauses).(*[]ast.Clause)
}

//export get_ast_ListComprehension_Expr
func get_ast_ListComprehension_Expr(object uint32) uint32 {
	Expr := (*get(object).(*ast.ListComprehension)).Expr
	return register(&Expr)
}

//export set_ast_ListComprehension_Expr
func set_ast_ListComprehension_Expr(object uint32, Expr uint32) {
	(*get(object).(*ast.ListComprehension)).Expr = *get(Expr).(*ast.Expr)
}

//export get_ast_ListComprehension_Lbrack
func get_ast_ListComprehension_Lbrack(object uint32) uint32 {
	Lbrack := (*get(object).(*ast.ListComprehension)).Lbrack
	return register(&Lbrack)
}

//export set_ast_ListComprehension_Lbrack
func set_ast_ListComprehension_Lbrack(object uint32, Lbrack uint32) {
	(*get(object).(*ast.ListComprehension)).Lbrack = *get(Lbrack).(*token.Pos)
}

//export get_ast_ListComprehension_Rbrack
func get_ast_ListComprehension_Rbrack(object uint32) uint32 {
	Rbrack := (*get(object).(*ast.ListComprehension)).Rbrack
	return register(&Rbrack)
}

//export set_ast_ListComprehension_Rbrack
func set_ast_ListComprehension_Rbrack(object uint32, Rbrack uint32) {
	(*get(object).(*ast.ListComprehension)).Rbrack = *get(Rbrack).(*token.Pos)
}

//export defaultast_ListLit
func defaultast_ListLit() uint32 {
	var tmp ast.ListLit
	return register(&tmp)
}

//export get_ast_ListLit_Elts
func get_ast_ListLit_Elts(object uint32) uint32 {
	Elts := (*get(object).(*ast.ListLit)).Elts
	return register(&Elts)
}

//export set_ast_ListLit_Elts
func set_ast_ListLit_Elts(object uint32, Elts uint32) {
	(*get(object).(*ast.ListLit)).Elts = *get(Elts).(*[]ast.Expr)
}

//export get_ast_ListLit_Lbrack
func get_ast_ListLit_Lbrack(object uint32) uint32 {
	Lbrack := (*get(object).(*ast.ListLit)).Lbrack
	return register(&Lbrack)
}

//export set_ast_ListLit_Lbrack
func set_ast_ListLit_Lbrack(object uint32, Lbrack uint32) {
	(*get(object).(*ast.ListLit)).Lbrack = *get(Lbrack).(*token.Pos)
}

//export get_ast_ListLit_Rbrack
func get_ast_ListLit_Rbrack(object uint32) uint32 {
	Rbrack := (*get(object).(*ast.ListLit)).Rbrack
	return register(&Rbrack)
}

//export set_ast_ListLit_Rbrack
func set_ast_ListLit_Rbrack(object uint32, Rbrack uint32) {
	(*get(object).(*ast.ListLit)).Rbrack = *get(Rbrack).(*token.Pos)
}

//export as_ast_Node
func as_ast_Node(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Node)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}

//export defaultast_Package
func defaultast_Package() uint32 {
	var tmp ast.Package
	return register(&tmp)
}

//export get_ast_Package_Name
func get_ast_Package_Name(object uint32) uint32 {
	Name := (*get(object).(*ast.Package)).Name
	return register(&Name)
}

//export set_ast_Package_Name
func set_ast_Package_Name(object uint32, Name uint32) {
	(*get(object).(*ast.Package)).Name = *get(Name).(**ast.Ident)
}

//export get_ast_Package_PackagePos
func get_ast_Package_PackagePos(object uint32) uint32 {
	PackagePos := (*get(object).(*ast.Package)).PackagePos
	return register(&PackagePos)
}

//export set_ast_Package_PackagePos
func set_ast_Package_PackagePos(object uint32, PackagePos uint32) {
	(*get(object).(*ast.Package)).PackagePos = *get(PackagePos).(*token.Pos)
}

//export defaultast_ParenExpr
func defaultast_ParenExpr() uint32 {
	var tmp ast.ParenExpr
	return register(&tmp)
}

//export get_ast_ParenExpr_Lparen
func get_ast_ParenExpr_Lparen(object uint32) uint32 {
	Lparen := (*get(object).(*ast.ParenExpr)).Lparen
	return register(&Lparen)
}

//export set_ast_ParenExpr_Lparen
func set_ast_ParenExpr_Lparen(object uint32, Lparen uint32) {
	(*get(object).(*ast.ParenExpr)).Lparen = *get(Lparen).(*token.Pos)
}

//export get_ast_ParenExpr_Rparen
func get_ast_ParenExpr_Rparen(object uint32) uint32 {
	Rparen := (*get(object).(*ast.ParenExpr)).Rparen
	return register(&Rparen)
}

//export set_ast_ParenExpr_Rparen
func set_ast_ParenExpr_Rparen(object uint32, Rparen uint32) {
	(*get(object).(*ast.ParenExpr)).Rparen = *get(Rparen).(*token.Pos)
}

//export get_ast_ParenExpr_X
func get_ast_ParenExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.ParenExpr)).X
	return register(&X)
}

//export set_ast_ParenExpr_X
func set_ast_ParenExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.ParenExpr)).X = *get(X).(*ast.Expr)
}

//export defaultast_SelectorExpr
func defaultast_SelectorExpr() uint32 {
	var tmp ast.SelectorExpr
	return register(&tmp)
}

//export get_ast_SelectorExpr_Sel
func get_ast_SelectorExpr_Sel(object uint32) uint32 {
	Sel := (*get(object).(*ast.SelectorExpr)).Sel
	return register(&Sel)
}

//export set_ast_SelectorExpr_Sel
func set_ast_SelectorExpr_Sel(object uint32, Sel uint32) {
	(*get(object).(*ast.SelectorExpr)).Sel = *get(Sel).(*ast.Label)
}

//export get_ast_SelectorExpr_X
func get_ast_SelectorExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.SelectorExpr)).X
	return register(&X)
}

//export set_ast_SelectorExpr_X
func set_ast_SelectorExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.SelectorExpr)).X = *get(X).(*ast.Expr)
}

//export defaultast_SliceExpr
func defaultast_SliceExpr() uint32 {
	var tmp ast.SliceExpr
	return register(&tmp)
}

//export get_ast_SliceExpr_High
func get_ast_SliceExpr_High(object uint32) uint32 {
	High := (*get(object).(*ast.SliceExpr)).High
	return register(&High)
}

//export set_ast_SliceExpr_High
func set_ast_SliceExpr_High(object uint32, High uint32) {
	(*get(object).(*ast.SliceExpr)).High = *get(High).(*ast.Expr)
}

//export get_ast_SliceExpr_Lbrack
func get_ast_SliceExpr_Lbrack(object uint32) uint32 {
	Lbrack := (*get(object).(*ast.SliceExpr)).Lbrack
	return register(&Lbrack)
}

//export set_ast_SliceExpr_Lbrack
func set_ast_SliceExpr_Lbrack(object uint32, Lbrack uint32) {
	(*get(object).(*ast.SliceExpr)).Lbrack = *get(Lbrack).(*token.Pos)
}

//export get_ast_SliceExpr_Low
func get_ast_SliceExpr_Low(object uint32) uint32 {
	Low := (*get(object).(*ast.SliceExpr)).Low
	return register(&Low)
}

//export set_ast_SliceExpr_Low
func set_ast_SliceExpr_Low(object uint32, Low uint32) {
	(*get(object).(*ast.SliceExpr)).Low = *get(Low).(*ast.Expr)
}

//export get_ast_SliceExpr_Rbrack
func get_ast_SliceExpr_Rbrack(object uint32) uint32 {
	Rbrack := (*get(object).(*ast.SliceExpr)).Rbrack
	return register(&Rbrack)
}

//export set_ast_SliceExpr_Rbrack
func set_ast_SliceExpr_Rbrack(object uint32, Rbrack uint32) {
	(*get(object).(*ast.SliceExpr)).Rbrack = *get(Rbrack).(*token.Pos)
}

//export get_ast_SliceExpr_X
func get_ast_SliceExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.SliceExpr)).X
	return register(&X)
}

//export set_ast_SliceExpr_X
func set_ast_SliceExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.SliceExpr)).X = *get(X).(*ast.Expr)
}

//export as_ast_Spec
func as_ast_Spec(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(ast.Spec)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}

//export defaultast_StructLit
func defaultast_StructLit() uint32 {
	var tmp ast.StructLit
	return register(&tmp)
}

//export get_ast_StructLit_Elts
func get_ast_StructLit_Elts(object uint32) uint32 {
	Elts := (*get(object).(*ast.StructLit)).Elts
	return register(&Elts)
}

//export set_ast_StructLit_Elts
func set_ast_StructLit_Elts(object uint32, Elts uint32) {
	(*get(object).(*ast.StructLit)).Elts = *get(Elts).(*[]ast.Decl)
}

//export get_ast_StructLit_Lbrace
func get_ast_StructLit_Lbrace(object uint32) uint32 {
	Lbrace := (*get(object).(*ast.StructLit)).Lbrace
	return register(&Lbrace)
}

//export set_ast_StructLit_Lbrace
func set_ast_StructLit_Lbrace(object uint32, Lbrace uint32) {
	(*get(object).(*ast.StructLit)).Lbrace = *get(Lbrace).(*token.Pos)
}

//export get_ast_StructLit_Rbrace
func get_ast_StructLit_Rbrace(object uint32) uint32 {
	Rbrace := (*get(object).(*ast.StructLit)).Rbrace
	return register(&Rbrace)
}

//export set_ast_StructLit_Rbrace
func set_ast_StructLit_Rbrace(object uint32, Rbrace uint32) {
	(*get(object).(*ast.StructLit)).Rbrace = *get(Rbrace).(*token.Pos)
}

//export defaultast_TemplateLabel
func defaultast_TemplateLabel() uint32 {
	var tmp ast.TemplateLabel
	return register(&tmp)
}

//export get_ast_TemplateLabel_Ident
func get_ast_TemplateLabel_Ident(object uint32) uint32 {
	Ident := (*get(object).(*ast.TemplateLabel)).Ident
	return register(&Ident)
}

//export set_ast_TemplateLabel_Ident
func set_ast_TemplateLabel_Ident(object uint32, Ident uint32) {
	(*get(object).(*ast.TemplateLabel)).Ident = *get(Ident).(**ast.Ident)
}

//export get_ast_TemplateLabel_Langle
func get_ast_TemplateLabel_Langle(object uint32) uint32 {
	Langle := (*get(object).(*ast.TemplateLabel)).Langle
	return register(&Langle)
}

//export set_ast_TemplateLabel_Langle
func set_ast_TemplateLabel_Langle(object uint32, Langle uint32) {
	(*get(object).(*ast.TemplateLabel)).Langle = *get(Langle).(*token.Pos)
}

//export get_ast_TemplateLabel_Rangle
func get_ast_TemplateLabel_Rangle(object uint32) uint32 {
	Rangle := (*get(object).(*ast.TemplateLabel)).Rangle
	return register(&Rangle)
}

//export set_ast_TemplateLabel_Rangle
func set_ast_TemplateLabel_Rangle(object uint32, Rangle uint32) {
	(*get(object).(*ast.TemplateLabel)).Rangle = *get(Rangle).(*token.Pos)
}

//export defaultast_UnaryExpr
func defaultast_UnaryExpr() uint32 {
	var tmp ast.UnaryExpr
	return register(&tmp)
}

//export get_ast_UnaryExpr_Op
func get_ast_UnaryExpr_Op(object uint32) uint32 {
	Op := (*get(object).(*ast.UnaryExpr)).Op
	return register(&Op)
}

//export set_ast_UnaryExpr_Op
func set_ast_UnaryExpr_Op(object uint32, Op uint32) {
	(*get(object).(*ast.UnaryExpr)).Op = *get(Op).(*token.Token)
}

//export get_ast_UnaryExpr_OpPos
func get_ast_UnaryExpr_OpPos(object uint32) uint32 {
	OpPos := (*get(object).(*ast.UnaryExpr)).OpPos
	return register(&OpPos)
}

//export set_ast_UnaryExpr_OpPos
func set_ast_UnaryExpr_OpPos(object uint32, OpPos uint32) {
	(*get(object).(*ast.UnaryExpr)).OpPos = *get(OpPos).(*token.Pos)
}

//export get_ast_UnaryExpr_X
func get_ast_UnaryExpr_X(object uint32) uint32 {
	X := (*get(object).(*ast.UnaryExpr)).X
	return register(&X)
}

//export set_ast_UnaryExpr_X
func set_ast_UnaryExpr_X(object uint32, X uint32) {
	(*get(object).(*ast.UnaryExpr)).X = *get(X).(*ast.Expr)
}

//export as_astutil_Cursor
func as_astutil_Cursor(object uint32) (uint32, bool) {
	tmp, ok := reflect.ValueOf(get(object)).Elem().Interface().(astutil.Cursor)
	if !ok {
		return 0, false
	}
	return register(&tmp), true
}

//export defaultastutil_ImportInfo
func defaultastutil_ImportInfo() uint32 {
	var tmp astutil.ImportInfo
	return register(&tmp)
}

//export get_astutil_ImportInfo_Dir
func get_astutil_ImportInfo_Dir(object uint32) *C.char {
	Dir := (*get(object).(*astutil.ImportInfo)).Dir
	return C.CString(Dir)
}

//export set_astutil_ImportInfo_Dir
func set_astutil_ImportInfo_Dir(object uint32, Dir *C.char) {
	(*get(object).(*astutil.ImportInfo)).Dir = C.GoString(Dir)
}

//export get_astutil_ImportInfo_ID
func get_astutil_ImportInfo_ID(object uint32) *C.char {
	ID := (*get(object).(*astutil.ImportInfo)).ID
	return C.CString(ID)
}

//export set_astutil_ImportInfo_ID
func set_astutil_ImportInfo_ID(object uint32, ID *C.char) {
	(*get(object).(*astutil.ImportInfo)).ID = C.GoString(ID)
}

//export get_astutil_ImportInfo_Ident
func get_astutil_ImportInfo_Ident(object uint32) *C.char {
	Ident := (*get(object).(*astutil.ImportInfo)).Ident
	return C.CString(Ident)
}

//export set_astutil_ImportInfo_Ident
func set_astutil_ImportInfo_Ident(object uint32, Ident *C.char) {
	(*get(object).(*astutil.ImportInfo)).Ident = C.GoString(Ident)
}

//export get_astutil_ImportInfo_PkgName
func get_astutil_ImportInfo_PkgName(object uint32) *C.char {
	PkgName := (*get(object).(*astutil.ImportInfo)).PkgName
	return C.CString(PkgName)
}

//export set_astutil_ImportInfo_PkgName
func set_astutil_ImportInfo_PkgName(object uint32, PkgName *C.char) {
	(*get(object).(*astutil.ImportInfo)).PkgName = C.GoString(PkgName)
}

//export defaultbuild_Context
func defaultbuild_Context() uint32 {
	var tmp build.Context
	return register(&tmp)
}

//export defaultbuild_Encoding
func defaultbuild_Encoding() uint32 {
	var tmp build.Encoding
	return register(&tmp)
}

//export defaultbuild_File
func defaultbuild_File() uint32 {
	var tmp build.File
	return register(&tmp)
}

//export get_build_File_Encoding
func get_build_File_Encoding(object uint32) uint32 {
	Encoding := (*get(object).(*build.File)).Encoding
	return register(&Encoding)
}

//export set_build_File_Encoding
func set_build_File_Encoding(object uint32, Encoding uint32) {
	(*get(object).(*build.File)).Encoding = *get(Encoding).(*build.Encoding)
}

//export get_build_File_Filename
func get_build_File_Filename(object uint32) *C.char {
	Filename := (*get(object).(*build.File)).Filename
	return C.CString(Filename)
}

//export set_build_File_Filename
func set_build_File_Filename(object uint32, Filename *C.char) {
	(*get(object).(*build.File)).Filename = C.GoString(Filename)
}

//export get_build_File_Form
func get_build_File_Form(object uint32) uint32 {
	Form := (*get(object).(*build.File)).Form
	return register(&Form)
}

//export set_build_File_Form
func set_build_File_Form(object uint32, Form uint32) {
	(*get(object).(*build.File)).Form = *get(Form).(*build.Form)
}

//export get_build_File_Interpretation
func get_build_File_Interpretation(object uint32) uint32 {
	Interpretation := (*get(object).(*build.File)).Interpretation
	return register(&Interpretation)
}

//export set_build_File_Interpretation
func set_build_File_Interpretation(object uint32, Interpretation uint32) {
	(*get(object).(*build.File)).Interpretation = *get(Interpretation).(*build.Interpretation)
}

//export get_build_File_Source
func get_build_File_Source(object uint32) uint32 {
	Source := (*get(object).(*build.File)).Source
	return register(&Source)
}

//export set_build_File_Source
func set_build_File_Source(object uint32, Source uint32) {
	(*get(object).(*build.File)).Source = reflect.ValueOf(get(Source)).Elem().Interface()
}

//export get_build_File_Tags
func get_build_File_Tags(object uint32) uint32 {
	Tags := (*get(object).(*build.File)).Tags
	return register(&Tags)
}

//export set_build_File_Tags
func set_build_File_Tags(object uint32, Tags uint32) {
	(*get(object).(*build.File)).Tags = *get(Tags).(*map[string]string)
}

//export defaultbuild_Form
func defaultbuild_Form() uint32 {
	var tmp build.Form
	return register(&tmp)
}

//export defaultbuild_Instance
func defaultbuild_Instance() uint32 {
	var tmp build.Instance
	return register(&tmp)
}

//export get_build_Instance_AllTags
func get_build_Instance_AllTags(object uint32) uint32 {
	AllTags := (*get(object).(*build.Instance)).AllTags
	return register(&AllTags)
}

//export set_build_Instance_AllTags
func set_build_Instance_AllTags(object uint32, AllTags uint32) {
	(*get(object).(*build.Instance)).AllTags = *get(AllTags).(*[]string)
}

//export get_build_Instance_BuildFiles
func get_build_Instance_BuildFiles(object uint32) uint32 {
	BuildFiles := (*get(object).(*build.Instance)).BuildFiles
	return register(&BuildFiles)
}

//export set_build_Instance_BuildFiles
func set_build_Instance_BuildFiles(object uint32, BuildFiles uint32) {
	(*get(object).(*build.Instance)).BuildFiles = *get(BuildFiles).(*[]*build.File)
}

//export get_build_Instance_CUEFiles
func get_build_Instance_CUEFiles(object uint32) uint32 {
	CUEFiles := (*get(object).(*build.Instance)).CUEFiles
	return register(&CUEFiles)
}

//export set_build_Instance_CUEFiles
func set_build_Instance_CUEFiles(object uint32, CUEFiles uint32) {
	(*get(object).(*build.Instance)).CUEFiles = *get(CUEFiles).(*[]string)
}

//export get_build_Instance_DataFiles
func get_build_Instance_DataFiles(object uint32) uint32 {
	DataFiles := (*get(object).(*build.Instance)).DataFiles
	return register(&DataFiles)
}

//export set_build_Instance_DataFiles
func set_build_Instance_DataFiles(object uint32, DataFiles uint32) {
	(*get(object).(*build.Instance)).DataFiles = *get(DataFiles).(*[]string)
}

//export get_build_Instance_Deps
func get_build_Instance_Deps(object uint32) uint32 {
	Deps := (*get(object).(*build.Instance)).Deps
	return register(&Deps)
}

//export set_build_Instance_Deps
func set_build_Instance_Deps(object uint32, Deps uint32) {
	(*get(object).(*build.Instance)).Deps = *get(Deps).(*[]string)
}

//export get_build_Instance_DepsErrors
func get_build_Instance_DepsErrors(object uint32) uint32 {
	DepsErrors := (*get(object).(*build.Instance)).DepsErrors
	return register(&DepsErrors)
}

//export set_build_Instance_DepsErrors
func set_build_Instance_DepsErrors(object uint32, DepsErrors uint32) {
	(*get(object).(*build.Instance)).DepsErrors = *get(DepsErrors).(*[]error)
}

//export get_build_Instance_Dir
func get_build_Instance_Dir(object uint32) *C.char {
	Dir := (*get(object).(*build.Instance)).Dir
	return C.CString(Dir)
}

//export set_build_Instance_Dir
func set_build_Instance_Dir(object uint32, Dir *C.char) {
	(*get(object).(*build.Instance)).Dir = C.GoString(Dir)
}

//export get_build_Instance_DisplayPath
func get_build_Instance_DisplayPath(object uint32) *C.char {
	DisplayPath := (*get(object).(*build.Instance)).DisplayPath
	return C.CString(DisplayPath)
}

//export set_build_Instance_DisplayPath
func set_build_Instance_DisplayPath(object uint32, DisplayPath *C.char) {
	(*get(object).(*build.Instance)).DisplayPath = C.GoString(DisplayPath)
}

//export get_build_Instance_Err
func get_build_Instance_Err(object uint32) uint32 {
	Err := (*get(object).(*build.Instance)).Err
	return register(&Err)
}

//export set_build_Instance_Err
func set_build_Instance_Err(object uint32, Err uint32) {
	(*get(object).(*build.Instance)).Err = *get(Err).(*errors.Error)
}

//export get_build_Instance_Files
func get_build_Instance_Files(object uint32) uint32 {
	Files := (*get(object).(*build.Instance)).Files
	return register(&Files)
}

//export set_build_Instance_Files
func set_build_Instance_Files(object uint32, Files uint32) {
	(*get(object).(*build.Instance)).Files = *get(Files).(*[]*ast.File)
}

//export get_build_Instance_IgnoredCUEFiles
func get_build_Instance_IgnoredCUEFiles(object uint32) uint32 {
	IgnoredCUEFiles := (*get(object).(*build.Instance)).IgnoredCUEFiles
	return register(&IgnoredCUEFiles)
}

//export set_build_Instance_IgnoredCUEFiles
func set_build_Instance_IgnoredCUEFiles(object uint32, IgnoredCUEFiles uint32) {
	(*get(object).(*build.Instance)).IgnoredCUEFiles = *get(IgnoredCUEFiles).(*[]string)
}

//export get_build_Instance_IgnoredFiles
func get_build_Instance_IgnoredFiles(object uint32) uint32 {
	IgnoredFiles := (*get(object).(*build.Instance)).IgnoredFiles
	return register(&IgnoredFiles)
}

//export set_build_Instance_IgnoredFiles
func set_build_Instance_IgnoredFiles(object uint32, IgnoredFiles uint32) {
	(*get(object).(*build.Instance)).IgnoredFiles = *get(IgnoredFiles).(*[]*build.File)
}

//export get_build_Instance_ImportComment
func get_build_Instance_ImportComment(object uint32) *C.char {
	ImportComment := (*get(object).(*build.Instance)).ImportComment
	return C.CString(ImportComment)
}

//export set_build_Instance_ImportComment
func set_build_Instance_ImportComment(object uint32, ImportComment *C.char) {
	(*get(object).(*build.Instance)).ImportComment = C.GoString(ImportComment)
}

//export get_build_Instance_ImportPath
func get_build_Instance_ImportPath(object uint32) *C.char {
	ImportPath := (*get(object).(*build.Instance)).ImportPath
	return C.CString(ImportPath)
}

//export set_build_Instance_ImportPath
func set_build_Instance_ImportPath(object uint32, ImportPath *C.char) {
	(*get(object).(*build.Instance)).ImportPath = C.GoString(ImportPath)
}

//export get_build_Instance_ImportPaths
func get_build_Instance_ImportPaths(object uint32) uint32 {
	ImportPaths := (*get(object).(*build.Instance)).ImportPaths
	return register(&ImportPaths)
}

//export set_build_Instance_ImportPaths
func set_build_Instance_ImportPaths(object uint32, ImportPaths uint32) {
	(*get(object).(*build.Instance)).ImportPaths = *get(ImportPaths).(*[]string)
}

//export get_build_Instance_ImportPos
func get_build_Instance_ImportPos(object uint32) uint32 {
	ImportPos := (*get(object).(*build.Instance)).ImportPos
	return register(&ImportPos)
}

//export set_build_Instance_ImportPos
func set_build_Instance_ImportPos(object uint32, ImportPos uint32) {
	(*get(object).(*build.Instance)).ImportPos = *get(ImportPos).(*map[string][]token.Pos)
}

//export get_build_Instance_Imports
func get_build_Instance_Imports(object uint32) uint32 {
	Imports := (*get(object).(*build.Instance)).Imports
	return register(&Imports)
}

//export set_build_Instance_Imports
func set_build_Instance_Imports(object uint32, Imports uint32) {
	(*get(object).(*build.Instance)).Imports = *get(Imports).(*[]*build.Instance)
}

//export get_build_Instance_Incomplete
func get_build_Instance_Incomplete(object uint32) bool {
	Incomplete := (*get(object).(*build.Instance)).Incomplete
	return Incomplete
}

//export set_build_Instance_Incomplete
func set_build_Instance_Incomplete(object uint32, Incomplete bool) {
	(*get(object).(*build.Instance)).Incomplete = Incomplete
}

//export get_build_Instance_InvalidCUEFiles
func get_build_Instance_InvalidCUEFiles(object uint32) uint32 {
	InvalidCUEFiles := (*get(object).(*build.Instance)).InvalidCUEFiles
	return register(&InvalidCUEFiles)
}

//export set_build_Instance_InvalidCUEFiles
func set_build_Instance_InvalidCUEFiles(object uint32, InvalidCUEFiles uint32) {
	(*get(object).(*build.Instance)).InvalidCUEFiles = *get(InvalidCUEFiles).(*[]string)
}

//export get_build_Instance_InvalidFiles
func get_build_Instance_InvalidFiles(object uint32) uint32 {
	InvalidFiles := (*get(object).(*build.Instance)).InvalidFiles
	return register(&InvalidFiles)
}

//export set_build_Instance_InvalidFiles
func set_build_Instance_InvalidFiles(object uint32, InvalidFiles uint32) {
	(*get(object).(*build.Instance)).InvalidFiles = *get(InvalidFiles).(*[]*build.File)
}

//export get_build_Instance_Match
func get_build_Instance_Match(object uint32) uint32 {
	Match := (*get(object).(*build.Instance)).Match
	return register(&Match)
}

//export set_build_Instance_Match
func set_build_Instance_Match(object uint32, Match uint32) {
	(*get(object).(*build.Instance)).Match = *get(Match).(*[]string)
}

//export get_build_Instance_Module
func get_build_Instance_Module(object uint32) *C.char {
	Module := (*get(object).(*build.Instance)).Module
	return C.CString(Module)
}

//export set_build_Instance_Module
func set_build_Instance_Module(object uint32, Module *C.char) {
	(*get(object).(*build.Instance)).Module = C.GoString(Module)
}

//export get_build_Instance_OrphanedFiles
func get_build_Instance_OrphanedFiles(object uint32) uint32 {
	OrphanedFiles := (*get(object).(*build.Instance)).OrphanedFiles
	return register(&OrphanedFiles)
}

//export set_build_Instance_OrphanedFiles
func set_build_Instance_OrphanedFiles(object uint32, OrphanedFiles uint32) {
	(*get(object).(*build.Instance)).OrphanedFiles = *get(OrphanedFiles).(*[]*build.File)
}

//export get_build_Instance_PkgName
func get_build_Instance_PkgName(object uint32) *C.char {
	PkgName := (*get(object).(*build.Instance)).PkgName
	return C.CString(PkgName)
}

//export set_build_Instance_PkgName
func set_build_Instance_PkgName(object uint32, PkgName *C.char) {
	(*get(object).(*build.Instance)).PkgName = C.GoString(PkgName)
}

//export get_build_Instance_Root
func get_build_Instance_Root(object uint32) *C.char {
	Root := (*get(object).(*build.Instance)).Root
	return C.CString(Root)
}

//export set_build_Instance_Root
func set_build_Instance_Root(object uint32, Root *C.char) {
	(*get(object).(*build.Instance)).Root = C.GoString(Root)
}

//export get_build_Instance_Scope
func get_build_Instance_Scope(object uint32) uint32 {
	Scope := (*get(object).(*build.Instance)).Scope
	return register(&Scope)
}

//export set_build_Instance_Scope
func set_build_Instance_Scope(object uint32, Scope uint32) {
	(*get(object).(*build.Instance)).Scope = *get(Scope).(**build.Instance)
}

//export get_build_Instance_Standard
func get_build_Instance_Standard(object uint32) bool {
	Standard := (*get(object).(*build.Instance)).Standard
	return Standard
}

//export set_build_Instance_Standard
func set_build_Instance_Standard(object uint32, Standard bool) {
	(*get(object).(*build.Instance)).Standard = Standard
}

//export get_build_Instance_TestCUEFiles
func get_build_Instance_TestCUEFiles(object uint32) uint32 {
	TestCUEFiles := (*get(object).(*build.Instance)).TestCUEFiles
	return register(&TestCUEFiles)
}

//export set_build_Instance_TestCUEFiles
func set_build_Instance_TestCUEFiles(object uint32, TestCUEFiles uint32) {
	(*get(object).(*build.Instance)).TestCUEFiles = *get(TestCUEFiles).(*[]string)
}

//export get_build_Instance_ToolCUEFiles
func get_build_Instance_ToolCUEFiles(object uint32) uint32 {
	ToolCUEFiles := (*get(object).(*build.Instance)).ToolCUEFiles
	return register(&ToolCUEFiles)
}

//export set_build_Instance_ToolCUEFiles
func set_build_Instance_ToolCUEFiles(object uint32, ToolCUEFiles uint32) {
	(*get(object).(*build.Instance)).ToolCUEFiles = *get(ToolCUEFiles).(*[]string)
}

//export get_build_Instance_UnknownFiles
func get_build_Instance_UnknownFiles(object uint32) uint32 {
	UnknownFiles := (*get(object).(*build.Instance)).UnknownFiles
	return register(&UnknownFiles)
}

//export set_build_Instance_UnknownFiles
func set_build_Instance_UnknownFiles(object uint32, UnknownFiles uint32) {
	(*get(object).(*build.Instance)).UnknownFiles = *get(UnknownFiles).(*[]*build.File)
}

//export get_build_Instance_User
func get_build_Instance_User(object uint32) bool {
	User := (*get(object).(*build.Instance)).User
	return User
}

//export set_build_Instance_User
func set_build_Instance_User(object uint32, User bool) {
	(*get(object).(*build.Instance)).User = User
}

//export defaultbuild_Interpretation
func defaultbuild_Interpretation() uint32 {
	var tmp build.Interpretation
	return register(&tmp)
}

//export defaultparser_DeprecationError
func defaultparser_DeprecationError() uint32 {
	var tmp parser.DeprecationError
	return register(&tmp)
}

//export get_parser_DeprecationError_Version
func get_parser_DeprecationError_Version(object uint32) int {
	Version := (*get(object).(*parser.DeprecationError)).Version
	return Version
}

//export set_parser_DeprecationError_Version
func set_parser_DeprecationError_Version(object uint32, Version int) {
	(*get(object).(*parser.DeprecationError)).Version = Version
}

//export f_cue_0_All
func f_cue_0_All() uint32 {
	r0 := cue.All()
	return register(&r0)
}

//export f_cue_0_AppendFloat
func f_cue_0_AppendFloat(o uint32, a0 uint32, a1 byte, a2 int) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).AppendFloat(*get(a0).(*[]byte), a1, a2)
	return register(&r0), exportError(r1)
}

//export f_cue_0_AppendInt
func f_cue_0_AppendInt(o uint32, a0 uint32, a1 int) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).AppendInt(*get(a0).(*[]byte), a1)
	return register(&r0), exportError(r1)
}

//export f_cue_0_Attribute
func f_cue_0_Attribute(o uint32, a0 *C.char) uint32 {
	r0 := (*get(o).(*cue.Value)).Attribute(C.GoString(a0))
	return register(&r0)
}

//export f_cue_0_Attributes
func f_cue_0_Attributes(a0 bool) uint32 {
	r0 := cue.Attributes(a0)
	return register(&r0)
}

//export f_cue_0_Bool
func f_cue_0_Bool(o uint32) (bool, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).Bool()
	return r0, exportError(r1)
}

//export f_cue_0_Build
func f_cue_0_Build(a0 uint32) uint32 {
	r0 := cue.Build(*get(a0).(*[]*build.Instance))
	return register(&r0)
}

//export f_cue_1_Build
func f_cue_1_Build(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(**cue.Instance)).Build(*get(a0).(**build.Instance))
	return register(&r0)
}

//export f_cue_2_Build
func f_cue_2_Build(o uint32, a0 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Runtime)).Build(*get(a0).(**build.Instance))
	return register(&r0), exportError(r1)
}

//export f_cue_0_Bytes
func f_cue_0_Bytes(o uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).Bytes()
	return register(&r0), exportError(r1)
}

//export f_cue_0_CanString
func f_cue_0_CanString(o uint32) bool {
	r0 := (*get(o).(*cue.Kind)).CanString()
	return r0
}

//export f_cue_0_Compile
func f_cue_0_Compile(o uint32, a0 *C.char, a1 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Runtime)).Compile(C.GoString(a0), reflect.ValueOf(get(a1)).Elem().Interface())
	return register(&r0), exportError(r1)
}

//export f_cue_0_CompileExpr
func f_cue_0_CompileExpr(o uint32, a0 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Runtime)).CompileExpr(*get(a0).(*ast.Expr))
	return register(&r0), exportError(r1)
}

//export f_cue_0_CompileFile
func f_cue_0_CompileFile(o uint32, a0 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Runtime)).CompileFile(*get(a0).(**ast.File))
	return register(&r0), exportError(r1)
}

//export f_cue_0_Concrete
func f_cue_0_Concrete(a0 bool) uint32 {
	r0 := cue.Concrete(a0)
	return register(&r0)
}

//export f_cue_0_Decimal
func f_cue_0_Decimal(o uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).Decimal()
	return register(&r0), exportError(r1)
}

//export f_cue_0_Decode
func f_cue_0_Decode(o uint32, a0 uint32) *C.char {
	r0 := (*get(o).(*cue.Value)).Decode(reflect.ValueOf(get(a0)).Elem().Interface())
	return exportError(r0)
}

//export f_cue_0_Def
func f_cue_0_Def(a0 *C.char) uint32 {
	r0 := cue.Def(C.GoString(a0))
	return register(&r0)
}

//export f_cue_0_Default
func f_cue_0_Default(o uint32) (uint32, bool) {
	r0, r1 := (*get(o).(*cue.Value)).Default()
	return register(&r0), r1
}

//export f_cue_0_Definitions
func f_cue_0_Definitions(a0 bool) uint32 {
	r0 := cue.Definitions(a0)
	return register(&r0)
}

//export f_cue_0_Dereference
func f_cue_0_Dereference(a0 uint32) uint32 {
	r0 := cue.Dereference(*get(a0).(*cue.Value))
	return register(&r0)
}

//export f_cue_0_DisallowCycles
func f_cue_0_DisallowCycles(a0 bool) uint32 {
	r0 := cue.DisallowCycles(a0)
	return register(&r0)
}

//export f_cue_0_Doc
func f_cue_0_Doc(o uint32) uint32 {
	r0 := (*get(o).(**cue.Instance)).Doc()
	return register(&r0)
}

//export f_cue_1_Doc
func f_cue_1_Doc(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Doc()
	return register(&r0)
}

//export f_cue_0_Docs
func f_cue_0_Docs(a0 bool) uint32 {
	r0 := cue.Docs(a0)
	return register(&r0)
}

//export f_cue_0_Elem
func f_cue_0_Elem(o uint32) (uint32, bool) {
	r0, r1 := (*get(o).(*cue.Value)).Elem()
	return register(&r0), r1
}

//export f_cue_0_Equals
func f_cue_0_Equals(o uint32, a0 uint32) bool {
	r0 := (*get(o).(*cue.Value)).Equals(*get(a0).(*cue.Value))
	return r0
}

//export f_cue_0_Err
func f_cue_0_Err(o uint32) *C.char {
	r0 := (*get(o).(**cue.Attribute)).Err()
	return exportError(r0)
}

//export f_cue_1_Err
func f_cue_1_Err(o uint32) *C.char {
	r0 := (*get(o).(*cue.Path)).Err()
	return exportError(r0)
}

//export f_cue_2_Err
func f_cue_2_Err(o uint32) *C.char {
	r0 := (*get(o).(*cue.Value)).Err()
	return exportError(r0)
}

//export f_cue_0_Eval
func f_cue_0_Eval(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(**cue.Instance)).Eval(*get(a0).(*ast.Expr))
	return register(&r0)
}

//export f_cue_1_Eval
func f_cue_1_Eval(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Eval()
	return register(&r0)
}

//export f_cue_0_Exists
func f_cue_0_Exists(o uint32) bool {
	r0 := (*get(o).(*cue.Value)).Exists()
	return r0
}

//export f_cue_0_Expr
func f_cue_0_Expr(o uint32) (uint32, uint32) {
	r0, r1 := (*get(o).(*cue.Value)).Expr()
	return register(&r0), register(&r1)
}

//export f_cue_0_Field
func f_cue_0_Field(o uint32, a0 int) uint32 {
	r0 := (*get(o).(**cue.Struct)).Field(a0)
	return register(&r0)
}

//export f_cue_0_FieldByName
func f_cue_0_FieldByName(o uint32, a0 *C.char, a1 bool) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Struct)).FieldByName(C.GoString(a0), a1)
	return register(&r0), exportError(r1)
}

//export f_cue_1_FieldByName
func f_cue_1_FieldByName(o uint32, a0 *C.char, a1 bool) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).FieldByName(C.GoString(a0), a1)
	return register(&r0), exportError(r1)
}

//export f_cue_0_Fields
func f_cue_0_Fields(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(**cue.Struct)).Fields(*get(a0).(*[]cue.Option)...)
	return register(&r0)
}

//export f_cue_1_Fields
func f_cue_1_Fields(o uint32, a0 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).Fields(*get(a0).(*[]cue.Option)...)
	return register(&r0), exportError(r1)
}

//export f_cue_0_Fill
func f_cue_0_Fill(o uint32, a0 uint32, a1 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Instance)).Fill(reflect.ValueOf(get(a0)).Elem().Interface(), *get(a1).(*[]string)...)
	return register(&r0), exportError(r1)
}

//export f_cue_1_Fill
func f_cue_1_Fill(o uint32, a0 uint32, a1 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Fill(reflect.ValueOf(get(a0)).Elem().Interface(), *get(a1).(*[]string)...)
	return register(&r0)
}

//export f_cue_0_Final
func f_cue_0_Final() uint32 {
	r0 := cue.Final()
	return register(&r0)
}

//export f_cue_0_Flag
func f_cue_0_Flag(o uint32, a0 int, a1 *C.char) (bool, *C.char) {
	r0, r1 := (*get(o).(**cue.Attribute)).Flag(a0, C.GoString(a1))
	return r0, exportError(r1)
}

//export f_cue_0_Float64
func f_cue_0_Float64(o uint32) (float64, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).Float64()
	return r0, exportError(r1)
}

//export f_cue_0_Format
func f_cue_0_Format(o uint32, a0 uint32, a1 rune) {
	(*get(o).(*cue.Value)).Format(*get(a0).(*fmt.State), a1)
}

//export f_cue_0_FromExpr
func f_cue_0_FromExpr(o uint32, a0 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Runtime)).FromExpr(*get(a0).(*ast.Expr))
	return register(&r0), exportError(r1)
}

//export f_cue_0_Hidden
func f_cue_0_Hidden(a0 bool) uint32 {
	r0 := cue.Hidden(a0)
	return register(&r0)
}

//export f_cue_0_ID
func f_cue_0_ID(o uint32) *C.char {
	r0 := (*get(o).(**cue.Instance)).ID()
	return C.CString(r0)
}

//export f_cue_0_IncompleteKind
func f_cue_0_IncompleteKind(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).IncompleteKind()
	return register(&r0)
}

//export f_cue_0_Index
func f_cue_0_Index(a0 int) uint32 {
	r0 := cue.Index(a0)
	return register(&r0)
}

//export f_cue_0_Int
func f_cue_0_Int(o uint32, a0 int) (int64, *C.char) {
	r0, r1 := (*get(o).(**cue.Attribute)).Int(a0)
	return r0, exportError(r1)
}

//export f_cue_1_Int
func f_cue_1_Int(o uint32, a0 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).Int(*get(a0).(**big.Int))
	return register(&r0), exportError(r1)
}

//export f_cue_0_Int64
func f_cue_0_Int64(o uint32) (int64, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).Int64()
	return r0, exportError(r1)
}

//export f_cue_0_IsAnyOf
func f_cue_0_IsAnyOf(o uint32, a0 uint32) bool {
	r0 := (*get(o).(*cue.Kind)).IsAnyOf(*get(a0).(*cue.Kind))
	return r0
}

//export f_cue_0_IsClosed
func f_cue_0_IsClosed(o uint32) bool {
	r0 := (*get(o).(*cue.Value)).IsClosed()
	return r0
}

//export f_cue_0_IsConcrete
func f_cue_0_IsConcrete(o uint32) bool {
	r0 := (*get(o).(*cue.Value)).IsConcrete()
	return r0
}

//export f_cue_0_IsDefinition
func f_cue_0_IsDefinition(o uint32) bool {
	r0 := (*get(o).(**cue.Iterator)).IsDefinition()
	return r0
}

//export f_cue_0_IsHidden
func f_cue_0_IsHidden(o uint32) bool {
	r0 := (*get(o).(**cue.Iterator)).IsHidden()
	return r0
}

//export f_cue_0_IsOptional
func f_cue_0_IsOptional(o uint32) bool {
	r0 := (*get(o).(**cue.Iterator)).IsOptional()
	return r0
}

//export f_cue_0_Kind
func f_cue_0_Kind(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Kind()
	return register(&r0)
}

//export f_cue_0_Label
func f_cue_0_Label(o uint32) *C.char {
	r0 := (*get(o).(**cue.Iterator)).Label()
	return C.CString(r0)
}

//export f_cue_1_Label
func f_cue_1_Label(o uint32) (*C.char, bool) {
	r0, r1 := (*get(o).(*cue.Value)).Label()
	return C.CString(r0), r1
}

//export f_cue_0_Len
func f_cue_0_Len(o uint32) int {
	r0 := (*get(o).(**cue.Struct)).Len()
	return r0
}

//export f_cue_1_Len
func f_cue_1_Len(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Len()
	return register(&r0)
}

//export f_cue_0_List
func f_cue_0_List(o uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).List()
	return register(&r0), exportError(r1)
}

//export f_cue_0_Lookup
func f_cue_0_Lookup(o uint32, a0 int, a1 *C.char) (*C.char, bool, *C.char) {
	r0, r1, r2 := (*get(o).(**cue.Attribute)).Lookup(a0, C.GoString(a1))
	return C.CString(r0), r1, exportError(r2)
}

//export f_cue_1_Lookup
func f_cue_1_Lookup(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(**cue.Instance)).Lookup(*get(a0).(*[]string)...)
	return register(&r0)
}

//export f_cue_2_Lookup
func f_cue_2_Lookup(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Lookup(*get(a0).(*[]string)...)
	return register(&r0)
}

//export f_cue_0_LookupDef
func f_cue_0_LookupDef(o uint32, a0 *C.char) uint32 {
	r0 := (*get(o).(**cue.Instance)).LookupDef(C.GoString(a0))
	return register(&r0)
}

//export f_cue_1_LookupDef
func f_cue_1_LookupDef(o uint32, a0 *C.char) uint32 {
	r0 := (*get(o).(*cue.Value)).LookupDef(C.GoString(a0))
	return register(&r0)
}

//export f_cue_0_LookupField
func f_cue_0_LookupField(o uint32, a0 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Instance)).LookupField(*get(a0).(*[]string)...)
	return register(&r0), exportError(r1)
}

//export f_cue_1_LookupField
func f_cue_1_LookupField(o uint32, a0 *C.char) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).LookupField(C.GoString(a0))
	return register(&r0), exportError(r1)
}

//export f_cue_0_LookupPath
func f_cue_0_LookupPath(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).LookupPath(*get(a0).(*cue.Path))
	return register(&r0)
}

//export f_cue_0_MakePath
func f_cue_0_MakePath(a0 uint32) uint32 {
	r0 := cue.MakePath(*get(a0).(*[]cue.Selector)...)
	return register(&r0)
}

//export f_cue_0_MantExp
func f_cue_0_MantExp(o uint32, a0 uint32) (int, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).MantExp(*get(a0).(**big.Int))
	return r0, exportError(r1)
}

//export f_cue_0_Marshal
func f_cue_0_Marshal(o uint32, a0 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Runtime)).Marshal(*get(a0).(*[]*cue.Instance)...)
	return register(&r0), exportError(r1)
}

//export f_cue_0_MarshalJSON
func f_cue_0_MarshalJSON(o uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).MarshalJSON()
	return register(&r0), exportError(r1)
}

//export f_cue_0_Merge
func f_cue_0_Merge(a0 uint32) uint32 {
	r0 := cue.Merge(*get(a0).(*[]*cue.Instance)...)
	return register(&r0)
}

//export f_cue_0_Next
func f_cue_0_Next(o uint32) bool {
	r0 := (*get(o).(**cue.Iterator)).Next()
	return r0
}

//export f_cue_0_Null
func f_cue_0_Null(o uint32) *C.char {
	r0 := (*get(o).(*cue.Value)).Null()
	return exportError(r0)
}

//export f_cue_0_Optional
func f_cue_0_Optional(a0 bool) uint32 {
	r0 := cue.Optional(a0)
	return register(&r0)
}

//export f_cue_0_Parse
func f_cue_0_Parse(o uint32, a0 *C.char, a1 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Runtime)).Parse(C.GoString(a0), reflect.ValueOf(get(a1)).Elem().Interface())
	return register(&r0), exportError(r1)
}

//export f_cue_0_ParsePath
func f_cue_0_ParsePath(a0 *C.char) uint32 {
	r0 := cue.ParsePath(C.GoString(a0))
	return register(&r0)
}

//export f_cue_0_Path
func f_cue_0_Path(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Path()
	return register(&r0)
}

//export f_cue_0_Pos
func f_cue_0_Pos(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Pos()
	return register(&r0)
}

//export f_cue_0_Raw
func f_cue_0_Raw() uint32 {
	r0 := cue.Raw()
	return register(&r0)
}

//export f_cue_0_Reader
func f_cue_0_Reader(o uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).Reader()
	return register(&r0), exportError(r1)
}

//export f_cue_0_Reference
func f_cue_0_Reference(o uint32) (uint32, uint32) {
	r0, r1 := (*get(o).(*cue.Value)).Reference()
	return register(&r0), register(&r1)
}

//export f_cue_0_ResolveReferences
func f_cue_0_ResolveReferences(a0 bool) uint32 {
	r0 := cue.ResolveReferences(a0)
	return register(&r0)
}

//export f_cue_0_Schema
func f_cue_0_Schema() uint32 {
	r0 := cue.Schema()
	return register(&r0)
}

//export f_cue_0_Selectors
func f_cue_0_Selectors(o uint32) uint32 {
	r0 := (*get(o).(*cue.Path)).Selectors()
	return register(&r0)
}

//export f_cue_0_Source
func f_cue_0_Source(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Source()
	return register(&r0)
}

//export f_cue_0_Split
func f_cue_0_Split(o uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Split()
	return register(&r0)
}

//export f_cue_0_Str
func f_cue_0_Str(a0 *C.char) uint32 {
	r0 := cue.Str(C.GoString(a0))
	return register(&r0)
}

//export f_cue_0_String
func f_cue_0_String(o uint32, a0 int) (*C.char, *C.char) {
	r0, r1 := (*get(o).(**cue.Attribute)).String(a0)
	return C.CString(r0), exportError(r1)
}

//export f_cue_1_String
func f_cue_1_String(o uint32) *C.char {
	r0 := (*get(o).(*cue.Kind)).String()
	return C.CString(r0)
}

//export f_cue_2_String
func f_cue_2_String(o uint32) *C.char {
	r0 := (*get(o).(*cue.Op)).String()
	return C.CString(r0)
}

//export f_cue_3_String
func f_cue_3_String(o uint32) *C.char {
	r0 := (*get(o).(*cue.Path)).String()
	return C.CString(r0)
}

//export f_cue_4_String
func f_cue_4_String(o uint32) *C.char {
	r0 := (*get(o).(*cue.Selector)).String()
	return C.CString(r0)
}

//export f_cue_5_String
func f_cue_5_String(o uint32) (*C.char, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).String()
	return C.CString(r0), exportError(r1)
}

//export f_cue_0_Struct
func f_cue_0_Struct(o uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).Struct()
	return register(&r0), exportError(r1)
}

//export f_cue_0_Subsume
func f_cue_0_Subsume(o uint32, a0 uint32, a1 uint32) *C.char {
	r0 := (*get(o).(*cue.Value)).Subsume(*get(a0).(*cue.Value), *get(a1).(*[]cue.Option)...)
	return exportError(r0)
}

//export f_cue_0_Subsumes
func f_cue_0_Subsumes(o uint32, a0 uint32) bool {
	r0 := (*get(o).(*cue.Value)).Subsumes(*get(a0).(*cue.Value))
	return r0
}

//export f_cue_0_Syntax
func f_cue_0_Syntax(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Syntax(*get(a0).(*[]cue.Option)...)
	return register(&r0)
}

//export f_cue_0_Token
func f_cue_0_Token(o uint32) uint32 {
	r0 := (*get(o).(*cue.Op)).Token()
	return register(&r0)
}

//export f_cue_0_TypeString
func f_cue_0_TypeString(o uint32) *C.char {
	r0 := (*get(o).(*cue.Kind)).TypeString()
	return C.CString(r0)
}

//export f_cue_0_Uint64
func f_cue_0_Uint64(o uint32) (uint64, *C.char) {
	r0, r1 := (*get(o).(*cue.Value)).Uint64()
	return r0, exportError(r1)
}

//export f_cue_0_Unify
func f_cue_0_Unify(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).Unify(*get(a0).(*cue.Value))
	return register(&r0)
}

//export f_cue_0_UnifyAccept
func f_cue_0_UnifyAccept(o uint32, a0 uint32, a1 uint32) uint32 {
	r0 := (*get(o).(*cue.Value)).UnifyAccept(*get(a0).(*cue.Value), *get(a1).(*cue.Value))
	return register(&r0)
}

//export f_cue_0_Unmarshal
func f_cue_0_Unmarshal(o uint32, a0 uint32) (uint32, *C.char) {
	r0, r1 := (*get(o).(**cue.Runtime)).Unmarshal(*get(a0).(*[]byte))
	return register(&r0), exportError(r1)
}

//export f_cue_0_Validate
func f_cue_0_Validate(o uint32, a0 uint32) *C.char {
	r0 := (*get(o).(*cue.Value)).Validate(*get(a0).(*[]cue.Option)...)
	return exportError(r0)
}

//export f_cue_0_Value
func f_cue_0_Value(o uint32) uint32 {
	r0 := (*get(o).(**cue.Instance)).Value()
	return register(&r0)
}

//export f_cue_1_Value
func f_cue_1_Value(o uint32) uint32 {
	r0 := (*get(o).(**cue.Iterator)).Value()
	return register(&r0)
}

//export f_ast_0_AddComment
func f_ast_0_AddComment(a0 uint32, a1 uint32) {
	ast.AddComment(*get(a0).(*ast.Node), *get(a1).(**ast.CommentGroup))
}

//export f_ast_1_AddComment
func f_ast_1_AddComment(o uint32, a0 uint32) {
	(*get(o).(**ast.Comment)).AddComment(*get(a0).(**ast.CommentGroup))
}

//export f_ast_2_AddComment
func f_ast_2_AddComment(o uint32, a0 uint32) {
	(*get(o).(**ast.CommentGroup)).AddComment(*get(a0).(**ast.CommentGroup))
}

//export f_ast_0_Comments
func f_ast_0_Comments(a0 uint32) uint32 {
	r0 := ast.Comments(*get(a0).(*ast.Node))
	return register(&r0)
}

//export f_ast_1_Comments
func f_ast_1_Comments(o uint32) uint32 {
	r0 := (*get(o).(**ast.Comment)).Comments()
	return register(&r0)
}

//export f_ast_2_Comments
func f_ast_2_Comments(o uint32) uint32 {
	r0 := (*get(o).(**ast.CommentGroup)).Comments()
	return register(&r0)
}

//export f_ast_0_Embed
func f_ast_0_Embed(a0 uint32) uint32 {
	r0 := ast.Embed(*get(a0).(*ast.Expr))
	return register(&r0)
}

//export f_ast_0_End
func f_ast_0_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Alias)).End()
	return register(&r0)
}

//export f_ast_1_End
func f_ast_1_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Attribute)).End()
	return register(&r0)
}

//export f_ast_2_End
func f_ast_2_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.BadDecl)).End()
	return register(&r0)
}

//export f_ast_3_End
func f_ast_3_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.BadExpr)).End()
	return register(&r0)
}

//export f_ast_4_End
func f_ast_4_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.BasicLit)).End()
	return register(&r0)
}

//export f_ast_5_End
func f_ast_5_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.BinaryExpr)).End()
	return register(&r0)
}

//export f_ast_6_End
func f_ast_6_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.BottomLit)).End()
	return register(&r0)
}

//export f_ast_7_End
func f_ast_7_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.CallExpr)).End()
	return register(&r0)
}

//export f_ast_8_End
func f_ast_8_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Comment)).End()
	return register(&r0)
}

//export f_ast_9_End
func f_ast_9_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.CommentGroup)).End()
	return register(&r0)
}

//export f_ast_10_End
func f_ast_10_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Comprehension)).End()
	return register(&r0)
}

//export f_ast_11_End
func f_ast_11_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Ellipsis)).End()
	return register(&r0)
}

//export f_ast_12_End
func f_ast_12_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.EmbedDecl)).End()
	return register(&r0)
}

//export f_ast_13_End
func f_ast_13_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Field)).End()
	return register(&r0)
}

//export f_ast_14_End
func f_ast_14_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.File)).End()
	return register(&r0)
}

//export f_ast_15_End
func f_ast_15_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ForClause)).End()
	return register(&r0)
}

//export f_ast_16_End
func f_ast_16_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Ident)).End()
	return register(&r0)
}

//export f_ast_17_End
func f_ast_17_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.IfClause)).End()
	return register(&r0)
}

//export f_ast_18_End
func f_ast_18_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ImportDecl)).End()
	return register(&r0)
}

//export f_ast_19_End
func f_ast_19_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ImportSpec)).End()
	return register(&r0)
}

//export f_ast_20_End
func f_ast_20_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.IndexExpr)).End()
	return register(&r0)
}

//export f_ast_21_End
func f_ast_21_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Interpolation)).End()
	return register(&r0)
}

//export f_ast_22_End
func f_ast_22_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.LetClause)).End()
	return register(&r0)
}

//export f_ast_23_End
func f_ast_23_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ListComprehension)).End()
	return register(&r0)
}

//export f_ast_24_End
func f_ast_24_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ListLit)).End()
	return register(&r0)
}

//export f_ast_25_End
func f_ast_25_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.Package)).End()
	return register(&r0)
}

//export f_ast_26_End
func f_ast_26_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.ParenExpr)).End()
	return register(&r0)
}

//export f_ast_27_End
func f_ast_27_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.SelectorExpr)).End()
	return register(&r0)
}

//export f_ast_28_End
func f_ast_28_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.SliceExpr)).End()
	return register(&r0)
}

//export f_ast_29_End
func f_ast_29_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.StructLit)).End()
	return register(&r0)
}

//export f_ast_30_End
func f_ast_30_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.TemplateLabel)).End()
	return register(&r0)
}

//export f_ast_31_End
func f_ast_31_End(o uint32) uint32 {
	r0 := (*get(o).(**ast.UnaryExpr)).End()
	return register(&r0)
}

//export f_ast_0_IsValidIdent
func f_ast_0_IsValidIdent(a0 *C.char) bool {
	r0 := ast.IsValidIdent(C.GoString(a0))
	return r0
}

//export f_ast_0_LabelName
func f_ast_0_LabelName(a0 uint32) (*C.char, bool, *C.char) {
	r0, r1, r2 := ast.LabelName(*get(a0).(*ast.Label))
	return C.CString(r0), r1, exportError(r2)
}

//export f_ast_0_Name
func f_ast_0_Name(a0 uint32) *C.char {
	r0 := ast.Name(*get(a0).(*ast.Node))
	return C.CString(r0)
}

//export f_ast_0_NewBinExpr
func f_ast_0_NewBinExpr(a0 uint32, a1 uint32) uint32 {
	r0 := ast.NewBinExpr(*get(a0).(*token.Token), *get(a1).(*[]ast.Expr)...)
	return register(&r0)
}

//export f_ast_0_NewBool
func f_ast_0_NewBool(a0 bool) uint32 {
	r0 := ast.NewBool(a0)
	return register(&r0)
}

//export f_ast_0_NewCall
func f_ast_0_NewCall(a0 uint32, a1 uint32) uint32 {
	r0 := ast.NewCall(*get(a0).(*ast.Expr), *get(a1).(*[]ast.Expr)...)
	return register(&r0)
}

//export f_ast_0_NewIdent
func f_ast_0_NewIdent(a0 *C.char) uint32 {
	r0 := ast.NewIdent(C.GoString(a0))
	return register(&r0)
}

//export f_ast_0_NewImport
func f_ast_0_NewImport(a0 uint32, a1 *C.char) uint32 {
	r0 := ast.NewImport(*get(a0).(**ast.Ident), C.GoString(a1))
	return register(&r0)
}

//export f_ast_0_NewList
func f_ast_0_NewList(a0 uint32) uint32 {
	r0 := ast.NewList(*get(a0).(*[]ast.Expr)...)
	return register(&r0)
}

//export f_ast_0_NewLit
func f_ast_0_NewLit(a0 uint32, a1 *C.char) uint32 {
	r0 := ast.NewLit(*get(a0).(*token.Token), C.GoString(a1))
	return register(&r0)
}

//export f_ast_0_NewNull
func f_ast_0_NewNull() uint32 {
	r0 := ast.NewNull()
	return register(&r0)
}

//export f_ast_0_NewSel
func f_ast_0_NewSel(a0 uint32, a1 uint32) uint32 {
	r0 := ast.NewSel(*get(a0).(*ast.Expr), *get(a1).(*[]string)...)
	return register(&r0)
}

//export f_ast_0_NewString
func f_ast_0_NewString(a0 *C.char) uint32 {
	r0 := ast.NewString(C.GoString(a0))
	return register(&r0)
}

//export f_ast_0_NewStruct
func f_ast_0_NewStruct(a0 uint32) uint32 {
	r0 := ast.NewStruct(*get(a0).(*[]interface {
	})...)
	return register(&r0)
}

//export f_ast_0_PackageName
func f_ast_0_PackageName(o uint32) *C.char {
	r0 := (*get(o).(**ast.File)).PackageName()
	return C.CString(r0)
}

//export f_ast_0_ParseIdent
func f_ast_0_ParseIdent(a0 uint32) (*C.char, *C.char) {
	r0, r1 := ast.ParseIdent(*get(a0).(**ast.Ident))
	return C.CString(r0), exportError(r1)
}

//export f_ast_0_Pos
func f_ast_0_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Alias)).Pos()
	return register(&r0)
}

//export f_ast_1_Pos
func f_ast_1_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Attribute)).Pos()
	return register(&r0)
}

//export f_ast_2_Pos
func f_ast_2_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.BadDecl)).Pos()
	return register(&r0)
}

//export f_ast_3_Pos
func f_ast_3_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.BadExpr)).Pos()
	return register(&r0)
}

//export f_ast_4_Pos
func f_ast_4_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.BasicLit)).Pos()
	return register(&r0)
}

//export f_ast_5_Pos
func f_ast_5_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.BinaryExpr)).Pos()
	return register(&r0)
}

//export f_ast_6_Pos
func f_ast_6_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.BottomLit)).Pos()
	return register(&r0)
}

//export f_ast_7_Pos
func f_ast_7_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.CallExpr)).Pos()
	return register(&r0)
}

//export f_ast_8_Pos
func f_ast_8_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Comment)).Pos()
	return register(&r0)
}

//export f_ast_9_Pos
func f_ast_9_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.CommentGroup)).Pos()
	return register(&r0)
}

//export f_ast_10_Pos
func f_ast_10_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Comprehension)).Pos()
	return register(&r0)
}

//export f_ast_11_Pos
func f_ast_11_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Ellipsis)).Pos()
	return register(&r0)
}

//export f_ast_12_Pos
func f_ast_12_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.EmbedDecl)).Pos()
	return register(&r0)
}

//export f_ast_13_Pos
func f_ast_13_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Field)).Pos()
	return register(&r0)
}

//export f_ast_14_Pos
func f_ast_14_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.File)).Pos()
	return register(&r0)
}

//export f_ast_15_Pos
func f_ast_15_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ForClause)).Pos()
	return register(&r0)
}

//export f_ast_16_Pos
func f_ast_16_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Ident)).Pos()
	return register(&r0)
}

//export f_ast_17_Pos
func f_ast_17_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.IfClause)).Pos()
	return register(&r0)
}

//export f_ast_18_Pos
func f_ast_18_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ImportDecl)).Pos()
	return register(&r0)
}

//export f_ast_19_Pos
func f_ast_19_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ImportSpec)).Pos()
	return register(&r0)
}

//export f_ast_20_Pos
func f_ast_20_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.IndexExpr)).Pos()
	return register(&r0)
}

//export f_ast_21_Pos
func f_ast_21_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Interpolation)).Pos()
	return register(&r0)
}

//export f_ast_22_Pos
func f_ast_22_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.LetClause)).Pos()
	return register(&r0)
}

//export f_ast_23_Pos
func f_ast_23_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ListComprehension)).Pos()
	return register(&r0)
}

//export f_ast_24_Pos
func f_ast_24_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ListLit)).Pos()
	return register(&r0)
}

//export f_ast_25_Pos
func f_ast_25_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.Package)).Pos()
	return register(&r0)
}

//export f_ast_26_Pos
func f_ast_26_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.ParenExpr)).Pos()
	return register(&r0)
}

//export f_ast_27_Pos
func f_ast_27_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.SelectorExpr)).Pos()
	return register(&r0)
}

//export f_ast_28_Pos
func f_ast_28_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.SliceExpr)).Pos()
	return register(&r0)
}

//export f_ast_29_Pos
func f_ast_29_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.StructLit)).Pos()
	return register(&r0)
}

//export f_ast_30_Pos
func f_ast_30_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.TemplateLabel)).Pos()
	return register(&r0)
}

//export f_ast_31_Pos
func f_ast_31_Pos(o uint32) uint32 {
	r0 := (*get(o).(**ast.UnaryExpr)).Pos()
	return register(&r0)
}

//export f_ast_0_Preamble
func f_ast_0_Preamble(o uint32) uint32 {
	r0 := (*get(o).(**ast.File)).Preamble()
	return register(&r0)
}

//export f_ast_0_QuoteIdent
func f_ast_0_QuoteIdent(a0 *C.char) (*C.char, *C.char) {
	r0, r1 := ast.QuoteIdent(C.GoString(a0))
	return C.CString(r0), exportError(r1)
}

//export f_ast_0_SetComments
func f_ast_0_SetComments(a0 uint32, a1 uint32) {
	ast.SetComments(*get(a0).(*ast.Node), *get(a1).(*[]*ast.CommentGroup))
}

//export f_ast_0_SetPos
func f_ast_0_SetPos(a0 uint32, a1 uint32) {
	ast.SetPos(*get(a0).(*ast.Node), *get(a1).(*token.Pos))
}

//export f_ast_0_SetRelPos
func f_ast_0_SetRelPos(a0 uint32, a1 uint32) {
	ast.SetRelPos(*get(a0).(*ast.Node), *get(a1).(*token.RelPos))
}

//export f_ast_0_Split
func f_ast_0_Split(o uint32) (*C.char, *C.char) {
	r0, r1 := (*get(o).(**ast.Attribute)).Split()
	return C.CString(r0), C.CString(r1)
}

//export f_ast_0_String
func f_ast_0_String(o uint32) *C.char {
	r0 := (*get(o).(**ast.Ident)).String()
	return C.CString(r0)
}

//export f_ast_0_Text
func f_ast_0_Text(o uint32) *C.char {
	r0 := (*get(o).(**ast.CommentGroup)).Text()
	return C.CString(r0)
}

//export f_astutil_0_ApplyRecursively
func f_astutil_0_ApplyRecursively(a0 uint32) uint32 {
	r0 := astutil.ApplyRecursively(*get(a0).(*ast.Node))
	return register(&r0)
}

//export f_astutil_0_CopyComments
func f_astutil_0_CopyComments(a0 uint32, a1 uint32) {
	astutil.CopyComments(*get(a0).(*ast.Node), *get(a1).(*ast.Node))
}

//export f_astutil_0_CopyMeta
func f_astutil_0_CopyMeta(a0 uint32, a1 uint32) uint32 {
	r0 := astutil.CopyMeta(*get(a0).(*ast.Node), *get(a1).(*ast.Node))
	return register(&r0)
}

//export f_astutil_0_CopyPosition
func f_astutil_0_CopyPosition(a0 uint32, a1 uint32) {
	astutil.CopyPosition(*get(a0).(*ast.Node), *get(a1).(*ast.Node))
}

//export f_astutil_0_ParseImportSpec
func f_astutil_0_ParseImportSpec(a0 uint32) (uint32, *C.char) {
	r0, r1 := astutil.ParseImportSpec(*get(a0).(**ast.ImportSpec))
	return register(&r0), exportError(r1)
}

//export f_astutil_0_Resolve
func f_astutil_0_Resolve(a0 uint32, a1 uint32) {
	astutil.Resolve(*get(a0).(**ast.File), *get(a1).(*astutil.ErrFunc))
}

//export f_astutil_0_ResolveExpr
func f_astutil_0_ResolveExpr(a0 uint32, a1 uint32) {
	astutil.ResolveExpr(*get(a0).(*ast.Expr), *get(a1).(*astutil.ErrFunc))
}

//export f_astutil_0_Sanitize
func f_astutil_0_Sanitize(a0 uint32) *C.char {
	r0 := astutil.Sanitize(*get(a0).(**ast.File))
	return exportError(r0)
}

//export f_astutil_0_ToFile
func f_astutil_0_ToFile(a0 uint32) (uint32, *C.char) {
	r0, r1 := astutil.ToFile(*get(a0).(*ast.Expr))
	return register(&r0), exportError(r1)
}

//export f_build_0_Abs
func f_build_0_Abs(o uint32, a0 *C.char) *C.char {
	r0 := (*get(o).(**build.Instance)).Abs(C.GoString(a0))
	return C.CString(r0)
}

//export f_build_0_AddFile
func f_build_0_AddFile(o uint32, a0 *C.char, a1 uint32) *C.char {
	r0 := (*get(o).(**build.Instance)).AddFile(C.GoString(a0), reflect.ValueOf(get(a1)).Elem().Interface())
	return exportError(r0)
}

//export f_build_0_AddSyntax
func f_build_0_AddSyntax(o uint32, a0 uint32) uint32 {
	r0 := (*get(o).(**build.Instance)).AddSyntax(*get(a0).(**ast.File))
	return register(&r0)
}

//export f_build_0_Complete
func f_build_0_Complete(o uint32) *C.char {
	r0 := (*get(o).(**build.Instance)).Complete()
	return exportError(r0)
}

//export f_build_0_Context
func f_build_0_Context(o uint32) uint32 {
	r0 := (*get(o).(**build.Instance)).Context()
	return register(&r0)
}

//export f_build_0_Dependencies
func f_build_0_Dependencies(o uint32) uint32 {
	r0 := (*get(o).(**build.Instance)).Dependencies()
	return register(&r0)
}

//export f_build_0_ID
func f_build_0_ID(o uint32) *C.char {
	r0 := (*get(o).(**build.Instance)).ID()
	return C.CString(r0)
}

//export f_build_0_IsLocalImport
func f_build_0_IsLocalImport(a0 *C.char) bool {
	r0 := build.IsLocalImport(C.GoString(a0))
	return r0
}

//export f_build_0_Loader
func f_build_0_Loader(a0 uint32) uint32 {
	r0 := build.Loader(*get(a0).(*build.LoadFunc))
	return register(&r0)
}

//export f_build_0_LookupImport
func f_build_0_LookupImport(o uint32, a0 *C.char) uint32 {
	r0 := (*get(o).(**build.Instance)).LookupImport(C.GoString(a0))
	return register(&r0)
}

//export f_build_0_NewContext
func f_build_0_NewContext(a0 uint32) uint32 {
	r0 := build.NewContext(*get(a0).(*[]build.Option)...)
	return register(&r0)
}

//export f_build_0_NewInstance
func f_build_0_NewInstance(o uint32, a0 *C.char, a1 uint32) uint32 {
	r0 := (*get(o).(**build.Context)).NewInstance(C.GoString(a0), *get(a1).(*build.LoadFunc))
	return register(&r0)
}

//export f_build_0_ReportError
func f_build_0_ReportError(o uint32, a0 uint32) {
	(*get(o).(**build.Instance)).ReportError(*get(a0).(*errors.Error))
}

//export f_parser_0_Error
func f_parser_0_Error(o uint32) *C.char {
	r0 := (*get(o).(**parser.DeprecationError)).Error()
	return C.CString(r0)
}

//export f_parser_0_FileOffset
func f_parser_0_FileOffset(a0 int) uint32 {
	r0 := parser.FileOffset(a0)
	return register(&r0)
}

//export f_parser_0_FromVersion
func f_parser_0_FromVersion(a0 int) uint32 {
	r0 := parser.FromVersion(a0)
	return register(&r0)
}

//export f_parser_0_ParseExpr
func f_parser_0_ParseExpr(a0 *C.char, a1 uint32, a2 uint32) (uint32, *C.char) {
	r0, r1 := parser.ParseExpr(C.GoString(a0), reflect.ValueOf(get(a1)).Elem().Interface(), *get(a2).(*[]parser.Option)...)
	return register(&r0), exportError(r1)
}

//export f_parser_0_ParseFile
func f_parser_0_ParseFile(a0 *C.char, a1 uint32, a2 uint32) (uint32, *C.char) {
	r0, r1 := parser.ParseFile(C.GoString(a0), reflect.ValueOf(get(a1)).Elem().Interface(), *get(a2).(*[]parser.Option)...)
	return register(&r0), exportError(r1)
}
func exportError(object error) *C.char {
	if object == nil {
		return nil
	}
	return C.CString(object.Error())
}

//export dereference
func dereference(object uint32) (uint32, bool) {
	tmp := reflect.ValueOf(get(object)).Elem()
	ty := tmp.Kind()
	if ty != reflect.Ptr && ty != reflect.Interface {
		return 0, false
	}
	return register(tmp.Interface()), true
}

//export isNil
func isNil(object uint32) bool {
	return reflect.ValueOf(get(object)).Elem().IsNil()
}

//export nilInterface
func nilInterface() uint32 {
	var tmp interface {
	}
	return register(&tmp)
}

//export cast
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

//export makeSlice
func makeSlice(object uint32, capacity int) uint32 {
	proxyMap.lock.RLock()
	ty := reflect.SliceOf(proxyMap.all[object].ty)
	proxyMap.lock.RUnlock()
	tmp := reflect.New(ty)
	tmp.Elem().Set(reflect.MakeSlice(ty, 0, capacity))
	return register(tmp.Interface())
}

//export sliceLen
func sliceLen(object uint32) int {
	return reflect.ValueOf(get(object)).Elem().Len()
}

//export getObjectInSlice
func getObjectInSlice(object uint32, index int) (uint32, bool) {
	tmp := reflect.ValueOf(get(object)).Elem()
	if index >= tmp.Len() {
		return 0, false
	}
	return register(tmp.Index(index).Addr().Interface()), true
}

//export push
func push(this uint32, object uint32) {
	proxyMap.lock.Lock()
	defer proxyMap.lock.Unlock()
	tmp := proxyMap.all[this]
	reflect.ValueOf(tmp.object).Elem().Set(reflect.Append(reflect.ValueOf(tmp.object).Elem(), reflect.ValueOf(proxyMap.all[object].object).Elem()))
	proxyMap.all[this] = tmp
}

//export makeMap
func makeMap(key uint32, value uint32, capacity int) uint32 {
	proxyMap.lock.RLock()
	ty := reflect.MapOf(proxyMap.all[key].ty, proxyMap.all[value].ty)
	proxyMap.lock.RUnlock()
	tmp := reflect.New(ty)
	tmp.Elem().Set(reflect.MakeMapWithSize(ty, capacity))
	return register(tmp.Interface())
}

//export insert
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

//export newPointer
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

//export forget
func forget(id uint32) {
	proxyMap.lock.Lock()
	defer proxyMap.lock.Unlock()
	delete(proxyMap.all, id)
}

//export proxyBool
func proxyBool(object bool) uint32 {
	return register(&object)
}

//export proxyFloat32
func proxyFloat32(object float32) uint32 {
	return register(&object)
}

//export proxyFloat64
func proxyFloat64(object float64) uint32 {
	return register(&object)
}

//export proxyInt16
func proxyInt16(object int16) uint32 {
	return register(&object)
}

//export proxyInt32
func proxyInt32(object int32) uint32 {
	return register(&object)
}

//export proxyInt64
func proxyInt64(object int64) uint32 {
	return register(&object)
}

//export proxyInt8
func proxyInt8(object int8) uint32 {
	return register(&object)
}

//export proxyUint16
func proxyUint16(object uint16) uint32 {
	return register(&object)
}

//export proxyUint32
func proxyUint32(object uint32) uint32 {
	return register(&object)
}

//export proxyUint64
func proxyUint64(object uint64) uint32 {
	return register(&object)
}

//export proxyUint8
func proxyUint8(object uint8) uint32 {
	return register(&object)
}

//export proxyString
func proxyString(object *C.char) uint32 {
	tmp := C.GoString(object)
	return register(&tmp)
}
func main() {
}
