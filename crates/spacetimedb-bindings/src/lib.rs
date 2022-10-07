#[macro_use]
pub mod io;
mod impls;

use spacetimedb_lib::type_def::resolve_refs::{RefKind, TypeRef};
use spacetimedb_lib::type_def::PrimitiveType;
use spacetimedb_lib::{PrimaryKey, TupleDef, TupleValue, TypeDef};
use std::alloc::{alloc as _alloc, dealloc as _dealloc, Layout};
use std::mem::ManuallyDrop;
use std::ops::Range;
use std::panic;

#[cfg(feature = "macro")]
pub use spacetimedb_bindgen;
#[cfg(feature = "macro")]
pub use spacetimedb_bindgen::spacetimedb;
#[cfg(feature = "macro")]
pub use spacetimedb_bindgen::Index;
#[cfg(feature = "macro")]
pub use spacetimedb_bindgen::Unique;

pub use spacetimedb_lib;
pub use spacetimedb_lib::hash;
pub use spacetimedb_lib::Hash;
pub use spacetimedb_lib::TypeValue;

#[doc(hidden)]
pub mod __private {
    use super::*;
    pub use once_cell::sync::OnceCell;
    pub use spacetimedb_lib::type_def::resolve_refs::TypeRef;
    pub fn get_ref_or_schema<T: SchemaType>() -> TypeDef<TypeRef> {
        T::get_ref().map(TypeDef::Ref).unwrap_or_else(|| T::get_ref_schema())
    }
}

// #[cfg(target_arch = "wasm32")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern "C" {
    fn _create_table(ptr: *mut u8) -> u32;
    fn _get_table_id(ptr: *mut u8) -> u32;
    fn _create_index(table_id: u32, col_id: u32, index_type: u8);

    fn _insert(table_id: u32, ptr: *mut u8);

    fn _delete_pk(table_id: u32, ptr: *mut u8) -> u8;
    fn _delete_value(table_id: u32, ptr: *mut u8) -> u8;
    fn _delete_eq(table_id: u32, col_id: u32, ptr: *mut u8) -> i32;
    fn _delete_range(table_id: u32, col_id: u32, ptr: *mut u8) -> i32;

    fn _filter_eq(table_id: u32, col_id: u32, src_ptr: *mut u8, result_ptr: *mut u8);

    fn _iter(table_id: u32) -> u64;
    fn _console_log(level: u8, ptr: *const u8, len: u32);
}

const ROW_BUF_LEN: usize = 1024 * 1024;
static mut ROW_BUF: Option<*mut u8> = None;

#[no_mangle]
extern "C" fn alloc(size: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, align);
        _alloc(layout)
    }
}

#[no_mangle]
extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    let align = std::mem::align_of::<usize>();
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, align);
        _dealloc(ptr, layout);
    }
}

fn row_buf() -> ManuallyDrop<Vec<u8>> {
    unsafe {
        if ROW_BUF.is_none() {
            let ptr = alloc(ROW_BUF_LEN);
            ROW_BUF = Some(ptr);
        }
        ManuallyDrop::new(Vec::from_raw_parts(ROW_BUF.unwrap(), 0, ROW_BUF_LEN))
    }
}

pub fn encode_row(row: TupleValue, bytes: &mut Vec<u8>) {
    row.encode(bytes);
}

pub fn decode_row(schema: &TupleDef, bytes: &mut &[u8]) -> (Result<TupleValue, &'static str>, usize) {
    TupleValue::decode(schema, bytes)
}

pub fn encode_schema(schema: TupleDef, bytes: &mut Vec<u8>) {
    schema.encode(bytes);
}

pub fn decode_schema(bytes: &mut &[u8]) -> (Result<TupleDef, String>, usize) {
    TupleDef::decode(bytes)
}

pub fn create_table(table_name: &str, schema: TupleDef) -> u32 {
    let mut bytes = row_buf();

    let mut schema_bytes = Vec::new();
    schema.encode(&mut schema_bytes);

    let table_info = TupleValue {
        elements: vec![
            TypeValue::String(table_name.to_string()),
            TypeValue::Bytes(schema_bytes),
        ]
        .into(),
    };

    table_info.encode(&mut bytes);

    unsafe { _create_table(bytes.as_mut_ptr()) }
}

pub fn get_table_id(table_name: &str) -> u32 {
    let mut bytes = row_buf();

    let table_name = TypeValue::String(table_name.to_string());
    table_name.encode(&mut bytes);

    unsafe { _get_table_id(bytes.as_mut_ptr()) }
}

pub fn insert(table_id: u32, row: TupleValue) {
    let mut bytes = row_buf();
    row.encode(&mut bytes);
    unsafe {
        _insert(table_id, bytes.as_mut_ptr());
    }
}

pub fn delete_pk(table_id: u32, primary_key: PrimaryKey) -> Option<usize> {
    let mut bytes = row_buf();
    primary_key.encode(&mut bytes);
    let result = unsafe { _delete_pk(table_id, bytes.as_mut_ptr()) };
    if result == 0 {
        return None;
    }
    return Some(1);
}

pub fn delete_filter<F: Fn(&TupleValue) -> bool>(table_id: u32, f: F) -> Option<usize> {
    let mut count = 0;
    for tuple_value in __iter__(table_id).unwrap() {
        if f(&tuple_value) {
            count += 1;
            let mut bytes = row_buf();
            tuple_value.encode(&mut bytes);
            if unsafe { _delete_value(table_id, bytes.as_mut_ptr()) } == 0 {
                panic!("Something ain't right.");
            }
        }
    }
    Some(count)
}

pub fn delete_eq(table_id: u32, col_id: u8, eq_value: TypeValue) -> Option<usize> {
    let mut bytes = row_buf();
    eq_value.encode(&mut bytes);
    let result = unsafe { _delete_eq(table_id, col_id.into(), bytes.as_mut_ptr()) };
    if result == -1 {
        return None;
    }
    return Some(result as usize);
}

pub fn delete_range(table_id: u32, col_id: u8, range: Range<TypeValue>) -> Option<usize> {
    let mut bytes = row_buf();
    let start = TypeValue::from(range.start);
    let end = TypeValue::from(range.end);
    let tuple = TupleValue {
        elements: vec![start, end].into(),
    };
    tuple.encode(&mut bytes);
    let result = unsafe { _delete_range(table_id, col_id.into(), bytes.as_mut_ptr()) };
    if result == -1 {
        return None;
    }
    return Some(result as usize);
}

pub fn create_index(_table_id: u32, _index_type: u8, _col_ids: Vec<u8>) {}

// TODO: going to have to somehow ensure TypeValue is equatable
pub fn filter_eq(_table_id: u32, _col_id: u8, _eq_value: TypeValue) -> Option<TupleValue> {
    return None;
}

//
// fn page_table(table_id : u32, pager_token : u32, read_entries : u32) {
//
// }

pub fn __iter__(table_id: u32) -> Option<TableIter> {
    let data = unsafe { _iter(table_id) };
    let ptr = (data >> 32) as u32 as *mut u8;
    let size = data as u32;
    let bytes: Vec<u8> = unsafe { Vec::from_raw_parts(ptr, size as usize, size as usize) };

    let slice = &mut &bytes[..];
    let (schema, schema_size) = decode_schema(slice);
    if let Err(e) = schema {
        panic!("__iter__: Could not decode schema. Err: {}", e);
    }

    let data_index = schema_size as usize;

    Some(TableIter {
        bytes,
        data_index,
        schema: schema.unwrap(),
    })
}

pub struct TableIter {
    bytes: Vec<u8>,
    data_index: usize,
    schema: TupleDef,
}

impl Iterator for TableIter {
    type Item = TupleValue;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = &mut &self.bytes[self.data_index..];
        if slice.len() > 0 {
            let (row, num_read) = decode_row(&self.schema, slice);
            if let Err(e) = row {
                panic!("TableIter::next: Failed to decode row! Err: {}", e);
            }
            self.data_index += num_read;
            return Some(row.unwrap());
        }
        return None;
    }
}

pub trait MaybeRef {
    fn get_ref() -> Option<TypeRef> {
        None
    }
}

pub trait SchemaType: MaybeRef + Sized + 'static {
    fn get_schema() -> TypeDef;
    /// get_ref_schema().resolve() should be the same as get_schema()
    fn get_ref_schema() -> TypeDef<TypeRef>;
}

pub trait FromValue: SchemaType {
    fn from_value(v: TypeValue) -> Option<Self>;
}

pub trait IntoValue: SchemaType {
    fn into_value(self) -> TypeValue;
}

pub trait TupleType: MaybeRef + Sized + 'static {
    fn get_tupledef() -> TupleDef;
    fn get_ref_tupledef() -> TupleDef<TypeRef>;

    #[doc(hidden)]
    fn describe_tuple() -> u64 {
        describe_tuple(Self::get_tupledef())
    }
    #[doc(hidden)]
    fn describe_tuple_ref() -> u64 {
        describe_tuple(Self::get_ref_tupledef())
    }
}

fn describe_tuple<Ref: RefKind>(tuple_def: TupleDef<Ref>) -> u64 {
    const _: () = assert!(std::mem::size_of::<usize>() == std::mem::size_of::<u32>());
    let mut bytes = vec![];
    tuple_def.encode(&mut bytes);
    let offset = bytes.as_ptr() as u64;
    let length = bytes.len() as u64;
    std::mem::forget(bytes);
    offset << 32 | length
}

impl<T: TupleType> SchemaType for T {
    fn get_schema() -> TypeDef {
        TypeDef::Tuple(T::get_tupledef())
    }
    fn get_ref_schema() -> TypeDef<TypeRef> {
        TypeDef::Tuple(T::get_ref_tupledef())
    }
}

pub trait FromTuple: TupleType {
    fn from_tuple(v: TupleValue) -> Option<Self>;
}

pub trait IntoTuple: TupleType {
    fn into_tuple(self) -> TupleValue;
}

impl<T: FromTuple> FromValue for T {
    fn from_value(v: TypeValue) -> Option<Self> {
        match v {
            TypeValue::Tuple(v) => Self::from_tuple(v),
            _ => None,
        }
    }
}
impl<T: IntoTuple> IntoValue for T {
    fn into_value(self) -> TypeValue {
        TypeValue::Tuple(self.into_tuple())
    }
}

pub trait TableDef: TupleType + FromTuple + IntoTuple {
    const TABLE_NAME: &'static str;

    fn create_table() -> u32 {
        let tuple_def = Self::get_tupledef();
        create_table(Self::TABLE_NAME, tuple_def)
    }
}

pub trait FilterableValue: FromValue + IntoValue {
    fn equals(&self, other: &TypeValue) -> bool;
}

pub trait UniqueValue: FilterableValue {
    fn into_primarykey(self) -> PrimaryKey;
}
