﻿//HintName: CustomStruct.cs
// <auto-generated />
#nullable enable

partial struct CustomStruct : SpacetimeDB.BSATN.IStructuralReadWrite
{
    public void ReadFields(System.IO.BinaryReader reader)
    {
        IntField = BSATN.IntField.Read(reader);
        StringField = BSATN.StringField.Read(reader);
    }

    public void WriteFields(System.IO.BinaryWriter writer)
    {
        BSATN.IntField.Write(writer, IntField);
        BSATN.StringField.Write(writer, StringField);
    }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<CustomStruct>
    {
        internal static readonly SpacetimeDB.BSATN.I32 IntField = new();
        internal static readonly SpacetimeDB.BSATN.String StringField = new();

        public CustomStruct Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<CustomStruct>(reader);

        public void Write(System.IO.BinaryWriter writer, CustomStruct value) =>
            value.WriteFields(writer);

        public SpacetimeDB.BSATN.AlgebraicType GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<CustomStruct>(_ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                new SpacetimeDB.BSATN.AggregateElement[]
                {
                    new(nameof(IntField), IntField.GetAlgebraicType(registrar)),
                    new(nameof(StringField), StringField.GetAlgebraicType(registrar))
                }
            ));
    }
} // CustomStruct
