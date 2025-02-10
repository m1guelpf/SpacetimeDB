namespace SpacetimeDB;

using System.Diagnostics;
using System.Runtime.InteropServices;
using SpacetimeDB.BSATN;

internal static class Util
{
    public static Span<byte> AsBytes<T>(ref T val)
        where T : struct => MemoryMarshal.AsBytes(MemoryMarshal.CreateSpan(ref val, 1));

    /// <summary>
    /// Convert this object to a BIG-ENDIAN hex string.
    ///
    /// Big endian is almost always the correct convention here. It puts the most significant bytes
    /// of the number at the lowest indexes of the resulting string; assuming the string is printed
    /// with low indexes to the left, this will result in the correct hex number being displayed.
    ///
    /// (This might be wrong if the string is printed after, say, a unicode right-to-left marker.
    /// But, well, what can you do.)
    /// </summary>
    /// <typeparam name="T"></typeparam>
    /// <param name="val"></param>
    /// <returns></returns>
    public static string ToHexBigEndian<T>(T val)
        where T : struct
    {
        var bytes = AsBytes(ref val);
        // If host is little-endian, reverse the bytes.
        // Note that this reverses our stack copy of `val`, not the original value, and doesn't require heap `byte[]` allocation.
        if (BitConverter.IsLittleEndian)
        {
            bytes.Reverse();
        }
#if NET5_0_OR_GREATER
        return Convert.ToHexString(bytes);
#else
        // Similar to `Convert.ToHexString`, but that method is not available in .NET Standard
        // which we need to target for Unity support.
        return BitConverter.ToString(bytes.ToArray()).Replace("-", "");
#endif
    }

    /// <summary>
    /// Convert the passed byte array to a value of type T, optionally reversing it before performing the conversion.
    /// If the input is not reversed, it is treated as having the native endianness of the host system.
    /// (The endianness of the host system can be checked via System.BitConverter.IsLittleEndian.)
    /// </summary>
    /// <typeparam name="T"></typeparam>
    /// <param name="source"></param>
    /// <param name="littleEndian"></param>
    /// <returns></returns>
    public static T Read<T>(ReadOnlySpan<byte> source, bool littleEndian)
        where T : struct
    {
        Debug.Assert(
            source.Length == Marshal.SizeOf<T>(),
            $"Error while reading ${typeof(T).FullName}: expected source span to be {Marshal.SizeOf<T>()} bytes long, but was {source.Length} bytes."
        );

        var result = MemoryMarshal.Read<T>(source);

        if (littleEndian != BitConverter.IsLittleEndian)
        {
            AsBytes(ref result).Reverse();
        }

        return result;
    }

    /// <summary>
    /// Convert a hex string to a byte array.
    /// </summary>
    /// <param name="hex"></param>
    /// <returns></returns>
    public static byte[] StringToByteArray(string hex)
    {
#if NET5_0_OR_GREATER
        return Convert.FromHexString(hex);
#else
        // Manual implementation for .NET Standard compatibility.
        Debug.Assert(
            hex.Length % 2 == 0,
            $"Expected input string (\"{hex}\") to be of even length"
        );

        var NumberChars = hex.Length;
        var bytes = new byte[NumberChars / 2];
        for (var i = 0; i < NumberChars; i += 2)
        {
            bytes[i / 2] = Convert.ToByte(hex.Substring(i, 2), 16);
        }
        return bytes;
#endif
    }

    // Similarly, we need some constants that are not available in .NET Standard.
    public const long TicksPerMicrosecond = 10;
    public const long MicrosecondsPerSecond = 1_000_000;
}

// The following types are "special" types: they has a special (Ref-less) AlgebraicType representations.
// See `spacetimedb-sats::AlgebraicType::is_valid_for_client_type_[use|generate]` for more information.
// We don't use [Type] here; instead we manually implement the serialization stuff that would be generated by
// [Type] so that we can override GetAlgebraicType to return types in a special, Ref-less form.
public readonly partial struct Unit
{
    public readonly struct BSATN : IReadWrite<Unit>
    {
        public Unit Read(BinaryReader reader) => default;

        public void Write(BinaryWriter writer, Unit value) { }

        public AlgebraicType GetAlgebraicType(ITypeRegistrar registrar) =>
            // Return a Product directly, not a Ref, because this is a special type.
            new AlgebraicType.Product([]);
    }
}

[StructLayout(LayoutKind.Sequential)]
public readonly record struct ConnectionId
{
    private readonly U128 value;

    internal ConnectionId(U128 v) => value = v;

    /// <summary>
    /// Create a ConnectionId from a LITTLE-ENDIAN byte array.
    ///
    /// If you are parsing a ConnectionId from a string, you probably want FromHexString instead,
    /// or, failing that, FromBigEndian.
    ///
    /// Returns null if the resulting ConnectionId is the default.
    /// </summary>
    /// <param name="bytes"></param>
    public static ConnectionId? From(ReadOnlySpan<byte> bytes)
    {
        var id = Util.Read<ConnectionId>(bytes, littleEndian: true);
        return id == default ? null : id;
    }

    /// <summary>
    /// Create a ConnectionId from a BIG-ENDIAN byte array.
    ///
    /// This method is the correct choice if you have converted the bytes of a hexadecimal-formatted ConnectionId
    /// to a byte array in the following way:
    ///
    /// "0xb0b1b2..."
    /// ->
    /// [0xb0, 0xb1, 0xb2, ...]
    ///
    /// Returns null if the resulting ConnectionId is the default.
    /// </summary>
    /// <param name="bytes"></param>
    public static ConnectionId? FromBigEndian(ReadOnlySpan<byte> bytes)
    {
        var id = Util.Read<ConnectionId>(bytes, littleEndian: false);
        return id == default ? null : id;
    }

    /// <summary>
    /// Create a ConnectionId from a hex string.
    /// </summary>
    /// <param name="hex"></param>
    /// <returns></returns>
    public static ConnectionId? FromHexString(string hex) =>
        FromBigEndian(Util.StringToByteArray(hex));

    public static ConnectionId Random()
    {
        var random = new Random();
        var id = new ConnectionId();
        random.NextBytes(Util.AsBytes(ref id));
        return id;
    }

    // --- auto-generated ---
    public readonly struct BSATN : IReadWrite<ConnectionId>
    {
        public ConnectionId Read(BinaryReader reader) =>
            new(new SpacetimeDB.BSATN.U128Stdb().Read(reader));

        public void Write(BinaryWriter writer, ConnectionId value) =>
            new SpacetimeDB.BSATN.U128Stdb().Write(writer, value.value);

        // --- / auto-generated ---

        // --- customized ---
        public AlgebraicType GetAlgebraicType(ITypeRegistrar registrar) =>
            // Return a Product directly, not a Ref, because this is a special type.
            new AlgebraicType.Product(
                [
                    // Using this specific name here is important.
                    new("__connection_id__", new AlgebraicType.U128(default)),
                ]
            );
        // --- / customized ---
    }

    public override string ToString() => Util.ToHexBigEndian(value);
}

[StructLayout(LayoutKind.Sequential)]
public readonly record struct Identity
{
    private readonly U256 value;

    internal Identity(U256 val) => value = val;

    /// <summary>
    /// Create an Identity from a LITTLE-ENDIAN byte array.
    ///
    /// If you are parsing an Identity from a string, you probably want FromHexString instead,
    /// or, failing that, FromBigEndian.
    /// </summary>
    /// <param name="bytes"></param>
    public Identity(ReadOnlySpan<byte> bytes) => this = From(bytes);

    /// <summary>
    /// Create an Identity from a LITTLE-ENDIAN byte array.
    ///
    /// If you are parsing an Identity from a string, you probably want FromHexString instead,
    /// or, failing that, FromBigEndian.
    /// </summary>
    /// <param name="bytes"></param>
    public static Identity From(ReadOnlySpan<byte> bytes) =>
        Util.Read<Identity>(bytes, littleEndian: true);

    /// <summary>
    /// Create an Identity from a BIG-ENDIAN byte array.
    ///
    /// This method is the correct choice if you have converted the bytes of a hexadecimal-formatted `Identity`
    /// to a byte array in the following way:
    ///
    /// "0xb0b1b2..."
    /// ->
    /// [0xb0, 0xb1, 0xb2, ...]
    /// </summary>
    /// <param name="bytes"></param>
    public static Identity FromBigEndian(ReadOnlySpan<byte> bytes) =>
        Util.Read<Identity>(bytes, littleEndian: false);

    /// <summary>
    /// Create an Identity from a hex string.
    /// </summary>
    /// <param name="hex"></param>
    /// <returns></returns>
    public static Identity FromHexString(string hex) => FromBigEndian(Util.StringToByteArray(hex));

    // --- auto-generated ---
    public readonly struct BSATN : IReadWrite<Identity>
    {
        public Identity Read(BinaryReader reader) => new(new SpacetimeDB.BSATN.U256().Read(reader));

        public void Write(BinaryWriter writer, Identity value) =>
            new SpacetimeDB.BSATN.U256().Write(writer, value.value);

        // --- / auto-generated ---

        // --- customized ---
        public AlgebraicType GetAlgebraicType(ITypeRegistrar registrar) =>
            // Return a Product directly, not a Ref, because this is a special type.
            new AlgebraicType.Product(
                [
                    // Using this specific name here is important.
                    new("__identity__", new AlgebraicType.U256(default)),
                ]
            );
        // --- / customized ---
    }

    // This must be explicitly implemented, otherwise record will generate a new implementation.
    public override string ToString() => Util.ToHexBigEndian(value);
}

/// <summary>
/// A timestamp that represents a unique moment in time (in the Earth's reference frame).
///
/// This type may be converted to/from a DateTimeOffset, but the conversion can lose precision.
/// This type has less precision than DateTimeOffset (units of microseconds rather than units of 100ns).
/// </summary>
[StructLayout(LayoutKind.Sequential)] // we should be able to use it in FFI
public record struct Timestamp(long MicrosecondsSinceUnixEpoch) : IStructuralReadWrite, IComparable<Timestamp>
{
    public static implicit operator DateTimeOffset(Timestamp t) =>
        DateTimeOffset.UnixEpoch.AddTicks(t.MicrosecondsSinceUnixEpoch * Util.TicksPerMicrosecond);

    public static implicit operator Timestamp(DateTimeOffset offset) =>
        new Timestamp(offset.Subtract(DateTimeOffset.UnixEpoch).Ticks / Util.TicksPerMicrosecond);

    // For backwards-compatibility.
    public readonly DateTimeOffset ToStd() => this;

    // Should be consistent with Rust implementation of Display.
    public override readonly string ToString()
    {
        var sign = MicrosecondsSinceUnixEpoch < 0 ? "-" : "";
        var pos = Math.Abs(MicrosecondsSinceUnixEpoch);
        var secs = pos / Util.MicrosecondsPerSecond;
        var microsRemaining = pos % Util.MicrosecondsPerSecond;
        return $"{sign}{secs}.{microsRemaining:D6}";
    }

    public static readonly Timestamp UNIX_EPOCH = new(0);

    public static Timestamp FromTimeDurationSinceUnixEpoch(TimeDuration timeDuration) =>
        new Timestamp(timeDuration.Microseconds);

    public readonly TimeDuration ToTimeDurationSinceUnixEpoch() => TimeDurationSince(UNIX_EPOCH);

    public static Timestamp FromTimeSpanSinceUnixEpoch(TimeSpan timeSpan) =>
        FromTimeDurationSinceUnixEpoch((TimeDuration)timeSpan);

    public readonly TimeSpan ToTimeSpanSinceUnixEpoch() => (TimeSpan)ToTimeDurationSinceUnixEpoch();

    public readonly TimeDuration TimeDurationSince(Timestamp earlier) =>
        new TimeDuration(MicrosecondsSinceUnixEpoch - earlier.MicrosecondsSinceUnixEpoch);

    public static Timestamp operator +(Timestamp point, TimeDuration interval) =>
        new Timestamp(point.MicrosecondsSinceUnixEpoch + interval.Microseconds);

    public int CompareTo(Timestamp that)
    {
        return this.MicrosecondsSinceUnixEpoch.CompareTo(that.MicrosecondsSinceUnixEpoch);
    }

    public static bool operator <(Timestamp l, Timestamp r)
    {
        return l.CompareTo(r) == -1;
    }

    public static bool operator >(Timestamp l, Timestamp r)
    {
        return l.CompareTo(r) == 1;
    }

    // --- auto-generated ---

    public void ReadFields(BinaryReader reader)
    {
        MicrosecondsSinceUnixEpoch = BSATN.MicrosecondsSinceUnixEpoch.Read(reader);
    }

    public readonly void WriteFields(BinaryWriter writer)
    {
        BSATN.MicrosecondsSinceUnixEpoch.Write(writer, MicrosecondsSinceUnixEpoch);
    }

    public readonly partial struct BSATN : IReadWrite<Timestamp>
    {
        internal static readonly I64 MicrosecondsSinceUnixEpoch = new();

        public Timestamp Read(BinaryReader reader) => IStructuralReadWrite.Read<Timestamp>(reader);

        public void Write(BinaryWriter writer, Timestamp value)
        {
            value.WriteFields(writer);
        }

        // --- / auto-generated ---

        // --- customized ---
        public AlgebraicType GetAlgebraicType(ITypeRegistrar registrar) =>
            // Return a Product directly, not a Ref, because this is a special type.
            new AlgebraicType.Product(
                // Using this specific name here is important.
                [new("__timestamp_micros_since_unix_epoch__", new AlgebraicType.I64(default))]
            );
        // --- / customized ---
    }
}

/// <summary>
/// A duration that represents an interval between two events (in a particular reference frame).
///
/// This type may be converted to/from a TimeSpan, but the conversion can lose precision.
/// This type has less precision than TimeSpan (units of microseconds rather than units of 100ns).
/// </summary>
[StructLayout(LayoutKind.Sequential)]
public record struct TimeDuration(long Microseconds) : IStructuralReadWrite
{
    public static readonly TimeDuration ZERO = new(0);

    public static implicit operator TimeSpan(TimeDuration d) =>
        new(d.Microseconds * Util.TicksPerMicrosecond);

    public static implicit operator TimeDuration(TimeSpan timeSpan) =>
        new(timeSpan.Ticks / Util.TicksPerMicrosecond);

    // For backwards-compatibility.
    public readonly TimeSpan ToStd() => this;

    // Should be consistent with Rust implementation of Display.
    public override readonly string ToString()
    {
        var sign = Microseconds < 0 ? "-" : "+";
        var pos = Math.Abs(Microseconds);
        var secs = pos / Util.MicrosecondsPerSecond;
        var microsRemaining = pos % Util.MicrosecondsPerSecond;
        return $"{sign}{secs}.{microsRemaining:D6}";
    }

    // --- auto-generated ---
    public void ReadFields(BinaryReader reader)
    {
        Microseconds = BSATN.__time_duration_micros__.Read(reader);
    }

    public readonly void WriteFields(BinaryWriter writer)
    {
        BSATN.__time_duration_micros__.Write(writer, Microseconds);
    }

    public readonly partial struct BSATN : IReadWrite<TimeDuration>
    {
        internal static readonly I64 __time_duration_micros__ = new();

        public TimeDuration Read(BinaryReader reader) =>
            IStructuralReadWrite.Read<TimeDuration>(reader);

        public void Write(BinaryWriter writer, TimeDuration value)
        {
            value.WriteFields(writer);
        }

        // --- customized ---
        public AlgebraicType GetAlgebraicType(ITypeRegistrar registrar) =>
            // Return a Product directly, not a Ref, because this is a special type.
            new AlgebraicType.Product(
                // Using this specific name here is important.
                [new("__time_duration_micros__", new AlgebraicType.I64(default))]
            );
        // --- / customized ---
    }
}

public partial record ScheduleAt : TaggedEnum<(TimeDuration Interval, Timestamp Time)>
{
    public static implicit operator ScheduleAt(TimeDuration duration) => new Interval(duration);

    public static implicit operator ScheduleAt(Timestamp time) => new Time(time);

    public static implicit operator ScheduleAt(TimeSpan duration) => new Interval(duration);

    public static implicit operator ScheduleAt(DateTimeOffset time) => new Time(time);

    public static long ToMicroseconds(TimeSpan interval) => ((TimeDuration)interval).Microseconds;

    public static TimeSpan TimeSpanFromMicroseconds(long intervalMicros) =>
        (TimeSpan)(new TimeDuration(intervalMicros));

    public static long ToMicrosecondsSinceUnixEpoch(DateTimeOffset time) =>
        ((Timestamp)time).MicrosecondsSinceUnixEpoch;

    public static DateTimeOffset DateTimeOffsetFromMicrosSinceUnixEpoch(
        long microsSinceUnixEpoch
    ) => (DateTimeOffset)(new Timestamp(microsSinceUnixEpoch));

    // --- auto-generated ---
    private ScheduleAt() { }

    internal enum @enum : byte
    {
        Interval,
        Time,
    }

    public sealed record Interval(TimeDuration Interval_) : ScheduleAt;

    public sealed record Time(Timestamp Time_) : ScheduleAt;

    public readonly partial struct BSATN : IReadWrite<ScheduleAt>
    {
        internal static readonly SpacetimeDB.BSATN.Enum<@enum> __enumTag = new();
        internal static readonly TimeDuration.BSATN Interval = new();
        internal static readonly Timestamp.BSATN Time = new();

        public ScheduleAt Read(BinaryReader reader) =>
            __enumTag.Read(reader) switch
            {
                @enum.Interval => new Interval(Interval.Read(reader)),
                @enum.Time => new Time(Time.Read(reader)),
                _ => throw new InvalidOperationException(
                    "Invalid tag value, this state should be unreachable."
                ),
            };

        public void Write(BinaryWriter writer, ScheduleAt value)
        {
            switch (value)
            {
                case Interval(var inner):
                    __enumTag.Write(writer, @enum.Interval);
                    Interval.Write(writer, inner);
                    break;

                case Time(var inner):
                    __enumTag.Write(writer, @enum.Time);
                    Time.Write(writer, inner);
                    break;
            }
        }

        // --- / auto-generated ---

        // --- customized ---
        public AlgebraicType GetAlgebraicType(ITypeRegistrar registrar) =>
            // Return a Sum directly, not a Ref, because this is a special type.
            new AlgebraicType.Sum(
                [
                    // Using these specific names here is important.
                    new("Interval", Interval.GetAlgebraicType(registrar)),
                    new("Time", Time.GetAlgebraicType(registrar)),
                ]
            );
        // --- / customized ---
    }
}
