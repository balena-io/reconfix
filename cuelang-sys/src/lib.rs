#![allow(warnings)]

use std::{convert::TryInto, marker::PhantomData, mem::ManuallyDrop};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use libc::c_char;
#[cfg(not(target_arch = "wasm32"))]
use mbox::MString;
#[cfg(not(target_arch = "wasm32"))]
use std::ffi::CString;

#[cfg(target_arch = "wasm32")]
pub type ffiString = String;
#[cfg(not(target_arch = "wasm32"))]
pub type ffiString = MString;

#[cfg(target_arch = "wasm32")]
fn import_bool(x: bool) -> bool {
    x
}
#[cfg(not(target_arch = "wasm32"))]
fn import_bool(x: go::GoUint8) -> bool {
    x != 0
}

#[cfg(target_arch = "wasm32")]
fn export_bool(x: bool) -> bool {
    x
}
#[cfg(not(target_arch = "wasm32"))]
fn export_bool(x: bool) -> go::GoUint8 {
    go::GoUint8::from(x)
}

#[cfg(target_arch = "wasm32")]
fn import_string(x: String) -> String {
    x
}
#[cfg(not(target_arch = "wasm32"))]
fn import_string(x: *mut c_char) -> MString {
    if x.is_null() {
        MString::from_str("")
    } else {
        unsafe { MString::from_raw(x).unwrap() }
    }
}

#[cfg(target_arch = "wasm32")]
fn export_string(x: &str) -> &str {
    x
}
#[cfg(not(target_arch = "wasm32"))]
fn export_string(x: &str) -> *mut c_char {
    CString::new(x).unwrap().into_raw() as *mut c_char
}

#[cfg(target_arch = "wasm32")]
mod go {
    use js_sys::Array;
    use wasm_bindgen::prelude::wasm_bindgen;

    pub type GoInt = isize;
    pub type GoInt8 = i8;
    pub type GoInt16 = i16;
    pub type GoInt32 = i32;
    pub type GoInt64 = i64;
    pub type GoUint = usize;
    pub type GoUint8 = u8;
    pub type GoUint16 = u16;
    pub type GoUint32 = u32;
    pub type GoUint64 = u64;
    pub type GoFloat32 = f32;
    pub type GoFloat64 = f64;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_Attribute() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_FieldInfo() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_FieldInfo_IsDefinition(this: u32) -> bool;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_FieldInfo_IsDefinition(this: u32, x: bool);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_FieldInfo_IsHidden(this: u32) -> bool;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_FieldInfo_IsHidden(this: u32, x: bool);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_FieldInfo_IsOptional(this: u32) -> bool;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_FieldInfo_IsOptional(this: u32, x: bool);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_FieldInfo_Name(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_FieldInfo_Name(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_FieldInfo_Pos(this: u32) -> GoInt;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_FieldInfo_Pos(this: u32, x: GoInt);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_FieldInfo_Selector(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_FieldInfo_Selector(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_FieldInfo_Value(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_FieldInfo_Value(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_Instance() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_Instance_Dir(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_Instance_Dir(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_Instance_DisplayName(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_Instance_DisplayName(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_Instance_Err(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_Instance_Err(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_Instance_ImportPath(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_Instance_ImportPath(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_Instance_Incomplete(this: u32) -> bool;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_Instance_Incomplete(this: u32, x: bool);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_cue_Instance_PkgName(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_cue_Instance_PkgName(this: u32, x: &str);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_Iterator() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_Kind() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_Op() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_Path() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_Runtime() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_Selector() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_Struct() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultcue_Value() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_Alias() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Alias_Equal(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Alias_Equal(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Alias_Expr(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Alias_Expr(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Alias_Ident(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Alias_Ident(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_Attribute() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Attribute_At(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Attribute_At(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Attribute_Text(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Attribute_Text(this: u32, x: &str);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_BadDecl() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BadDecl_From(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BadDecl_From(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BadDecl_To(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BadDecl_To(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_BadExpr() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BadExpr_From(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BadExpr_From(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BadExpr_To(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BadExpr_To(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_BasicLit() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BasicLit_Kind(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BasicLit_Kind(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BasicLit_Value(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BasicLit_Value(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BasicLit_ValuePos(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BasicLit_ValuePos(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_BinaryExpr() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BinaryExpr_Op(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BinaryExpr_Op(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BinaryExpr_OpPos(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BinaryExpr_OpPos(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BinaryExpr_X(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BinaryExpr_X(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BinaryExpr_Y(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BinaryExpr_Y(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_BottomLit() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_BottomLit_Bottom(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_BottomLit_Bottom(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_CallExpr() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_CallExpr_Args(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_CallExpr_Args(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_CallExpr_Fun(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_CallExpr_Fun(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_CallExpr_Lparen(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_CallExpr_Lparen(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_CallExpr_Rparen(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_CallExpr_Rparen(this: u32, x: GoUint32);
    }

    #[derive(Debug)]
    pub struct as_ast_Clause_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for as_ast_Clause_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn as_ast_Clause(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_Comment() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Comment_Slash(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Comment_Slash(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Comment_Text(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Comment_Text(this: u32, x: &str);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_CommentGroup() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_CommentGroup_Doc(this: u32) -> bool;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_CommentGroup_Doc(this: u32, x: bool);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_CommentGroup_Line(this: u32) -> bool;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_CommentGroup_Line(this: u32, x: bool);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_CommentGroup_List(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_CommentGroup_List(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_CommentGroup_Position(this: u32) -> GoInt8;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_CommentGroup_Position(this: u32, x: GoInt8);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_Comprehension() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Comprehension_Clauses(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Comprehension_Clauses(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Comprehension_Value(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Comprehension_Value(this: u32, x: GoUint32);
    }

    #[derive(Debug)]
    pub struct as_ast_Decl_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for as_ast_Decl_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn as_ast_Decl(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_Ellipsis() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Ellipsis_Ellipsis(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Ellipsis_Ellipsis(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Ellipsis_Type(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Ellipsis_Type(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_EmbedDecl() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_EmbedDecl_Expr(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_EmbedDecl_Expr(this: u32, x: GoUint32);
    }

    #[derive(Debug)]
    pub struct as_ast_Expr_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for as_ast_Expr_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn as_ast_Expr(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_Field() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Field_Attrs(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Field_Attrs(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Field_Label(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Field_Label(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Field_Optional(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Field_Optional(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Field_Token(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Field_Token(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Field_TokenPos(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Field_TokenPos(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Field_Value(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Field_Value(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_File() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_File_Decls(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_File_Decls(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_File_Filename(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_File_Filename(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_File_Imports(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_File_Imports(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_File_Unresolved(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_File_Unresolved(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_ForClause() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ForClause_Colon(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ForClause_Colon(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ForClause_For(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ForClause_For(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ForClause_In(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ForClause_In(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ForClause_Key(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ForClause_Key(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ForClause_Source(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ForClause_Source(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ForClause_Value(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ForClause_Value(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_Ident() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Ident_Name(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Ident_Name(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Ident_NamePos(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Ident_NamePos(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Ident_Node(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Ident_Node(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Ident_Scope(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Ident_Scope(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_IfClause() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_IfClause_Condition(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_IfClause_Condition(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_IfClause_If(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_IfClause_If(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_ImportDecl() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ImportDecl_Import(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ImportDecl_Import(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ImportDecl_Lparen(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ImportDecl_Lparen(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ImportDecl_Rparen(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ImportDecl_Rparen(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ImportDecl_Specs(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ImportDecl_Specs(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_ImportSpec() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ImportSpec_EndPos(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ImportSpec_EndPos(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ImportSpec_Name(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ImportSpec_Name(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ImportSpec_Path(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ImportSpec_Path(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_IndexExpr() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_IndexExpr_Index(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_IndexExpr_Index(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_IndexExpr_Lbrack(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_IndexExpr_Lbrack(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_IndexExpr_Rbrack(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_IndexExpr_Rbrack(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_IndexExpr_X(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_IndexExpr_X(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_Interpolation() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Interpolation_Elts(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Interpolation_Elts(this: u32, x: GoUint32);
    }

    #[derive(Debug)]
    pub struct as_ast_Label_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for as_ast_Label_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn as_ast_Label(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_LetClause() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_LetClause_Equal(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_LetClause_Equal(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_LetClause_Expr(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_LetClause_Expr(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_LetClause_Ident(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_LetClause_Ident(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_LetClause_Let(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_LetClause_Let(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_ListComprehension() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ListComprehension_Clauses(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ListComprehension_Clauses(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ListComprehension_Expr(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ListComprehension_Expr(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ListComprehension_Lbrack(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ListComprehension_Lbrack(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ListComprehension_Rbrack(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ListComprehension_Rbrack(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_ListLit() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ListLit_Elts(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ListLit_Elts(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ListLit_Lbrack(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ListLit_Lbrack(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ListLit_Rbrack(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ListLit_Rbrack(this: u32, x: GoUint32);
    }

    #[derive(Debug)]
    pub struct as_ast_Node_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for as_ast_Node_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn as_ast_Node(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_Package() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Package_Name(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Package_Name(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_Package_PackagePos(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_Package_PackagePos(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_ParenExpr() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ParenExpr_Lparen(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ParenExpr_Lparen(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ParenExpr_Rparen(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ParenExpr_Rparen(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_ParenExpr_X(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_ParenExpr_X(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_SelectorExpr() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_SelectorExpr_Sel(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_SelectorExpr_Sel(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_SelectorExpr_X(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_SelectorExpr_X(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_SliceExpr() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_SliceExpr_High(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_SliceExpr_High(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_SliceExpr_Lbrack(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_SliceExpr_Lbrack(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_SliceExpr_Low(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_SliceExpr_Low(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_SliceExpr_Rbrack(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_SliceExpr_Rbrack(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_SliceExpr_X(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_SliceExpr_X(this: u32, x: GoUint32);
    }

    #[derive(Debug)]
    pub struct as_ast_Spec_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for as_ast_Spec_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn as_ast_Spec(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_StructLit() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_StructLit_Elts(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_StructLit_Elts(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_StructLit_Lbrace(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_StructLit_Lbrace(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_StructLit_Rbrace(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_StructLit_Rbrace(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_TemplateLabel() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_TemplateLabel_Ident(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_TemplateLabel_Ident(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_TemplateLabel_Langle(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_TemplateLabel_Langle(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_TemplateLabel_Rangle(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_TemplateLabel_Rangle(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultast_UnaryExpr() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_UnaryExpr_Op(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_UnaryExpr_Op(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_UnaryExpr_OpPos(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_UnaryExpr_OpPos(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_ast_UnaryExpr_X(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_ast_UnaryExpr_X(this: u32, x: GoUint32);
    }

    #[derive(Debug)]
    pub struct as_astutil_Cursor_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for as_astutil_Cursor_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn as_astutil_Cursor(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultastutil_ImportInfo() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_astutil_ImportInfo_Dir(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_astutil_ImportInfo_Dir(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_astutil_ImportInfo_ID(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_astutil_ImportInfo_ID(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_astutil_ImportInfo_Ident(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_astutil_ImportInfo_Ident(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_astutil_ImportInfo_PkgName(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_astutil_ImportInfo_PkgName(this: u32, x: &str);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultbuild_Context() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultbuild_Encoding() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultbuild_File() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_File_Encoding(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_File_Encoding(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_File_Filename(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_File_Filename(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_File_Form(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_File_Form(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_File_Interpretation(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_File_Interpretation(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_File_Source(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_File_Source(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_File_Tags(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_File_Tags(this: u32, x: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultbuild_Form() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultbuild_Instance() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_AllTags(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_AllTags(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_BuildFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_BuildFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_CUEFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_CUEFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_DataFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_DataFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Deps(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Deps(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_DepsErrors(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_DepsErrors(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Dir(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Dir(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_DisplayPath(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_DisplayPath(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Err(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Err(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Files(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Files(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_IgnoredCUEFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_IgnoredCUEFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_IgnoredFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_IgnoredFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_ImportComment(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_ImportComment(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_ImportPath(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_ImportPath(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_ImportPaths(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_ImportPaths(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_ImportPos(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_ImportPos(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Imports(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Imports(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Incomplete(this: u32) -> bool;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Incomplete(this: u32, x: bool);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_InvalidCUEFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_InvalidCUEFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_InvalidFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_InvalidFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Match(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Match(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Module(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Module(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_OrphanedFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_OrphanedFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_PkgName(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_PkgName(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Root(this: u32) -> String;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Root(this: u32, x: &str);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Scope(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Scope(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_Standard(this: u32) -> bool;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_Standard(this: u32, x: bool);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_TestCUEFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_TestCUEFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_ToolCUEFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_ToolCUEFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_UnknownFiles(this: u32) -> GoUint32;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_UnknownFiles(this: u32, x: GoUint32);
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_build_Instance_User(this: u32) -> bool;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_build_Instance_User(this: u32, x: bool);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultbuild_Interpretation() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn defaultparser_DeprecationError() -> GoUint32;
    }
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn get_parser_DeprecationError_Version(this: u32) -> GoInt;

        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn set_parser_DeprecationError_Version(this: u32, x: GoInt);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_All() -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_AppendFloat_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_AppendFloat_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_AppendFloat(
            _: GoUint32,
            _: GoUint32,
            _: GoUint8,
            _: GoInt,
        ) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_AppendInt_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_AppendInt_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_AppendInt(_: GoUint32, _: GoUint32, _: GoInt) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Attribute(_: GoUint32, _: &str) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Attributes(_: bool) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Bool_return {
        pub r0: bool,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Bool_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Bool(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Build(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Build(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_2_Build_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_2_Build_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_2_Build(_: GoUint32, _: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Bytes_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Bytes_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Bytes(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_CanString(_: GoUint32) -> bool;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Compile_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Compile_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Compile(_: GoUint32, _: &str, _: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_CompileExpr_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_CompileExpr_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_CompileExpr(_: GoUint32, _: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_CompileFile_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_CompileFile_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_CompileFile(_: GoUint32, _: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Concrete(_: bool) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Decimal_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Decimal_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Decimal(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Decode(_: GoUint32, _: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Def(_: &str) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Default_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for f_cue_0_Default_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Default(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Definitions(_: bool) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Dereference(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_DisallowCycles(_: bool) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Doc(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Doc(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Docs(_: bool) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Elem_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for f_cue_0_Elem_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Elem(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Equals(_: GoUint32, _: GoUint32) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Err(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Err(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_2_Err(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Eval(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Eval(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Exists(_: GoUint32) -> bool;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Expr_return {
        pub r0: GoUint32,
        pub r1: GoUint32,
    }

    impl From<Array> for f_cue_0_Expr_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Expr(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Field(_: GoUint32, _: GoInt) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_FieldByName_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_FieldByName_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_FieldByName(_: GoUint32, _: &str, _: bool) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_1_FieldByName_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_1_FieldByName_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_FieldByName(_: GoUint32, _: &str, _: bool) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Fields(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_1_Fields_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_1_Fields_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Fields(_: GoUint32, _: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Fill_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Fill_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Fill(_: GoUint32, _: GoUint32, _: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Fill(_: GoUint32, _: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Final() -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Flag_return {
        pub r0: bool,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Flag_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Flag(_: GoUint32, _: GoInt, _: &str) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Float64_return {
        pub r0: GoFloat64,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Float64_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Float64(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Format(_: GoUint32, _: GoUint32, _: GoInt32);
    }

    #[derive(Debug)]
    pub struct f_cue_0_FromExpr_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_FromExpr_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_FromExpr(_: GoUint32, _: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Hidden(_: bool) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_ID(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_IncompleteKind(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Index(_: GoInt) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Int_return {
        pub r0: GoInt64,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Int_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Int(_: GoUint32, _: GoInt) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_1_Int_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_1_Int_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Int(_: GoUint32, _: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Int64_return {
        pub r0: GoInt64,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Int64_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Int64(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_IsAnyOf(_: GoUint32, _: GoUint32) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_IsClosed(_: GoUint32) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_IsConcrete(_: GoUint32) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_IsDefinition(_: GoUint32) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_IsHidden(_: GoUint32) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_IsOptional(_: GoUint32) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Kind(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Label(_: GoUint32) -> String;
    }

    #[derive(Debug)]
    pub struct f_cue_1_Label_return {
        pub r0: String,
        pub r1: bool,
    }

    impl From<Array> for f_cue_1_Label_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Label(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Len(_: GoUint32) -> GoInt;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Len(_: GoUint32) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_List_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_List_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_List(_: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Lookup_return {
        pub r0: String,
        pub r1: bool,
        pub r2: String,
    }

    impl From<Array> for f_cue_0_Lookup_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
                r2: serde_wasm_bindgen::from_value(x.get(2)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Lookup(_: GoUint32, _: GoInt, _: &str) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Lookup(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_2_Lookup(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_LookupDef(_: GoUint32, _: &str) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_LookupDef(_: GoUint32, _: &str) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_LookupField_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_LookupField_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_LookupField(_: GoUint32, _: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_1_LookupField_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_1_LookupField_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_LookupField(_: GoUint32, _: &str) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_LookupPath(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_MakePath(_: GoUint32) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_MantExp_return {
        pub r0: GoInt,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_MantExp_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_MantExp(_: GoUint32, _: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Marshal_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Marshal_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Marshal(_: GoUint32, _: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_MarshalJSON_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_MarshalJSON_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_MarshalJSON(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Merge(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Next(_: GoUint32) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Null(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Optional(_: bool) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Parse_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Parse_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Parse(_: GoUint32, _: &str, _: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_ParsePath(_: &str) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Path(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Raw() -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Reader_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Reader_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Reader(_: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Reference_return {
        pub r0: GoUint32,
        pub r1: GoUint32,
    }

    impl From<Array> for f_cue_0_Reference_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Reference(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_ResolveReferences(_: bool) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Schema() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Selectors(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Source(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Split(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Str(_: &str) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_String_return {
        pub r0: String,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_String_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_String(_: GoUint32, _: GoInt) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_String(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_2_String(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_3_String(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_4_String(_: GoUint32) -> String;
    }

    #[derive(Debug)]
    pub struct f_cue_5_String_return {
        pub r0: String,
        pub r1: String,
    }

    impl From<Array> for f_cue_5_String_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_5_String(_: GoUint32) -> Array;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Struct_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Struct_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Struct(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Subsume(_: GoUint32, _: GoUint32, _: GoUint32)
            -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Subsumes(_: GoUint32, _: GoUint32) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Syntax(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Token(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_TypeString(_: GoUint32) -> String;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Uint64_return {
        pub r0: GoUint64,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Uint64_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Uint64(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Unify(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_UnifyAccept(
            _: GoUint32,
            _: GoUint32,
            _: GoUint32,
        ) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_cue_0_Unmarshal_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_cue_0_Unmarshal_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Unmarshal(_: GoUint32, _: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Validate(_: GoUint32, _: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_0_Value(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_cue_1_Value(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_AddComment(_: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_1_AddComment(_: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_2_AddComment(_: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_Comments(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_1_Comments(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_2_Comments(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_Embed(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_1_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_2_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_3_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_4_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_5_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_6_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_7_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_8_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_9_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_10_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_11_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_12_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_13_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_14_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_15_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_16_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_17_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_18_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_19_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_20_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_21_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_22_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_23_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_24_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_25_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_26_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_27_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_28_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_29_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_30_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_31_End(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_IsValidIdent(_: &str) -> bool;
    }

    #[derive(Debug)]
    pub struct f_ast_0_LabelName_return {
        pub r0: String,
        pub r1: bool,
        pub r2: String,
    }

    impl From<Array> for f_ast_0_LabelName_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
                r2: serde_wasm_bindgen::from_value(x.get(2)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_LabelName(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_Name(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewBinExpr(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewBool(_: bool) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewCall(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewIdent(_: &str) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewImport(_: GoUint32, _: &str) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewList(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewLit(_: GoUint32, _: &str) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewNull() -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewSel(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewString(_: &str) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_NewStruct(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_PackageName(_: GoUint32) -> String;
    }

    #[derive(Debug)]
    pub struct f_ast_0_ParseIdent_return {
        pub r0: String,
        pub r1: String,
    }

    impl From<Array> for f_ast_0_ParseIdent_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_ParseIdent(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_1_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_2_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_3_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_4_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_5_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_6_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_7_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_8_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_9_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_10_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_11_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_12_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_13_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_14_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_15_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_16_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_17_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_18_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_19_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_20_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_21_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_22_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_23_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_24_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_25_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_26_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_27_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_28_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_29_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_30_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_31_Pos(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_Preamble(_: GoUint32) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_ast_0_QuoteIdent_return {
        pub r0: String,
        pub r1: String,
    }

    impl From<Array> for f_ast_0_QuoteIdent_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_QuoteIdent(_: &str) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_SetComments(_: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_SetPos(_: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_SetRelPos(_: GoUint32, _: GoUint32);
    }

    #[derive(Debug)]
    pub struct f_ast_0_Split_return {
        pub r0: String,
        pub r1: String,
    }

    impl From<Array> for f_ast_0_Split_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_Split(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_String(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_ast_0_Text(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_astutil_0_ApplyRecursively(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_astutil_0_CopyComments(_: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_astutil_0_CopyMeta(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_astutil_0_CopyPosition(_: GoUint32, _: GoUint32);
    }

    #[derive(Debug)]
    pub struct f_astutil_0_ParseImportSpec_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_astutil_0_ParseImportSpec_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_astutil_0_ParseImportSpec(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_astutil_0_Resolve(_: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_astutil_0_ResolveExpr(_: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_astutil_0_Sanitize(_: GoUint32) -> String;
    }

    #[derive(Debug)]
    pub struct f_astutil_0_ToFile_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_astutil_0_ToFile_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_astutil_0_ToFile(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_Abs(_: GoUint32, _: &str) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_AddFile(_: GoUint32, _: &str, _: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_AddSyntax(_: GoUint32, _: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_Complete(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_Context(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_Dependencies(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_ID(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_IsLocalImport(_: &str) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_Loader(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_LookupImport(_: GoUint32, _: &str) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_NewContext(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_NewInstance(
            _: GoUint32,
            _: &str,
            _: GoUint32,
        ) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_build_0_ReportError(_: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_parser_0_Error(_: GoUint32) -> String;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_parser_0_FileOffset(_: GoInt) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_parser_0_FromVersion(_: GoInt) -> GoUint32;
    }

    #[derive(Debug)]
    pub struct f_parser_0_ParseExpr_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_parser_0_ParseExpr_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_parser_0_ParseExpr(_: &str, _: GoUint32, _: GoUint32)
            -> Array;
    }

    #[derive(Debug)]
    pub struct f_parser_0_ParseFile_return {
        pub r0: GoUint32,
        pub r1: String,
    }

    impl From<Array> for f_parser_0_ParseFile_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn f_parser_0_ParseFile(_: &str, _: GoUint32, _: GoUint32)
            -> Array;
    }

    #[derive(Debug)]
    pub struct dereference_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for dereference_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn dereference(_: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn isNil(_: GoUint32) -> bool;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn nilInterface() -> GoUint32;
    }

    #[derive(Debug)]
    pub struct cast_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for cast_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn cast(_: GoUint32, _: GoUint32) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn makeSlice(_: GoUint32, _: GoInt) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn sliceLen(_: GoUint32) -> GoInt;
    }

    #[derive(Debug)]
    pub struct getObjectInSlice_return {
        pub r0: GoUint32,
        pub r1: bool,
    }

    impl From<Array> for getObjectInSlice_return {
        fn from(x: Array) -> Self {
            Self {
                r0: serde_wasm_bindgen::from_value(x.get(0)).unwrap(),
                r1: serde_wasm_bindgen::from_value(x.get(1)).unwrap(),
            }
        }
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn getObjectInSlice(_: GoUint32, _: GoInt) -> Array;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn push(_: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn makeMap(_: GoUint32, _: GoUint32, _: GoInt) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn insert(_: GoUint32, _: GoUint32, _: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn newPointer(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn forget(_: GoUint32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyBool(_: bool) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyFloat32(_: GoFloat32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyFloat64(_: GoFloat64) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyInt16(_: GoInt16) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyInt32(_: GoInt32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyInt64(_: GoInt64) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyInt8(_: GoInt8) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyString(_: &str) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyUint16(_: GoUint16) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyUint32(_: GoUint32) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyUint64(_: GoUint64) -> GoUint32;
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["Go[\"cuelang-sys\"]"])]
        pub fn proxyUint8(_: GoUint8) -> GoUint32;
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod go {
    include!(concat!(env!("OUT_DIR"), "/go.rs"));
}

pub trait GoObject {
    unsafe fn from_handle(handle: go::GoUint32) -> Self;
    fn handle(&self) -> go::GoUint32;

    fn cast<T>(&self) -> Option<T>
    where
        T: Default + GoObject,
    {
        self.cast_as(&T::default())
    }

    fn cast_as<T>(&self, example: &T) -> Option<T>
    where
        T: GoObject,
    {
        let go::cast_return { r0, r1 } =
            unsafe { go::cast(self.handle(), example.handle()).into() };

        if import_bool(r1) {
            Some(unsafe { T::from_handle(r0) })
        } else {
            None
        }
    }
}

trait NotGoAny: GoObject {}

#[derive(Debug)]
pub struct GoAny(go::GoUint32);

impl GoAny {
    pub fn from_str(x: &str) -> Self {
        Self(unsafe { go::proxyString(export_string(x)) })
    }

    pub fn nil_interface() -> Self {
        Self(unsafe { go::nilInterface() })
    }
}

impl Default for GoAny {
    fn default() -> Self {
        Self::nil_interface()
    }
}

impl GoObject for GoAny {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl From<bool> for GoAny {
    fn from(x: bool) -> Self {
        Self(unsafe { go::proxyBool(export_bool(x)) })
    }
}

impl From<go::GoInt8> for GoAny {
    fn from(x: go::GoInt8) -> Self {
        Self(unsafe { go::proxyInt8(x) })
    }
}

impl From<go::GoInt16> for GoAny {
    fn from(x: go::GoInt16) -> Self {
        Self(unsafe { go::proxyInt16(x) })
    }
}

impl From<go::GoInt32> for GoAny {
    fn from(x: go::GoInt32) -> Self {
        Self(unsafe { go::proxyInt32(x) })
    }
}

impl From<go::GoInt64> for GoAny {
    fn from(x: go::GoInt64) -> Self {
        Self(unsafe { go::proxyInt64(x) })
    }
}

impl From<go::GoUint8> for GoAny {
    fn from(x: go::GoUint8) -> Self {
        Self(unsafe { go::proxyUint8(x) })
    }
}

impl From<go::GoUint16> for GoAny {
    fn from(x: go::GoUint16) -> Self {
        Self(unsafe { go::proxyUint16(x) })
    }
}

impl From<go::GoUint32> for GoAny {
    fn from(x: go::GoUint32) -> Self {
        Self(unsafe { go::proxyUint32(x) })
    }
}

impl From<go::GoUint64> for GoAny {
    fn from(x: go::GoUint64) -> Self {
        Self(unsafe { go::proxyUint64(x) })
    }
}

impl From<go::GoFloat32> for GoAny {
    fn from(x: go::GoFloat32) -> Self {
        Self(unsafe { go::proxyFloat32(x) })
    }
}

impl From<go::GoFloat64> for GoAny {
    fn from(x: go::GoFloat64) -> Self {
        Self(unsafe { go::proxyFloat64(x) })
    }
}

impl From<String> for GoAny {
    fn from(x: String) -> Self {
        Self::from_str(&x)
    }
}

impl<'a> From<&'a str> for GoAny {
    fn from(x: &'a str) -> Self {
        Self::from_str(x)
    }
}

impl<T> From<T> for GoAny
where
    T: NotGoAny,
{
    fn from(x: T) -> Self {
        Self(ManuallyDrop::new(x).handle())
    }
}

impl Drop for GoAny {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

#[derive(Debug)]
pub struct GoPtr<T> {
    handle: go::GoUint32,
    phantom: PhantomData<T>,
}

impl<T> GoPtr<T>
where
    T: GoObject,
{
    pub fn new<R>(obj: R) -> Self
    where
        R: Into<T>,
    {
        Self {
            handle: unsafe { go::newPointer(obj.into().handle()) },
            phantom: PhantomData,
        }
    }

    pub fn is_nil(&self) -> bool {
        import_bool(unsafe { go::isNil(self.handle) })
    }

    pub fn dereference(&self) -> T {
        let go::dereference_return { r0, r1 } =
            unsafe { go::dereference(self.handle) }.into();
        assert!(import_bool(r1));

        unsafe { T::from_handle(r0) }
    }
}

impl<T> Default for GoPtr<T>
where
    T: Default + GoObject,
{
    fn default() -> Self {
        GoPtr::new(T::default())
    }
}

impl<T> GoObject for GoPtr<T> {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }

    fn handle(&self) -> go::GoUint32 {
        self.handle
    }
}

impl<T> NotGoAny for GoPtr<T> {}

impl<T> Drop for GoPtr<T> {
    fn drop(&mut self) {
        unsafe { go::forget(self.handle) };
    }
}

#[derive(Debug)]
pub struct GoSlice<T> {
    handle: go::GoUint32,
    phantom: PhantomData<T>,
}

impl<T> GoSlice<T>
where
    T: Default + Into<GoAny>,
{
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::make(T::default(), capacity)
    }
}

impl<T> GoSlice<T>
where
    T: Into<GoAny>,
{
    pub fn make(typeExample: T, capacity: usize) -> Self {
        Self {
            handle: unsafe {
                go::makeSlice(
                    typeExample.into().handle(),
                    capacity.try_into().unwrap(),
                )
            },
            phantom: PhantomData,
        }
    }

    pub fn push(&mut self, x: T) {
        unsafe { go::push(self.handle, x.into().handle()) }
    }
}

impl<T> GoSlice<T>
where
    T: GoObject,
{
    pub fn get(&self, index: usize) -> Option<T> {
        let go::getObjectInSlice_return { r0, r1 } = unsafe {
            go::getObjectInSlice(self.handle, index.try_into().unwrap())
        }
        .into();

        if import_bool(r1) {
            Some(unsafe { T::from_handle(r0) })
        } else {
            None
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = T> + 'a {
        (0..)
            .into_iter()
            .map(move |idx| self.get(idx))
            .take_while(|x| x.is_some())
            .map(|x| x.unwrap())
    }
}

impl<T> GoSlice<T> {
    pub fn len(&self) -> usize {
        unsafe { go::sliceLen(self.handle) }.try_into().unwrap()
    }
}

impl GoSlice<String> {
    pub fn push_str(&mut self, x: &str) {
        unsafe { go::push(self.handle, GoAny::from_str(x).handle()) }
    }
}

impl<T> IntoIterator for GoSlice<T>
where
    T: GoObject,
{
    type Item = T;
    type IntoIter = GoSliceIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<T> GoObject for GoSlice<T> {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }

    fn handle(&self) -> go::GoUint32 {
        self.handle
    }
}

impl<T> NotGoAny for GoSlice<T> {}

impl<T> Drop for GoSlice<T> {
    fn drop(&mut self) {
        unsafe { go::forget(self.handle) };
    }
}

pub struct GoSliceIntoIter<T> {
    slice: GoSlice<T>,
    idx: usize,
}

impl<T> GoSliceIntoIter<T> {
    pub fn new(slice: GoSlice<T>) -> Self {
        Self { slice, idx: 0 }
    }
}

impl<T> Iterator for GoSliceIntoIter<T>
where
    T: GoObject,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.slice.get(self.idx) {
            x @ Some(_) => {
                self.idx += 1;

                x
            }
            None => None,
        }
    }
}

pub struct GoMap<K, V> {
    handle: go::GoUint32,
    phantom: PhantomData<(K, V)>,
}

impl<K, V> GoMap<K, V>
where
    K: Default + Into<GoAny>,
    V: Default + Into<GoAny>,
{
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::make(K::default(), V::default(), capacity)
    }
}

impl<K, V> GoMap<K, V>
where
    K: Into<GoAny>,
    V: Into<GoAny>,
{
    pub fn make(keyExample: K, valueExample: V, capacity: usize) -> Self {
        Self {
            handle: unsafe {
                go::makeMap(
                    keyExample.into().handle(),
                    valueExample.into().handle(),
                    capacity.try_into().unwrap(),
                )
            },
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        unsafe {
            go::insert(self.handle, key.into().handle(), value.into().handle())
        }
    }
}

impl<K, V> GoObject for GoMap<K, V> {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }

    fn handle(&self) -> go::GoUint32 {
        self.handle
    }
}

impl<K, V> NotGoAny for GoMap<K, V> {}

impl<K, V> Drop for GoMap<K, V> {
    fn drop(&mut self) {
        unsafe { go::forget(self.handle) };
    }
}

#[derive(Debug)]
pub struct cue_Attribute(go::GoUint32);

impl GoObject for cue_Attribute {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Attribute {}

impl Drop for cue_Attribute {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for cue_Attribute {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_Attribute()) }
    }
}

#[derive(Debug)]
pub struct cue_FieldInfo(go::GoUint32);

impl GoObject for cue_FieldInfo {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_FieldInfo {}

impl Drop for cue_FieldInfo {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl cue_FieldInfo {
    pub fn getIsDefinition(&self) -> bool {
        let r = unsafe { go::get_cue_FieldInfo_IsDefinition(self.handle()) };
        import_bool(r)
    }

    pub fn setIsDefinition(&mut self, x: bool) {
        unsafe {
            go::set_cue_FieldInfo_IsDefinition(self.handle(), export_bool(x))
        };
    }
}

impl GoPtr<cue_FieldInfo> {
    pub fn getIsDefinition(&self) -> bool {
        let r = unsafe {
            go::get_cue_FieldInfo_IsDefinition(self.dereference().handle())
        };
        import_bool(r)
    }

    pub fn setIsDefinition(&mut self, x: bool) {
        unsafe {
            go::set_cue_FieldInfo_IsDefinition(
                self.dereference().handle(),
                export_bool(x),
            )
        };
    }
}

impl cue_FieldInfo {
    pub fn getIsHidden(&self) -> bool {
        let r = unsafe { go::get_cue_FieldInfo_IsHidden(self.handle()) };
        import_bool(r)
    }

    pub fn setIsHidden(&mut self, x: bool) {
        unsafe {
            go::set_cue_FieldInfo_IsHidden(self.handle(), export_bool(x))
        };
    }
}

impl GoPtr<cue_FieldInfo> {
    pub fn getIsHidden(&self) -> bool {
        let r = unsafe {
            go::get_cue_FieldInfo_IsHidden(self.dereference().handle())
        };
        import_bool(r)
    }

    pub fn setIsHidden(&mut self, x: bool) {
        unsafe {
            go::set_cue_FieldInfo_IsHidden(
                self.dereference().handle(),
                export_bool(x),
            )
        };
    }
}

impl cue_FieldInfo {
    pub fn getIsOptional(&self) -> bool {
        let r = unsafe { go::get_cue_FieldInfo_IsOptional(self.handle()) };
        import_bool(r)
    }

    pub fn setIsOptional(&mut self, x: bool) {
        unsafe {
            go::set_cue_FieldInfo_IsOptional(self.handle(), export_bool(x))
        };
    }
}

impl GoPtr<cue_FieldInfo> {
    pub fn getIsOptional(&self) -> bool {
        let r = unsafe {
            go::get_cue_FieldInfo_IsOptional(self.dereference().handle())
        };
        import_bool(r)
    }

    pub fn setIsOptional(&mut self, x: bool) {
        unsafe {
            go::set_cue_FieldInfo_IsOptional(
                self.dereference().handle(),
                export_bool(x),
            )
        };
    }
}

impl cue_FieldInfo {
    pub fn getName(&self) -> ffiString {
        let r = unsafe { go::get_cue_FieldInfo_Name(self.handle()) };
        import_string(r)
    }

    pub fn setName(&mut self, x: &str) {
        unsafe { go::set_cue_FieldInfo_Name(self.handle(), export_string(x)) };
    }
}

impl GoPtr<cue_FieldInfo> {
    pub fn getName(&self) -> ffiString {
        let r =
            unsafe { go::get_cue_FieldInfo_Name(self.dereference().handle()) };
        import_string(r)
    }

    pub fn setName(&mut self, x: &str) {
        unsafe {
            go::set_cue_FieldInfo_Name(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl cue_FieldInfo {
    pub fn getPos(&self) -> go::GoInt {
        let r = unsafe { go::get_cue_FieldInfo_Pos(self.handle()) };
        r
    }

    pub fn setPos(&mut self, x: go::GoInt) {
        unsafe { go::set_cue_FieldInfo_Pos(self.handle(), x) };
    }
}

impl GoPtr<cue_FieldInfo> {
    pub fn getPos(&self) -> go::GoInt {
        let r =
            unsafe { go::get_cue_FieldInfo_Pos(self.dereference().handle()) };
        r
    }

    pub fn setPos(&mut self, x: go::GoInt) {
        unsafe { go::set_cue_FieldInfo_Pos(self.dereference().handle(), x) };
    }
}

impl cue_FieldInfo {
    pub fn getSelector(&self) -> ffiString {
        let r = unsafe { go::get_cue_FieldInfo_Selector(self.handle()) };
        import_string(r)
    }

    pub fn setSelector(&mut self, x: &str) {
        unsafe {
            go::set_cue_FieldInfo_Selector(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<cue_FieldInfo> {
    pub fn getSelector(&self) -> ffiString {
        let r = unsafe {
            go::get_cue_FieldInfo_Selector(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setSelector(&mut self, x: &str) {
        unsafe {
            go::set_cue_FieldInfo_Selector(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl cue_FieldInfo {
    pub fn getValue(&self) -> cue_Value {
        let r = unsafe { go::get_cue_FieldInfo_Value(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setValue(&mut self, x: cue_Value) {
        unsafe { go::set_cue_FieldInfo_Value(self.handle(), x.handle()) };
    }
}

impl GoPtr<cue_FieldInfo> {
    pub fn getValue(&self) -> cue_Value {
        let r =
            unsafe { go::get_cue_FieldInfo_Value(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setValue(&mut self, x: cue_Value) {
        unsafe {
            go::set_cue_FieldInfo_Value(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for cue_FieldInfo {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_FieldInfo()) }
    }
}

#[derive(Debug)]
pub struct cue_Instance(go::GoUint32);

impl GoObject for cue_Instance {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Instance {}

impl Drop for cue_Instance {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl cue_Instance {
    pub fn getDir(&self) -> ffiString {
        let r = unsafe { go::get_cue_Instance_Dir(self.handle()) };
        import_string(r)
    }

    pub fn setDir(&mut self, x: &str) {
        unsafe { go::set_cue_Instance_Dir(self.handle(), export_string(x)) };
    }
}

impl GoPtr<cue_Instance> {
    pub fn getDir(&self) -> ffiString {
        let r =
            unsafe { go::get_cue_Instance_Dir(self.dereference().handle()) };
        import_string(r)
    }

    pub fn setDir(&mut self, x: &str) {
        unsafe {
            go::set_cue_Instance_Dir(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl cue_Instance {
    pub fn getDisplayName(&self) -> ffiString {
        let r = unsafe { go::get_cue_Instance_DisplayName(self.handle()) };
        import_string(r)
    }

    pub fn setDisplayName(&mut self, x: &str) {
        unsafe {
            go::set_cue_Instance_DisplayName(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<cue_Instance> {
    pub fn getDisplayName(&self) -> ffiString {
        let r = unsafe {
            go::get_cue_Instance_DisplayName(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setDisplayName(&mut self, x: &str) {
        unsafe {
            go::set_cue_Instance_DisplayName(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl cue_Instance {
    pub fn getErr(&self) -> errors_Error {
        let r = unsafe { go::get_cue_Instance_Err(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setErr(&mut self, x: errors_Error) {
        unsafe { go::set_cue_Instance_Err(self.handle(), x.handle()) };
    }
}

impl GoPtr<cue_Instance> {
    pub fn getErr(&self) -> errors_Error {
        let r =
            unsafe { go::get_cue_Instance_Err(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setErr(&mut self, x: errors_Error) {
        unsafe {
            go::set_cue_Instance_Err(self.dereference().handle(), x.handle())
        };
    }
}

impl cue_Instance {
    pub fn getImportPath(&self) -> ffiString {
        let r = unsafe { go::get_cue_Instance_ImportPath(self.handle()) };
        import_string(r)
    }

    pub fn setImportPath(&mut self, x: &str) {
        unsafe {
            go::set_cue_Instance_ImportPath(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<cue_Instance> {
    pub fn getImportPath(&self) -> ffiString {
        let r = unsafe {
            go::get_cue_Instance_ImportPath(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setImportPath(&mut self, x: &str) {
        unsafe {
            go::set_cue_Instance_ImportPath(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl cue_Instance {
    pub fn getIncomplete(&self) -> bool {
        let r = unsafe { go::get_cue_Instance_Incomplete(self.handle()) };
        import_bool(r)
    }

    pub fn setIncomplete(&mut self, x: bool) {
        unsafe {
            go::set_cue_Instance_Incomplete(self.handle(), export_bool(x))
        };
    }
}

impl GoPtr<cue_Instance> {
    pub fn getIncomplete(&self) -> bool {
        let r = unsafe {
            go::get_cue_Instance_Incomplete(self.dereference().handle())
        };
        import_bool(r)
    }

    pub fn setIncomplete(&mut self, x: bool) {
        unsafe {
            go::set_cue_Instance_Incomplete(
                self.dereference().handle(),
                export_bool(x),
            )
        };
    }
}

impl cue_Instance {
    pub fn getPkgName(&self) -> ffiString {
        let r = unsafe { go::get_cue_Instance_PkgName(self.handle()) };
        import_string(r)
    }

    pub fn setPkgName(&mut self, x: &str) {
        unsafe {
            go::set_cue_Instance_PkgName(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<cue_Instance> {
    pub fn getPkgName(&self) -> ffiString {
        let r = unsafe {
            go::get_cue_Instance_PkgName(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setPkgName(&mut self, x: &str) {
        unsafe {
            go::set_cue_Instance_PkgName(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl Default for cue_Instance {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_Instance()) }
    }
}

#[derive(Debug)]
pub struct cue_Iterator(go::GoUint32);

impl GoObject for cue_Iterator {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Iterator {}

impl Drop for cue_Iterator {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for cue_Iterator {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_Iterator()) }
    }
}

#[derive(Debug)]
pub struct cue_Kind(go::GoUint32);

impl GoObject for cue_Kind {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Kind {}

impl Drop for cue_Kind {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for cue_Kind {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_Kind()) }
    }
}

#[derive(Debug)]
pub struct cue_Op(go::GoUint32);

impl GoObject for cue_Op {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Op {}

impl Drop for cue_Op {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for cue_Op {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_Op()) }
    }
}

#[derive(Debug)]
pub struct cue_Path(go::GoUint32);

impl GoObject for cue_Path {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Path {}

impl Drop for cue_Path {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for cue_Path {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_Path()) }
    }
}

#[derive(Debug)]
pub struct cue_Runtime(go::GoUint32);

impl GoObject for cue_Runtime {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Runtime {}

impl Drop for cue_Runtime {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for cue_Runtime {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_Runtime()) }
    }
}

#[derive(Debug)]
pub struct cue_Selector(go::GoUint32);

impl GoObject for cue_Selector {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Selector {}

impl Drop for cue_Selector {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for cue_Selector {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_Selector()) }
    }
}

#[derive(Debug)]
pub struct cue_Struct(go::GoUint32);

impl GoObject for cue_Struct {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Struct {}

impl Drop for cue_Struct {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for cue_Struct {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_Struct()) }
    }
}

#[derive(Debug)]
pub struct cue_Value(go::GoUint32);

impl GoObject for cue_Value {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Value {}

impl Drop for cue_Value {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for cue_Value {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultcue_Value()) }
    }
}

#[derive(Debug)]
pub struct ast_Alias(go::GoUint32);

impl GoObject for ast_Alias {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Alias {}

impl Drop for ast_Alias {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Alias {
    pub fn getEqual(&self) -> token_Pos {
        let r = unsafe { go::get_ast_Alias_Equal(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setEqual(&mut self, x: token_Pos) {
        unsafe { go::set_ast_Alias_Equal(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Alias> {
    pub fn getEqual(&self) -> token_Pos {
        let r = unsafe { go::get_ast_Alias_Equal(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setEqual(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_Alias_Equal(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Alias {
    pub fn getExpr(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_Alias_Expr(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setExpr(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_Alias_Expr(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Alias> {
    pub fn getExpr(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_Alias_Expr(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setExpr(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_Alias_Expr(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Alias {
    pub fn getIdent(&self) -> GoPtr<ast_Ident> {
        let r = unsafe { go::get_ast_Alias_Ident(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIdent(&mut self, x: GoPtr<ast_Ident>) {
        unsafe { go::set_ast_Alias_Ident(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Alias> {
    pub fn getIdent(&self) -> GoPtr<ast_Ident> {
        let r = unsafe { go::get_ast_Alias_Ident(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIdent(&mut self, x: GoPtr<ast_Ident>) {
        unsafe {
            go::set_ast_Alias_Ident(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_Alias {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_Alias()) }
    }
}

#[derive(Debug)]
pub struct ast_Attribute(go::GoUint32);

impl GoObject for ast_Attribute {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Attribute {}

impl Drop for ast_Attribute {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Attribute {
    pub fn getAt(&self) -> token_Pos {
        let r = unsafe { go::get_ast_Attribute_At(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setAt(&mut self, x: token_Pos) {
        unsafe { go::set_ast_Attribute_At(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Attribute> {
    pub fn getAt(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_Attribute_At(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setAt(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_Attribute_At(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Attribute {
    pub fn getText(&self) -> ffiString {
        let r = unsafe { go::get_ast_Attribute_Text(self.handle()) };
        import_string(r)
    }

    pub fn setText(&mut self, x: &str) {
        unsafe { go::set_ast_Attribute_Text(self.handle(), export_string(x)) };
    }
}

impl GoPtr<ast_Attribute> {
    pub fn getText(&self) -> ffiString {
        let r =
            unsafe { go::get_ast_Attribute_Text(self.dereference().handle()) };
        import_string(r)
    }

    pub fn setText(&mut self, x: &str) {
        unsafe {
            go::set_ast_Attribute_Text(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl Default for ast_Attribute {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_Attribute()) }
    }
}

#[derive(Debug)]
pub struct ast_BadDecl(go::GoUint32);

impl GoObject for ast_BadDecl {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_BadDecl {}

impl Drop for ast_BadDecl {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_BadDecl {
    pub fn getFrom(&self) -> token_Pos {
        let r = unsafe { go::get_ast_BadDecl_From(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setFrom(&mut self, x: token_Pos) {
        unsafe { go::set_ast_BadDecl_From(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BadDecl> {
    pub fn getFrom(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_BadDecl_From(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setFrom(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_BadDecl_From(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_BadDecl {
    pub fn getTo(&self) -> token_Pos {
        let r = unsafe { go::get_ast_BadDecl_To(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setTo(&mut self, x: token_Pos) {
        unsafe { go::set_ast_BadDecl_To(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BadDecl> {
    pub fn getTo(&self) -> token_Pos {
        let r = unsafe { go::get_ast_BadDecl_To(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setTo(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_BadDecl_To(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_BadDecl {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_BadDecl()) }
    }
}

#[derive(Debug)]
pub struct ast_BadExpr(go::GoUint32);

impl GoObject for ast_BadExpr {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_BadExpr {}

impl Drop for ast_BadExpr {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_BadExpr {
    pub fn getFrom(&self) -> token_Pos {
        let r = unsafe { go::get_ast_BadExpr_From(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setFrom(&mut self, x: token_Pos) {
        unsafe { go::set_ast_BadExpr_From(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BadExpr> {
    pub fn getFrom(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_BadExpr_From(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setFrom(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_BadExpr_From(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_BadExpr {
    pub fn getTo(&self) -> token_Pos {
        let r = unsafe { go::get_ast_BadExpr_To(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setTo(&mut self, x: token_Pos) {
        unsafe { go::set_ast_BadExpr_To(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BadExpr> {
    pub fn getTo(&self) -> token_Pos {
        let r = unsafe { go::get_ast_BadExpr_To(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setTo(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_BadExpr_To(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_BadExpr {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_BadExpr()) }
    }
}

#[derive(Debug)]
pub struct ast_BasicLit(go::GoUint32);

impl GoObject for ast_BasicLit {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_BasicLit {}

impl Drop for ast_BasicLit {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_BasicLit {
    pub fn getKind(&self) -> token_Token {
        let r = unsafe { go::get_ast_BasicLit_Kind(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setKind(&mut self, x: token_Token) {
        unsafe { go::set_ast_BasicLit_Kind(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BasicLit> {
    pub fn getKind(&self) -> token_Token {
        let r =
            unsafe { go::get_ast_BasicLit_Kind(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setKind(&mut self, x: token_Token) {
        unsafe {
            go::set_ast_BasicLit_Kind(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_BasicLit {
    pub fn getValue(&self) -> ffiString {
        let r = unsafe { go::get_ast_BasicLit_Value(self.handle()) };
        import_string(r)
    }

    pub fn setValue(&mut self, x: &str) {
        unsafe { go::set_ast_BasicLit_Value(self.handle(), export_string(x)) };
    }
}

impl GoPtr<ast_BasicLit> {
    pub fn getValue(&self) -> ffiString {
        let r =
            unsafe { go::get_ast_BasicLit_Value(self.dereference().handle()) };
        import_string(r)
    }

    pub fn setValue(&mut self, x: &str) {
        unsafe {
            go::set_ast_BasicLit_Value(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl ast_BasicLit {
    pub fn getValuePos(&self) -> token_Pos {
        let r = unsafe { go::get_ast_BasicLit_ValuePos(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setValuePos(&mut self, x: token_Pos) {
        unsafe { go::set_ast_BasicLit_ValuePos(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BasicLit> {
    pub fn getValuePos(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_BasicLit_ValuePos(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setValuePos(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_BasicLit_ValuePos(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl Default for ast_BasicLit {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_BasicLit()) }
    }
}

#[derive(Debug)]
pub struct ast_BinaryExpr(go::GoUint32);

impl GoObject for ast_BinaryExpr {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_BinaryExpr {}

impl Drop for ast_BinaryExpr {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_BinaryExpr {
    pub fn getOp(&self) -> token_Token {
        let r = unsafe { go::get_ast_BinaryExpr_Op(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOp(&mut self, x: token_Token) {
        unsafe { go::set_ast_BinaryExpr_Op(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BinaryExpr> {
    pub fn getOp(&self) -> token_Token {
        let r =
            unsafe { go::get_ast_BinaryExpr_Op(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOp(&mut self, x: token_Token) {
        unsafe {
            go::set_ast_BinaryExpr_Op(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_BinaryExpr {
    pub fn getOpPos(&self) -> token_Pos {
        let r = unsafe { go::get_ast_BinaryExpr_OpPos(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOpPos(&mut self, x: token_Pos) {
        unsafe { go::set_ast_BinaryExpr_OpPos(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BinaryExpr> {
    pub fn getOpPos(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_BinaryExpr_OpPos(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOpPos(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_BinaryExpr_OpPos(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_BinaryExpr {
    pub fn getX(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_BinaryExpr_X(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_BinaryExpr_X(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BinaryExpr> {
    pub fn getX(&self) -> ast_Expr {
        let r =
            unsafe { go::get_ast_BinaryExpr_X(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_BinaryExpr_X(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_BinaryExpr {
    pub fn getY(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_BinaryExpr_Y(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setY(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_BinaryExpr_Y(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BinaryExpr> {
    pub fn getY(&self) -> ast_Expr {
        let r =
            unsafe { go::get_ast_BinaryExpr_Y(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setY(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_BinaryExpr_Y(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_BinaryExpr {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_BinaryExpr()) }
    }
}

#[derive(Debug)]
pub struct ast_BottomLit(go::GoUint32);

impl GoObject for ast_BottomLit {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_BottomLit {}

impl Drop for ast_BottomLit {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_BottomLit {
    pub fn getBottom(&self) -> token_Pos {
        let r = unsafe { go::get_ast_BottomLit_Bottom(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setBottom(&mut self, x: token_Pos) {
        unsafe { go::set_ast_BottomLit_Bottom(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_BottomLit> {
    pub fn getBottom(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_BottomLit_Bottom(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setBottom(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_BottomLit_Bottom(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl Default for ast_BottomLit {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_BottomLit()) }
    }
}

#[derive(Debug)]
pub struct ast_CallExpr(go::GoUint32);

impl GoObject for ast_CallExpr {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_CallExpr {}

impl Drop for ast_CallExpr {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_CallExpr {
    pub fn getArgs(&self) -> GoSlice<ast_Expr> {
        let r = unsafe { go::get_ast_CallExpr_Args(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setArgs(&mut self, x: GoSlice<ast_Expr>) {
        unsafe { go::set_ast_CallExpr_Args(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_CallExpr> {
    pub fn getArgs(&self) -> GoSlice<ast_Expr> {
        let r =
            unsafe { go::get_ast_CallExpr_Args(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setArgs(&mut self, x: GoSlice<ast_Expr>) {
        unsafe {
            go::set_ast_CallExpr_Args(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_CallExpr {
    pub fn getFun(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_CallExpr_Fun(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setFun(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_CallExpr_Fun(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_CallExpr> {
    pub fn getFun(&self) -> ast_Expr {
        let r =
            unsafe { go::get_ast_CallExpr_Fun(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setFun(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_CallExpr_Fun(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_CallExpr {
    pub fn getLparen(&self) -> token_Pos {
        let r = unsafe { go::get_ast_CallExpr_Lparen(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLparen(&mut self, x: token_Pos) {
        unsafe { go::set_ast_CallExpr_Lparen(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_CallExpr> {
    pub fn getLparen(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_CallExpr_Lparen(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLparen(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_CallExpr_Lparen(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_CallExpr {
    pub fn getRparen(&self) -> token_Pos {
        let r = unsafe { go::get_ast_CallExpr_Rparen(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRparen(&mut self, x: token_Pos) {
        unsafe { go::set_ast_CallExpr_Rparen(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_CallExpr> {
    pub fn getRparen(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_CallExpr_Rparen(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRparen(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_CallExpr_Rparen(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_CallExpr {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_CallExpr()) }
    }
}

#[derive(Debug)]
pub struct ast_Clause(go::GoUint32);

impl GoObject for ast_Clause {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Clause {}

impl Drop for ast_Clause {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Clause {
    pub fn from_goobject<T>(x: &T) -> Option<Self>
    where
        T: GoObject,
    {
        let go::as_ast_Clause_return { r0, r1 } =
            unsafe { go::as_ast_Clause(x.handle()) }.into();

        if import_bool(r1) {
            Some(unsafe { Self::from_handle(r0) })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ast_Comment(go::GoUint32);

impl GoObject for ast_Comment {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Comment {}

impl Drop for ast_Comment {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Comment {
    pub fn getSlash(&self) -> token_Pos {
        let r = unsafe { go::get_ast_Comment_Slash(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setSlash(&mut self, x: token_Pos) {
        unsafe { go::set_ast_Comment_Slash(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Comment> {
    pub fn getSlash(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_Comment_Slash(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setSlash(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_Comment_Slash(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Comment {
    pub fn getText(&self) -> ffiString {
        let r = unsafe { go::get_ast_Comment_Text(self.handle()) };
        import_string(r)
    }

    pub fn setText(&mut self, x: &str) {
        unsafe { go::set_ast_Comment_Text(self.handle(), export_string(x)) };
    }
}

impl GoPtr<ast_Comment> {
    pub fn getText(&self) -> ffiString {
        let r =
            unsafe { go::get_ast_Comment_Text(self.dereference().handle()) };
        import_string(r)
    }

    pub fn setText(&mut self, x: &str) {
        unsafe {
            go::set_ast_Comment_Text(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl Default for ast_Comment {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_Comment()) }
    }
}

#[derive(Debug)]
pub struct ast_CommentGroup(go::GoUint32);

impl GoObject for ast_CommentGroup {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_CommentGroup {}

impl Drop for ast_CommentGroup {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_CommentGroup {
    pub fn getDoc(&self) -> bool {
        let r = unsafe { go::get_ast_CommentGroup_Doc(self.handle()) };
        import_bool(r)
    }

    pub fn setDoc(&mut self, x: bool) {
        unsafe { go::set_ast_CommentGroup_Doc(self.handle(), export_bool(x)) };
    }
}

impl GoPtr<ast_CommentGroup> {
    pub fn getDoc(&self) -> bool {
        let r = unsafe {
            go::get_ast_CommentGroup_Doc(self.dereference().handle())
        };
        import_bool(r)
    }

    pub fn setDoc(&mut self, x: bool) {
        unsafe {
            go::set_ast_CommentGroup_Doc(
                self.dereference().handle(),
                export_bool(x),
            )
        };
    }
}

impl ast_CommentGroup {
    pub fn getLine(&self) -> bool {
        let r = unsafe { go::get_ast_CommentGroup_Line(self.handle()) };
        import_bool(r)
    }

    pub fn setLine(&mut self, x: bool) {
        unsafe { go::set_ast_CommentGroup_Line(self.handle(), export_bool(x)) };
    }
}

impl GoPtr<ast_CommentGroup> {
    pub fn getLine(&self) -> bool {
        let r = unsafe {
            go::get_ast_CommentGroup_Line(self.dereference().handle())
        };
        import_bool(r)
    }

    pub fn setLine(&mut self, x: bool) {
        unsafe {
            go::set_ast_CommentGroup_Line(
                self.dereference().handle(),
                export_bool(x),
            )
        };
    }
}

impl ast_CommentGroup {
    pub fn getList(&self) -> GoSlice<GoPtr<ast_Comment>> {
        let r = unsafe { go::get_ast_CommentGroup_List(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setList(&mut self, x: GoSlice<GoPtr<ast_Comment>>) {
        unsafe { go::set_ast_CommentGroup_List(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_CommentGroup> {
    pub fn getList(&self) -> GoSlice<GoPtr<ast_Comment>> {
        let r = unsafe {
            go::get_ast_CommentGroup_List(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setList(&mut self, x: GoSlice<GoPtr<ast_Comment>>) {
        unsafe {
            go::set_ast_CommentGroup_List(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_CommentGroup {
    pub fn getPosition(&self) -> go::GoInt8 {
        let r = unsafe { go::get_ast_CommentGroup_Position(self.handle()) };
        r
    }

    pub fn setPosition(&mut self, x: go::GoInt8) {
        unsafe { go::set_ast_CommentGroup_Position(self.handle(), x) };
    }
}

impl GoPtr<ast_CommentGroup> {
    pub fn getPosition(&self) -> go::GoInt8 {
        let r = unsafe {
            go::get_ast_CommentGroup_Position(self.dereference().handle())
        };
        r
    }

    pub fn setPosition(&mut self, x: go::GoInt8) {
        unsafe {
            go::set_ast_CommentGroup_Position(self.dereference().handle(), x)
        };
    }
}

impl Default for ast_CommentGroup {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_CommentGroup()) }
    }
}

#[derive(Debug)]
pub struct ast_Comprehension(go::GoUint32);

impl GoObject for ast_Comprehension {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Comprehension {}

impl Drop for ast_Comprehension {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Comprehension {
    pub fn getClauses(&self) -> GoSlice<ast_Clause> {
        let r = unsafe { go::get_ast_Comprehension_Clauses(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setClauses(&mut self, x: GoSlice<ast_Clause>) {
        unsafe { go::set_ast_Comprehension_Clauses(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Comprehension> {
    pub fn getClauses(&self) -> GoSlice<ast_Clause> {
        let r = unsafe {
            go::get_ast_Comprehension_Clauses(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setClauses(&mut self, x: GoSlice<ast_Clause>) {
        unsafe {
            go::set_ast_Comprehension_Clauses(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_Comprehension {
    pub fn getValue(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_Comprehension_Value(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setValue(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_Comprehension_Value(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Comprehension> {
    pub fn getValue(&self) -> ast_Expr {
        let r = unsafe {
            go::get_ast_Comprehension_Value(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setValue(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_Comprehension_Value(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl Default for ast_Comprehension {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_Comprehension()) }
    }
}

#[derive(Debug)]
pub struct ast_Decl(go::GoUint32);

impl GoObject for ast_Decl {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Decl {}

impl Drop for ast_Decl {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Decl {
    pub fn from_goobject<T>(x: &T) -> Option<Self>
    where
        T: GoObject,
    {
        let go::as_ast_Decl_return { r0, r1 } =
            unsafe { go::as_ast_Decl(x.handle()) }.into();

        if import_bool(r1) {
            Some(unsafe { Self::from_handle(r0) })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ast_Ellipsis(go::GoUint32);

impl GoObject for ast_Ellipsis {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Ellipsis {}

impl Drop for ast_Ellipsis {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Ellipsis {
    pub fn getEllipsis(&self) -> token_Pos {
        let r = unsafe { go::get_ast_Ellipsis_Ellipsis(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setEllipsis(&mut self, x: token_Pos) {
        unsafe { go::set_ast_Ellipsis_Ellipsis(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Ellipsis> {
    pub fn getEllipsis(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_Ellipsis_Ellipsis(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setEllipsis(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_Ellipsis_Ellipsis(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_Ellipsis {
    pub fn getType(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_Ellipsis_Type(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setType(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_Ellipsis_Type(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Ellipsis> {
    pub fn getType(&self) -> ast_Expr {
        let r =
            unsafe { go::get_ast_Ellipsis_Type(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setType(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_Ellipsis_Type(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_Ellipsis {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_Ellipsis()) }
    }
}

#[derive(Debug)]
pub struct ast_EmbedDecl(go::GoUint32);

impl GoObject for ast_EmbedDecl {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_EmbedDecl {}

impl Drop for ast_EmbedDecl {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_EmbedDecl {
    pub fn getExpr(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_EmbedDecl_Expr(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setExpr(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_EmbedDecl_Expr(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_EmbedDecl> {
    pub fn getExpr(&self) -> ast_Expr {
        let r =
            unsafe { go::get_ast_EmbedDecl_Expr(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setExpr(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_EmbedDecl_Expr(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_EmbedDecl {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_EmbedDecl()) }
    }
}

#[derive(Debug)]
pub struct ast_Expr(go::GoUint32);

impl GoObject for ast_Expr {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Expr {}

impl Drop for ast_Expr {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Expr {
    pub fn from_goobject<T>(x: &T) -> Option<Self>
    where
        T: GoObject,
    {
        let go::as_ast_Expr_return { r0, r1 } =
            unsafe { go::as_ast_Expr(x.handle()) }.into();

        if import_bool(r1) {
            Some(unsafe { Self::from_handle(r0) })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ast_Field(go::GoUint32);

impl GoObject for ast_Field {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Field {}

impl Drop for ast_Field {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Field {
    pub fn getAttrs(&self) -> GoSlice<GoPtr<ast_Attribute>> {
        let r = unsafe { go::get_ast_Field_Attrs(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setAttrs(&mut self, x: GoSlice<GoPtr<ast_Attribute>>) {
        unsafe { go::set_ast_Field_Attrs(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Field> {
    pub fn getAttrs(&self) -> GoSlice<GoPtr<ast_Attribute>> {
        let r = unsafe { go::get_ast_Field_Attrs(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setAttrs(&mut self, x: GoSlice<GoPtr<ast_Attribute>>) {
        unsafe {
            go::set_ast_Field_Attrs(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Field {
    pub fn getLabel(&self) -> ast_Label {
        let r = unsafe { go::get_ast_Field_Label(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLabel(&mut self, x: ast_Label) {
        unsafe { go::set_ast_Field_Label(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Field> {
    pub fn getLabel(&self) -> ast_Label {
        let r = unsafe { go::get_ast_Field_Label(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLabel(&mut self, x: ast_Label) {
        unsafe {
            go::set_ast_Field_Label(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Field {
    pub fn getOptional(&self) -> token_Pos {
        let r = unsafe { go::get_ast_Field_Optional(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOptional(&mut self, x: token_Pos) {
        unsafe { go::set_ast_Field_Optional(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Field> {
    pub fn getOptional(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_Field_Optional(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOptional(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_Field_Optional(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Field {
    pub fn getToken(&self) -> token_Token {
        let r = unsafe { go::get_ast_Field_Token(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setToken(&mut self, x: token_Token) {
        unsafe { go::set_ast_Field_Token(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Field> {
    pub fn getToken(&self) -> token_Token {
        let r = unsafe { go::get_ast_Field_Token(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setToken(&mut self, x: token_Token) {
        unsafe {
            go::set_ast_Field_Token(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Field {
    pub fn getTokenPos(&self) -> token_Pos {
        let r = unsafe { go::get_ast_Field_TokenPos(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setTokenPos(&mut self, x: token_Pos) {
        unsafe { go::set_ast_Field_TokenPos(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Field> {
    pub fn getTokenPos(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_Field_TokenPos(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setTokenPos(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_Field_TokenPos(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Field {
    pub fn getValue(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_Field_Value(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setValue(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_Field_Value(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Field> {
    pub fn getValue(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_Field_Value(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setValue(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_Field_Value(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_Field {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_Field()) }
    }
}

#[derive(Debug)]
pub struct ast_File(go::GoUint32);

impl GoObject for ast_File {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_File {}

impl Drop for ast_File {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_File {
    pub fn getDecls(&self) -> GoSlice<ast_Decl> {
        let r = unsafe { go::get_ast_File_Decls(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setDecls(&mut self, x: GoSlice<ast_Decl>) {
        unsafe { go::set_ast_File_Decls(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_File> {
    pub fn getDecls(&self) -> GoSlice<ast_Decl> {
        let r = unsafe { go::get_ast_File_Decls(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setDecls(&mut self, x: GoSlice<ast_Decl>) {
        unsafe {
            go::set_ast_File_Decls(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_File {
    pub fn getFilename(&self) -> ffiString {
        let r = unsafe { go::get_ast_File_Filename(self.handle()) };
        import_string(r)
    }

    pub fn setFilename(&mut self, x: &str) {
        unsafe { go::set_ast_File_Filename(self.handle(), export_string(x)) };
    }
}

impl GoPtr<ast_File> {
    pub fn getFilename(&self) -> ffiString {
        let r =
            unsafe { go::get_ast_File_Filename(self.dereference().handle()) };
        import_string(r)
    }

    pub fn setFilename(&mut self, x: &str) {
        unsafe {
            go::set_ast_File_Filename(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl ast_File {
    pub fn getImports(&self) -> GoSlice<GoPtr<ast_ImportSpec>> {
        let r = unsafe { go::get_ast_File_Imports(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setImports(&mut self, x: GoSlice<GoPtr<ast_ImportSpec>>) {
        unsafe { go::set_ast_File_Imports(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_File> {
    pub fn getImports(&self) -> GoSlice<GoPtr<ast_ImportSpec>> {
        let r =
            unsafe { go::get_ast_File_Imports(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setImports(&mut self, x: GoSlice<GoPtr<ast_ImportSpec>>) {
        unsafe {
            go::set_ast_File_Imports(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_File {
    pub fn getUnresolved(&self) -> GoSlice<GoPtr<ast_Ident>> {
        let r = unsafe { go::get_ast_File_Unresolved(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setUnresolved(&mut self, x: GoSlice<GoPtr<ast_Ident>>) {
        unsafe { go::set_ast_File_Unresolved(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_File> {
    pub fn getUnresolved(&self) -> GoSlice<GoPtr<ast_Ident>> {
        let r =
            unsafe { go::get_ast_File_Unresolved(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setUnresolved(&mut self, x: GoSlice<GoPtr<ast_Ident>>) {
        unsafe {
            go::set_ast_File_Unresolved(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_File {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_File()) }
    }
}

#[derive(Debug)]
pub struct ast_ForClause(go::GoUint32);

impl GoObject for ast_ForClause {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_ForClause {}

impl Drop for ast_ForClause {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_ForClause {
    pub fn getColon(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ForClause_Colon(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setColon(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ForClause_Colon(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ForClause> {
    pub fn getColon(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_ForClause_Colon(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setColon(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ForClause_Colon(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_ForClause {
    pub fn getFor(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ForClause_For(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setFor(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ForClause_For(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ForClause> {
    pub fn getFor(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_ForClause_For(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setFor(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ForClause_For(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_ForClause {
    pub fn getIn(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ForClause_In(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIn(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ForClause_In(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ForClause> {
    pub fn getIn(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_ForClause_In(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIn(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ForClause_In(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_ForClause {
    pub fn getKey(&self) -> GoPtr<ast_Ident> {
        let r = unsafe { go::get_ast_ForClause_Key(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setKey(&mut self, x: GoPtr<ast_Ident>) {
        unsafe { go::set_ast_ForClause_Key(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ForClause> {
    pub fn getKey(&self) -> GoPtr<ast_Ident> {
        let r =
            unsafe { go::get_ast_ForClause_Key(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setKey(&mut self, x: GoPtr<ast_Ident>) {
        unsafe {
            go::set_ast_ForClause_Key(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_ForClause {
    pub fn getSource(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_ForClause_Source(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setSource(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_ForClause_Source(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ForClause> {
    pub fn getSource(&self) -> ast_Expr {
        let r = unsafe {
            go::get_ast_ForClause_Source(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setSource(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_ForClause_Source(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_ForClause {
    pub fn getValue(&self) -> GoPtr<ast_Ident> {
        let r = unsafe { go::get_ast_ForClause_Value(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setValue(&mut self, x: GoPtr<ast_Ident>) {
        unsafe { go::set_ast_ForClause_Value(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ForClause> {
    pub fn getValue(&self) -> GoPtr<ast_Ident> {
        let r =
            unsafe { go::get_ast_ForClause_Value(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setValue(&mut self, x: GoPtr<ast_Ident>) {
        unsafe {
            go::set_ast_ForClause_Value(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_ForClause {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_ForClause()) }
    }
}

#[derive(Debug)]
pub struct ast_Ident(go::GoUint32);

impl GoObject for ast_Ident {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Ident {}

impl Drop for ast_Ident {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Ident {
    pub fn getName(&self) -> ffiString {
        let r = unsafe { go::get_ast_Ident_Name(self.handle()) };
        import_string(r)
    }

    pub fn setName(&mut self, x: &str) {
        unsafe { go::set_ast_Ident_Name(self.handle(), export_string(x)) };
    }
}

impl GoPtr<ast_Ident> {
    pub fn getName(&self) -> ffiString {
        let r = unsafe { go::get_ast_Ident_Name(self.dereference().handle()) };
        import_string(r)
    }

    pub fn setName(&mut self, x: &str) {
        unsafe {
            go::set_ast_Ident_Name(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl ast_Ident {
    pub fn getNamePos(&self) -> token_Pos {
        let r = unsafe { go::get_ast_Ident_NamePos(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setNamePos(&mut self, x: token_Pos) {
        unsafe { go::set_ast_Ident_NamePos(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Ident> {
    pub fn getNamePos(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_Ident_NamePos(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setNamePos(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_Ident_NamePos(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Ident {
    pub fn getNode(&self) -> ast_Node {
        let r = unsafe { go::get_ast_Ident_Node(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setNode(&mut self, x: ast_Node) {
        unsafe { go::set_ast_Ident_Node(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Ident> {
    pub fn getNode(&self) -> ast_Node {
        let r = unsafe { go::get_ast_Ident_Node(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setNode(&mut self, x: ast_Node) {
        unsafe {
            go::set_ast_Ident_Node(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Ident {
    pub fn getScope(&self) -> ast_Node {
        let r = unsafe { go::get_ast_Ident_Scope(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setScope(&mut self, x: ast_Node) {
        unsafe { go::set_ast_Ident_Scope(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Ident> {
    pub fn getScope(&self) -> ast_Node {
        let r = unsafe { go::get_ast_Ident_Scope(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setScope(&mut self, x: ast_Node) {
        unsafe {
            go::set_ast_Ident_Scope(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_Ident {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_Ident()) }
    }
}

#[derive(Debug)]
pub struct ast_IfClause(go::GoUint32);

impl GoObject for ast_IfClause {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_IfClause {}

impl Drop for ast_IfClause {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_IfClause {
    pub fn getCondition(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_IfClause_Condition(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setCondition(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_IfClause_Condition(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_IfClause> {
    pub fn getCondition(&self) -> ast_Expr {
        let r = unsafe {
            go::get_ast_IfClause_Condition(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setCondition(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_IfClause_Condition(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_IfClause {
    pub fn getIf(&self) -> token_Pos {
        let r = unsafe { go::get_ast_IfClause_If(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIf(&mut self, x: token_Pos) {
        unsafe { go::set_ast_IfClause_If(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_IfClause> {
    pub fn getIf(&self) -> token_Pos {
        let r = unsafe { go::get_ast_IfClause_If(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIf(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_IfClause_If(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_IfClause {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_IfClause()) }
    }
}

#[derive(Debug)]
pub struct ast_ImportDecl(go::GoUint32);

impl GoObject for ast_ImportDecl {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_ImportDecl {}

impl Drop for ast_ImportDecl {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_ImportDecl {
    pub fn getImport(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ImportDecl_Import(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setImport(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ImportDecl_Import(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ImportDecl> {
    pub fn getImport(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_ImportDecl_Import(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setImport(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ImportDecl_Import(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_ImportDecl {
    pub fn getLparen(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ImportDecl_Lparen(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLparen(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ImportDecl_Lparen(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ImportDecl> {
    pub fn getLparen(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_ImportDecl_Lparen(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLparen(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ImportDecl_Lparen(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_ImportDecl {
    pub fn getRparen(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ImportDecl_Rparen(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRparen(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ImportDecl_Rparen(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ImportDecl> {
    pub fn getRparen(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_ImportDecl_Rparen(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRparen(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ImportDecl_Rparen(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_ImportDecl {
    pub fn getSpecs(&self) -> GoSlice<GoPtr<ast_ImportSpec>> {
        let r = unsafe { go::get_ast_ImportDecl_Specs(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setSpecs(&mut self, x: GoSlice<GoPtr<ast_ImportSpec>>) {
        unsafe { go::set_ast_ImportDecl_Specs(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ImportDecl> {
    pub fn getSpecs(&self) -> GoSlice<GoPtr<ast_ImportSpec>> {
        let r = unsafe {
            go::get_ast_ImportDecl_Specs(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setSpecs(&mut self, x: GoSlice<GoPtr<ast_ImportSpec>>) {
        unsafe {
            go::set_ast_ImportDecl_Specs(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl Default for ast_ImportDecl {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_ImportDecl()) }
    }
}

#[derive(Debug)]
pub struct ast_ImportSpec(go::GoUint32);

impl GoObject for ast_ImportSpec {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_ImportSpec {}

impl Drop for ast_ImportSpec {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_ImportSpec {
    pub fn getEndPos(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ImportSpec_EndPos(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setEndPos(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ImportSpec_EndPos(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ImportSpec> {
    pub fn getEndPos(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_ImportSpec_EndPos(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setEndPos(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ImportSpec_EndPos(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_ImportSpec {
    pub fn getName(&self) -> GoPtr<ast_Ident> {
        let r = unsafe { go::get_ast_ImportSpec_Name(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setName(&mut self, x: GoPtr<ast_Ident>) {
        unsafe { go::set_ast_ImportSpec_Name(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ImportSpec> {
    pub fn getName(&self) -> GoPtr<ast_Ident> {
        let r =
            unsafe { go::get_ast_ImportSpec_Name(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setName(&mut self, x: GoPtr<ast_Ident>) {
        unsafe {
            go::set_ast_ImportSpec_Name(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_ImportSpec {
    pub fn getPath(&self) -> GoPtr<ast_BasicLit> {
        let r = unsafe { go::get_ast_ImportSpec_Path(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setPath(&mut self, x: GoPtr<ast_BasicLit>) {
        unsafe { go::set_ast_ImportSpec_Path(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ImportSpec> {
    pub fn getPath(&self) -> GoPtr<ast_BasicLit> {
        let r =
            unsafe { go::get_ast_ImportSpec_Path(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setPath(&mut self, x: GoPtr<ast_BasicLit>) {
        unsafe {
            go::set_ast_ImportSpec_Path(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_ImportSpec {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_ImportSpec()) }
    }
}

#[derive(Debug)]
pub struct ast_IndexExpr(go::GoUint32);

impl GoObject for ast_IndexExpr {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_IndexExpr {}

impl Drop for ast_IndexExpr {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_IndexExpr {
    pub fn getIndex(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_IndexExpr_Index(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIndex(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_IndexExpr_Index(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_IndexExpr> {
    pub fn getIndex(&self) -> ast_Expr {
        let r =
            unsafe { go::get_ast_IndexExpr_Index(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIndex(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_IndexExpr_Index(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_IndexExpr {
    pub fn getLbrack(&self) -> token_Pos {
        let r = unsafe { go::get_ast_IndexExpr_Lbrack(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLbrack(&mut self, x: token_Pos) {
        unsafe { go::set_ast_IndexExpr_Lbrack(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_IndexExpr> {
    pub fn getLbrack(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_IndexExpr_Lbrack(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLbrack(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_IndexExpr_Lbrack(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_IndexExpr {
    pub fn getRbrack(&self) -> token_Pos {
        let r = unsafe { go::get_ast_IndexExpr_Rbrack(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRbrack(&mut self, x: token_Pos) {
        unsafe { go::set_ast_IndexExpr_Rbrack(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_IndexExpr> {
    pub fn getRbrack(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_IndexExpr_Rbrack(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRbrack(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_IndexExpr_Rbrack(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_IndexExpr {
    pub fn getX(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_IndexExpr_X(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_IndexExpr_X(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_IndexExpr> {
    pub fn getX(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_IndexExpr_X(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_IndexExpr_X(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_IndexExpr {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_IndexExpr()) }
    }
}

#[derive(Debug)]
pub struct ast_Interpolation(go::GoUint32);

impl GoObject for ast_Interpolation {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Interpolation {}

impl Drop for ast_Interpolation {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Interpolation {
    pub fn getElts(&self) -> GoSlice<ast_Expr> {
        let r = unsafe { go::get_ast_Interpolation_Elts(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setElts(&mut self, x: GoSlice<ast_Expr>) {
        unsafe { go::set_ast_Interpolation_Elts(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Interpolation> {
    pub fn getElts(&self) -> GoSlice<ast_Expr> {
        let r = unsafe {
            go::get_ast_Interpolation_Elts(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setElts(&mut self, x: GoSlice<ast_Expr>) {
        unsafe {
            go::set_ast_Interpolation_Elts(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl Default for ast_Interpolation {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_Interpolation()) }
    }
}

#[derive(Debug)]
pub struct ast_Label(go::GoUint32);

impl GoObject for ast_Label {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Label {}

impl Drop for ast_Label {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Label {
    pub fn from_goobject<T>(x: &T) -> Option<Self>
    where
        T: GoObject,
    {
        let go::as_ast_Label_return { r0, r1 } =
            unsafe { go::as_ast_Label(x.handle()) }.into();

        if import_bool(r1) {
            Some(unsafe { Self::from_handle(r0) })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ast_LetClause(go::GoUint32);

impl GoObject for ast_LetClause {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_LetClause {}

impl Drop for ast_LetClause {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_LetClause {
    pub fn getEqual(&self) -> token_Pos {
        let r = unsafe { go::get_ast_LetClause_Equal(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setEqual(&mut self, x: token_Pos) {
        unsafe { go::set_ast_LetClause_Equal(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_LetClause> {
    pub fn getEqual(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_LetClause_Equal(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setEqual(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_LetClause_Equal(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_LetClause {
    pub fn getExpr(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_LetClause_Expr(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setExpr(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_LetClause_Expr(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_LetClause> {
    pub fn getExpr(&self) -> ast_Expr {
        let r =
            unsafe { go::get_ast_LetClause_Expr(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setExpr(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_LetClause_Expr(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_LetClause {
    pub fn getIdent(&self) -> GoPtr<ast_Ident> {
        let r = unsafe { go::get_ast_LetClause_Ident(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIdent(&mut self, x: GoPtr<ast_Ident>) {
        unsafe { go::set_ast_LetClause_Ident(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_LetClause> {
    pub fn getIdent(&self) -> GoPtr<ast_Ident> {
        let r =
            unsafe { go::get_ast_LetClause_Ident(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIdent(&mut self, x: GoPtr<ast_Ident>) {
        unsafe {
            go::set_ast_LetClause_Ident(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_LetClause {
    pub fn getLet(&self) -> token_Pos {
        let r = unsafe { go::get_ast_LetClause_Let(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLet(&mut self, x: token_Pos) {
        unsafe { go::set_ast_LetClause_Let(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_LetClause> {
    pub fn getLet(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_LetClause_Let(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLet(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_LetClause_Let(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_LetClause {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_LetClause()) }
    }
}

#[derive(Debug)]
pub struct ast_ListComprehension(go::GoUint32);

impl GoObject for ast_ListComprehension {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_ListComprehension {}

impl Drop for ast_ListComprehension {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_ListComprehension {
    pub fn getClauses(&self) -> GoSlice<ast_Clause> {
        let r = unsafe { go::get_ast_ListComprehension_Clauses(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setClauses(&mut self, x: GoSlice<ast_Clause>) {
        unsafe {
            go::set_ast_ListComprehension_Clauses(self.handle(), x.handle())
        };
    }
}

impl GoPtr<ast_ListComprehension> {
    pub fn getClauses(&self) -> GoSlice<ast_Clause> {
        let r = unsafe {
            go::get_ast_ListComprehension_Clauses(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setClauses(&mut self, x: GoSlice<ast_Clause>) {
        unsafe {
            go::set_ast_ListComprehension_Clauses(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_ListComprehension {
    pub fn getExpr(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_ListComprehension_Expr(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setExpr(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_ListComprehension_Expr(self.handle(), x.handle())
        };
    }
}

impl GoPtr<ast_ListComprehension> {
    pub fn getExpr(&self) -> ast_Expr {
        let r = unsafe {
            go::get_ast_ListComprehension_Expr(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setExpr(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_ListComprehension_Expr(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_ListComprehension {
    pub fn getLbrack(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ListComprehension_Lbrack(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLbrack(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ListComprehension_Lbrack(self.handle(), x.handle())
        };
    }
}

impl GoPtr<ast_ListComprehension> {
    pub fn getLbrack(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_ListComprehension_Lbrack(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLbrack(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ListComprehension_Lbrack(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_ListComprehension {
    pub fn getRbrack(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ListComprehension_Rbrack(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRbrack(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ListComprehension_Rbrack(self.handle(), x.handle())
        };
    }
}

impl GoPtr<ast_ListComprehension> {
    pub fn getRbrack(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_ListComprehension_Rbrack(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRbrack(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ListComprehension_Rbrack(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl Default for ast_ListComprehension {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_ListComprehension()) }
    }
}

#[derive(Debug)]
pub struct ast_ListLit(go::GoUint32);

impl GoObject for ast_ListLit {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_ListLit {}

impl Drop for ast_ListLit {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_ListLit {
    pub fn getElts(&self) -> GoSlice<ast_Expr> {
        let r = unsafe { go::get_ast_ListLit_Elts(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setElts(&mut self, x: GoSlice<ast_Expr>) {
        unsafe { go::set_ast_ListLit_Elts(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ListLit> {
    pub fn getElts(&self) -> GoSlice<ast_Expr> {
        let r =
            unsafe { go::get_ast_ListLit_Elts(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setElts(&mut self, x: GoSlice<ast_Expr>) {
        unsafe {
            go::set_ast_ListLit_Elts(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_ListLit {
    pub fn getLbrack(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ListLit_Lbrack(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLbrack(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ListLit_Lbrack(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ListLit> {
    pub fn getLbrack(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_ListLit_Lbrack(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLbrack(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ListLit_Lbrack(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_ListLit {
    pub fn getRbrack(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ListLit_Rbrack(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRbrack(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ListLit_Rbrack(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ListLit> {
    pub fn getRbrack(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_ListLit_Rbrack(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRbrack(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ListLit_Rbrack(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_ListLit {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_ListLit()) }
    }
}

#[derive(Debug)]
pub struct ast_Node(go::GoUint32);

impl GoObject for ast_Node {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Node {}

impl Drop for ast_Node {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Node {
    pub fn from_goobject<T>(x: &T) -> Option<Self>
    where
        T: GoObject,
    {
        let go::as_ast_Node_return { r0, r1 } =
            unsafe { go::as_ast_Node(x.handle()) }.into();

        if import_bool(r1) {
            Some(unsafe { Self::from_handle(r0) })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ast_Package(go::GoUint32);

impl GoObject for ast_Package {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Package {}

impl Drop for ast_Package {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Package {
    pub fn getName(&self) -> GoPtr<ast_Ident> {
        let r = unsafe { go::get_ast_Package_Name(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setName(&mut self, x: GoPtr<ast_Ident>) {
        unsafe { go::set_ast_Package_Name(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Package> {
    pub fn getName(&self) -> GoPtr<ast_Ident> {
        let r =
            unsafe { go::get_ast_Package_Name(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setName(&mut self, x: GoPtr<ast_Ident>) {
        unsafe {
            go::set_ast_Package_Name(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_Package {
    pub fn getPackagePos(&self) -> token_Pos {
        let r = unsafe { go::get_ast_Package_PackagePos(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setPackagePos(&mut self, x: token_Pos) {
        unsafe { go::set_ast_Package_PackagePos(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_Package> {
    pub fn getPackagePos(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_Package_PackagePos(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setPackagePos(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_Package_PackagePos(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl Default for ast_Package {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_Package()) }
    }
}

#[derive(Debug)]
pub struct ast_ParenExpr(go::GoUint32);

impl GoObject for ast_ParenExpr {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_ParenExpr {}

impl Drop for ast_ParenExpr {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_ParenExpr {
    pub fn getLparen(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ParenExpr_Lparen(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLparen(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ParenExpr_Lparen(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ParenExpr> {
    pub fn getLparen(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_ParenExpr_Lparen(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLparen(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ParenExpr_Lparen(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_ParenExpr {
    pub fn getRparen(&self) -> token_Pos {
        let r = unsafe { go::get_ast_ParenExpr_Rparen(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRparen(&mut self, x: token_Pos) {
        unsafe { go::set_ast_ParenExpr_Rparen(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ParenExpr> {
    pub fn getRparen(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_ParenExpr_Rparen(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRparen(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_ParenExpr_Rparen(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_ParenExpr {
    pub fn getX(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_ParenExpr_X(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_ParenExpr_X(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_ParenExpr> {
    pub fn getX(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_ParenExpr_X(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_ParenExpr_X(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_ParenExpr {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_ParenExpr()) }
    }
}

#[derive(Debug)]
pub struct ast_SelectorExpr(go::GoUint32);

impl GoObject for ast_SelectorExpr {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_SelectorExpr {}

impl Drop for ast_SelectorExpr {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_SelectorExpr {
    pub fn getSel(&self) -> ast_Label {
        let r = unsafe { go::get_ast_SelectorExpr_Sel(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setSel(&mut self, x: ast_Label) {
        unsafe { go::set_ast_SelectorExpr_Sel(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_SelectorExpr> {
    pub fn getSel(&self) -> ast_Label {
        let r = unsafe {
            go::get_ast_SelectorExpr_Sel(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setSel(&mut self, x: ast_Label) {
        unsafe {
            go::set_ast_SelectorExpr_Sel(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_SelectorExpr {
    pub fn getX(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_SelectorExpr_X(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_SelectorExpr_X(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_SelectorExpr> {
    pub fn getX(&self) -> ast_Expr {
        let r =
            unsafe { go::get_ast_SelectorExpr_X(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_SelectorExpr_X(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_SelectorExpr {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_SelectorExpr()) }
    }
}

#[derive(Debug)]
pub struct ast_SliceExpr(go::GoUint32);

impl GoObject for ast_SliceExpr {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_SliceExpr {}

impl Drop for ast_SliceExpr {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_SliceExpr {
    pub fn getHigh(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_SliceExpr_High(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setHigh(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_SliceExpr_High(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_SliceExpr> {
    pub fn getHigh(&self) -> ast_Expr {
        let r =
            unsafe { go::get_ast_SliceExpr_High(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setHigh(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_SliceExpr_High(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_SliceExpr {
    pub fn getLbrack(&self) -> token_Pos {
        let r = unsafe { go::get_ast_SliceExpr_Lbrack(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLbrack(&mut self, x: token_Pos) {
        unsafe { go::set_ast_SliceExpr_Lbrack(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_SliceExpr> {
    pub fn getLbrack(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_SliceExpr_Lbrack(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLbrack(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_SliceExpr_Lbrack(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_SliceExpr {
    pub fn getLow(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_SliceExpr_Low(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLow(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_SliceExpr_Low(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_SliceExpr> {
    pub fn getLow(&self) -> ast_Expr {
        let r =
            unsafe { go::get_ast_SliceExpr_Low(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLow(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_SliceExpr_Low(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_SliceExpr {
    pub fn getRbrack(&self) -> token_Pos {
        let r = unsafe { go::get_ast_SliceExpr_Rbrack(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRbrack(&mut self, x: token_Pos) {
        unsafe { go::set_ast_SliceExpr_Rbrack(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_SliceExpr> {
    pub fn getRbrack(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_SliceExpr_Rbrack(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRbrack(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_SliceExpr_Rbrack(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_SliceExpr {
    pub fn getX(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_SliceExpr_X(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_SliceExpr_X(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_SliceExpr> {
    pub fn getX(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_SliceExpr_X(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_SliceExpr_X(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_SliceExpr {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_SliceExpr()) }
    }
}

#[derive(Debug)]
pub struct ast_Spec(go::GoUint32);

impl GoObject for ast_Spec {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_Spec {}

impl Drop for ast_Spec {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_Spec {
    pub fn from_goobject<T>(x: &T) -> Option<Self>
    where
        T: GoObject,
    {
        let go::as_ast_Spec_return { r0, r1 } =
            unsafe { go::as_ast_Spec(x.handle()) }.into();

        if import_bool(r1) {
            Some(unsafe { Self::from_handle(r0) })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ast_StructLit(go::GoUint32);

impl GoObject for ast_StructLit {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_StructLit {}

impl Drop for ast_StructLit {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_StructLit {
    pub fn getElts(&self) -> GoSlice<ast_Decl> {
        let r = unsafe { go::get_ast_StructLit_Elts(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setElts(&mut self, x: GoSlice<ast_Decl>) {
        unsafe { go::set_ast_StructLit_Elts(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_StructLit> {
    pub fn getElts(&self) -> GoSlice<ast_Decl> {
        let r =
            unsafe { go::get_ast_StructLit_Elts(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setElts(&mut self, x: GoSlice<ast_Decl>) {
        unsafe {
            go::set_ast_StructLit_Elts(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_StructLit {
    pub fn getLbrace(&self) -> token_Pos {
        let r = unsafe { go::get_ast_StructLit_Lbrace(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLbrace(&mut self, x: token_Pos) {
        unsafe { go::set_ast_StructLit_Lbrace(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_StructLit> {
    pub fn getLbrace(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_StructLit_Lbrace(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLbrace(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_StructLit_Lbrace(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_StructLit {
    pub fn getRbrace(&self) -> token_Pos {
        let r = unsafe { go::get_ast_StructLit_Rbrace(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRbrace(&mut self, x: token_Pos) {
        unsafe { go::set_ast_StructLit_Rbrace(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_StructLit> {
    pub fn getRbrace(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_StructLit_Rbrace(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRbrace(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_StructLit_Rbrace(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl Default for ast_StructLit {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_StructLit()) }
    }
}

#[derive(Debug)]
pub struct ast_TemplateLabel(go::GoUint32);

impl GoObject for ast_TemplateLabel {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_TemplateLabel {}

impl Drop for ast_TemplateLabel {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_TemplateLabel {
    pub fn getIdent(&self) -> GoPtr<ast_Ident> {
        let r = unsafe { go::get_ast_TemplateLabel_Ident(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIdent(&mut self, x: GoPtr<ast_Ident>) {
        unsafe { go::set_ast_TemplateLabel_Ident(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_TemplateLabel> {
    pub fn getIdent(&self) -> GoPtr<ast_Ident> {
        let r = unsafe {
            go::get_ast_TemplateLabel_Ident(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIdent(&mut self, x: GoPtr<ast_Ident>) {
        unsafe {
            go::set_ast_TemplateLabel_Ident(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_TemplateLabel {
    pub fn getLangle(&self) -> token_Pos {
        let r = unsafe { go::get_ast_TemplateLabel_Langle(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLangle(&mut self, x: token_Pos) {
        unsafe { go::set_ast_TemplateLabel_Langle(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_TemplateLabel> {
    pub fn getLangle(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_TemplateLabel_Langle(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setLangle(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_TemplateLabel_Langle(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl ast_TemplateLabel {
    pub fn getRangle(&self) -> token_Pos {
        let r = unsafe { go::get_ast_TemplateLabel_Rangle(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRangle(&mut self, x: token_Pos) {
        unsafe { go::set_ast_TemplateLabel_Rangle(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_TemplateLabel> {
    pub fn getRangle(&self) -> token_Pos {
        let r = unsafe {
            go::get_ast_TemplateLabel_Rangle(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setRangle(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_TemplateLabel_Rangle(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl Default for ast_TemplateLabel {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_TemplateLabel()) }
    }
}

#[derive(Debug)]
pub struct ast_UnaryExpr(go::GoUint32);

impl GoObject for ast_UnaryExpr {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_UnaryExpr {}

impl Drop for ast_UnaryExpr {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl ast_UnaryExpr {
    pub fn getOp(&self) -> token_Token {
        let r = unsafe { go::get_ast_UnaryExpr_Op(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOp(&mut self, x: token_Token) {
        unsafe { go::set_ast_UnaryExpr_Op(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_UnaryExpr> {
    pub fn getOp(&self) -> token_Token {
        let r =
            unsafe { go::get_ast_UnaryExpr_Op(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOp(&mut self, x: token_Token) {
        unsafe {
            go::set_ast_UnaryExpr_Op(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_UnaryExpr {
    pub fn getOpPos(&self) -> token_Pos {
        let r = unsafe { go::get_ast_UnaryExpr_OpPos(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOpPos(&mut self, x: token_Pos) {
        unsafe { go::set_ast_UnaryExpr_OpPos(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_UnaryExpr> {
    pub fn getOpPos(&self) -> token_Pos {
        let r =
            unsafe { go::get_ast_UnaryExpr_OpPos(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOpPos(&mut self, x: token_Pos) {
        unsafe {
            go::set_ast_UnaryExpr_OpPos(self.dereference().handle(), x.handle())
        };
    }
}

impl ast_UnaryExpr {
    pub fn getX(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_UnaryExpr_X(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe { go::set_ast_UnaryExpr_X(self.handle(), x.handle()) };
    }
}

impl GoPtr<ast_UnaryExpr> {
    pub fn getX(&self) -> ast_Expr {
        let r = unsafe { go::get_ast_UnaryExpr_X(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setX(&mut self, x: ast_Expr) {
        unsafe {
            go::set_ast_UnaryExpr_X(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for ast_UnaryExpr {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultast_UnaryExpr()) }
    }
}

#[derive(Debug)]
pub struct astutil_Cursor(go::GoUint32);

impl GoObject for astutil_Cursor {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for astutil_Cursor {}

impl Drop for astutil_Cursor {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl astutil_Cursor {
    pub fn from_goobject<T>(x: &T) -> Option<Self>
    where
        T: GoObject,
    {
        let go::as_astutil_Cursor_return { r0, r1 } =
            unsafe { go::as_astutil_Cursor(x.handle()) }.into();

        if import_bool(r1) {
            Some(unsafe { Self::from_handle(r0) })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct astutil_ImportInfo(go::GoUint32);

impl GoObject for astutil_ImportInfo {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for astutil_ImportInfo {}

impl Drop for astutil_ImportInfo {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl astutil_ImportInfo {
    pub fn getDir(&self) -> ffiString {
        let r = unsafe { go::get_astutil_ImportInfo_Dir(self.handle()) };
        import_string(r)
    }

    pub fn setDir(&mut self, x: &str) {
        unsafe {
            go::set_astutil_ImportInfo_Dir(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<astutil_ImportInfo> {
    pub fn getDir(&self) -> ffiString {
        let r = unsafe {
            go::get_astutil_ImportInfo_Dir(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setDir(&mut self, x: &str) {
        unsafe {
            go::set_astutil_ImportInfo_Dir(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl astutil_ImportInfo {
    pub fn getID(&self) -> ffiString {
        let r = unsafe { go::get_astutil_ImportInfo_ID(self.handle()) };
        import_string(r)
    }

    pub fn setID(&mut self, x: &str) {
        unsafe {
            go::set_astutil_ImportInfo_ID(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<astutil_ImportInfo> {
    pub fn getID(&self) -> ffiString {
        let r = unsafe {
            go::get_astutil_ImportInfo_ID(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setID(&mut self, x: &str) {
        unsafe {
            go::set_astutil_ImportInfo_ID(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl astutil_ImportInfo {
    pub fn getIdent(&self) -> ffiString {
        let r = unsafe { go::get_astutil_ImportInfo_Ident(self.handle()) };
        import_string(r)
    }

    pub fn setIdent(&mut self, x: &str) {
        unsafe {
            go::set_astutil_ImportInfo_Ident(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<astutil_ImportInfo> {
    pub fn getIdent(&self) -> ffiString {
        let r = unsafe {
            go::get_astutil_ImportInfo_Ident(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setIdent(&mut self, x: &str) {
        unsafe {
            go::set_astutil_ImportInfo_Ident(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl astutil_ImportInfo {
    pub fn getPkgName(&self) -> ffiString {
        let r = unsafe { go::get_astutil_ImportInfo_PkgName(self.handle()) };
        import_string(r)
    }

    pub fn setPkgName(&mut self, x: &str) {
        unsafe {
            go::set_astutil_ImportInfo_PkgName(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<astutil_ImportInfo> {
    pub fn getPkgName(&self) -> ffiString {
        let r = unsafe {
            go::get_astutil_ImportInfo_PkgName(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setPkgName(&mut self, x: &str) {
        unsafe {
            go::set_astutil_ImportInfo_PkgName(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl Default for astutil_ImportInfo {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultastutil_ImportInfo()) }
    }
}

#[derive(Debug)]
pub struct build_Context(go::GoUint32);

impl GoObject for build_Context {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for build_Context {}

impl Drop for build_Context {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for build_Context {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultbuild_Context()) }
    }
}

#[derive(Debug)]
pub struct build_Encoding(go::GoUint32);

impl GoObject for build_Encoding {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for build_Encoding {}

impl Drop for build_Encoding {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for build_Encoding {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultbuild_Encoding()) }
    }
}

#[derive(Debug)]
pub struct build_File(go::GoUint32);

impl GoObject for build_File {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for build_File {}

impl Drop for build_File {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl build_File {
    pub fn getEncoding(&self) -> build_Encoding {
        let r = unsafe { go::get_build_File_Encoding(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setEncoding(&mut self, x: build_Encoding) {
        unsafe { go::set_build_File_Encoding(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_File> {
    pub fn getEncoding(&self) -> build_Encoding {
        let r =
            unsafe { go::get_build_File_Encoding(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setEncoding(&mut self, x: build_Encoding) {
        unsafe {
            go::set_build_File_Encoding(self.dereference().handle(), x.handle())
        };
    }
}

impl build_File {
    pub fn getFilename(&self) -> ffiString {
        let r = unsafe { go::get_build_File_Filename(self.handle()) };
        import_string(r)
    }

    pub fn setFilename(&mut self, x: &str) {
        unsafe { go::set_build_File_Filename(self.handle(), export_string(x)) };
    }
}

impl GoPtr<build_File> {
    pub fn getFilename(&self) -> ffiString {
        let r =
            unsafe { go::get_build_File_Filename(self.dereference().handle()) };
        import_string(r)
    }

    pub fn setFilename(&mut self, x: &str) {
        unsafe {
            go::set_build_File_Filename(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl build_File {
    pub fn getForm(&self) -> build_Form {
        let r = unsafe { go::get_build_File_Form(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setForm(&mut self, x: build_Form) {
        unsafe { go::set_build_File_Form(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_File> {
    pub fn getForm(&self) -> build_Form {
        let r = unsafe { go::get_build_File_Form(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setForm(&mut self, x: build_Form) {
        unsafe {
            go::set_build_File_Form(self.dereference().handle(), x.handle())
        };
    }
}

impl build_File {
    pub fn getInterpretation(&self) -> build_Interpretation {
        let r = unsafe { go::get_build_File_Interpretation(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setInterpretation(&mut self, x: build_Interpretation) {
        unsafe { go::set_build_File_Interpretation(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_File> {
    pub fn getInterpretation(&self) -> build_Interpretation {
        let r = unsafe {
            go::get_build_File_Interpretation(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setInterpretation(&mut self, x: build_Interpretation) {
        unsafe {
            go::set_build_File_Interpretation(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_File {
    pub fn getSource(&self) -> GoAny {
        let r = unsafe { go::get_build_File_Source(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setSource(&mut self, x: GoAny) {
        unsafe { go::set_build_File_Source(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_File> {
    pub fn getSource(&self) -> GoAny {
        let r =
            unsafe { go::get_build_File_Source(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setSource(&mut self, x: GoAny) {
        unsafe {
            go::set_build_File_Source(self.dereference().handle(), x.handle())
        };
    }
}

impl build_File {
    pub fn getTags(&self) -> GoMap<String, String> {
        let r = unsafe { go::get_build_File_Tags(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setTags(&mut self, x: GoMap<String, String>) {
        unsafe { go::set_build_File_Tags(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_File> {
    pub fn getTags(&self) -> GoMap<String, String> {
        let r = unsafe { go::get_build_File_Tags(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setTags(&mut self, x: GoMap<String, String>) {
        unsafe {
            go::set_build_File_Tags(self.dereference().handle(), x.handle())
        };
    }
}

impl Default for build_File {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultbuild_File()) }
    }
}

#[derive(Debug)]
pub struct build_Form(go::GoUint32);

impl GoObject for build_Form {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for build_Form {}

impl Drop for build_Form {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for build_Form {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultbuild_Form()) }
    }
}

#[derive(Debug)]
pub struct build_Instance(go::GoUint32);

impl GoObject for build_Instance {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for build_Instance {}

impl Drop for build_Instance {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl build_Instance {
    pub fn getAllTags(&self) -> GoSlice<String> {
        let r = unsafe { go::get_build_Instance_AllTags(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setAllTags(&mut self, x: GoSlice<String>) {
        unsafe { go::set_build_Instance_AllTags(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getAllTags(&self) -> GoSlice<String> {
        let r = unsafe {
            go::get_build_Instance_AllTags(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setAllTags(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_AllTags(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getBuildFiles(&self) -> GoSlice<GoPtr<build_File>> {
        let r = unsafe { go::get_build_Instance_BuildFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setBuildFiles(&mut self, x: GoSlice<GoPtr<build_File>>) {
        unsafe { go::set_build_Instance_BuildFiles(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getBuildFiles(&self) -> GoSlice<GoPtr<build_File>> {
        let r = unsafe {
            go::get_build_Instance_BuildFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setBuildFiles(&mut self, x: GoSlice<GoPtr<build_File>>) {
        unsafe {
            go::set_build_Instance_BuildFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getCUEFiles(&self) -> GoSlice<String> {
        let r = unsafe { go::get_build_Instance_CUEFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setCUEFiles(&mut self, x: GoSlice<String>) {
        unsafe { go::set_build_Instance_CUEFiles(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getCUEFiles(&self) -> GoSlice<String> {
        let r = unsafe {
            go::get_build_Instance_CUEFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setCUEFiles(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_CUEFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getDataFiles(&self) -> GoSlice<String> {
        let r = unsafe { go::get_build_Instance_DataFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setDataFiles(&mut self, x: GoSlice<String>) {
        unsafe { go::set_build_Instance_DataFiles(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getDataFiles(&self) -> GoSlice<String> {
        let r = unsafe {
            go::get_build_Instance_DataFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setDataFiles(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_DataFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getDeps(&self) -> GoSlice<String> {
        let r = unsafe { go::get_build_Instance_Deps(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setDeps(&mut self, x: GoSlice<String>) {
        unsafe { go::set_build_Instance_Deps(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getDeps(&self) -> GoSlice<String> {
        let r =
            unsafe { go::get_build_Instance_Deps(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setDeps(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_Deps(self.dereference().handle(), x.handle())
        };
    }
}

impl build_Instance {
    pub fn getDepsErrors(&self) -> GoSlice<String> {
        let r = unsafe { go::get_build_Instance_DepsErrors(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setDepsErrors(&mut self, x: GoSlice<String>) {
        unsafe { go::set_build_Instance_DepsErrors(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getDepsErrors(&self) -> GoSlice<String> {
        let r = unsafe {
            go::get_build_Instance_DepsErrors(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setDepsErrors(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_DepsErrors(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getDir(&self) -> ffiString {
        let r = unsafe { go::get_build_Instance_Dir(self.handle()) };
        import_string(r)
    }

    pub fn setDir(&mut self, x: &str) {
        unsafe { go::set_build_Instance_Dir(self.handle(), export_string(x)) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getDir(&self) -> ffiString {
        let r =
            unsafe { go::get_build_Instance_Dir(self.dereference().handle()) };
        import_string(r)
    }

    pub fn setDir(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_Dir(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl build_Instance {
    pub fn getDisplayPath(&self) -> ffiString {
        let r = unsafe { go::get_build_Instance_DisplayPath(self.handle()) };
        import_string(r)
    }

    pub fn setDisplayPath(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_DisplayPath(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getDisplayPath(&self) -> ffiString {
        let r = unsafe {
            go::get_build_Instance_DisplayPath(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setDisplayPath(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_DisplayPath(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl build_Instance {
    pub fn getErr(&self) -> errors_Error {
        let r = unsafe { go::get_build_Instance_Err(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setErr(&mut self, x: errors_Error) {
        unsafe { go::set_build_Instance_Err(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getErr(&self) -> errors_Error {
        let r =
            unsafe { go::get_build_Instance_Err(self.dereference().handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setErr(&mut self, x: errors_Error) {
        unsafe {
            go::set_build_Instance_Err(self.dereference().handle(), x.handle())
        };
    }
}

impl build_Instance {
    pub fn getFiles(&self) -> GoSlice<GoPtr<ast_File>> {
        let r = unsafe { go::get_build_Instance_Files(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setFiles(&mut self, x: GoSlice<GoPtr<ast_File>>) {
        unsafe { go::set_build_Instance_Files(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getFiles(&self) -> GoSlice<GoPtr<ast_File>> {
        let r = unsafe {
            go::get_build_Instance_Files(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setFiles(&mut self, x: GoSlice<GoPtr<ast_File>>) {
        unsafe {
            go::set_build_Instance_Files(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getIgnoredCUEFiles(&self) -> GoSlice<String> {
        let r =
            unsafe { go::get_build_Instance_IgnoredCUEFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIgnoredCUEFiles(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_IgnoredCUEFiles(self.handle(), x.handle())
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getIgnoredCUEFiles(&self) -> GoSlice<String> {
        let r = unsafe {
            go::get_build_Instance_IgnoredCUEFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIgnoredCUEFiles(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_IgnoredCUEFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getIgnoredFiles(&self) -> GoSlice<GoPtr<build_File>> {
        let r = unsafe { go::get_build_Instance_IgnoredFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIgnoredFiles(&mut self, x: GoSlice<GoPtr<build_File>>) {
        unsafe {
            go::set_build_Instance_IgnoredFiles(self.handle(), x.handle())
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getIgnoredFiles(&self) -> GoSlice<GoPtr<build_File>> {
        let r = unsafe {
            go::get_build_Instance_IgnoredFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setIgnoredFiles(&mut self, x: GoSlice<GoPtr<build_File>>) {
        unsafe {
            go::set_build_Instance_IgnoredFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getImportComment(&self) -> ffiString {
        let r = unsafe { go::get_build_Instance_ImportComment(self.handle()) };
        import_string(r)
    }

    pub fn setImportComment(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_ImportComment(
                self.handle(),
                export_string(x),
            )
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getImportComment(&self) -> ffiString {
        let r = unsafe {
            go::get_build_Instance_ImportComment(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setImportComment(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_ImportComment(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl build_Instance {
    pub fn getImportPath(&self) -> ffiString {
        let r = unsafe { go::get_build_Instance_ImportPath(self.handle()) };
        import_string(r)
    }

    pub fn setImportPath(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_ImportPath(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getImportPath(&self) -> ffiString {
        let r = unsafe {
            go::get_build_Instance_ImportPath(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setImportPath(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_ImportPath(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl build_Instance {
    pub fn getImportPaths(&self) -> GoSlice<String> {
        let r = unsafe { go::get_build_Instance_ImportPaths(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setImportPaths(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_ImportPaths(self.handle(), x.handle())
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getImportPaths(&self) -> GoSlice<String> {
        let r = unsafe {
            go::get_build_Instance_ImportPaths(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setImportPaths(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_ImportPaths(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getImportPos(&self) -> GoMap<String, GoSlice<token_Pos>> {
        let r = unsafe { go::get_build_Instance_ImportPos(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setImportPos(&mut self, x: GoMap<String, GoSlice<token_Pos>>) {
        unsafe { go::set_build_Instance_ImportPos(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getImportPos(&self) -> GoMap<String, GoSlice<token_Pos>> {
        let r = unsafe {
            go::get_build_Instance_ImportPos(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setImportPos(&mut self, x: GoMap<String, GoSlice<token_Pos>>) {
        unsafe {
            go::set_build_Instance_ImportPos(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getImports(&self) -> GoSlice<GoPtr<build_Instance>> {
        let r = unsafe { go::get_build_Instance_Imports(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setImports(&mut self, x: GoSlice<GoPtr<build_Instance>>) {
        unsafe { go::set_build_Instance_Imports(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getImports(&self) -> GoSlice<GoPtr<build_Instance>> {
        let r = unsafe {
            go::get_build_Instance_Imports(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setImports(&mut self, x: GoSlice<GoPtr<build_Instance>>) {
        unsafe {
            go::set_build_Instance_Imports(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getIncomplete(&self) -> bool {
        let r = unsafe { go::get_build_Instance_Incomplete(self.handle()) };
        import_bool(r)
    }

    pub fn setIncomplete(&mut self, x: bool) {
        unsafe {
            go::set_build_Instance_Incomplete(self.handle(), export_bool(x))
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getIncomplete(&self) -> bool {
        let r = unsafe {
            go::get_build_Instance_Incomplete(self.dereference().handle())
        };
        import_bool(r)
    }

    pub fn setIncomplete(&mut self, x: bool) {
        unsafe {
            go::set_build_Instance_Incomplete(
                self.dereference().handle(),
                export_bool(x),
            )
        };
    }
}

impl build_Instance {
    pub fn getInvalidCUEFiles(&self) -> GoSlice<String> {
        let r =
            unsafe { go::get_build_Instance_InvalidCUEFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setInvalidCUEFiles(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_InvalidCUEFiles(self.handle(), x.handle())
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getInvalidCUEFiles(&self) -> GoSlice<String> {
        let r = unsafe {
            go::get_build_Instance_InvalidCUEFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setInvalidCUEFiles(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_InvalidCUEFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getInvalidFiles(&self) -> GoSlice<GoPtr<build_File>> {
        let r = unsafe { go::get_build_Instance_InvalidFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setInvalidFiles(&mut self, x: GoSlice<GoPtr<build_File>>) {
        unsafe {
            go::set_build_Instance_InvalidFiles(self.handle(), x.handle())
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getInvalidFiles(&self) -> GoSlice<GoPtr<build_File>> {
        let r = unsafe {
            go::get_build_Instance_InvalidFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setInvalidFiles(&mut self, x: GoSlice<GoPtr<build_File>>) {
        unsafe {
            go::set_build_Instance_InvalidFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getMatch(&self) -> GoSlice<String> {
        let r = unsafe { go::get_build_Instance_Match(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setMatch(&mut self, x: GoSlice<String>) {
        unsafe { go::set_build_Instance_Match(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getMatch(&self) -> GoSlice<String> {
        let r = unsafe {
            go::get_build_Instance_Match(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setMatch(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_Match(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getModule(&self) -> ffiString {
        let r = unsafe { go::get_build_Instance_Module(self.handle()) };
        import_string(r)
    }

    pub fn setModule(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_Module(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getModule(&self) -> ffiString {
        let r = unsafe {
            go::get_build_Instance_Module(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setModule(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_Module(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl build_Instance {
    pub fn getOrphanedFiles(&self) -> GoSlice<GoPtr<build_File>> {
        let r = unsafe { go::get_build_Instance_OrphanedFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOrphanedFiles(&mut self, x: GoSlice<GoPtr<build_File>>) {
        unsafe {
            go::set_build_Instance_OrphanedFiles(self.handle(), x.handle())
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getOrphanedFiles(&self) -> GoSlice<GoPtr<build_File>> {
        let r = unsafe {
            go::get_build_Instance_OrphanedFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setOrphanedFiles(&mut self, x: GoSlice<GoPtr<build_File>>) {
        unsafe {
            go::set_build_Instance_OrphanedFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getPkgName(&self) -> ffiString {
        let r = unsafe { go::get_build_Instance_PkgName(self.handle()) };
        import_string(r)
    }

    pub fn setPkgName(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_PkgName(self.handle(), export_string(x))
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getPkgName(&self) -> ffiString {
        let r = unsafe {
            go::get_build_Instance_PkgName(self.dereference().handle())
        };
        import_string(r)
    }

    pub fn setPkgName(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_PkgName(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl build_Instance {
    pub fn getRoot(&self) -> ffiString {
        let r = unsafe { go::get_build_Instance_Root(self.handle()) };
        import_string(r)
    }

    pub fn setRoot(&mut self, x: &str) {
        unsafe { go::set_build_Instance_Root(self.handle(), export_string(x)) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getRoot(&self) -> ffiString {
        let r =
            unsafe { go::get_build_Instance_Root(self.dereference().handle()) };
        import_string(r)
    }

    pub fn setRoot(&mut self, x: &str) {
        unsafe {
            go::set_build_Instance_Root(
                self.dereference().handle(),
                export_string(x),
            )
        };
    }
}

impl build_Instance {
    pub fn getScope(&self) -> GoPtr<build_Instance> {
        let r = unsafe { go::get_build_Instance_Scope(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setScope(&mut self, x: GoPtr<build_Instance>) {
        unsafe { go::set_build_Instance_Scope(self.handle(), x.handle()) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getScope(&self) -> GoPtr<build_Instance> {
        let r = unsafe {
            go::get_build_Instance_Scope(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setScope(&mut self, x: GoPtr<build_Instance>) {
        unsafe {
            go::set_build_Instance_Scope(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getStandard(&self) -> bool {
        let r = unsafe { go::get_build_Instance_Standard(self.handle()) };
        import_bool(r)
    }

    pub fn setStandard(&mut self, x: bool) {
        unsafe {
            go::set_build_Instance_Standard(self.handle(), export_bool(x))
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getStandard(&self) -> bool {
        let r = unsafe {
            go::get_build_Instance_Standard(self.dereference().handle())
        };
        import_bool(r)
    }

    pub fn setStandard(&mut self, x: bool) {
        unsafe {
            go::set_build_Instance_Standard(
                self.dereference().handle(),
                export_bool(x),
            )
        };
    }
}

impl build_Instance {
    pub fn getTestCUEFiles(&self) -> GoSlice<String> {
        let r = unsafe { go::get_build_Instance_TestCUEFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setTestCUEFiles(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_TestCUEFiles(self.handle(), x.handle())
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getTestCUEFiles(&self) -> GoSlice<String> {
        let r = unsafe {
            go::get_build_Instance_TestCUEFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setTestCUEFiles(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_TestCUEFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getToolCUEFiles(&self) -> GoSlice<String> {
        let r = unsafe { go::get_build_Instance_ToolCUEFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setToolCUEFiles(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_ToolCUEFiles(self.handle(), x.handle())
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getToolCUEFiles(&self) -> GoSlice<String> {
        let r = unsafe {
            go::get_build_Instance_ToolCUEFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setToolCUEFiles(&mut self, x: GoSlice<String>) {
        unsafe {
            go::set_build_Instance_ToolCUEFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getUnknownFiles(&self) -> GoSlice<GoPtr<build_File>> {
        let r = unsafe { go::get_build_Instance_UnknownFiles(self.handle()) };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setUnknownFiles(&mut self, x: GoSlice<GoPtr<build_File>>) {
        unsafe {
            go::set_build_Instance_UnknownFiles(self.handle(), x.handle())
        };
    }
}

impl GoPtr<build_Instance> {
    pub fn getUnknownFiles(&self) -> GoSlice<GoPtr<build_File>> {
        let r = unsafe {
            go::get_build_Instance_UnknownFiles(self.dereference().handle())
        };
        unsafe { GoObject::from_handle(r) }
    }

    pub fn setUnknownFiles(&mut self, x: GoSlice<GoPtr<build_File>>) {
        unsafe {
            go::set_build_Instance_UnknownFiles(
                self.dereference().handle(),
                x.handle(),
            )
        };
    }
}

impl build_Instance {
    pub fn getUser(&self) -> bool {
        let r = unsafe { go::get_build_Instance_User(self.handle()) };
        import_bool(r)
    }

    pub fn setUser(&mut self, x: bool) {
        unsafe { go::set_build_Instance_User(self.handle(), export_bool(x)) };
    }
}

impl GoPtr<build_Instance> {
    pub fn getUser(&self) -> bool {
        let r =
            unsafe { go::get_build_Instance_User(self.dereference().handle()) };
        import_bool(r)
    }

    pub fn setUser(&mut self, x: bool) {
        unsafe {
            go::set_build_Instance_User(
                self.dereference().handle(),
                export_bool(x),
            )
        };
    }
}

impl Default for build_Instance {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultbuild_Instance()) }
    }
}

#[derive(Debug)]
pub struct build_Interpretation(go::GoUint32);

impl GoObject for build_Interpretation {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for build_Interpretation {}

impl Drop for build_Interpretation {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl Default for build_Interpretation {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultbuild_Interpretation()) }
    }
}

#[derive(Debug)]
pub struct parser_DeprecationError(go::GoUint32);

impl GoObject for parser_DeprecationError {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for parser_DeprecationError {}

impl Drop for parser_DeprecationError {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl parser_DeprecationError {
    pub fn getVersion(&self) -> go::GoInt {
        let r =
            unsafe { go::get_parser_DeprecationError_Version(self.handle()) };
        r
    }

    pub fn setVersion(&mut self, x: go::GoInt) {
        unsafe { go::set_parser_DeprecationError_Version(self.handle(), x) };
    }
}

impl GoPtr<parser_DeprecationError> {
    pub fn getVersion(&self) -> go::GoInt {
        let r = unsafe {
            go::get_parser_DeprecationError_Version(self.dereference().handle())
        };
        r
    }

    pub fn setVersion(&mut self, x: go::GoInt) {
        unsafe {
            go::set_parser_DeprecationError_Version(
                self.dereference().handle(),
                x,
            )
        };
    }
}

impl Default for parser_DeprecationError {
    fn default() -> Self {
        unsafe { Self::from_handle(go::defaultparser_DeprecationError()) }
    }
}

pub fn All() -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_All() });
    (unsafe { GoObject::from_handle(r0) })
}

#[derive(Debug)]
pub struct cue_Option(go::GoUint32);

impl GoObject for cue_Option {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for cue_Option {}

impl Drop for cue_Option {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl cue_Value {
    pub fn AppendFloat(
        &self,
        a0: &GoSlice<go::GoUint8>,
        a1: go::GoUint8,
        a2: go::GoInt,
    ) -> (GoSlice<go::GoUint8>, ffiString) {
        let go::f_cue_0_AppendFloat_return { r0, r1 } =
            go::f_cue_0_AppendFloat_return::from(unsafe {
                go::f_cue_0_AppendFloat(self.handle(), a0.handle(), a1, a2)
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Value {
    pub fn AppendInt(
        &self,
        a0: &GoSlice<go::GoUint8>,
        a1: go::GoInt,
    ) -> (GoSlice<go::GoUint8>, ffiString) {
        let go::f_cue_0_AppendInt_return { r0, r1 } =
            go::f_cue_0_AppendInt_return::from(unsafe {
                go::f_cue_0_AppendInt(self.handle(), a0.handle(), a1)
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Value {
    pub fn Attribute(&self, a0: &str) -> (cue_Attribute) {
        let r0 = (unsafe {
            go::f_cue_0_Attribute(self.handle(), export_string(a0))
        });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn Attributes(a0: bool) -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_Attributes(export_bool(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

impl cue_Value {
    pub fn Bool(&self) -> (bool, ffiString) {
        let go::f_cue_0_Bool_return { r0, r1 } =
            go::f_cue_0_Bool_return::from(unsafe {
                go::f_cue_0_Bool(self.handle())
            });
        (import_bool(r0), import_string(r1))
    }
}

pub fn Build(
    a0: &GoSlice<GoPtr<build_Instance>>,
) -> (GoSlice<GoPtr<cue_Instance>>) {
    let r0 = (unsafe { go::f_cue_0_Build(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<cue_Instance> {
    pub fn Build(&self, a0: &GoPtr<build_Instance>) -> (GoPtr<cue_Instance>) {
        let r0 = (unsafe { go::f_cue_1_Build(self.handle(), a0.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<cue_Runtime> {
    pub fn Build(
        &self,
        a0: &GoPtr<build_Instance>,
    ) -> (GoPtr<cue_Instance>, ffiString) {
        let go::f_cue_2_Build_return { r0, r1 } =
            go::f_cue_2_Build_return::from(unsafe {
                go::f_cue_2_Build(self.handle(), a0.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Value {
    pub fn Bytes(&self) -> (GoSlice<go::GoUint8>, ffiString) {
        let go::f_cue_0_Bytes_return { r0, r1 } =
            go::f_cue_0_Bytes_return::from(unsafe {
                go::f_cue_0_Bytes(self.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Kind {
    pub fn CanString(&self) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_CanString(self.handle()) });
        (import_bool(r0))
    }
}

impl GoPtr<cue_Runtime> {
    pub fn Compile(
        &self,
        a0: &str,
        a1: &GoAny,
    ) -> (GoPtr<cue_Instance>, ffiString) {
        let go::f_cue_0_Compile_return { r0, r1 } =
            go::f_cue_0_Compile_return::from(unsafe {
                go::f_cue_0_Compile(
                    self.handle(),
                    export_string(a0),
                    a1.handle(),
                )
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl GoPtr<cue_Runtime> {
    pub fn CompileExpr(
        &self,
        a0: &ast_Expr,
    ) -> (GoPtr<cue_Instance>, ffiString) {
        let go::f_cue_0_CompileExpr_return { r0, r1 } =
            go::f_cue_0_CompileExpr_return::from(unsafe {
                go::f_cue_0_CompileExpr(self.handle(), a0.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl GoPtr<cue_Runtime> {
    pub fn CompileFile(
        &self,
        a0: &GoPtr<ast_File>,
    ) -> (GoPtr<cue_Instance>, ffiString) {
        let go::f_cue_0_CompileFile_return { r0, r1 } =
            go::f_cue_0_CompileFile_return::from(unsafe {
                go::f_cue_0_CompileFile(self.handle(), a0.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

pub fn Concrete(a0: bool) -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_Concrete(export_bool(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

impl cue_Value {
    pub fn Decimal(&self) -> (GoPtr<apd_Decimal>, ffiString) {
        let go::f_cue_0_Decimal_return { r0, r1 } =
            go::f_cue_0_Decimal_return::from(unsafe {
                go::f_cue_0_Decimal(self.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

#[derive(Debug)]
pub struct apd_Decimal(go::GoUint32);

impl GoObject for apd_Decimal {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for apd_Decimal {}

impl Drop for apd_Decimal {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl cue_Value {
    pub fn Decode(&self, a0: &GoAny) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_0_Decode(self.handle(), a0.handle()) });
        (import_string(r0))
    }
}

pub fn Def(a0: &str) -> (cue_Selector) {
    let r0 = (unsafe { go::f_cue_0_Def(export_string(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

impl cue_Value {
    pub fn Default(&self) -> (cue_Value, bool) {
        let go::f_cue_0_Default_return { r0, r1 } =
            go::f_cue_0_Default_return::from(unsafe {
                go::f_cue_0_Default(self.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_bool(r1))
    }
}

pub fn Definitions(a0: bool) -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_Definitions(export_bool(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn Dereference(a0: &cue_Value) -> (cue_Value) {
    let r0 = (unsafe { go::f_cue_0_Dereference(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn DisallowCycles(a0: bool) -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_DisallowCycles(export_bool(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<cue_Instance> {
    pub fn Doc(&self) -> (GoSlice<GoPtr<ast_CommentGroup>>) {
        let r0 = (unsafe { go::f_cue_0_Doc(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn Doc(&self) -> (GoSlice<GoPtr<ast_CommentGroup>>) {
        let r0 = (unsafe { go::f_cue_1_Doc(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn Docs(a0: bool) -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_Docs(export_bool(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

impl cue_Value {
    pub fn Elem(&self) -> (cue_Value, bool) {
        let go::f_cue_0_Elem_return { r0, r1 } =
            go::f_cue_0_Elem_return::from(unsafe {
                go::f_cue_0_Elem(self.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_bool(r1))
    }
}

impl cue_Value {
    pub fn Equals(&self, a0: &cue_Value) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_Equals(self.handle(), a0.handle()) });
        (import_bool(r0))
    }
}

impl GoPtr<cue_Attribute> {
    pub fn Err(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_0_Err(self.handle()) });
        (import_string(r0))
    }
}

impl cue_Path {
    pub fn Err(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_1_Err(self.handle()) });
        (import_string(r0))
    }
}

impl cue_Value {
    pub fn Err(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_2_Err(self.handle()) });
        (import_string(r0))
    }
}

impl GoPtr<cue_Instance> {
    pub fn Eval(&self, a0: &ast_Expr) -> (cue_Value) {
        let r0 = (unsafe { go::f_cue_0_Eval(self.handle(), a0.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn Eval(&self) -> (cue_Value) {
        let r0 = (unsafe { go::f_cue_1_Eval(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn Exists(&self) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_Exists(self.handle()) });
        (import_bool(r0))
    }
}

impl cue_Value {
    pub fn Expr(&self) -> (cue_Op, GoSlice<cue_Value>) {
        let go::f_cue_0_Expr_return { r0, r1 } =
            go::f_cue_0_Expr_return::from(unsafe {
                go::f_cue_0_Expr(self.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, unsafe {
            GoObject::from_handle(r1)
        })
    }
}

impl GoPtr<cue_Struct> {
    pub fn Field(&self, a0: go::GoInt) -> (cue_FieldInfo) {
        let r0 = (unsafe { go::f_cue_0_Field(self.handle(), a0) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<cue_Struct> {
    pub fn FieldByName(
        &self,
        a0: &str,
        a1: bool,
    ) -> (cue_FieldInfo, ffiString) {
        let go::f_cue_0_FieldByName_return { r0, r1 } =
            go::f_cue_0_FieldByName_return::from(unsafe {
                go::f_cue_0_FieldByName(
                    self.handle(),
                    export_string(a0),
                    export_bool(a1),
                )
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Value {
    pub fn FieldByName(
        &self,
        a0: &str,
        a1: bool,
    ) -> (cue_FieldInfo, ffiString) {
        let go::f_cue_1_FieldByName_return { r0, r1 } =
            go::f_cue_1_FieldByName_return::from(unsafe {
                go::f_cue_1_FieldByName(
                    self.handle(),
                    export_string(a0),
                    export_bool(a1),
                )
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl GoPtr<cue_Struct> {
    pub fn Fields(&self, a0: &GoSlice<cue_Option>) -> (GoPtr<cue_Iterator>) {
        let r0 = (unsafe { go::f_cue_0_Fields(self.handle(), a0.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn Fields(
        &self,
        a0: &GoSlice<cue_Option>,
    ) -> (GoPtr<cue_Iterator>, ffiString) {
        let go::f_cue_1_Fields_return { r0, r1 } =
            go::f_cue_1_Fields_return::from(unsafe {
                go::f_cue_1_Fields(self.handle(), a0.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl GoPtr<cue_Instance> {
    pub fn Fill(
        &self,
        a0: &GoAny,
        a1: &GoSlice<String>,
    ) -> (GoPtr<cue_Instance>, ffiString) {
        let go::f_cue_0_Fill_return { r0, r1 } =
            go::f_cue_0_Fill_return::from(unsafe {
                go::f_cue_0_Fill(self.handle(), a0.handle(), a1.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Value {
    pub fn Fill(&self, a0: &GoAny, a1: &GoSlice<String>) -> (cue_Value) {
        let r0 = (unsafe {
            go::f_cue_1_Fill(self.handle(), a0.handle(), a1.handle())
        });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn Final() -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_Final() });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<cue_Attribute> {
    pub fn Flag(&self, a0: go::GoInt, a1: &str) -> (bool, ffiString) {
        let go::f_cue_0_Flag_return { r0, r1 } =
            go::f_cue_0_Flag_return::from(unsafe {
                go::f_cue_0_Flag(self.handle(), a0, export_string(a1))
            });
        (import_bool(r0), import_string(r1))
    }
}

impl cue_Value {
    pub fn Float64(&self) -> (go::GoFloat64, ffiString) {
        let go::f_cue_0_Float64_return { r0, r1 } =
            go::f_cue_0_Float64_return::from(unsafe {
                go::f_cue_0_Float64(self.handle())
            });
        (r0, import_string(r1))
    }
}

impl cue_Value {
    pub fn Format(&self, a0: &fmt_State, a1: go::GoInt32) {
        (unsafe { go::f_cue_0_Format(self.handle(), a0.handle(), a1) });
        ()
    }
}

#[derive(Debug)]
pub struct fmt_State(go::GoUint32);

impl GoObject for fmt_State {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for fmt_State {}

impl Drop for fmt_State {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl GoPtr<cue_Runtime> {
    pub fn FromExpr(&self, a0: &ast_Expr) -> (GoPtr<cue_Instance>, ffiString) {
        let go::f_cue_0_FromExpr_return { r0, r1 } =
            go::f_cue_0_FromExpr_return::from(unsafe {
                go::f_cue_0_FromExpr(self.handle(), a0.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

pub fn Hidden(a0: bool) -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_Hidden(export_bool(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<cue_Instance> {
    pub fn ID(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_0_ID(self.handle()) });
        (import_string(r0))
    }
}

impl cue_Value {
    pub fn IncompleteKind(&self) -> (cue_Kind) {
        let r0 = (unsafe { go::f_cue_0_IncompleteKind(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn Index(a0: go::GoInt) -> (cue_Selector) {
    let r0 = (unsafe { go::f_cue_0_Index(a0) });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<cue_Attribute> {
    pub fn Int(&self, a0: go::GoInt) -> (go::GoInt64, ffiString) {
        let go::f_cue_0_Int_return { r0, r1 } =
            go::f_cue_0_Int_return::from(unsafe {
                go::f_cue_0_Int(self.handle(), a0)
            });
        (r0, import_string(r1))
    }
}

impl cue_Value {
    pub fn Int(&self, a0: &GoPtr<big_Int>) -> (GoPtr<big_Int>, ffiString) {
        let go::f_cue_1_Int_return { r0, r1 } =
            go::f_cue_1_Int_return::from(unsafe {
                go::f_cue_1_Int(self.handle(), a0.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

#[derive(Debug)]
pub struct big_Int(go::GoUint32);

impl GoObject for big_Int {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for big_Int {}

impl Drop for big_Int {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl cue_Value {
    pub fn Int64(&self) -> (go::GoInt64, ffiString) {
        let go::f_cue_0_Int64_return { r0, r1 } =
            go::f_cue_0_Int64_return::from(unsafe {
                go::f_cue_0_Int64(self.handle())
            });
        (r0, import_string(r1))
    }
}

impl cue_Kind {
    pub fn IsAnyOf(&self, a0: &cue_Kind) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_IsAnyOf(self.handle(), a0.handle()) });
        (import_bool(r0))
    }
}

impl cue_Value {
    pub fn IsClosed(&self) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_IsClosed(self.handle()) });
        (import_bool(r0))
    }
}

impl cue_Value {
    pub fn IsConcrete(&self) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_IsConcrete(self.handle()) });
        (import_bool(r0))
    }
}

impl GoPtr<cue_Iterator> {
    pub fn IsDefinition(&self) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_IsDefinition(self.handle()) });
        (import_bool(r0))
    }
}

impl GoPtr<cue_Iterator> {
    pub fn IsHidden(&self) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_IsHidden(self.handle()) });
        (import_bool(r0))
    }
}

impl GoPtr<cue_Iterator> {
    pub fn IsOptional(&self) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_IsOptional(self.handle()) });
        (import_bool(r0))
    }
}

impl cue_Value {
    pub fn Kind(&self) -> (cue_Kind) {
        let r0 = (unsafe { go::f_cue_0_Kind(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<cue_Iterator> {
    pub fn Label(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_0_Label(self.handle()) });
        (import_string(r0))
    }
}

impl cue_Value {
    pub fn Label(&self) -> (ffiString, bool) {
        let go::f_cue_1_Label_return { r0, r1 } =
            go::f_cue_1_Label_return::from(unsafe {
                go::f_cue_1_Label(self.handle())
            });
        (import_string(r0), import_bool(r1))
    }
}

impl GoPtr<cue_Struct> {
    pub fn Len(&self) -> (go::GoInt) {
        let r0 = (unsafe { go::f_cue_0_Len(self.handle()) });
        (r0)
    }
}

impl cue_Value {
    pub fn Len(&self) -> (cue_Value) {
        let r0 = (unsafe { go::f_cue_1_Len(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn List(&self) -> (cue_Iterator, ffiString) {
        let go::f_cue_0_List_return { r0, r1 } =
            go::f_cue_0_List_return::from(unsafe {
                go::f_cue_0_List(self.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl GoPtr<cue_Attribute> {
    pub fn Lookup(
        &self,
        a0: go::GoInt,
        a1: &str,
    ) -> (ffiString, bool, ffiString) {
        let go::f_cue_0_Lookup_return { r0, r1, r2 } =
            go::f_cue_0_Lookup_return::from(unsafe {
                go::f_cue_0_Lookup(self.handle(), a0, export_string(a1))
            });
        (import_string(r0), import_bool(r1), import_string(r2))
    }
}

impl GoPtr<cue_Instance> {
    pub fn Lookup(&self, a0: &GoSlice<String>) -> (cue_Value) {
        let r0 = (unsafe { go::f_cue_1_Lookup(self.handle(), a0.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn Lookup(&self, a0: &GoSlice<String>) -> (cue_Value) {
        let r0 = (unsafe { go::f_cue_2_Lookup(self.handle(), a0.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<cue_Instance> {
    pub fn LookupDef(&self, a0: &str) -> (cue_Value) {
        let r0 = (unsafe {
            go::f_cue_0_LookupDef(self.handle(), export_string(a0))
        });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn LookupDef(&self, a0: &str) -> (cue_Value) {
        let r0 = (unsafe {
            go::f_cue_1_LookupDef(self.handle(), export_string(a0))
        });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<cue_Instance> {
    pub fn LookupField(
        &self,
        a0: &GoSlice<String>,
    ) -> (cue_FieldInfo, ffiString) {
        let go::f_cue_0_LookupField_return { r0, r1 } =
            go::f_cue_0_LookupField_return::from(unsafe {
                go::f_cue_0_LookupField(self.handle(), a0.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Value {
    pub fn LookupField(&self, a0: &str) -> (cue_FieldInfo, ffiString) {
        let go::f_cue_1_LookupField_return { r0, r1 } =
            go::f_cue_1_LookupField_return::from(unsafe {
                go::f_cue_1_LookupField(self.handle(), export_string(a0))
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Value {
    pub fn LookupPath(&self, a0: &cue_Path) -> (cue_Value) {
        let r0 =
            (unsafe { go::f_cue_0_LookupPath(self.handle(), a0.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn MakePath(a0: &GoSlice<cue_Selector>) -> (cue_Path) {
    let r0 = (unsafe { go::f_cue_0_MakePath(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

impl cue_Value {
    pub fn MantExp(&self, a0: &GoPtr<big_Int>) -> (go::GoInt, ffiString) {
        let go::f_cue_0_MantExp_return { r0, r1 } =
            go::f_cue_0_MantExp_return::from(unsafe {
                go::f_cue_0_MantExp(self.handle(), a0.handle())
            });
        (r0, import_string(r1))
    }
}

impl GoPtr<cue_Runtime> {
    pub fn Marshal(
        &self,
        a0: &GoSlice<GoPtr<cue_Instance>>,
    ) -> (GoSlice<go::GoUint8>, ffiString) {
        let go::f_cue_0_Marshal_return { r0, r1 } =
            go::f_cue_0_Marshal_return::from(unsafe {
                go::f_cue_0_Marshal(self.handle(), a0.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Value {
    pub fn MarshalJSON(&self) -> (GoSlice<go::GoUint8>, ffiString) {
        let go::f_cue_0_MarshalJSON_return { r0, r1 } =
            go::f_cue_0_MarshalJSON_return::from(unsafe {
                go::f_cue_0_MarshalJSON(self.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

pub fn Merge(a0: &GoSlice<GoPtr<cue_Instance>>) -> (GoPtr<cue_Instance>) {
    let r0 = (unsafe { go::f_cue_0_Merge(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<cue_Iterator> {
    pub fn Next(&self) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_Next(self.handle()) });
        (import_bool(r0))
    }
}

impl cue_Value {
    pub fn Null(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_0_Null(self.handle()) });
        (import_string(r0))
    }
}

pub fn Optional(a0: bool) -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_Optional(export_bool(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<cue_Runtime> {
    pub fn Parse(
        &self,
        a0: &str,
        a1: &GoAny,
    ) -> (GoPtr<cue_Instance>, ffiString) {
        let go::f_cue_0_Parse_return { r0, r1 } =
            go::f_cue_0_Parse_return::from(unsafe {
                go::f_cue_0_Parse(self.handle(), export_string(a0), a1.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

pub fn ParsePath(a0: &str) -> (cue_Path) {
    let r0 = (unsafe { go::f_cue_0_ParsePath(export_string(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

impl cue_Value {
    pub fn Path(&self) -> (cue_Path) {
        let r0 = (unsafe { go::f_cue_0_Path(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_cue_0_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

#[derive(Debug)]
pub struct token_Pos(go::GoUint32);

impl GoObject for token_Pos {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for token_Pos {}

impl Drop for token_Pos {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

pub fn Raw() -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_Raw() });
    (unsafe { GoObject::from_handle(r0) })
}

impl cue_Value {
    pub fn Reader(&self) -> (io_Reader, ffiString) {
        let go::f_cue_0_Reader_return { r0, r1 } =
            go::f_cue_0_Reader_return::from(unsafe {
                go::f_cue_0_Reader(self.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

#[derive(Debug)]
pub struct io_Reader(go::GoUint32);

impl GoObject for io_Reader {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for io_Reader {}

impl Drop for io_Reader {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl cue_Value {
    pub fn Reference(&self) -> (GoPtr<cue_Instance>, GoSlice<String>) {
        let go::f_cue_0_Reference_return { r0, r1 } =
            go::f_cue_0_Reference_return::from(unsafe {
                go::f_cue_0_Reference(self.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, unsafe {
            GoObject::from_handle(r1)
        })
    }
}

pub fn ResolveReferences(a0: bool) -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_ResolveReferences(export_bool(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn Schema() -> (cue_Option) {
    let r0 = (unsafe { go::f_cue_0_Schema() });
    (unsafe { GoObject::from_handle(r0) })
}

impl cue_Path {
    pub fn Selectors(&self) -> (GoSlice<cue_Selector>) {
        let r0 = (unsafe { go::f_cue_0_Selectors(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn Source(&self) -> (ast_Node) {
        let r0 = (unsafe { go::f_cue_0_Source(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn Split(&self) -> (GoSlice<cue_Value>) {
        let r0 = (unsafe { go::f_cue_0_Split(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn Str(a0: &str) -> (cue_Selector) {
    let r0 = (unsafe { go::f_cue_0_Str(export_string(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<cue_Attribute> {
    pub fn String(&self, a0: go::GoInt) -> (ffiString, ffiString) {
        let go::f_cue_0_String_return { r0, r1 } =
            go::f_cue_0_String_return::from(unsafe {
                go::f_cue_0_String(self.handle(), a0)
            });
        (import_string(r0), import_string(r1))
    }
}

impl cue_Kind {
    pub fn String(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_1_String(self.handle()) });
        (import_string(r0))
    }
}

impl cue_Op {
    pub fn String(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_2_String(self.handle()) });
        (import_string(r0))
    }
}

impl cue_Path {
    pub fn String(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_3_String(self.handle()) });
        (import_string(r0))
    }
}

impl cue_Selector {
    pub fn String(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_4_String(self.handle()) });
        (import_string(r0))
    }
}

impl cue_Value {
    pub fn String(&self) -> (ffiString, ffiString) {
        let go::f_cue_5_String_return { r0, r1 } =
            go::f_cue_5_String_return::from(unsafe {
                go::f_cue_5_String(self.handle())
            });
        (import_string(r0), import_string(r1))
    }
}

impl cue_Value {
    pub fn Struct(&self) -> (GoPtr<cue_Struct>, ffiString) {
        let go::f_cue_0_Struct_return { r0, r1 } =
            go::f_cue_0_Struct_return::from(unsafe {
                go::f_cue_0_Struct(self.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Value {
    pub fn Subsume(
        &self,
        a0: &cue_Value,
        a1: &GoSlice<cue_Option>,
    ) -> (ffiString) {
        let r0 = (unsafe {
            go::f_cue_0_Subsume(self.handle(), a0.handle(), a1.handle())
        });
        (import_string(r0))
    }
}

impl cue_Value {
    pub fn Subsumes(&self, a0: &cue_Value) -> (bool) {
        let r0 = (unsafe { go::f_cue_0_Subsumes(self.handle(), a0.handle()) });
        (import_bool(r0))
    }
}

impl cue_Value {
    pub fn Syntax(&self, a0: &GoSlice<cue_Option>) -> (ast_Node) {
        let r0 = (unsafe { go::f_cue_0_Syntax(self.handle(), a0.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Op {
    pub fn Token(&self) -> (token_Token) {
        let r0 = (unsafe { go::f_cue_0_Token(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

#[derive(Debug)]
pub struct token_Token(go::GoUint32);

impl GoObject for token_Token {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for token_Token {}

impl Drop for token_Token {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl cue_Kind {
    pub fn TypeString(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_0_TypeString(self.handle()) });
        (import_string(r0))
    }
}

impl cue_Value {
    pub fn Uint64(&self) -> (go::GoUint64, ffiString) {
        let go::f_cue_0_Uint64_return { r0, r1 } =
            go::f_cue_0_Uint64_return::from(unsafe {
                go::f_cue_0_Uint64(self.handle())
            });
        (r0, import_string(r1))
    }
}

impl cue_Value {
    pub fn Unify(&self, a0: &cue_Value) -> (cue_Value) {
        let r0 = (unsafe { go::f_cue_0_Unify(self.handle(), a0.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl cue_Value {
    pub fn UnifyAccept(&self, a0: &cue_Value, a1: &cue_Value) -> (cue_Value) {
        let r0 = (unsafe {
            go::f_cue_0_UnifyAccept(self.handle(), a0.handle(), a1.handle())
        });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<cue_Runtime> {
    pub fn Unmarshal(
        &self,
        a0: &GoSlice<go::GoUint8>,
    ) -> (GoSlice<GoPtr<cue_Instance>>, ffiString) {
        let go::f_cue_0_Unmarshal_return { r0, r1 } =
            go::f_cue_0_Unmarshal_return::from(unsafe {
                go::f_cue_0_Unmarshal(self.handle(), a0.handle())
            });
        (unsafe { GoObject::from_handle(r0) }, import_string(r1))
    }
}

impl cue_Value {
    pub fn Validate(&self, a0: &GoSlice<cue_Option>) -> (ffiString) {
        let r0 = (unsafe { go::f_cue_0_Validate(self.handle(), a0.handle()) });
        (import_string(r0))
    }
}

impl GoPtr<cue_Instance> {
    pub fn Value(&self) -> (cue_Value) {
        let r0 = (unsafe { go::f_cue_0_Value(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<cue_Iterator> {
    pub fn Value(&self) -> (cue_Value) {
        let r0 = (unsafe { go::f_cue_1_Value(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn AddComment(a0: &ast_Node, a1: &GoPtr<ast_CommentGroup>) {
    (unsafe { go::f_ast_0_AddComment(a0.handle(), a1.handle()) });
    ()
}

impl GoPtr<ast_Comment> {
    pub fn AddComment(&self, a0: &GoPtr<ast_CommentGroup>) {
        (unsafe { go::f_ast_1_AddComment(self.handle(), a0.handle()) });
        ()
    }
}

impl GoPtr<ast_CommentGroup> {
    pub fn AddComment(&self, a0: &GoPtr<ast_CommentGroup>) {
        (unsafe { go::f_ast_2_AddComment(self.handle(), a0.handle()) });
        ()
    }
}

pub fn Comments(a0: &ast_Node) -> (GoSlice<GoPtr<ast_CommentGroup>>) {
    let r0 = (unsafe { go::f_ast_0_Comments(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<ast_Comment> {
    pub fn Comments(&self) -> (GoSlice<GoPtr<ast_CommentGroup>>) {
        let r0 = (unsafe { go::f_ast_1_Comments(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_CommentGroup> {
    pub fn Comments(&self) -> (GoSlice<GoPtr<ast_CommentGroup>>) {
        let r0 = (unsafe { go::f_ast_2_Comments(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn Embed(a0: &ast_Expr) -> (GoPtr<ast_embedding>) {
    let r0 = (unsafe { go::f_ast_0_Embed(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

#[derive(Debug)]
pub struct ast_embedding(go::GoUint32);

impl GoObject for ast_embedding {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for ast_embedding {}

impl Drop for ast_embedding {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl GoPtr<ast_Alias> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_0_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Attribute> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_1_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_BadDecl> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_2_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_BadExpr> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_3_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_BasicLit> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_4_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_BinaryExpr> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_5_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_BottomLit> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_6_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_CallExpr> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_7_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Comment> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_8_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_CommentGroup> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_9_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Comprehension> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_10_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Ellipsis> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_11_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_EmbedDecl> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_12_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Field> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_13_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_File> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_14_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ForClause> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_15_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Ident> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_16_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_IfClause> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_17_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ImportDecl> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_18_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ImportSpec> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_19_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_IndexExpr> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_20_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Interpolation> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_21_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_LetClause> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_22_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ListComprehension> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_23_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ListLit> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_24_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Package> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_25_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ParenExpr> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_26_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_SelectorExpr> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_27_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_SliceExpr> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_28_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_StructLit> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_29_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_TemplateLabel> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_30_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_UnaryExpr> {
    pub fn End(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_31_End(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn IsValidIdent(a0: &str) -> (bool) {
    let r0 = (unsafe { go::f_ast_0_IsValidIdent(export_string(a0)) });
    (import_bool(r0))
}

pub fn LabelName(a0: &ast_Label) -> (ffiString, bool, ffiString) {
    let go::f_ast_0_LabelName_return { r0, r1, r2 } =
        go::f_ast_0_LabelName_return::from(unsafe {
            go::f_ast_0_LabelName(a0.handle())
        });
    (import_string(r0), import_bool(r1), import_string(r2))
}

pub fn Name(a0: &ast_Node) -> (ffiString) {
    let r0 = (unsafe { go::f_ast_0_Name(a0.handle()) });
    (import_string(r0))
}

pub fn NewBinExpr(a0: &token_Token, a1: &GoSlice<ast_Expr>) -> (ast_Expr) {
    let r0 = (unsafe { go::f_ast_0_NewBinExpr(a0.handle(), a1.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn NewBool(a0: bool) -> (GoPtr<ast_BasicLit>) {
    let r0 = (unsafe { go::f_ast_0_NewBool(export_bool(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn NewCall(a0: &ast_Expr, a1: &GoSlice<ast_Expr>) -> (GoPtr<ast_CallExpr>) {
    let r0 = (unsafe { go::f_ast_0_NewCall(a0.handle(), a1.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn NewIdent(a0: &str) -> (GoPtr<ast_Ident>) {
    let r0 = (unsafe { go::f_ast_0_NewIdent(export_string(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn NewImport(a0: &GoPtr<ast_Ident>, a1: &str) -> (GoPtr<ast_ImportSpec>) {
    let r0 = (unsafe { go::f_ast_0_NewImport(a0.handle(), export_string(a1)) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn NewList(a0: &GoSlice<ast_Expr>) -> (GoPtr<ast_ListLit>) {
    let r0 = (unsafe { go::f_ast_0_NewList(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn NewLit(a0: &token_Token, a1: &str) -> (GoPtr<ast_BasicLit>) {
    let r0 = (unsafe { go::f_ast_0_NewLit(a0.handle(), export_string(a1)) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn NewNull() -> (GoPtr<ast_BasicLit>) {
    let r0 = (unsafe { go::f_ast_0_NewNull() });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn NewSel(a0: &ast_Expr, a1: &GoSlice<String>) -> (ast_Expr) {
    let r0 = (unsafe { go::f_ast_0_NewSel(a0.handle(), a1.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn NewString(a0: &str) -> (GoPtr<ast_BasicLit>) {
    let r0 = (unsafe { go::f_ast_0_NewString(export_string(a0)) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn NewStruct(a0: &GoSlice<GoAny>) -> (GoPtr<ast_StructLit>) {
    let r0 = (unsafe { go::f_ast_0_NewStruct(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<ast_File> {
    pub fn PackageName(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_ast_0_PackageName(self.handle()) });
        (import_string(r0))
    }
}

pub fn ParseIdent(a0: &GoPtr<ast_Ident>) -> (ffiString, ffiString) {
    let go::f_ast_0_ParseIdent_return { r0, r1 } =
        go::f_ast_0_ParseIdent_return::from(unsafe {
            go::f_ast_0_ParseIdent(a0.handle())
        });
    (import_string(r0), import_string(r1))
}

impl GoPtr<ast_Alias> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_0_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Attribute> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_1_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_BadDecl> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_2_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_BadExpr> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_3_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_BasicLit> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_4_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_BinaryExpr> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_5_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_BottomLit> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_6_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_CallExpr> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_7_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Comment> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_8_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_CommentGroup> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_9_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Comprehension> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_10_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Ellipsis> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_11_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_EmbedDecl> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_12_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Field> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_13_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_File> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_14_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ForClause> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_15_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Ident> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_16_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_IfClause> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_17_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ImportDecl> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_18_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ImportSpec> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_19_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_IndexExpr> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_20_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Interpolation> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_21_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_LetClause> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_22_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ListComprehension> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_23_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ListLit> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_24_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_Package> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_25_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_ParenExpr> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_26_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_SelectorExpr> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_27_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_SliceExpr> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_28_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_StructLit> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_29_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_TemplateLabel> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_30_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_UnaryExpr> {
    pub fn Pos(&self) -> (token_Pos) {
        let r0 = (unsafe { go::f_ast_31_Pos(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<ast_File> {
    pub fn Preamble(&self) -> (GoSlice<ast_Decl>) {
        let r0 = (unsafe { go::f_ast_0_Preamble(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn QuoteIdent(a0: &str) -> (ffiString, ffiString) {
    let go::f_ast_0_QuoteIdent_return { r0, r1 } =
        go::f_ast_0_QuoteIdent_return::from(unsafe {
            go::f_ast_0_QuoteIdent(export_string(a0))
        });
    (import_string(r0), import_string(r1))
}

pub fn SetComments(a0: &ast_Node, a1: &GoSlice<GoPtr<ast_CommentGroup>>) {
    (unsafe { go::f_ast_0_SetComments(a0.handle(), a1.handle()) });
    ()
}

pub fn SetPos(a0: &ast_Node, a1: &token_Pos) {
    (unsafe { go::f_ast_0_SetPos(a0.handle(), a1.handle()) });
    ()
}

pub fn SetRelPos(a0: &ast_Node, a1: &token_RelPos) {
    (unsafe { go::f_ast_0_SetRelPos(a0.handle(), a1.handle()) });
    ()
}

#[derive(Debug)]
pub struct token_RelPos(go::GoUint32);

impl GoObject for token_RelPos {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for token_RelPos {}

impl Drop for token_RelPos {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl GoPtr<ast_Attribute> {
    pub fn Split(&self) -> (ffiString, ffiString) {
        let go::f_ast_0_Split_return { r0, r1 } =
            go::f_ast_0_Split_return::from(unsafe {
                go::f_ast_0_Split(self.handle())
            });
        (import_string(r0), import_string(r1))
    }
}

impl GoPtr<ast_Ident> {
    pub fn String(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_ast_0_String(self.handle()) });
        (import_string(r0))
    }
}

impl GoPtr<ast_CommentGroup> {
    pub fn Text(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_ast_0_Text(self.handle()) });
        (import_string(r0))
    }
}

pub fn ApplyRecursively(a0: &ast_Node) -> (ast_Node) {
    let r0 = (unsafe { go::f_astutil_0_ApplyRecursively(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn CopyComments(a0: &ast_Node, a1: &ast_Node) {
    (unsafe { go::f_astutil_0_CopyComments(a0.handle(), a1.handle()) });
    ()
}

pub fn CopyMeta(a0: &ast_Node, a1: &ast_Node) -> (ast_Node) {
    let r0 = (unsafe { go::f_astutil_0_CopyMeta(a0.handle(), a1.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn CopyPosition(a0: &ast_Node, a1: &ast_Node) {
    (unsafe { go::f_astutil_0_CopyPosition(a0.handle(), a1.handle()) });
    ()
}

pub fn ParseImportSpec(
    a0: &GoPtr<ast_ImportSpec>,
) -> (astutil_ImportInfo, ffiString) {
    let go::f_astutil_0_ParseImportSpec_return { r0, r1 } =
        go::f_astutil_0_ParseImportSpec_return::from(unsafe {
            go::f_astutil_0_ParseImportSpec(a0.handle())
        });
    (unsafe { GoObject::from_handle(r0) }, import_string(r1))
}

pub fn Resolve(a0: &GoPtr<ast_File>, a1: &astutil_ErrFunc) {
    (unsafe { go::f_astutil_0_Resolve(a0.handle(), a1.handle()) });
    ()
}

#[derive(Debug)]
pub struct astutil_ErrFunc(go::GoUint32);

impl GoObject for astutil_ErrFunc {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for astutil_ErrFunc {}

impl Drop for astutil_ErrFunc {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

pub fn ResolveExpr(a0: &ast_Expr, a1: &astutil_ErrFunc) {
    (unsafe { go::f_astutil_0_ResolveExpr(a0.handle(), a1.handle()) });
    ()
}

pub fn Sanitize(a0: &GoPtr<ast_File>) -> (ffiString) {
    let r0 = (unsafe { go::f_astutil_0_Sanitize(a0.handle()) });
    (import_string(r0))
}

pub fn ToFile(a0: &ast_Expr) -> (GoPtr<ast_File>, ffiString) {
    let go::f_astutil_0_ToFile_return { r0, r1 } =
        go::f_astutil_0_ToFile_return::from(unsafe {
            go::f_astutil_0_ToFile(a0.handle())
        });
    (unsafe { GoObject::from_handle(r0) }, import_string(r1))
}

impl GoPtr<build_Instance> {
    pub fn Abs(&self, a0: &str) -> (ffiString) {
        let r0 =
            (unsafe { go::f_build_0_Abs(self.handle(), export_string(a0)) });
        (import_string(r0))
    }
}

impl GoPtr<build_Instance> {
    pub fn AddFile(&self, a0: &str, a1: &GoAny) -> (ffiString) {
        let r0 = (unsafe {
            go::f_build_0_AddFile(self.handle(), export_string(a0), a1.handle())
        });
        (import_string(r0))
    }
}

impl GoPtr<build_Instance> {
    pub fn AddSyntax(&self, a0: &GoPtr<ast_File>) -> (errors_Error) {
        let r0 =
            (unsafe { go::f_build_0_AddSyntax(self.handle(), a0.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

#[derive(Debug)]
pub struct errors_Error(go::GoUint32);

impl GoObject for errors_Error {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for errors_Error {}

impl Drop for errors_Error {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl GoPtr<build_Instance> {
    pub fn Complete(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_build_0_Complete(self.handle()) });
        (import_string(r0))
    }
}

impl GoPtr<build_Instance> {
    pub fn Context(&self) -> (GoPtr<build_Context>) {
        let r0 = (unsafe { go::f_build_0_Context(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<build_Instance> {
    pub fn Dependencies(&self) -> (GoSlice<GoPtr<build_Instance>>) {
        let r0 = (unsafe { go::f_build_0_Dependencies(self.handle()) });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<build_Instance> {
    pub fn ID(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_build_0_ID(self.handle()) });
        (import_string(r0))
    }
}

pub fn IsLocalImport(a0: &str) -> (bool) {
    let r0 = (unsafe { go::f_build_0_IsLocalImport(export_string(a0)) });
    (import_bool(r0))
}

pub fn Loader(a0: &build_LoadFunc) -> (build_Option) {
    let r0 = (unsafe { go::f_build_0_Loader(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

#[derive(Debug)]
pub struct build_LoadFunc(go::GoUint32);

impl GoObject for build_LoadFunc {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for build_LoadFunc {}

impl Drop for build_LoadFunc {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

#[derive(Debug)]
pub struct build_Option(go::GoUint32);

impl GoObject for build_Option {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for build_Option {}

impl Drop for build_Option {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

impl GoPtr<build_Instance> {
    pub fn LookupImport(&self, a0: &str) -> (GoPtr<build_Instance>) {
        let r0 = (unsafe {
            go::f_build_0_LookupImport(self.handle(), export_string(a0))
        });
        (unsafe { GoObject::from_handle(r0) })
    }
}

pub fn NewContext(a0: &GoSlice<build_Option>) -> (GoPtr<build_Context>) {
    let r0 = (unsafe { go::f_build_0_NewContext(a0.handle()) });
    (unsafe { GoObject::from_handle(r0) })
}

impl GoPtr<build_Context> {
    pub fn NewInstance(
        &self,
        a0: &str,
        a1: &build_LoadFunc,
    ) -> (GoPtr<build_Instance>) {
        let r0 = (unsafe {
            go::f_build_0_NewInstance(
                self.handle(),
                export_string(a0),
                a1.handle(),
            )
        });
        (unsafe { GoObject::from_handle(r0) })
    }
}

impl GoPtr<build_Instance> {
    pub fn ReportError(&self, a0: &errors_Error) {
        (unsafe { go::f_build_0_ReportError(self.handle(), a0.handle()) });
        ()
    }
}

impl GoPtr<parser_DeprecationError> {
    pub fn Error(&self) -> (ffiString) {
        let r0 = (unsafe { go::f_parser_0_Error(self.handle()) });
        (import_string(r0))
    }
}

pub fn FileOffset(a0: go::GoInt) -> (parser_Option) {
    let r0 = (unsafe { go::f_parser_0_FileOffset(a0) });
    (unsafe { GoObject::from_handle(r0) })
}

#[derive(Debug)]
pub struct parser_Option(go::GoUint32);

impl GoObject for parser_Option {
    unsafe fn from_handle(handle: go::GoUint32) -> Self {
        Self(handle)
    }

    fn handle(&self) -> go::GoUint32 {
        self.0
    }
}

impl NotGoAny for parser_Option {}

impl Drop for parser_Option {
    fn drop(&mut self) {
        unsafe { go::forget(self.0) };
    }
}

pub fn FromVersion(a0: go::GoInt) -> (parser_Option) {
    let r0 = (unsafe { go::f_parser_0_FromVersion(a0) });
    (unsafe { GoObject::from_handle(r0) })
}

pub fn ParseExpr(
    a0: &str,
    a1: &GoAny,
    a2: &GoSlice<parser_Option>,
) -> (ast_Expr, ffiString) {
    let go::f_parser_0_ParseExpr_return { r0, r1 } =
        go::f_parser_0_ParseExpr_return::from(unsafe {
            go::f_parser_0_ParseExpr(
                export_string(a0),
                a1.handle(),
                a2.handle(),
            )
        });
    (unsafe { GoObject::from_handle(r0) }, import_string(r1))
}

pub fn ParseFile(
    a0: &str,
    a1: &GoAny,
    a2: &GoSlice<parser_Option>,
) -> (GoPtr<ast_File>, ffiString) {
    let go::f_parser_0_ParseFile_return { r0, r1 } =
        go::f_parser_0_ParseFile_return::from(unsafe {
            go::f_parser_0_ParseFile(
                export_string(a0),
                a1.handle(),
                a2.handle(),
            )
        });
    (unsafe { GoObject::from_handle(r0) }, import_string(r1))
}
