use std::fmt::{self, Write};

use convert_case::{Case, Casing};
use spacetimedb_lib::type_def::resolve_refs::RefKind;
use spacetimedb_lib::type_def::PrimitiveType;
use spacetimedb_lib::{ElementDef, TupleDef, TypeDef};

use crate::code_indenter::CodeIndenter;
use crate::INDENT;

const NAMESPACE: &str = "SpacetimeDB";

fn primitive_to_csharp(prim: PrimitiveType) -> &'static str {
    match prim {
        PrimitiveType::Bool => "bool",
        PrimitiveType::I8 => "sbyte",
        PrimitiveType::U8 => "byte",
        PrimitiveType::I16 => "short",
        PrimitiveType::U16 => "ushort",
        PrimitiveType::I32 => "int",
        PrimitiveType::U32 => "uint",
        PrimitiveType::I64 => "long",
        PrimitiveType::U64 => "ulong",
        // PrimitiveType::I128 => "int128", Not a supported type in csharp
        // PrimitiveType::U128 => "uint128", Not a supported type in csharp
        PrimitiveType::I128 => panic!("i128 not supported for csharp"),
        PrimitiveType::U128 => panic!("i128 not supported for csharp"),
        PrimitiveType::String => "string",
        PrimitiveType::F32 => "float",
        PrimitiveType::F64 => "double",
        PrimitiveType::Bytes => "byte[]",
        PrimitiveType::Unit => todo!(), // does this exist? System.Void can't be used from C# :(
    }
}
fn ty_fmt<Ref: RefKind>(ty: &TypeDef<Ref>) -> impl fmt::Display + '_ {
    fmt_fn(move |f| match ty {
        TypeDef::Tuple(_) | TypeDef::Enum(_) => {
            unreachable!("tuples and enums should always be behind a ref")
        }
        TypeDef::Vec { element_type } => write!(f, "System.Collections.Generic.List<{}>", ty_fmt(element_type)),
        TypeDef::Primitive(prim) => f.write_str(primitive_to_csharp(*prim)),
        TypeDef::Ref(r) => f.write_str(csharp_refname(&r.as_typeref().name)),
    })
}

fn csharp_refname(s: &str) -> &str {
    match s {
        "Hash" => "SpacetimeDB.Hash",
        other => other,
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

macro_rules! indent_scope {
    ($x:ident) => {
        let mut $x = $x.indented(1);
    };
}

fn convert_typedef<Ref: RefKind>(ty: &TypeDef<Ref>) -> impl fmt::Display + '_ {
    fmt_fn(move |f| match ty {
        TypeDef::Tuple(_) | TypeDef::Enum(_) => {
            unreachable!("tuples and enums should always be behind a ref")
        }
        TypeDef::Vec { element_type } => {
            write!(f, "SpacetimeDB.TypeDef.GetVec({})", convert_typedef(element_type))
        }
        TypeDef::Primitive(prim) => write!(f, "SpacetimeDB.TypeDef.BuiltInType(SpacetimeDB.TypeDef.Def.{:?})", prim),
        TypeDef::Ref(r) => write!(f, "{}.GetTypeDef()", csharp_refname(&r.as_typeref().name)),
    })
}

fn convert_elementdef<Ref: RefKind>(elem: &ElementDef<Ref>) -> impl fmt::Display + '_ {
    fmt_fn(move |f| {
        write!(
            f,
            "new SpacetimeDB.ElementDef({}, {})",
            elem.tag,
            convert_typedef(&elem.element_type)
        )
    })
}

fn convert_tupledef<Ref: RefKind>(tuple: &TupleDef<Ref>) -> impl fmt::Display + '_ {
    fmt_fn(move |f| {
        writeln!(f, "TypeDef.Tuple(new ElementDef[]")?;
        writeln!(f, "{INDENT}{{")?;
        for (i, elem) in tuple.elements.iter().enumerate() {
            let comma = if i == tuple.elements.len() - 1 { "" } else { "," };
            writeln!(f, "{INDENT}{INDENT}{}{}", convert_elementdef(elem), comma)?;
        }
        write!(f, "{INDENT}}})")
    })
}

pub fn autogen_csharp_tuple<Ref: RefKind>(
    name: &str,
    tuple: &TupleDef<Ref>,
    table_name: Option<&str>,
    unique_fields: &[u8],
) -> String {
    let mut output = CodeIndenter::new(String::new());

    let struct_name_pascal_case = name.to_case(Case::Pascal);

    writeln!(
        output,
        "// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE",
    )
    .unwrap();
    writeln!(output, "// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.").unwrap();
    writeln!(output).unwrap();

    writeln!(output, "namespace {NAMESPACE}").unwrap();
    writeln!(output, "{{").unwrap();
    {
        indent_scope!(output);
        writeln!(output, "public partial class {struct_name_pascal_case}").unwrap();
        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);

            for field in &tuple.elements {
                let field_name = field.name.as_ref().expect("autogen'd tuples should have field names");
                writeln!(output, "[Newtonsoft.Json.JsonProperty(\"{field_name}\")]").unwrap();
                writeln!(
                    output,
                    "public {} {};",
                    ty_fmt(&field.element_type),
                    field_name.to_case(Case::Camel)
                )
                .unwrap();
            }

            writeln!(output, "public static TypeDef GetTypeDef()").unwrap();
            writeln!(output, "{{").unwrap();
            {
                indent_scope!(output);
                writeln!(output, "return {};", convert_tupledef(&tuple)).unwrap();
            }
            writeln!(output, "}}").unwrap();
            writeln!(output).unwrap();

            write!(
                output,
                "{}",
                autogen_csharp_tuple_to_struct(&struct_name_pascal_case, tuple)
            )
            .unwrap();

            // If this is a table, we want to include functions for accessing the table data
            if let Some(table_name) = table_name {
                // Insert the funcs for accessing this struct
                autogen_csharp_access_funcs_for_struct(
                    &mut output,
                    &struct_name_pascal_case,
                    tuple,
                    table_name,
                    unique_fields,
                );
            }
        }
        writeln!(output, "}}").unwrap();
    }
    writeln!(output, "}}").unwrap();

    output.into_inner()
}

fn autogen_csharp_tuple_to_struct<Ref: RefKind>(struct_name_pascal_case: &str, tuple: &TupleDef<Ref>) -> String {
    let mut output_contents_header: String = String::new();
    let mut vec_conversion: String = String::new();
    let mut output_contents_return: String = String::new();

    writeln!(
        output_contents_header,
        "public static explicit operator {struct_name_pascal_case}(TypeValue value)",
    )
    .unwrap();
    writeln!(output_contents_header, "{{").unwrap();
    writeln!(
        output_contents_header,
        "\tvar tupleValue = value.GetValue(TypeDef.Def.Tuple) as TypeValue[];"
    )
    .unwrap();
    writeln!(output_contents_header, "\tif (tupleValue == null)").unwrap();
    writeln!(output_contents_header, "\t{{").unwrap();
    writeln!(
        output_contents_header,
        "\t\tthrow new System.InvalidOperationException($\"Invalid value (must be Tuple): {{value.TypeDef.Type}}\");"
    )
    .unwrap();
    writeln!(output_contents_header, "\t}}").unwrap();
    writeln!(output_contents_header).unwrap();
    // vec conversion go here
    writeln!(output_contents_return, "\treturn new {}", struct_name_pascal_case).unwrap();
    writeln!(output_contents_return, "\t{{").unwrap();

    for field in &tuple.elements {
        let field_name = field.name.as_ref().expect("autogen'd tuples should have field names");
        let field_type = &field.element_type;
        let csharp_type = ty_fmt(field_type);
        let csharp_field_name = field_name.to_string().to_case(Case::Camel);

        match field_type {
            TypeDef::Tuple(_) | TypeDef::Enum(_) => {
                unreachable!("tuples and enums should always be behind a ref")
            }
            TypeDef::Primitive(prim) => {
                writeln!(
                    output_contents_return,
                    "\t\t{} = ({})tupleValue[{}].GetValue(TypeDef.Def.{:?}),",
                    csharp_field_name, csharp_type, field.tag, prim
                )
                .unwrap();
            }
            TypeDef::Ref(r) => {
                let name = csharp_refname(&r.as_typeref().name);
                writeln!(
                    output_contents_return,
                    "\t\t{} = ({name})tupleValue[{}],",
                    csharp_field_name, field.tag,
                )
                .unwrap();
            }
            TypeDef::Vec { element_type } => match &**element_type {
                TypeDef::Tuple(_) | TypeDef::Enum(_) => {
                    unreachable!("tuples and enums should always be behind a ref")
                }
                TypeDef::Primitive(prim) => {
                    let csharp_type = primitive_to_csharp(*prim);
                    writeln!(
                        vec_conversion,
                        "\tvar {}_vec = new System.Collections.Generic.List<{}>();",
                        field_name, csharp_type
                    )
                    .unwrap();
                    writeln!(
                        vec_conversion,
                        "\tvar {}_vec_source = tupleValue[{}].GetValue(TypeDef.Def.Vec) as System.Collections.Generic.List<SpacetimeDB.TypeValue>;",
                        field_name, field.tag
                    ).unwrap();
                    writeln!(vec_conversion, "\tforeach(var entry in {}_vec_source!)", field_name).unwrap();
                    writeln!(vec_conversion, "\t{{").unwrap();
                    if let PrimitiveType::String = prim {
                        writeln!(
                            vec_conversion,
                            "\t\t{}_vec.Add(entry.GetValue(TypeDef.Def.{:?}) as string);",
                            field_name, prim,
                        )
                        .unwrap();
                    } else {
                        writeln!(
                            vec_conversion,
                            "\t\t{}_vec.Add(({})entry.GetValue(TypeDef.Def.{:?}));",
                            field_name, csharp_type, prim,
                        )
                        .unwrap();
                    }
                    writeln!(vec_conversion, "\t}}").unwrap();
                    writeln!(
                        output_contents_return,
                        "\t\t{} = {}_vec,",
                        csharp_field_name, field_name
                    )
                    .unwrap();
                }
                TypeDef::Ref(r) => {
                    let name = csharp_refname(&r.as_typeref().name);

                    writeln!(
                        vec_conversion,
                        "\tvar {}_vec = new System.Collections.Generic.List<{name}>();",
                        field_name
                    )
                    .unwrap();
                    writeln!(
                        vec_conversion,
                        "\tvar {}_vec_source = tupleValue[{}].GetValue(SpacetimeDB.TypeDef.Def.Vec) as System.Collections.Generic.List<SpacetimeDB.TypeValue>;",
                        field_name, field.tag
                    ).unwrap();

                    writeln!(vec_conversion, "\tforeach(var entry in {}_vec_source!)", field_name).unwrap();
                    writeln!(vec_conversion, "\t{{").unwrap();
                    writeln!(vec_conversion, "\t\t{}_vec.Add(({name})entry);", field_name).unwrap();
                    writeln!(vec_conversion, "\t}}").unwrap();
                    writeln!(
                        output_contents_return,
                        "\t\t{} = {}_vec,",
                        csharp_field_name, field_name
                    )
                    .unwrap();
                }
                TypeDef::Vec { .. } => panic!("nested vecs are disallowed?"),
            },
        }
    }

    // End Struct
    writeln!(output_contents_return, "\t}};").unwrap();
    // End Func
    writeln!(output_contents_return, "}}").unwrap();

    output_contents_header + &vec_conversion + &output_contents_return
}

fn autogen_csharp_access_funcs_for_struct<Ref: RefKind>(
    output: &mut CodeIndenter<String>,
    struct_name_pascal_case: &str,
    tuple: &TupleDef<Ref>,
    table_name: &str,
    unique_fields: &[u8],
) {
    for field in &tuple.elements {
        let field_name = field.name.as_ref().expect("autogen'd tuples should have field names");
        let field_type = &field.element_type;
        let csharp_field_name_pascal = field_name.to_case(Case::Pascal);

        enum AccessorType {
            Primitive(PrimitiveType),
            Hash,
        }
        let field_type = match field_type {
            TypeDef::Tuple(_) | TypeDef::Enum(_) => unreachable!("tuples and enums should always be behind a ref"),
            TypeDef::Primitive(prim) => AccessorType::Primitive(*prim),
            TypeDef::Ref(r) => {
                let name = csharp_refname(&r.as_typeref().name);
                if name != "SpacetimeDB.Hash" {
                    // TODO: We don't allow filtering on tuples right now, its possible we may consider it for the future.
                    continue;
                }
                AccessorType::Hash
            }
            TypeDef::Vec { .. } => {
                // TODO: We don't allow filtering based on a vec type, but we might want other functionality here in the future.
                // TODO: It would be nice to be able to say, give me all entries where this vec contains this value, which we can do.
                continue;
            }
        };
        let csharp_field_type = match field_type {
            AccessorType::Primitive(prim) => primitive_to_csharp(prim),
            AccessorType::Hash => "SpacetimeDB.Hash",
        };

        let is_unique = unique_fields.binary_search(&field.tag).is_ok();
        let filter_return_type = fmt_fn(|f| {
            if is_unique {
                f.write_str(&struct_name_pascal_case)
            } else {
                write!(f, "System.Collections.Generic.IEnumerable<{}>", struct_name_pascal_case)
            }
        });

        writeln!(
            output,
            "public static {filter_return_type} FilterBy{}({} value)",
            csharp_field_name_pascal, csharp_field_type
        )
        .unwrap();

        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);
            writeln!(output, "var typeDef = GetTypeDef();").unwrap();
            writeln!(
                output,
                "foreach(var entry in StdbNetworkManager.clientDB.GetEntries(\"{}\"))",
                table_name
            )
            .unwrap();
            writeln!(output, "{{").unwrap();
            {
                indent_scope!(output);
                write!(
                    output,
                    "varln tupleArr = entry.GetValue(TypeDef.Def.Tuple) as TypeValue[];"
                )
                .unwrap();
                writeln!(output, "if (tupleArr == null) continue;").unwrap();

                match field_type {
                    AccessorType::Primitive(prim) => {
                        writeln!(
                            output,
                            "var compareValue = ({})tupleArr[{}].GetValue(TypeDef.Def.{:?});",
                            csharp_field_type, field.tag, prim
                        )
                        .unwrap();
                        writeln!(output, "if (compareValue == value)").unwrap();
                    }
                    AccessorType::Hash => {
                        writeln!(output, "var compareValue = SpacetimeDB.Hash.From(tupleArr[{}].GetValue(TypeDef.Def.Bytes) as byte[]);", field.tag).unwrap();
                        writeln!(output, "if (compareValue.Equals(value))").unwrap();
                    }
                }

                writeln!(output, "{{").unwrap();
                {
                    writeln!(output, "var tuple = TypeValue.GetTuple(typeDef, tupleArr);").unwrap();
                    if is_unique {
                        writeln!(output, "return From(tuple);").unwrap();
                    } else {
                        writeln!(output, "yield return From(tuple);").unwrap();
                    }
                }
                // End if
                writeln!(output, "}}").unwrap();
            }
            // End foreach
            writeln!(output, "}}").unwrap();

            if is_unique {
                writeln!(output, "return null;").unwrap();
            }
        }
        // End Func
        writeln!(output, "}}").unwrap();
        writeln!(output).unwrap();
    }
}

// fn convert_enumdef<Ref: RefKind>(tuple: &EnumDef<Ref>) -> impl fmt::Display + '_ {
//     fmt_fn(move |f| {
//         writeln!(f, "TypeDef.Tuple(new ElementDef[]")?;
//         writeln!(f, "{{")?;
//         for (i, elem) in tuple.elements.iter().enumerate() {
//             let comma = if i == tuple.elements.len() - 1 { "" } else { "," };
//             writeln!(f, "{INDENT}{}{}", convert_elementdef(elem), comma)?;
//         }
//         writeln!(f, "}}")
//     })
// }

pub struct Reducer<Ref: RefKind> {
    name: String,
    args: Vec<ReducerArg<Ref>>,
}
pub struct ReducerArg<Ref: RefKind> {
    name: String,
    ty: TypeDef<Ref>,
}

pub fn autogen_csharp_reducer<Ref: RefKind>(original_function: Reducer<Ref>) -> String {
    let func_name = &original_function.name;
    // let reducer_pascal_name = func_name.to_case(Case::Pascal);
    let use_namespace = true;
    let func_name_pascal_case = func_name.to_case(Case::Pascal);

    let mut output = CodeIndenter::new(String::new());

    let mut func_arguments: String = String::new();
    let mut arg_names: String = String::new();

    writeln!(
        output,
        "// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE"
    )
    .unwrap();
    writeln!(output, "// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.").unwrap();
    writeln!(output).unwrap();

    if use_namespace {
        writeln!(output, "namespace {NAMESPACE}").unwrap();
        writeln!(output, "{{").unwrap();
        output.indent(1);
    }

    writeln!(output, "public static partial class Reducer").unwrap();
    writeln!(output, "{{").unwrap();

    {
        indent_scope!(output);

        for (arg_i, arg) in original_function.args.into_iter().enumerate() {
            let ReducerArg { name, ty } = arg;
            let arg_name = name.to_case(Case::Camel);

            if arg_i > 0 {
                func_arguments.push_str(", ");
                arg_names.push_str(", ");
            }

            write!(func_arguments, "{} {}", ty_fmt(&ty), arg_name).unwrap();

            arg_names.push_str(&arg_name);
        }

        writeln!(output, "public static void {func_name_pascal_case}({func_arguments})").unwrap();
        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);

            //            StdbNetworkManager.instance.InternalCallReducer(new StdbNetworkManager.Message
            // 			{
            // 				fn = "create_new_player",
            // 				args = new object[] { playerId, position },
            // 			});

            // Tell the network manager to send this message
            // UPGRADE FOR LATER
            // write!(output, "{}\t\tStdbNetworkManager.instance.InternalCallReducer(new Websocket.FunctionCall\n", namespace_tab).unwrap();
            // write!(output, "{}\t\t{{\n", namespace_tab).unwrap();
            // write!(output, "{}\t\t\tReducer = \"{}\",\n", namespace_tab, func_name).unwrap();
            // write!(output, "{}\t\t\tArgBytes = Google.Protobuf.ByteString.CopyFrom(Newtonsoft.Json.JsonConvert.SerializeObject(new object[] {{ {} }}), System.Text.Encoding.UTF8),\n", namespace_tab, arg_names).unwrap();
            // write!(output, "{}\t\t}});\n", namespace_tab).unwrap();

            // TEMPORARY OLD FUNCTIONALITY
            writeln!(
                output,
                "StdbNetworkManager.instance.InternalCallReducer(new StdbNetworkManager.Message",
            )
            .unwrap();
            writeln!(output, "{{").unwrap();
            writeln!(output, "fn = \"{func_name}\",").unwrap();
            writeln!(output, "args = new object[] {{ {arg_names} }},").unwrap();
            writeln!(output, "}});").unwrap();
        }
        // Closing brace for reducer
        writeln!(output, "}}").unwrap();
    }
    // Closing brace for class
    writeln!(output, "}}").unwrap();

    if use_namespace {
        output.dedent(1);
        writeln!(output, "}}").unwrap();
    }

    output.into_inner()
}
