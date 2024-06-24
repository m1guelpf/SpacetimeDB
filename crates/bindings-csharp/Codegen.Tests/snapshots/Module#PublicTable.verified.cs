﻿//HintName: PublicTable.cs

// <auto-generated />
#nullable enable

partial struct PublicTable
{
    private static readonly Lazy<SpacetimeDB.Internal.FFI.TableId> tableId =
        new(() => SpacetimeDB.Runtime.GetTableId(nameof(PublicTable)));

    public static IEnumerable<PublicTable> Iter() =>
        new SpacetimeDB.Runtime.RawTableIter(tableId.Value).Parse<PublicTable>();

    public static SpacetimeDB.Internal.Module.TableDesc MakeTableDesc(
        SpacetimeDB.BSATN.ITypeRegistrar registrar
    ) =>
        new(
            new(
                nameof(PublicTable),
                new SpacetimeDB.Internal.Module.ColumnDefWithAttrs[]
                {
                    new(
                        new(nameof(Id), BSATN.Id.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.PrimaryKeyAuto
                    ),
                    new(
                        new(nameof(ByteField), BSATN.ByteField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(UshortField), BSATN.UshortField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(UintField), BSATN.UintField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(UlongField), BSATN.UlongField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(Uint128Field), BSATN.Uint128Field.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(SbyteField), BSATN.SbyteField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(ShortField), BSATN.ShortField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(IntField), BSATN.IntField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(LongField), BSATN.LongField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(Int128Field), BSATN.Int128Field.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(BoolField), BSATN.BoolField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(FloatField), BSATN.FloatField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(DoubleField), BSATN.DoubleField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(StringField), BSATN.StringField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(IdentityField), BSATN.IdentityField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(AddressField), BSATN.AddressField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(
                            nameof(CustomStructField),
                            BSATN.CustomStructField.GetAlgebraicType(registrar)
                        ),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(
                            nameof(CustomClassField),
                            BSATN.CustomClassField.GetAlgebraicType(registrar)
                        ),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(
                            nameof(CustomEnumField),
                            BSATN.CustomEnumField.GetAlgebraicType(registrar)
                        ),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(
                            nameof(CustomTaggedEnumField),
                            BSATN.CustomTaggedEnumField.GetAlgebraicType(registrar)
                        ),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(nameof(ListField), BSATN.ListField.GetAlgebraicType(registrar)),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(
                            nameof(DictionaryField),
                            BSATN.DictionaryField.GetAlgebraicType(registrar)
                        ),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(
                            nameof(NullableValueField),
                            BSATN.NullableValueField.GetAlgebraicType(registrar)
                        ),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(
                            nameof(NullableReferenceField),
                            BSATN.NullableReferenceField.GetAlgebraicType(registrar)
                        ),
                        SpacetimeDB.ColumnAttrs.UnSet
                    ),
                    new(
                        new(
                            nameof(ComplexNestedField),
                            BSATN.ComplexNestedField.GetAlgebraicType(registrar)
                        ),
                        SpacetimeDB.ColumnAttrs.UnSet
                    )
                },
                false
            ),
            (SpacetimeDB.BSATN.AlgebraicType.Ref)new BSATN().GetAlgebraicType(registrar)
        );

    private static readonly Lazy<KeyValuePair<
        string,
        Action<BinaryWriter, object?>
    >[]> fieldTypeInfos =
        new(
            () =>
                new KeyValuePair<string, Action<BinaryWriter, object?>>[]
                {
                    new(nameof(Id), (w, v) => BSATN.Id.Write(w, (int)v!)),
                    new(nameof(ByteField), (w, v) => BSATN.ByteField.Write(w, (byte)v!)),
                    new(nameof(UshortField), (w, v) => BSATN.UshortField.Write(w, (ushort)v!)),
                    new(nameof(UintField), (w, v) => BSATN.UintField.Write(w, (uint)v!)),
                    new(nameof(UlongField), (w, v) => BSATN.UlongField.Write(w, (ulong)v!)),
                    new(
                        nameof(Uint128Field),
                        (w, v) => BSATN.Uint128Field.Write(w, (System.UInt128)v!)
                    ),
                    new(nameof(SbyteField), (w, v) => BSATN.SbyteField.Write(w, (sbyte)v!)),
                    new(nameof(ShortField), (w, v) => BSATN.ShortField.Write(w, (short)v!)),
                    new(nameof(IntField), (w, v) => BSATN.IntField.Write(w, (int)v!)),
                    new(nameof(LongField), (w, v) => BSATN.LongField.Write(w, (long)v!)),
                    new(
                        nameof(Int128Field),
                        (w, v) => BSATN.Int128Field.Write(w, (System.Int128)v!)
                    ),
                    new(nameof(BoolField), (w, v) => BSATN.BoolField.Write(w, (bool)v!)),
                    new(nameof(FloatField), (w, v) => BSATN.FloatField.Write(w, (float)v!)),
                    new(nameof(DoubleField), (w, v) => BSATN.DoubleField.Write(w, (double)v!)),
                    new(nameof(StringField), (w, v) => BSATN.StringField.Write(w, (string)v!)),
                    new(
                        nameof(IdentityField),
                        (w, v) => BSATN.IdentityField.Write(w, (SpacetimeDB.Identity)v!)
                    ),
                    new(
                        nameof(AddressField),
                        (w, v) => BSATN.AddressField.Write(w, (SpacetimeDB.Address)v!)
                    ),
                    new(
                        nameof(CustomStructField),
                        (w, v) => BSATN.CustomStructField.Write(w, (CustomStruct)v!)
                    ),
                    new(
                        nameof(CustomClassField),
                        (w, v) => BSATN.CustomClassField.Write(w, (CustomClass)v!)
                    ),
                    new(
                        nameof(CustomEnumField),
                        (w, v) => BSATN.CustomEnumField.Write(w, (CustomEnum)v!)
                    ),
                    new(
                        nameof(CustomTaggedEnumField),
                        (w, v) => BSATN.CustomTaggedEnumField.Write(w, (CustomTaggedEnum)v!)
                    ),
                    new(
                        nameof(ListField),
                        (w, v) => BSATN.ListField.Write(w, (System.Collections.Generic.List<int>)v!)
                    ),
                    new(
                        nameof(DictionaryField),
                        (w, v) =>
                            BSATN.DictionaryField.Write(
                                w,
                                (System.Collections.Generic.Dictionary<string, int>)v!
                            )
                    ),
                    new(
                        nameof(NullableValueField),
                        (w, v) => BSATN.NullableValueField.Write(w, (int?)v!)
                    ),
                    new(
                        nameof(NullableReferenceField),
                        (w, v) => BSATN.NullableReferenceField.Write(w, (string?)v!)
                    ),
                    new(
                        nameof(ComplexNestedField),
                        (w, v) =>
                            BSATN.ComplexNestedField.Write(
                                w,
                                (System.Collections.Generic.Dictionary<
                                    CustomEnum,
                                    System.Collections.Generic.List<int?>?
                                >?)
                                    v!
                            )
                    ),
                }
        );

    public static IEnumerable<PublicTable> Query(
        System.Linq.Expressions.Expression<Func<PublicTable, bool>> filter
    ) =>
        new SpacetimeDB.Runtime.RawTableIterFiltered(
            tableId.Value,
            SpacetimeDB.Internal.Filter.Compile<PublicTable>(fieldTypeInfos.Value, filter)
        ).Parse<PublicTable>();

    public void Insert()
    {
        var bytes = SpacetimeDB.Runtime.Insert(tableId.Value, this);
        // bytes should contain modified value now with autoinc fields updated

        using var stream = new System.IO.MemoryStream(bytes);
        using var reader = new System.IO.BinaryReader(stream);
        ReadFields(reader);
    }

    public static IEnumerable<PublicTable> FilterById(int Id) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            0,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.Id, Id)
        ).Parse<PublicTable>();

    public static PublicTable? FindById(int Id) =>
        FilterById(Id).Cast<PublicTable?>().SingleOrDefault();

    public static bool DeleteById(int Id) =>
        SpacetimeDB.Runtime.DeleteByColEq(
            tableId.Value,
            0,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.Id, Id)
        ) > 0;

    public static bool UpdateById(int Id, PublicTable value) =>
        SpacetimeDB.Runtime.UpdateByColEq(
            tableId.Value,
            0,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.Id, Id),
            value
        );

    public static IEnumerable<PublicTable> FilterByByteField(byte ByteField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            1,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.ByteField, ByteField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByUshortField(ushort UshortField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            2,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.UshortField, UshortField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByUintField(uint UintField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            3,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.UintField, UintField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByUlongField(ulong UlongField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            4,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.UlongField, UlongField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByUint128Field(System.UInt128 Uint128Field) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            5,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.Uint128Field, Uint128Field)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterBySbyteField(sbyte SbyteField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            6,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.SbyteField, SbyteField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByShortField(short ShortField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            7,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.ShortField, ShortField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByIntField(int IntField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            8,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.IntField, IntField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByLongField(long LongField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            9,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.LongField, LongField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByInt128Field(System.Int128 Int128Field) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            10,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.Int128Field, Int128Field)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByBoolField(bool BoolField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            11,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.BoolField, BoolField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByStringField(string StringField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            14,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.StringField, StringField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByIdentityField(
        SpacetimeDB.Identity IdentityField
    ) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            15,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.IdentityField, IdentityField)
        ).Parse<PublicTable>();

    public static IEnumerable<PublicTable> FilterByAddressField(SpacetimeDB.Address AddressField) =>
        new SpacetimeDB.Runtime.RawTableIterByColEq(
            tableId.Value,
            16,
            SpacetimeDB.BSATN.IStructuralReadWrite.ToBytes(BSATN.AddressField, AddressField)
        ).Parse<PublicTable>();
} // PublicTable
