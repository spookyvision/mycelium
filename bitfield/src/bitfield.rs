/// Generates a typed bitfield struct.
///
/// By default, the [`fmt::Debug`], [`fmt::Display`], [`fmt::Binary`], [`Copy`],
/// and [`Clone`] traits are automatically derived for bitfields.
///
/// All bitfield types are [`#[repr(transparent)]`][transparent].
///
/// For a complete example of the methods generated by the `bitfield!` macro,
/// see the [`example`] module's [`ExampleBitfield`] type.
///
/// # Generated Implementations
///
/// The `bitfield!` macro generates a type with the following functions, where
/// `T` is the integer type that represents the bitfield (one of `u8`, `u16`,
/// `u32`, `u64`, or `usize`):
///
/// | Function | Description |
/// |:--|:--|
/// | `fn new() -> Self` | Returns a new instance of the bitfield type with all bits zeroed. |
/// | `fn from_bits(bits: T) -> Self` | Converts a `T` into an instance of the bitfield type. |
/// | `fn with<U>(self, packer: Self::Packer<U>, value: U) -> Self` | Given one of this type's generated packing specs for a `U`-typed value, and a `U`-typed value, returns a new instance of `Self` with the bit representation of `value` packed into the range represented by `packer`. |
/// | `fn set<U>(&mut self, packer: Self::Packer<U>, value: U) -> &mut Self` | Similar to `with`, except `self` is mutated in place, rather than returning a new  instance of `Self`. |
/// | `fn get<U>(&self, packer: Self::Packer<U>) -> U` | Given one of this type's generated packing specs for a `U`-typed value, unpacks the bit range represented by that value as a `U` and returns it. This method panics if the requested bit range does not contain a valid bit pattern for a `U`-typed value, as determined by `U`'s implementation of the [`FromBits`] trait. |
/// | `fn try_get<U>(&self, packer: Self::Packer<U>) -> Result<U, <U as FromBits>::Error>` | Like `get`, but returns a `Result` instead of panicking. |
/// | `fn assert_valid()` | Asserts that the generated bitfield type is valid. This is primarily intended to be used in tests; the macro cannot generate tests for a bitfield type on its own, so a test that simply calls `assert_valid` can be added to check the bitfield type's validity. |
///
/// The visibility of these methods depends on the visibility of the bitfield
/// struct --- if the struct is defined as `pub(crate) struct MyBitfield<u16> {
/// ... }`, then these functions will all be `pub(crate)` as well.
///
/// If a bitfield type is defined with one visibility, but particular subfields
/// of that bitfield should not be public, the individual fields may also have
/// visibility specifiers. For example, if the bitfield struct `MyBitfield` is
/// `pub`, but the subfield named `PRIVATE_SUBFIELD` is `pub(crate)`, then
/// `my_bitfield.get(MyBitfield::PRIVATE_SUBRANGE)` can only be called inside
/// the crate defining the type, because the `PRIVATE_SUBRANGE` constant is not
/// publicly visible.
///
/// In addition to the inherent methods discussed above, the following trait
/// implementations are always generated:
///
/// | Trait | Description |
/// |:--|:--|
/// | [`fmt::Debug`] | The `Debug` implementation prints the bitfield as a "struct", with a "field" for each packing spec in the bitfield. If any of the bitfield's packing specs pack typed values, that type's [`fmt::Debug`] implementation is used rather than printing the value as an integer. |
/// | [`fmt::Binary`] | Prints the raw bits of this bitfield as a binary number. |
/// | [`fmt::UpperHex`] and [`fmt::LowerHex`] | Prints the raw bits of this bitfield in hexadecimal. |
/// | [`fmt::Display`] | Pretty-prints the bitfield in a very nice-looking multi-line format which I'm rather proud of. See [here](#example-display-output) for examples of this format. |
/// | [`Copy`] | Behaves identically as the [`Copy`] implementation for the underlying integer type. |
/// | [`Clone`] | Behaves identically as the [`Clone`] implementation for the underlying integer type. |
///
/// Additional traits may be derived for the bitfield type, such as
/// [`PartialEq`], [`Eq`], and [`Default`]. These traits are not automatically
/// derived, as custom implementations may also be desired, depending on the
/// use-case. For example, the `Default` value for a bitfield may _not_ be all
/// zeroes.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// mycelium_bitfield::bitfield! {
///     /// Bitfield types can have doc comments.
///     #[derive(Eq, PartialEq)] // ...and attributes
///     pub struct MyBitfield<u16> {
///         // Generates a packing spec named `HELLO` for the first 6
///         // least-significant bits.
///         pub const HELLO = 6;
///         // Fields with names starting with `_` can be used to mark bits as
///         // reserved.
///         const _RESERVED = 4;
///         // Generates a packing spec named `WORLD` for the next 3 bits.
///         pub const WORLD = 3;
///     }
/// }
///
/// // Bitfield types can be cheaply constructed from a raw numeric
/// // representation:
/// let bitfield = MyBitfield::from_bits(0b10100_0011_0101);
///
/// // `get` methods can be used to unpack fields from a bitfield type:
/// assert_eq!(bitfield.get(MyBitfield::HELLO), 0b11_0101);
/// assert_eq!(bitfield.get(MyBitfield::WORLD), 0b0101);
///
/// // `with` methods can be used to pack bits into a bitfield type by
/// // value:
/// let bitfield2 = MyBitfield::new()
///     .with(MyBitfield::HELLO, 0b11_0101)
///     .with(MyBitfield::WORLD, 0b0101);
///
/// assert_eq!(bitfield, bitfield2);
///
/// // `set` methods can be used to mutate a bitfield type in place:
/// let mut bitfield3 = MyBitfield::new();
///
/// bitfield3
///     .set(MyBitfield::HELLO, 0b011_0101)
///     .set(MyBitfield::WORLD, 0b0101);
///
/// assert_eq!(bitfield, bitfield3);
/// ```
///
/// Bitfields may also contain typed values, as long as those values implement
/// the [`FromBits`] trait:
///
/// ```
/// use mycelium_bitfield::{bitfield, FromBits};
///
/// // An enum type can implement the `FromBits` trait if it has a
/// // `#[repr(uN)]` attribute.
/// #[repr(u8)]
/// #[derive(Debug, Eq, PartialEq)]
/// enum MyEnum {
///     Foo = 0b00,
///     Bar = 0b01,
///     Baz = 0b10,
/// }
///
/// impl FromBits<u32> for MyEnum {
///     // Two bits can represent all possible `MyEnum` values.
///     const BITS: u32 = 2;
///     type Error = &'static str;
///
///     fn try_from_bits(bits: u32) -> Result<Self, Self::Error> {
///         match bits as u8 {
///             bits if bits == Self::Foo as u8 => Ok(Self::Foo),
///             bits if bits == Self::Bar as u8 => Ok(Self::Bar),
///             bits if bits == Self::Baz as u8 => Ok(Self::Baz),
///             _ => Err("expected one of 0b00, 0b01, or 0b10"),
///         }
///     }
///
///     fn into_bits(self) -> u32 {
///         self as u8 as u32
///     }
/// }
///
/// bitfield! {
///     pub struct TypedBitfield<u32> {
///         /// Use the first two bits to represent a typed `MyEnum` value.
///         const ENUM_VALUE: MyEnum;
///
///         /// Typed values and untyped raw bit fields can be used in the
///         /// same bitfield type.
///         pub const SOME_BITS = 6;
///
///         /// The `FromBits` trait is also implemented for `bool`, which
///         /// can be used to implement bitflags.
///         pub const FLAG_1: bool;
///         pub const FLAG_2: bool;
///
///         /// `FromBits` is also implemented by (signed and unsigned) integer
///         /// types. This will allow the next 8 bits to be treated as a `u8`.
///         pub const A_BYTE: u8;
///     }
/// }
///
/// // Unpacking a typed value with `get` will return that value, or panic if
/// // the bit pattern is invalid:
/// let my_bitfield = TypedBitfield::from_bits(0b0011_0101_1001_1110);
///
/// assert_eq!(my_bitfield.get(TypedBitfield::ENUM_VALUE), MyEnum::Baz);
/// assert_eq!(my_bitfield.get(TypedBitfield::FLAG_1), true);
/// assert_eq!(my_bitfield.get(TypedBitfield::FLAG_2), false);
///
/// // The `try_get` method will return an error rather than panicking if an
/// // invalid bit pattern is encountered:
///
/// let invalid = TypedBitfield::from_bits(0b0011);
///
/// // There is no `MyEnum` variant for 0b11.
/// assert!(invalid.try_get(TypedBitfield::ENUM_VALUE).is_err());
/// ```
///
/// Packing specs from one bitfield type may *not* be used with a different
/// bitfield type's `get`, `set`, or `with` methods. For example, the following
/// is a type error:
///
/// ```compile_fail
/// use mycelium_bitfield::bitfield;
///
/// bitfield! {
///     struct Bitfield1<u8> {
///         pub const FOO: bool;
///         pub const BAR: bool;
///         pub const BAZ = 6;
///     }
/// }
///
/// bitfield! {
///     struct Bitfield2<u8> {
///         pub const ALICE = 2;
///         pub const BOB = 4;
///         pub const CHARLIE = 2;
///     }
/// }
///
///
/// // This is a *type error*, because `Bitfield2`'s field `ALICE` cannot be
/// // used with a `Bitfield2` value:
/// let bits = Bitfield1::new().with(Bitfield2::ALICE, 0b11);
/// ```
///
/// ## Example `Display` Output
///
/// Bitfields will automatically generate a pretty [`fmt::Display`]
/// implementation:
///
/// ```
/// # use mycelium_bitfield::{bitfield, FromBits};
/// #
/// # #[repr(u8)]
/// # #[derive(Debug, Eq, PartialEq)]
/// # enum MyEnum {
/// #     Foo = 0b00,
/// #     Bar = 0b01,
/// #     Baz = 0b10,
/// # }
/// #
/// # impl FromBits<u32> for MyEnum {
/// #     const BITS: u32 = 2;
/// #     type Error = &'static str;
/// #
/// #     fn try_from_bits(bits: u32) -> Result<Self, Self::Error> {
/// #         match bits as u8 {
/// #             bits if bits == Self::Foo as u8 => Ok(Self::Foo),
/// #             bits if bits == Self::Bar as u8 => Ok(Self::Bar),
/// #             bits if bits == Self::Baz as u8 => Ok(Self::Baz),
/// #             _ => Err("expected one of 0b00, 0b01, or 0b10"),
/// #         }
/// #     }
/// #
/// #     fn into_bits(self) -> u32 {
/// #         self as u8 as u32
/// #     }
/// # }
/// # bitfield! {
/// #      pub struct TypedBitfield<u32> {
/// #          const ENUM_VALUE: MyEnum;
/// #          pub const SOME_BITS = 6;
/// #          pub const FLAG_1: bool;
/// #          pub const FLAG_2: bool;
/// #          pub const A_BYTE: u8;
/// #      }
/// # }
///
/// let my_bitfield = TypedBitfield::from_bits(0b0011_0101_1001_1110);
/// let formatted = format!("{my_bitfield}");
/// let expected = r#"
/// 00000000000000000011010110011110
///               └┬─────┘││└┬───┘└┤
///                │      ││ │     └ ENUM_VALUE: Baz (10)
///                │      ││ └────── SOME_BITS: 39 (100111)
///                │      │└─────────── FLAG_1: true (1)
///                │      └──────────── FLAG_2: false (0)
///                └─────────────────── A_BYTE: 13 (00001101)
/// "#.trim_start();
/// assert_eq!(formatted, expected);
/// ```
/// [`fmt::Debug`]: core::fmt::Debug
/// [`fmt::Display`]: core::fmt::Display
/// [`fmt::Binary`]: core::fmt::Binary
/// [`fmt::UpperHex`]: core::fmt::UpperHex
/// [`fmt::LowerHex`]: core::fmt::LowerHex
/// [transparent]: https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation
/// [`example`]: crate::example
/// [`ExampleBitfield`]: crate::example::ExampleBitfield
/// [`FromBits`]: crate::FromBits
#[macro_export]
macro_rules! bitfield {
    (
        $(#[$($meta:meta)+])*
        $vis:vis struct $Name:ident<$T:ident> {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis const $Field:ident $(: $F:ty)? $( = $val:tt)?;
            )+
        }
    ) => {
        $(#[$($meta)+])*
        #[derive(Copy, Clone)]
        #[repr(transparent)]
        $vis struct $Name($T);

        #[automatically_derived]
        impl core::fmt::Debug for $Name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut dbg = f.debug_struct(stringify!($Name));
                $(
                    dbg.field(stringify!($Field), &self.get(Self::$Field));
                )+
                dbg.finish()

            }
        }

        // Some generated methods may not always be used, which may emit dead
        // code warnings if the type is private.
        #[allow(dead_code)]
        #[automatically_derived]
        impl $Name {
            $crate::bitfield! { @field<$T>:
                $(
                    $(#[$field_meta])*
                    $field_vis const $Field $(: $F)? $( = $val)?;
                )+
            }

            const FIELDS: &'static [(&'static str, $crate::bitfield! { @t $T, $T, Self })] = &[$(
                (stringify!($Field), Self::$Field.typed())
            ),+];

            /// Constructs a new instance of `Self` from the provided raw bits.
            $vis const fn from_bits(bits: $T) -> Self {
                Self(bits)
            }

            /// Constructs a new instance of `Self` with all bits set to 0.
            $vis const fn new() -> Self {
                Self(0)
            }

            /// Packs the bit representation of `value` into `self` at the bit
            /// range designated by `field`, returning a new bitfield.
            $vis fn with<T>(self, field: $crate::bitfield! { @t $T, T, Self }, value: T) -> Self
            where
                T: $crate::FromBits<$T>,
            {
                Self(field.pack(value, self.0))
            }


            /// Packs the bit representation of `value` into `self` at the range
            /// designated by `field`, mutating `self` in place.
            $vis fn set<T>(&mut self, field: $crate::bitfield! { @t $T, T, Self }, value: T) -> &mut Self
            where
                T: $crate::FromBits<$T>,
            {
                field.pack_into(value, &mut self.0);
                self
            }

            /// Unpacks the bit range represented by `field` from `self`, and
            /// converts it into a `T`-typed value.
            ///
            /// # Panics
            ///
            /// This method panics if `self` does not contain a valid bit
            /// pattern for a `T`-typed value, as determined by `T`'s
            /// `FromBits::try_from_bits` implementation.
            $vis fn get<T>(self, field: $crate::bitfield! { @t $T, T, Self }) -> T
            where
                T: $crate::FromBits<$T>,
            {
                field.unpack(self.0)
            }

            /// Unpacks the bit range represented by `field`
            /// from `self` and attempts to convert it into a `T`-typed value.
            ///
            /// # Returns
            ///
            /// - `Ok(T)` if a `T`-typed value could be constructed from the
            ///   bits in `src`
            /// - `Err(T::Error)` if `src` does not contain a valid bit
            ///   pattern for a `T`-typed value, as determined by `T`'s
            ///   [`FromBits::try_from_bits` implementation.
            $vis fn try_get<T>(self, field: $crate::bitfield! { @t $T, T, Self }) -> Result<T, T::Error>
            where
                T: $crate::FromBits<$T>,
            {
                field.try_unpack(self.0)
            }

            /// Asserts that all the packing specs for this type are valid.
            ///
            /// This is intended to be used in unit tests.
            $vis fn assert_valid() {
                <$crate::bitfield! { @t $T, $T, Self }>::assert_all_valid(&Self::FIELDS);
            }
        }

        #[automatically_derived]
        impl core::fmt::Display for $Name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.pad("")?;
                writeln!(f, "{:0width$b}", self.0, width = $T::BITS as usize)?;
                f.pad("")?;
                let mut cur_pos = $T::BITS;
                let mut max_len = 0;
                let mut rem = 0;
                let mut fields = Self::FIELDS.iter().rev().peekable();
                while let Some((name, field)) = fields.next() {
                    while cur_pos > field.most_significant_index() {
                        f.write_str(" ")?;
                        cur_pos -= 1;
                    }
                    let bits = field.bits();
                    match (name, bits) {
                        (name, bits) if name.starts_with("_") => {
                            for _ in 0..bits {
                                f.write_str(" ")?;
                            }
                            cur_pos -= bits;
                            continue;
                        }
                        (_, 1) => f.write_str("│")?,
                        (_, 2) => f.write_str("└┤")?,
                        (_, bits) => {
                            f.write_str("└┬")?;
                            for _ in 0..(bits - 3) {
                                f.write_str("─")?;
                            }
                            f.write_str("┘")?;
                        }
                    }

                    if fields.peek().is_none() {
                        rem = cur_pos - (bits - 1);
                    }

                    max_len = core::cmp::max(max_len, name.len());
                    cur_pos -= field.bits()
                }

                f.write_str("\n")?;

                $(
                    let field = Self::$Field;
                    let name = stringify!($Field);
                    if !name.starts_with("_") {
                        f.pad("")?;
                        cur_pos = $T::BITS;
                        for (cur_name, cur_field) in Self::FIELDS.iter().rev() {
                            while cur_pos > cur_field.most_significant_index() {
                                f.write_str(" ")?;
                                cur_pos -= 1;
                            }

                            if field == cur_field {
                                break;
                            }

                            let bits = cur_field.bits();
                            match (cur_name, bits) {
                                (name, bits) if name.starts_with("_") => {
                                    for _ in 0..bits {
                                        f.write_str(" ")?;
                                    }
                                }
                                (_, 1) => f.write_str("│")?,
                                (_, bits) => {
                                    f.write_str(" │")?;
                                    for _ in 0..(bits - 2) {
                                        f.write_str(" ")?;
                                    }
                                }
                            }

                            cur_pos -= bits;
                        }

                        let field_bits = field.bits();
                        if field_bits == 1 {
                            f.write_str("└")?;
                            cur_pos -= 1;
                        } else {
                            f.write_str(" └")?;
                            cur_pos -= 2;
                        }
                        let len = cur_pos as usize + (max_len - name.len());
                        for _ in rem as usize..len {
                            f.write_str("─")?;
                        }
                        writeln!(f, " {}: {:?} ({:0width$b})", name, field.unpack(self.0), field.unpack_bits(self.0), width = field_bits as usize)?
                    }

                )+

                Ok(())
            }
        }

        #[automatically_derived]
        impl core::fmt::Binary for $Name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if f.alternate() {
                    f.debug_tuple(stringify!($Name)).field(&format_args!("{:#b}", self.0)).finish()
                } else {
                    f.debug_tuple(stringify!($Name)).field(&format_args!("{:b}", self.0)).finish()
                }
            }
        }

        #[automatically_derived]
        impl core::fmt::UpperHex for $Name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if f.alternate() {
                    f.debug_tuple(stringify!($Name)).field(&format_args!("{:#X}", self.0)).finish()
                } else {
                    f.debug_tuple(stringify!($Name)).field(&format_args!("{:X}", self.0)).finish()
                }
            }
        }

        #[automatically_derived]
        impl core::fmt::LowerHex for $Name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if f.alternate() {
                    f.debug_tuple(stringify!($Name)).field(&format_args!("{:#x}", self.0)).finish()
                } else {
                    f.debug_tuple(stringify!($Name)).field(&format_args!("{:x}", self.0)).finish()
                }
            }
        }
    };
    (@field<$T:ident>, prev: $Prev:ident:
        $(#[$meta:meta])*
        $vis:vis const $Field:ident = ..;
    ) => {
        $(#[$meta])*
        $vis const $Field: $crate::bitfield!{ @t $T, $T, Self } = Self::$Prev.remaining();
    };
    (@field<$T:ident>, prev: $Prev:ident:
        $(#[$meta:meta])*
        $vis:vis const $Field:ident = $value:literal;
        $($rest:tt)*
    ) => {
        $(#[$meta])*
        $vis const $Field: $crate::bitfield!{ @t $T, $T, Self } = Self::$Prev.next($value);
        $crate::bitfield!{ @field<$T>, prev: $Field: $($rest)* }
    };

    (@field<$T:ident>, prev: $Prev:ident:
        $(#[$meta:meta])*
        $vis:vis const $Field:ident: $Val:ty;
        $($rest:tt)*
    ) => {
        $(#[$meta])*
        $vis const $Field: $crate::bitfield!{ @t $T, $Val, Self } = Self::$Prev.then::<$Val>();
        $crate::bitfield!{ @field<$T>, prev: $Field: $($rest)* }
    };


    (@field<$T:ident>, prev: $Prev:ident: ) => {  };
    (@field<$T:ident>:
        $(#[$meta:meta])*
        $vis:vis const $Field:ident = $value:literal;
        $($rest:tt)*
    ) => {
        $(#[$meta])*
        $vis const $Field: $crate::bitfield!{ @t $T, $T, Self } = <$crate::bitfield!{ @t $T, $T, () }>::least_significant($value).typed();
        $crate::bitfield!{ @field<$T>, prev: $Field: $($rest)* }
    };

    (@field<$T:ident>:
        $(#[$meta:meta])*
        $vis:vis const $Field:ident: $Val:ty;
        $($rest:tt)*
    ) => {
        $(#[$meta])*
        $vis const $Field: $crate::bitfield!{ @t $T, $Val, Self } = <$crate::bitfield!{ @t $T, $Val, Self } >::first();
        $crate::bitfield!{ @field<$T>, prev: $Field: $($rest)* }
    };


    // (@process_meta $vis:vis struct $Name:ident<$T:ty> { $(#[$before:meta])* } #[derive($($Derive:path),+)] $(#[$after:meta])*) => {
    //     $crate::bitfield! { @process_derives $vis struct $Name<$T> { } $($Derive),+ { $(#[$before])* $(#[$after])* } }

    // };
    // (@process_meta $vis:vis struct $Name:ident<$T:ty> {  }) => {
    //     #[derive(Copy, Clone)]
    //     #[repr(transparent)]
    //     $vis struct $Name($T);
    // };
    // (@process_meta $vis:vis struct $Name:ident<$T:ty> { $(#[$before:meta])+ }) => {
    //     $(#[$before])*
    //     #[derive(Copy, Clone)]
    //     #[repr(transparent)]
    //     $vis struct $Name($T);
    // };
    // (@process_meta $vis:vis struct $Name:ident<$T:ty>  { $(#[$before:meta])* } #[$current:meta] $(#[$after:meta])*) => {
    //     $crate::bitfield! { @process_meta $vis struct $Name<$T> { $(#[$before])* #[$current] } $(#[$after])* }
    // };
    // (@process_derives $vis:vis struct $Name:ident<$T:ty> { Debug, } { $($rest:tt)* }) => {
    //     impl core::fmt::Debug for $Name {
    //         fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    //             if f.alternate() {
    //                 f.debug_tuple(stringify!($Name)).field(&format_args!("{}", self)).finish()
    //             } else {
    //                 f.debug_tuple(stringify!($Name)).field(&format_args!("{:#b}", self)).finish()
    //             }

    //         }
    //     }
    //     #[derive(Copy, Clone)]
    //     #[repr(transparent)]
    //     $($rest)*
    //     $vis struct $Name($T);
    // };

    // (@process_derives $vis:vis struct $Name:ident<$T:ty> { Debug, $($Before:tt),+ } $($After:tt),+ { $($rest:tt)* }) => {
    //     impl core::fmt::Debug for $Name {
    //         fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    //             if f.alternate() {
    //                 f.debug_tuple(stringify!($Name)).field(&format_args!("{}", self)).finish()
    //             } else {
    //                 f.debug_tuple(stringify!($Name)).field(&format_args!("{:#b}", self)).finish()
    //             }

    //         }
    //     }
    //     #[derive(Copy, Clone, $($Before),+ $($After),+)]
    //     #[repr(transparent)]
    //     $($rest)*
    //     $vis struct $Name($T);
    // };
    // (@process_derives $vis:vis struct $Name:ident<$T:ty> { Debug, $($Before:tt),+ } { $($rest:tt)* }) => {

    //     #[derive(Copy, Clone, $($Before),+)]
    //     #[repr(transparent)]
    //     $($rest)*
    //     $vis struct $Name($T);
    // };
    // (@process_derives $vis:vis struct $Name:ident<$T:ty> { $($Before:tt),+ $(,)? } { $($rest:tt)* }) => {
    //     #[derive($($Before),+)]
    //     #[derive(Copy, Clone)]
    //     #[repr(transparent)]
    //     $($rest)*
    //     $vis struct $Name($T);
    // };
    // (@process_derives $vis:vis struct $Name:ident<$T:ty> { $($Before:tt),* $(,)? } $Next:tt, $($After:tt),* { $($rest:tt)* }) => {
    //     $crate::bitfield! { @process_derives $vis struct $Name<$T> { $Next, $($Before),*  } $($After),* { $($rest)* } }
    // };
    // (@process_derives $vis:vis struct $Name:ident<$T:ty> { $($Before:tt),* } $Next:tt { $($rest:tt)* }) => {
    //     $crate::bitfield! { @process_derives $vis struct $Name<$T> { $Next, $($Before),* } { $($rest)* } }
    // };

    (@t usize, $V:ty, $F:ty) => { $crate::PackUsize<$V, $F> };
    (@t u64, $V:ty, $F:ty) => { $crate::Pack64<$V, $F> };
    (@t u32, $V:ty, $F:ty) => { $crate::Pack32<$V, $F> };
    (@t u16, $V:ty, $F:ty) => { $crate::Pack16<$V, $F> };
    (@t u8, $V:ty, $F:ty) => { $crate::Pack8<$V, $F> };
    (@t $T:ty, $V:ty, $F:ty) => { compile_error!(concat!("unsupported bitfield type `", stringify!($T), "`; expected one of `usize`, `u64`, `u32`, `u16`, or `u8`")) }
}

#[cfg(test)]
mod tests {
    use crate::FromBits;

    bitfield! {
        #[allow(dead_code)]
        struct TestBitfield<u32> {
            const HELLO = 4;
            const _RESERVED_1 = 3;
            const WORLD: bool;
            const HAVE: TestEnum;
            const LOTS = 5;
            const OF = 1;
            const FUN = 6;
        }
    }

    #[repr(u8)]
    #[derive(Debug)]
    enum TestEnum {
        Foo = 0b00,
        Bar = 0b01,
        Baz = 0b10,
        Qux = 0b11,
    }

    impl FromBits<u32> for TestEnum {
        const BITS: u32 = 2;
        type Error = core::convert::Infallible;

        fn try_from_bits(bits: u32) -> Result<Self, Self::Error> {
            Ok(match bits as u8 {
                bits if bits == Self::Foo as u8 => Self::Foo,
                bits if bits == Self::Bar as u8 => Self::Bar,
                bits if bits == Self::Baz as u8 => Self::Baz,
                bits if bits == Self::Qux as u8 => Self::Qux,
                bits => unreachable!("all patterns are covered: {:#b}", bits),
            })
        }

        fn into_bits(self) -> u32 {
            self as u8 as u32
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct TestDebug {
        value: usize,
        bits: TestBitfield,
    }

    #[test]
    fn test_bitfield_format() {
        let test_bitfield = TestBitfield::new()
            .with(TestBitfield::HELLO, 0b1001)
            .with(TestBitfield::WORLD, true)
            .with(TestBitfield::HAVE, TestEnum::Bar)
            .with(TestBitfield::LOTS, 0b11010)
            .with(TestBitfield::OF, 0)
            .with(TestBitfield::FUN, 9);
        println!("{}", test_bitfield);

        let test_debug = TestDebug {
            value: 42,
            bits: test_bitfield,
        };

        println!("test_debug(alt): {:#?}", test_debug);

        println!("test_debug: {:?}", test_debug)
    }

    #[test]
    fn macro_bitfield_valid() {
        TestBitfield::assert_valid();
    }
}
