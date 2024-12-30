namespace SpacetimeDB.Codegen;

using System.Collections;
using System.Collections.Immutable;
using System.Text;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.CSharp.Syntax;
using static System.Collections.StructuralComparisons;

public static class Utils
{
    // Even `ImmutableArray<T>` is not deeply equatable, which makes it a common
    // pain point for source generators as they must use only cacheable types.
    // As a result, everyone builds their own `EquatableArray<T>` type.
    public readonly record struct EquatableArray<T>(ImmutableArray<T> Array) : IEnumerable<T>
        where T : IEquatable<T>
    {
        public int Length => Array.Length;
        public T this[int index] => Array[index];

        public bool Equals(EquatableArray<T> other) => Array.SequenceEqual(other.Array);

        public override int GetHashCode() => StructuralEqualityComparer.GetHashCode(Array);

        public IEnumerator<T> GetEnumerator() => ((IEnumerable<T>)Array).GetEnumerator();

        IEnumerator IEnumerable.GetEnumerator() => ((IEnumerable)Array).GetEnumerator();
    }

    private static readonly SymbolDisplayFormat SymbolFormat = SymbolDisplayFormat
        .FullyQualifiedFormat.WithGlobalNamespaceStyle(SymbolDisplayGlobalNamespaceStyle.Omitted)
        .AddMemberOptions(SymbolDisplayMemberOptions.IncludeContainingType)
        .AddMiscellaneousOptions(
            SymbolDisplayMiscellaneousOptions.IncludeNullableReferenceTypeModifier
        );

    public static string SymbolToName(ISymbol symbol)
    {
        return symbol.ToDisplayString(SymbolFormat);
    }

    public static void RegisterSourceOutputs(
        this IncrementalValuesProvider<Scope.Extensions> methods,
        IncrementalGeneratorInitializationContext context
    )
    {
        context.RegisterSourceOutput(
            methods,
            (context, method) =>
            {
                // Unfortunately, Roslyn doesn't expose its list of valid hintName characters
                // (https://github.com/dotnet/roslyn/blob/a69841b8ca9751bee0fe9fdeedc705e198e195d9/src/Compilers/Core/Portable/SourceGeneration/AdditionalSourcesCollection.cs#L43-L66)
                // but it does complain if you try to use invalid one. Let's do a conservative cleanup.
                var name = string.Concat(
                    method.FullName.Select(c =>
                        SyntaxFacts.IsIdentifierPartCharacter(c) || c == '.' ? c : '_'
                    )
                );
                context.AddSource(
                    $"{name}.cs",
                    $"""
                    // <auto-generated />
                    #nullable enable

                    {method}
                    """
                );
            }
        );
    }

    public static string MakeRwTypeParam(string typeParam) => typeParam + "RW";

    public static string GetTypeInfo(ITypeSymbol type)
    {
        // We need to distinguish handle nullable reference types specially:
        // compiler expands something like `int?` to `System.Nullable<int>` with the nullable annotation set to `Annotated`
        // while something like `string?` is expanded to `string` with the nullable annotation set to `Annotated`...
        // Beautiful design requires beautiful hacks.
        if (
            type.NullableAnnotation == NullableAnnotation.Annotated
            && type.OriginalDefinition.SpecialType != SpecialType.System_Nullable_T
        )
        {
            // If we're here, then this is a nullable reference type like `string?` and the original definition is `string`.
            type = type.WithNullableAnnotation(NullableAnnotation.None);
            return $"SpacetimeDB.BSATN.RefOption<{type}, {GetTypeInfo(type)}>";
        }
        return type switch
        {
            ITypeParameterSymbol typeParameter => MakeRwTypeParam(typeParameter.Name),
            INamedTypeSymbol namedType => type.SpecialType switch
            {
                SpecialType.System_Boolean => "SpacetimeDB.BSATN.Bool",
                SpecialType.System_SByte => "SpacetimeDB.BSATN.I8",
                SpecialType.System_Byte => "SpacetimeDB.BSATN.U8",
                SpecialType.System_Int16 => "SpacetimeDB.BSATN.I16",
                SpecialType.System_UInt16 => "SpacetimeDB.BSATN.U16",
                SpecialType.System_Int32 => "SpacetimeDB.BSATN.I32",
                SpecialType.System_UInt32 => "SpacetimeDB.BSATN.U32",
                SpecialType.System_Int64 => "SpacetimeDB.BSATN.I64",
                SpecialType.System_UInt64 => "SpacetimeDB.BSATN.U64",
                SpecialType.System_Single => "SpacetimeDB.BSATN.F32",
                SpecialType.System_Double => "SpacetimeDB.BSATN.F64",
                SpecialType.System_String => "SpacetimeDB.BSATN.String",
                SpecialType.None => GetTypeInfoForNamedType(namedType),
                _ => throw new InvalidOperationException(
                    $"Unsupported special type {type} ({type.SpecialType})"
                ),
            },
            IArrayTypeSymbol { ElementType: var elementType } => elementType.SpecialType
            == SpecialType.System_Byte
                ? "SpacetimeDB.BSATN.ByteArray"
                : $"SpacetimeDB.BSATN.Array<{elementType}, {GetTypeInfo(elementType)}>",
            _ => throw new InvalidOperationException($"Unsupported type {type}"),
        };

        static string GetTypeInfoForNamedType(INamedTypeSymbol type)
        {
            if (type.TypeKind == Microsoft.CodeAnalysis.TypeKind.Error)
            {
                throw new InvalidOperationException($"Could not resolve type {type}");
            }
            if (type.TypeKind == Microsoft.CodeAnalysis.TypeKind.Enum)
            {
                if (
                    !type.GetAttributes()
                        .Any(a => a.AttributeClass?.ToString() == "SpacetimeDB.TypeAttribute")
                )
                {
                    throw new InvalidOperationException(
                        $"Enum {type} does not have a [SpacetimeDB.Type] attribute"
                    );
                }
                return $"SpacetimeDB.BSATN.Enum<{SymbolToName(type)}>";
            }
            var result = type.OriginalDefinition.ToString() switch
            {
                // {U/I}{128/256} are not treated by C# as regular primitives, so we need to match them by type name.
                "System.Int128" => "SpacetimeDB.BSATN.I128",
                "System.UInt128" => "SpacetimeDB.BSATN.U128",
                "SpacetimeDB.I128" => "SpacetimeDB.BSATN.I128Stdb",
                "SpacetimeDB.U128" => "SpacetimeDB.BSATN.U128Stdb",
                "SpacetimeDB.I256" => "SpacetimeDB.BSATN.I256",
                "SpacetimeDB.U256" => "SpacetimeDB.BSATN.U256",
                "System.Collections.Generic.List<T>" => $"SpacetimeDB.BSATN.List",
                // If we're here, then this is nullable *value* type like `int?`.
                "System.Nullable<T>" => $"SpacetimeDB.BSATN.ValueOption",
                var name when name.StartsWith("System.") => throw new InvalidOperationException(
                    $"Unsupported system type {name}"
                ),
                _ => $"{SymbolToName(type)}.BSATN",
            };
            if (type.IsGenericType)
            {
                result =
                    $"{result}<{string.Join(", ", type.TypeArguments.Select(SymbolToName).Concat(type.TypeArguments.Select(GetTypeInfo)))}>";
            }

            return result;
        }
    }

    // Polyfill for .NET methods from .NET Standard 2.1+:
    private static StringBuilder AppendJoin<T>(
        this StringBuilder sb,
        string separator,
        IEnumerable<T> values
    )
    {
        var first = true;
        foreach (var value in values)
        {
            if (!first)
            {
                sb.Append(separator);
            }
            first = false;
            sb.Append(value);
        }
        return sb;
    }

    private static object? ResolveConstant(TypedConstant constant, System.Type targetType)
    {
        if (constant.Kind == TypedConstantKind.Array)
        {
            // We can't use LINQ ToArray() here because it doesn't support dynamic Type
            // and will build `object[]` instead of the desired `T[]`.
            var elementType = targetType.GetElementType();
            var array = Array.CreateInstance(elementType, constant.Values.Length);
            for (var i = 0; i < constant.Values.Length; i++)
            {
                array.SetValue(ResolveConstant(constant.Values[i], elementType), i);
            }
            return array;
        }
        return constant.Value;
    }

    public static T ParseAs<T>(this AttributeData attrData, System.Type? type = null)
        where T : Attribute
    {
        type ??= typeof(T);

        // For now only support attributes with a single constructor.
        //
        // Proper overload resolution is complicated due to implicit casts
        // (in particular, enums are represented as integers in the attribute data),
        // which prevent APIs like `Activator.CreateInstance` from finding the constructor.
        //
        // Expand logic in the future if it ever becomes actually necessary.
        var ctor = type.GetConstructors().Single();

        var ctorArgs = attrData
            .ConstructorArguments.Zip(
                ctor.GetParameters().Select(param => param.ParameterType),
                ResolveConstant
            )
            .ToArray();
        var attr = (T)ctor.Invoke(ctorArgs);
        foreach (var arg in attrData.NamedArguments)
        {
            var prop = type.GetProperty(arg.Key);
            prop.SetValue(attr, ResolveConstant(arg.Value, prop.PropertyType));
        }
        return attr;
    }

    // Borrowed & modified code for generating in-place extensions for partial structs/classes/etc. Source:
    // https://andrewlock.net/creating-a-source-generator-part-5-finding-a-type-declarations-namespace-and-type-hierarchy/

    public readonly record struct Scope
    {
        // Reversed list of typescopes, from innermost to outermost.
        private readonly EquatableArray<TypeScope> typeScopes;

        // Reversed list of namespaces, from innermost to outermost.
        private readonly EquatableArray<string> namespaces;

        public Scope(MemberDeclarationSyntax? node)
        {
            var typeScopes_ = ImmutableArray.CreateBuilder<TypeScope>();
            // Keep looping while we're in a supported nested type
            while (node is TypeDeclarationSyntax type)
            {
                // Record the parent type keyword (class/struct etc), name, and constraints
                typeScopes_.Add(
                    new TypeScope(
                        Keyword: type.Keyword.ValueText,
                        Name: type.Identifier.ToString() + type.TypeParameterList,
                        Constraints: type.ConstraintClauses.ToString()
                    )
                ); // set the child link (null initially)

                // Move to the next outer type
                node = type.Parent as MemberDeclarationSyntax;
            }
            typeScopes = new(typeScopes_.ToImmutable());

            // We've now reached the outermost type, so we can determine the namespace
            var namespaces_ = ImmutableArray.CreateBuilder<string>();
            while (node is BaseNamespaceDeclarationSyntax ns)
            {
                namespaces_.Add(ns.Name.ToString());
                node = node.Parent as MemberDeclarationSyntax;
            }
            namespaces = new(namespaces_.ToImmutable());
        }

        public readonly record struct TypeScope(string Keyword, string Name, string Constraints);

        public sealed record Extensions(Scope Scope, string FullName)
        {
            public readonly StringBuilder Contents = new();
            public readonly List<string> BaseTypes = [];
            public readonly List<string> ExtraAttrs = [];

            public override string ToString()
            {
                var sb = new StringBuilder();

                // Join all namespaces into a single namespace statement, starting with the outermost.
                if (Scope.namespaces.Length > 0)
                {
                    sb.Append("namespace ")
                        .AppendJoin(".", Scope.namespaces.Reverse())
                        .AppendLine(" {");
                }

                // Loop through the full parent type hiearchy, starting with the outermost.
                foreach (
                    var (i, typeScope) in Scope.typeScopes.Select((ts, i) => (i, ts)).Reverse()
                )
                {
                    if (i == 0)
                    {
                        foreach (var extraAttr in ExtraAttrs)
                        {
                            sb.AppendLine(extraAttr);
                        }
                    }

                    sb.Append("partial ")
                        .Append(typeScope.Keyword) // e.g. class/struct/record
                        .Append(' ')
                        .Append(typeScope.Name) // e.g. Outer/Generic<T>
                        .Append(' ');

                    if (i == 0 && BaseTypes.Count > 0)
                    {
                        sb.Append(" : ").AppendJoin(", ", BaseTypes);
                    }

                    if (typeScope.Constraints.Length > 0)
                    {
                        sb.Append(' ').Append(typeScope.Constraints);
                    }
                    sb.AppendLine(" {");
                }

                sb.AppendLine();
                sb.Append(Contents);
                sb.AppendLine();

                // We need to "close" each of the parent types, so write
                // the required number of '}'
                foreach (var typeScope in Scope.typeScopes)
                {
                    sb.Append("} // ").AppendLine(typeScope.Name);
                }

                // Close the namespace, if we had one
                if (Scope.namespaces.Length > 0)
                {
                    sb.AppendLine("} // namespace");
                }

                return sb.ToString();
            }
        }
    }
}
