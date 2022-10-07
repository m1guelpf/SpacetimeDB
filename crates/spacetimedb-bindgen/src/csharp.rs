extern crate core;
extern crate proc_macro;

use crate::{parse_generic_arg, rust_to_spacetimedb_ident};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::{self, Write};
use syn::{FnArg, ItemFn, ItemStruct};

/// This returns a function which will return the schema (TypeDef) for a struct. The signature
/// for this function is as follows:
/// fn __get_struct_schema__<struct_type_ident>() -> spacetimedb_lib::TypeDef {
///   ...
/// }
pub(crate) fn csharp_get_type_def_for_struct(original_struct: ItemStruct) -> String {
    let mut col_num: u8 = 0;
    let mut element_defs: String = String::new();

    for field in &original_struct.fields {
        let field_type = field.ty.clone().to_token_stream().to_string();
        if let syn::Type::Path(syn::TypePath { ref path, .. }) = field.ty {
            if path.segments.len() > 0 {
                match rust_to_spacetimedb_ident(path.segments[0].ident.to_string().as_str()) {
                    Some(spacetimedb_type) => {
                        write!(element_defs, "\t\t\t\tnew SpacetimeDB.ElementDef({}, SpacetimeDB.TypeDef.BuiltInType(SpacetimeDB.TypeDef.Def.{})),\n", col_num, spacetimedb_type).unwrap();
                    }
                    None => {
                        match path.segments[0].ident.to_string().as_str() {
                            "Hash" => {
                                write!(
                                    element_defs,
                                    "\t\t\t\tnew SpacetimeDB.ElementDef({}, SpacetimeDB.Hash.GetTypeDef()),\n",
                                    col_num
                                )
                                .unwrap();
                            }
                            "Vec" => {
                                match parse_generic_arg(path.segments[0].arguments.to_token_stream()) {
                                    Ok(arg) => {
                                        match rust_to_spacetimedb_ident(arg.to_token_stream().to_string().as_str()) {
                                            Some(spacetimedb_type) => {
                                                write!(element_defs, "\t\t\t\tnew SpacetimeDB.ElementDef({}, SpacetimeDB.TypeDef.GetVec(SpacetimeDB.TypeDef.BuiltInType(SpacetimeDB.TypeDef.Def.{}))),\n", col_num, spacetimedb_type).unwrap();
                                            }
                                            None => {
                                                // This case handles all other structs (including Hash)
                                                write!(element_defs, "\t\t\t\tnew SpacetimeDB.ElementDef({}, SpacetimeDB.TypeDef.GetVec({}.GetTypeDef())),\n", col_num, arg.to_string()).unwrap();
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        panic!(
                                            "Invalid vec definition: {}, E={}",
                                            field.ty.to_token_stream().to_string(),
                                            e
                                        );
                                    }
                                }
                            }
                            _ => {
                                // This is hopefully another type that is autogenerated
                                write!(
                                    element_defs,
                                    "\t\t\t\tnew SpacetimeDB.ElementDef({}, SpacetimeDB.{}.GetTypeDef()),\n",
                                    col_num, field_type
                                )
                                .unwrap();
                            }
                        }
                    }
                }
            }
        }

        col_num = col_num + 1;
    }

    let mut result: String = String::new();

    write!(result, "\t\tpublic static TypeDef GetTypeDef()\n").unwrap();
    write!(result, "\t\t{{\n").unwrap();
    write!(result, "\t\t\treturn TypeDef.Tuple(new ElementDef[]\n").unwrap();
    write!(result, "\t\t\t{{\n").unwrap();
    write!(result, "{}", element_defs).unwrap();
    write!(result, "\t\t\t}});\n").unwrap();
    write!(result, "\t\t}}\n\n").unwrap();
    return result;
}

struct Reducer {
    name: String,
    args: Vec<ReducerArg>,
}
struct ReducerArg {
    name: String,
    ty: ReducerType,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum RustPrimitive {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    // I128, Not a supported type in csharp
    // U128, Not a supported type in csharp
    String,
    Str,
    F32,
    F64,
}
impl RustPrimitive {
    fn to_csharp(self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::I8 => "sbyte",
            Self::U8 => "byte",
            Self::I16 => "short",
            Self::U16 => "ushort",
            Self::I32 => "int",
            Self::U32 => "uint",
            Self::I64 => "long",
            Self::U64 => "ulong",
            // Self::I128 => "int128", Not a supported type in csharp
            // Self::U128 => "uint128", Not a supported type in csharp
            Self::String => "string",
            Self::Str => "string",
            Self::F32 => "float",
            Self::F64 => "double",
        }
    }
}
#[derive(PartialEq, Eq)]
enum ReducerType {
    Hash,
    Primitive(RustPrimitive),
    Vec(RustPrimitive),
    Other(String),
}
impl ReducerType {
    fn fmt_csharp(&self) -> impl fmt::Display + '_ {
        fmt_fn(move |f| {
            match self {
                ReducerType::Hash => f.write_str("SpacetimeDB.Hash"),
                ReducerType::Primitive(prim) => f.write_str(prim.to_csharp()),
                ReducerType::Vec(prim) => write!(f, "System.Collections.Generic.List<{}>", prim.to_csharp()),
                // This is hopefully a type understood by C#
                ReducerType::Other(s) => f.write_str(s),
            }
        })
    }
}
fn fmt_fn(f: impl Fn(&mut fmt::Formatter) -> fmt::Result) -> impl fmt::Display {
    struct FDisplay<F>(F);
    impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for FDisplay<F> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (self.0)(f)
        }
    }
    FDisplay(f)
}

/// Creates a C# reducer function for the given rust reducer. All reducers are exported as static
/// functions in a class called Reducer.
pub(crate) fn autogen_csharp_reducer(original_function: Reducer) -> String {
    let func_name = &original_function.name;
    let reducer_pascal_name = func_name.to_case(Case::Pascal);
    let use_namespace = true;
    let namespace = "SpacetimeDB";
    let namespace_tab = if use_namespace { "\t" } else { "" };
    let func_name_pascal_case = func_name.to_case(Case::Pascal);

    let mut output_contents: String = String::new();
    let mut func_arguments: String = String::new();
    let mut arg_names: String = String::new();

    write!(
        output_contents,
        "// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE\n"
    )
    .unwrap();
    write!(
        output_contents,
        "// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.\n\n"
    )
    .unwrap();

    if use_namespace {
        write!(output_contents, "namespace {} \n{{\n", namespace).unwrap();
    }

    write!(
        output_contents,
        "{}public static partial class Reducer \n{}{{\n",
        namespace_tab, namespace_tab
    )
    .unwrap();

    let mut arg_i = 0usize;
    for arg in original_function.args {
        let ReducerArg { name, ty } = arg;
        let arg_name = name.to_case(Case::Camel);

        // Skip any arguments that are supplied by spacetimedb
        if arg_i == 0 && ty == ReducerType::Hash || arg_i == 1 && ty == ReducerType::Primitive(RustPrimitive::U64) {
            arg_i += 1;
            continue;
        }

        if arg_i > 0 {
            func_arguments.push_str(", ");
            arg_names.push_str(", ");
        }

        write!(func_arguments, "{} {}", ty.fmt_csharp(), arg_name).unwrap();

        write!(arg_names, "{}", arg_name.clone()).unwrap();

        arg_i += 1;
    }

    write!(
        output_contents,
        "{}\tpublic static void {}({})\n",
        namespace_tab, func_name_pascal_case, func_arguments
    )
    .unwrap();
    write!(output_contents, "{}\t{{\n", namespace_tab).unwrap();

    //            StdbNetworkManager.instance.InternalCallReducer(new StdbNetworkManager.Message
    // 			{
    // 				fn = "create_new_player",
    // 				args = new object[] { playerId, position },
    // 			});

    // Tell the network manager to send this message
    // UPGRADE FOR LATER
    // write!(output_contents, "{}\t\tStdbNetworkManager.instance.InternalCallReducer(new Websocket.FunctionCall\n", namespace_tab).unwrap();
    // write!(output_contents, "{}\t\t{{\n", namespace_tab).unwrap();
    // write!(output_contents, "{}\t\t\tReducer = \"{}\",\n", namespace_tab, func_name).unwrap();
    // write!(output_contents, "{}\t\t\tArgBytes = Google.Protobuf.ByteString.CopyFrom(Newtonsoft.Json.JsonConvert.SerializeObject(new object[] {{ {} }}), System.Text.Encoding.UTF8),\n", namespace_tab, arg_names).unwrap();
    // write!(output_contents, "{}\t\t}});\n", namespace_tab).unwrap();

    // TEMPORARY OLD FUNCTIONALITY
    write!(
        output_contents,
        "{}\t\tStdbNetworkManager.instance.InternalCallReducer(new StdbNetworkManager.Message\n",
        namespace_tab
    )
    .unwrap();
    write!(output_contents, "{}\t\t{{\n", namespace_tab).unwrap();
    write!(output_contents, "{}\t\t\tfn = \"{}\",\n", namespace_tab, func_name).unwrap();
    write!(
        output_contents,
        "{}\t\t\targs = new object[] {{ {} }},\n",
        namespace_tab, arg_names
    )
    .unwrap();
    write!(output_contents, "{}\t\t}});\n", namespace_tab).unwrap();

    // Closing brace for reducer
    write!(output_contents, "{}\t}}\n", namespace_tab).unwrap();
    // Closing brace for class
    write!(output_contents, "{}}}\n", namespace_tab).unwrap();

    if use_namespace {
        write!(output_contents, "}}\n").unwrap();
    }

    output_contents

    // // Write the csharp output
    // if !std::path::Path::new("cs-src").is_dir() {
    //     std::fs::create_dir(std::path::Path::new("cs-src")).unwrap();
    // }
    // let path = format!("cs-src/{}Reducer.cs", reducer_pascal_name);
    // std::fs::write(path, output_contents).unwrap();

    // proc_macro::TokenStream::from(quote! {
    //     // Reducer C# generation
    // })
}

/// Creates a C# class from an ItemStruct, with an optional table number (only for tables).
pub(crate) fn autogen_csharp_tuple(
    original_struct: ItemStruct,
    table_name: Option<String>,
) -> proc_macro2::TokenStream {
    let namespace = "SpacetimeDB";

    let original_struct_ident = &original_struct.clone().ident;
    let struct_name_pascal_case = original_struct_ident.to_string().to_case(Case::Pascal);

    let mut col_num: usize = 0;
    let mut output_contents: String = String::new();

    write!(
        output_contents,
        "{}{}",
        "// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE\n",
        "// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.\n\n"
    )
    .unwrap();
    write!(output_contents, "namespace {}\n{{\n", namespace).unwrap();

    write!(output_contents, "\tpublic partial class {}\n", struct_name_pascal_case).unwrap();
    write!(output_contents, "\t{{\n").unwrap();

    // Here we're just going to convert each rust field to a csharp field
    for field in &original_struct.fields {
        let col_name = &field.ident.clone().unwrap();
        let col_name_csharp = col_name.to_token_stream().to_string().to_case(Case::Camel);
        write!(
            output_contents,
            "\t\t[Newtonsoft.Json.JsonProperty(\"{}\")]\n",
            col_name
        )
        .unwrap();

        match rust_type_to_csharp_raw_type(field.ty.to_token_stream().to_string().as_str()) {
            Some(raw_type) => {
                write!(output_contents, "\t\tpublic {} {};\n", raw_type, col_name_csharp,).unwrap();
            }
            None => {
                if let syn::Type::Path(syn::TypePath { ref path, .. }) = field.ty {
                    if path.segments.len() > 0 {
                        match path.segments[0].ident.to_token_stream().to_string().as_str() {
                            "Hash" => {
                                write!(output_contents, "\t\tpublic SpacetimeDB.Hash {};\n", col_name_csharp).unwrap();
                            }
                            "Vec" => match parse_generic_arg(path.segments[0].arguments.to_token_stream()) {
                                Ok(argument) => match rust_type_to_csharp_raw_type(argument.to_string().as_str()) {
                                    Some(csharp_type) => {
                                        write!(
                                            output_contents,
                                            "\t\tpublic System.Collections.Generic.List<{}> {};\n",
                                            csharp_type, col_name_csharp
                                        )
                                        .unwrap();
                                    }
                                    None => {
                                        write!(
                                            output_contents,
                                            "\t\tpublic System.Collections.Generic.List<{}> {};\n",
                                            argument, col_name_csharp
                                        )
                                        .unwrap();
                                    }
                                },
                                Err(e) => {
                                    panic!("Failed to parse Vec: {}", e);
                                }
                            },
                            _ => {
                                // This is hopefully a type understood by C# (Vec will cause issues here)
                                write!(
                                    output_contents,
                                    "\t\tpublic {} {};\n",
                                    field.ty.to_token_stream().to_string().as_str(),
                                    col_name_csharp,
                                )
                                .unwrap();
                            }
                        }
                    }
                }
            }
        }

        col_num = col_num + 1;
    }

    // Insert the GetTypeDef func
    write!(
        output_contents,
        "{}",
        csharp_get_type_def_for_struct(original_struct.clone())
    )
    .unwrap();

    // Insert the tuple to struct function
    write!(
        output_contents,
        "{}",
        autogen_csharp_tuple_to_struct(original_struct.clone())
    )
    .unwrap();

    // If this is a table, we want to include functions for accessing the table data
    if let Some(table_name) = table_name {
        // Insert the funcs for accessing this struct
        match autogen_csharp_access_funcs_for_struct(original_struct.clone(), table_name) {
            Ok(func) => {
                write!(output_contents, "{}", func).unwrap();
            }
            Err(err) => {
                return quote! {
                    compile_error!("{}", #err);
                };
            }
        }
    }

    // class close brace
    write!(output_contents, "\t}}\n").unwrap();
    // namespace close brace
    write!(output_contents, "}}\n").unwrap();

    // Write the cs output
    if !std::path::Path::new("cs-src").is_dir() {
        std::fs::create_dir(std::path::Path::new("cs-src")).unwrap();
    }
    let path = format!("cs-src/{}.cs", struct_name_pascal_case);
    std::fs::write(path, output_contents).unwrap();

    // Output all macro data
    quote! {
        // C# generated
    }
}

/// Creates a C# function that takes in a TypeValue and converts it to the fully qualified type.
fn autogen_csharp_tuple_to_struct(original_struct: ItemStruct) -> String {
    let original_struct_ident = &original_struct.clone().ident;
    let mut col_num: usize = 0;

    let csharp_struct_type = original_struct_ident.to_string().to_case(Case::Pascal);

    let mut output_contents_header: String = String::new();
    let mut vec_conversion: String = String::new();
    let mut output_contents_return: String = String::new();

    write!(
        output_contents_header,
        "\t\tpublic static {} From(TypeValue value)\n",
        csharp_struct_type
    )
    .unwrap();
    write!(output_contents_header, "\t\t{{\n").unwrap();
    write!(
        output_contents_header,
        "\t\t\tvar tupleValue = value.GetValue(TypeDef.Def.Tuple) as TypeValue[];\n"
    )
    .unwrap();
    write!(output_contents_header, "\t\t\tif (tupleValue == null)\n").unwrap();
    write!(output_contents_header, "\t\t\t{{\n").unwrap();
    write!(
        output_contents_header,
        "\t\t\t\tthrow new System.InvalidOperationException($\"Invalid value (must be Tuple): {{value.TypeDef.Type}}\");\n"
    )
    .unwrap();
    write!(output_contents_header, "\t\t\t}}\n\n").unwrap();
    // vec conversion go here
    write!(output_contents_return, "\t\t\treturn new {}\n", csharp_struct_type).unwrap();
    write!(output_contents_return, "\t\t\t{{\n").unwrap();

    for field in &original_struct.fields {
        let field_type = field.ty.clone().to_token_stream().to_string();
        let field_type = field_type.as_str();
        let field_ident = field.clone().ident.unwrap();
        let csharp_field_type = field_type.to_case(Case::Pascal);
        let csharp_field_name = field_ident.to_string().to_case(Case::Camel);

        match rust_type_to_csharp_raw_type(field_type) {
            Some(csharp_type) => {
                write!(
                    output_contents_return,
                    "\t\t\t\t{} = ({})tupleValue[{}].GetValue(TypeDef.Def.{}),\n",
                    csharp_field_name,
                    csharp_type,
                    col_num,
                    field_type.to_case(Case::Pascal)
                )
                .unwrap();
            }
            None => {
                if let syn::Type::Path(syn::TypePath { ref path, .. }) = field.ty {
                    if path.segments.len() > 0 {
                        match path.segments[0].ident.to_token_stream().to_string().as_str() {
                            "Hash" => {
                                write!(output_contents_return, "\t\t\t\t{} = SpacetimeDB.Hash.From(tupleValue[{}].GetValue(TypeDef.Def.Bytes) as byte[]),\n",
                                       csharp_field_name, col_num).unwrap();
                            }
                            "Vec" => {
                                match parse_generic_arg(path.segments[0].arguments.to_token_stream()) {
                                    Ok(arg) => {
                                        match rust_to_spacetimedb_ident(arg.to_token_stream().to_string().as_str()) {
                                            Some(spacetimedb_type) => {
                                                match rust_type_to_csharp_raw_type(arg.to_string().as_str()) {
                                                    Some(csharp_type) => {
                                                        write!(vec_conversion, "\t\t\tvar {}_vec = new System.Collections.Generic.List<{}>();\n", field_ident.to_string(), csharp_type).unwrap();
                                                        write!(vec_conversion, "\t\t\tvar {}_vec_source = tupleValue[{}].GetValue(TypeDef.Def.Vec) as System.Collections.Generic.List<SpacetimeDB.TypeValue>;\n", field_ident.to_string(), col_num).unwrap();
                                                        write!(
                                                            vec_conversion,
                                                            "\t\t\tforeach(var entry in {}_vec_source!)\n",
                                                            field_ident.to_string()
                                                        )
                                                        .unwrap();
                                                        write!(vec_conversion, "\t\t\t{{\n").unwrap();
                                                        match csharp_type {
                                                            "string" => {
                                                                write!(
                                                                vec_conversion,
                                                                "\t\t\t\t{}_vec.Add(entry.GetValue(TypeDef.Def.{}) as string);\n",
                                                                field_ident.to_string(),
                                                                spacetimedb_type,
                                                            ).unwrap();
                                                            }
                                                            _ => {
                                                                write!(
                                                                vec_conversion,
                                                                "\t\t\t\t{}_vec.Add(({})entry.GetValue(TypeDef.Def.{}));\n",
                                                                field_ident.to_string(),
                                                                csharp_type,
                                                                spacetimedb_type,
                                                            ).unwrap();
                                                            }
                                                        }
                                                        write!(vec_conversion, "\t\t\t}}\n").unwrap();
                                                        write!(
                                                            output_contents_return,
                                                            "\t\t\t\t{} = {}_vec,\n",
                                                            csharp_field_name,
                                                            field_ident.to_string()
                                                        )
                                                        .unwrap();
                                                    }
                                                    None => {
                                                        panic!("This type is a native spacetimedb type, but has no C# type: {}", arg.to_string().as_str());
                                                    }
                                                }
                                            }
                                            None => match arg.to_string().as_str() {
                                                "Hash" => {
                                                    write!(vec_conversion, "\t\t\tvar {}_vec = new System.Collections.Generic.List<SpacetimeDB.Hash>();\n", field_ident.to_string()).unwrap();
                                                    write!(vec_conversion, "\t\t\tvar {}_vec_source = tupleValue[{}].GetValue(SpacetimeDB.TypeDef.Def.Vec) as System.Collections.Generic.List<SpacetimeDB.TypeValue>;\n", field_ident.to_string(), col_num).unwrap();

                                                    write!(
                                                        vec_conversion,
                                                        "\t\t\tforeach(var entry in {}_vec_source!)\n",
                                                        field_ident.to_string()
                                                    )
                                                    .unwrap();
                                                    write!(vec_conversion, "\t\t\t{{\n").unwrap();
                                                    write!(vec_conversion, "\t\t\t\t{}_vec.Add(SpacetimeDB.Hash.From(entry.GetValue(SpacetimeDB.TypeDef.Def.Bytes) as byte[]));\n", field_ident.to_string()).unwrap();
                                                    write!(vec_conversion, "\t\t\t}}\n").unwrap();
                                                    write!(
                                                        output_contents_return,
                                                        "\t\t\t\t{} = {}_vec,\n",
                                                        csharp_field_name,
                                                        field_ident.to_string()
                                                    )
                                                    .unwrap();
                                                }
                                                other_type => {
                                                    write!(vec_conversion, "\t\t\tvar {}_vec = new System.Collections.Generic.List<{}>();\n", field_ident.to_string(), other_type).unwrap();
                                                    write!(vec_conversion, "\t\t\tvar {}_vec_source = tupleValue[{}].GetValue(SpacetimeDB.TypeDef.Def.Vec) as System.Collections.Generic.List<SpacetimeDB.TypeValue>;\n", field_ident.to_string(), col_num).unwrap();
                                                    write!(
                                                        vec_conversion,
                                                        "\t\t\tforeach(var entry in {}_vec_source!)\n",
                                                        field_ident.to_string()
                                                    )
                                                    .unwrap();
                                                    write!(vec_conversion, "\t\t\t{{\n").unwrap();
                                                    write!(
                                                        vec_conversion,
                                                        "\t\t\t\t{}_vec.Add({}.From(entry));\n",
                                                        field_ident.to_string(),
                                                        other_type
                                                    )
                                                    .unwrap();
                                                    write!(vec_conversion, "\t\t\t}}\n").unwrap();
                                                    write!(
                                                        output_contents_return,
                                                        "\t\t\t\t{} = {}_vec,\n",
                                                        csharp_field_name,
                                                        field_ident.to_string()
                                                    )
                                                    .unwrap();
                                                }
                                            },
                                        }
                                    }
                                    Err(e) => {
                                        panic!(
                                            "Vec has unsupported generic param: {} E={}",
                                            field.ty.to_token_stream().to_string(),
                                            e
                                        );
                                    }
                                }
                            }
                            _ => {
                                // This is hopefully another stdb type
                                write!(
                                    output_contents_return,
                                    "\t\t\t\t{} = {}.From(tupleValue[{}]),\n",
                                    csharp_field_name, csharp_field_type, col_num
                                )
                                .unwrap();
                            }
                        }
                    }
                }
            }
        }

        col_num = col_num + 1;
    }

    // End Struct
    write!(output_contents_return, "\t\t\t}};\n").unwrap();
    // End Func
    write!(output_contents_return, "\t\t}}\n").unwrap();

    let mut output_contents_result = String::new();
    write!(
        output_contents_result,
        "{}{}{}\n",
        output_contents_header, vec_conversion, output_contents_return
    )
    .unwrap();

    return output_contents_result;
}

fn autogen_csharp_access_funcs_for_struct(
    original_struct: ItemStruct,
    table_name: String,
) -> Result<String, &'static str> {
    let original_struct_ident = &original_struct.clone().ident;
    let mut col_num: usize = 0;

    let csharp_struct_type = original_struct_ident.to_string().to_case(Case::Pascal);

    let mut output_contents: String = String::new();

    for field in &original_struct.fields {
        let field_type = field.ty.clone().to_token_stream().to_string();
        let field_type = field_type.as_str();
        let field_ident = field.clone().ident.unwrap();
        let csharp_field_type: String;
        let csharp_field_name_pascal = field_ident.to_token_stream().to_string().to_case(Case::Pascal);

        match rust_type_to_csharp_raw_type(field_type) {
            Some(csharp_type) => {
                csharp_field_type = csharp_type.to_string();
            }
            None => {
                if let syn::Type::Path(syn::TypePath { ref path, .. }) = field.ty {
                    if path.segments.len() > 0 {
                        match path.segments[0].ident.to_token_stream().to_string().as_str() {
                            "Vec" => {
                                // TODO: We don't allow filtering based on a vec type, but we might want other functionality here in the future.
                                // TODO: It would be nice to be able to say, give me all entries where this vec contains this value, which we can do.
                                continue;
                            }
                            "Hash" => {
                                // This is hopefully another stdb type
                                csharp_field_type = "Hash".to_string();
                            }
                            _ => {
                                // TODO: We don't allow filtering on tuples right now, its possible we may consider it for the future.
                                continue;
                            }
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
        }

        let mut is_primary = false;
        for attr in &field.attrs {
            if attr.path.to_token_stream().to_string().eq("unique") {
                is_primary = true;
            }
        }

        if is_primary {
            write!(
                output_contents,
                "\t\tpublic static {} FilterBy{}({} value)\n",
                csharp_struct_type, csharp_field_name_pascal, csharp_field_type
            )
            .unwrap();
        } else {
            write!(
                output_contents,
                "\t\tpublic static System.Collections.Generic.IEnumerable<{}> FilterBy{}({} value)\n",
                csharp_struct_type, csharp_field_name_pascal, csharp_field_type
            )
            .unwrap();
        }

        write!(output_contents, "\t\t{{\n").unwrap();
        write!(output_contents, "\t\t\tvar typeDef = GetTypeDef();\n").unwrap();
        write!(
            output_contents,
            "\t\t\tforeach(var entry in StdbNetworkManager.clientDB.GetEntries(\"{}\"))\n",
            table_name
        )
        .unwrap();
        write!(output_contents, "\t\t\t{{\n").unwrap();
        write!(
            output_contents,
            "\t\t\t\tvar tupleArr = entry.GetValue(TypeDef.Def.Tuple) as TypeValue[];\n"
        )
        .unwrap();
        write!(output_contents, "\t\t\t\tif (tupleArr == null)\n").unwrap();
        write!(output_contents, "\t\t\t\t{{\n").unwrap();
        write!(output_contents, "\t\t\t\t\tcontinue;\n").unwrap();
        write!(output_contents, "\t\t\t\t}}\n").unwrap();

        match rust_type_to_csharp_raw_type(field_type) {
            Some(csharp_type) => {
                write!(
                    output_contents,
                    "\t\t\t\tvar compareValue = ({})tupleArr[{}].GetValue(TypeDef.Def.{});\n",
                    csharp_type,
                    col_num,
                    field_type.to_case(Case::Pascal)
                )
                .unwrap();
                write!(output_contents, "\t\t\t\tif (compareValue == value)\n",).unwrap();
            }
            None => {
                if let syn::Type::Path(syn::TypePath { ref path, .. }) = field.ty {
                    if path.segments.len() > 0 {
                        match path.segments[0].ident.to_token_stream().to_string().as_str() {
                            "Hash" => {
                                // This is hopefully another stdb type
                                write!(output_contents, "\t\t\t\tvar compareValue = SpacetimeDB.Hash.From(tupleArr[{}].GetValue(TypeDef.Def.Bytes) as byte[]);\n", col_num).unwrap();
                                write!(output_contents, "\t\t\t\tif (compareValue.Equals(value))\n",).unwrap();
                            }
                            _ => {
                                panic!("This is an unsupported type and it should not receive a filter function.")
                            }
                        }
                    }
                }
            }
        }

        write!(output_contents, "\t\t\t\t{{\n",).unwrap();
        write!(
            output_contents,
            "\t\t\t\t\tvar tuple = TypeValue.GetTuple(typeDef, tupleArr);\n"
        )
        .unwrap();
        if is_primary {
            write!(output_contents, "\t\t\t\t\treturn From(tuple);\n").unwrap();
        } else {
            write!(output_contents, "\t\t\t\t\tyield return From(tuple);\n").unwrap();
        }
        // End foreach
        write!(output_contents, "\t\t\t\t}}\n",).unwrap();

        // End Struct
        write!(output_contents, "\t\t\t}}\n").unwrap();

        if is_primary {
            write!(output_contents, "\t\t\treturn null;\n").unwrap();
        }

        // End Func
        write!(output_contents, "\t\t}}\n\n").unwrap();
        col_num = col_num + 1;
    }

    return Ok(output_contents);
}

pub(crate) fn rust_type_to_csharp_raw_type(type_string: &str) -> Option<&str> {
    return match type_string {
        "bool" => Some("bool"),
        "i8" => Some("sbyte"),
        "u8" => Some("byte"),
        "i16" => Some("short"),
        "u16" => Some("ushort"),
        "i32" => Some("int"),
        "u32" => Some("uint"),
        "i64" => Some("long"),
        "u64" => Some("ulong"),
        // "i128" => "int128", Not a supported type in csharp
        // "u128" => "uint128", Not a supported type in csharp
        "String" => Some("string"),
        "&str" => Some("string"),
        "f32" => Some("float"),
        "f64" => Some("double"),
        _ => {
            return None;
        }
    };
}
