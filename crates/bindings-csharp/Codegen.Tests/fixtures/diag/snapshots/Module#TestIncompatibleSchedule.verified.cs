﻿//HintName: TestIncompatibleSchedule.cs
// <auto-generated />
#nullable enable

[System.Runtime.InteropServices.StructLayout(System.Runtime.InteropServices.LayoutKind.Auto)]
partial struct TestIncompatibleSchedule : SpacetimeDB.Internal.ITable<TestIncompatibleSchedule>
{
    public void ReadFields(System.IO.BinaryReader reader)
    {
        ScheduledId = BSATN.ScheduledId.Read(reader);
        ScheduledAt = BSATN.ScheduledAt.Read(reader);
    }

    public void WriteFields(System.IO.BinaryWriter writer)
    {
        BSATN.ScheduledId.Write(writer, ScheduledId);
        BSATN.ScheduledAt.Write(writer, ScheduledAt);
    }

    public readonly partial struct BSATN : SpacetimeDB.BSATN.IReadWrite<TestIncompatibleSchedule>
    {
        internal static readonly SpacetimeDB.BSATN.U64 ScheduledId = new();
        internal static readonly SpacetimeDB.ScheduleAt.BSATN ScheduledAt = new();

        public TestIncompatibleSchedule Read(System.IO.BinaryReader reader) =>
            SpacetimeDB.BSATN.IStructuralReadWrite.Read<TestIncompatibleSchedule>(reader);

        public void Write(System.IO.BinaryWriter writer, TestIncompatibleSchedule value)
        {
            value.WriteFields(writer);
        }

        public SpacetimeDB.BSATN.AlgebraicType GetAlgebraicType(
            SpacetimeDB.BSATN.ITypeRegistrar registrar
        ) =>
            registrar.RegisterType<TestIncompatibleSchedule>(
                _ => new SpacetimeDB.BSATN.AlgebraicType.Product(
                    new SpacetimeDB.BSATN.AggregateElement[]
                    {
                        new(nameof(ScheduledId), ScheduledId.GetAlgebraicType(registrar)),
                        new(nameof(ScheduledAt), ScheduledAt.GetAlgebraicType(registrar))
                    }
                )
            );
    }

    public ulong ScheduledId;
    public SpacetimeDB.ScheduleAt ScheduledAt;

    static IEnumerable<SpacetimeDB.Internal.TableDesc> SpacetimeDB.Internal.ITable<TestIncompatibleSchedule>.MakeTableDesc(
        SpacetimeDB.BSATN.ITypeRegistrar registrar
    ) =>
        [
            new(
                new(
                    TableName: nameof(SpacetimeDB.Local.TestIncompatibleSchedule1),
                    Columns:
                    [
                        new(nameof(ScheduledId), BSATN.ScheduledId.GetAlgebraicType(registrar)),
                        new(nameof(ScheduledAt), BSATN.ScheduledAt.GetAlgebraicType(registrar))
                    ],
                    Indexes: [],
                    Constraints:
                    [
                        new(
                            nameof(SpacetimeDB.Local.TestIncompatibleSchedule1),
                            0,
                            nameof(ScheduledId),
                            SpacetimeDB.Internal.ColumnAttrs.PrimaryKeyAuto
                        )
                    ],
                    Sequences: [],
                    // "system" | "user"
                    TableType: "user",
                    // "public" | "private"
                    TableAccess: "private",
                    Scheduled: "TestIncompatibleScheduleReducer"
                ),
                (uint)
                    (
                        (SpacetimeDB.BSATN.AlgebraicType.Ref)new BSATN().GetAlgebraicType(registrar)
                    ).Ref_
            ),
            new(
                new(
                    TableName: nameof(SpacetimeDB.Local.TestIncompatibleSchedule2),
                    Columns:
                    [
                        new(nameof(ScheduledId), BSATN.ScheduledId.GetAlgebraicType(registrar)),
                        new(nameof(ScheduledAt), BSATN.ScheduledAt.GetAlgebraicType(registrar))
                    ],
                    Indexes: [],
                    Constraints:
                    [
                        new(
                            nameof(SpacetimeDB.Local.TestIncompatibleSchedule2),
                            0,
                            nameof(ScheduledId),
                            SpacetimeDB.Internal.ColumnAttrs.PrimaryKeyAuto
                        )
                    ],
                    Sequences: [],
                    // "system" | "user"
                    TableType: "user",
                    // "public" | "private"
                    TableAccess: "private",
                    Scheduled: null
                ),
                (uint)
                    (
                        (SpacetimeDB.BSATN.AlgebraicType.Ref)new BSATN().GetAlgebraicType(registrar)
                    ).Ref_
            ),
        ];
} // TestIncompatibleSchedule
