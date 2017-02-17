//! Serialize a Rust data structure into ROSMSG binary data.
//!
//! Data types supported by ROSMSG are supported as well. This results in the
//! lack of support for:
//!
//! * Enums of any type, including `Option`
//! * `char`, so use one character `String`s instead
//! * Maps that can't be boiled down to `<String, String>`

use byteorder::{LittleEndian, WriteBytesExt};
use serde::ser::{self, Impossible};
use super::error::{Error, ErrorKind, Result};
use std::io;

/// A structure for serializing Rust values into ROSMSG binary data.
pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W>
    where W: io::Write
{
    /// Creates a new ROSMSG serializer.
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer { writer: writer }
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }

    #[inline]
    fn write_size(&mut self, len: usize) -> io::Result<()> {
        self.writer.write_u32::<LittleEndian>(len as u32)
    }
}

type SerializerResult = Result<()>;

macro_rules! impl_nums {
    ($ty:ty, $ser_method:ident, $writer_method:ident) => {
        #[inline]
        fn $ser_method(self, v: $ty) -> SerializerResult {
            self.writer.$writer_method::<LittleEndian>(v).map_err(|v| v.into())
        }
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = CompoundMap<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Impossible<(), Error>;

    #[inline]
    fn serialize_bool(self, v: bool) -> SerializerResult {
        self.writer.write_u8(if v { 1 } else { 0 }).map_err(|v| v.into())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> SerializerResult {
        self.writer.write_i8(v).map_err(|v| v.into())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> SerializerResult {
        self.writer.write_u8(v).map_err(|v| v.into())
    }

    impl_nums!(u16, serialize_u16, write_u16);
    impl_nums!(u32, serialize_u32, write_u32);
    impl_nums!(u64, serialize_u64, write_u64);
    impl_nums!(i16, serialize_i16, write_i16);
    impl_nums!(i32, serialize_i32, write_i32);
    impl_nums!(i64, serialize_i64, write_i64);
    impl_nums!(f32, serialize_f32, write_f32);
    impl_nums!(f64, serialize_f64, write_f64);


    #[inline]
    fn serialize_char(self, _v: char) -> SerializerResult {
        bail!(ErrorKind::UnsupportedCharType)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> SerializerResult {
        self.serialize_bytes(value.as_bytes())
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> SerializerResult {
        self.write_size(value.len())
            .and_then(|_| self.writer.write_all(value))
            .map_err(|v| v.into())
    }

    #[inline]
    fn serialize_none(self) -> SerializerResult {
        bail!(ErrorKind::UnsupportedEnumType)
    }

    #[inline]
    fn serialize_some<T: ?Sized + ser::Serialize>(self, _value: &T) -> SerializerResult {
        bail!(ErrorKind::UnsupportedEnumType)
    }

    #[inline]
    fn serialize_unit(self) -> SerializerResult {
        self.serialize_tuple(0).and(Ok(()))
    }

    #[inline]
    fn serialize_unit_struct(self, name: &'static str) -> SerializerResult {
        self.serialize_tuple_struct(name, 0).and(Ok(()))
    }

    #[inline]
    fn serialize_unit_variant(self,
                              _name: &'static str,
                              _variant_index: usize,
                              _variant: &'static str)
                              -> SerializerResult {
        bail!(ErrorKind::UnsupportedEnumType)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(self,
                                                            _name: &'static str,
                                                            value: &T)
                                                            -> SerializerResult {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(self,
                                                             _name: &'static str,
                                                             _variant_index: usize,
                                                             _variant: &'static str,
                                                             _value: &T)
                                                             -> SerializerResult {
        bail!(ErrorKind::UnsupportedEnumType)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        use serde::Serialize;
        let size = match len {
            Some(v) => v as u32,
            None => bail!(ErrorKind::VariableArraySizeAnnotation),
        };

        let mut v = Compound::new(self);
        size.serialize(&mut Serializer::new(&mut v.buffer))?;
        Ok(v)
    }

    #[inline]
    fn serialize_seq_fixed_size(self, _size: usize) -> Result<Self::SerializeSeq> {
        Ok(Compound::new(self))
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq_fixed_size(len)
    }

    #[inline]
    fn serialize_tuple_struct(self,
                              _name: &'static str,
                              len: usize)
                              -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq_fixed_size(len)
    }

    #[inline]
    fn serialize_tuple_variant(self,
                               _name: &'static str,
                               _variant_index: usize,
                               _variant: &'static str,
                               _len: usize)
                               -> Result<Self::SerializeTupleVariant> {
        bail!(ErrorKind::UnsupportedEnumType)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(CompoundMap::new(self))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_seq_fixed_size(len)
    }

    #[inline]
    fn serialize_struct_variant(self,
                                _name: &'static str,
                                _variant_index: usize,
                                _variant: &'static str,
                                _len: usize)
                                -> Result<Self::SerializeStructVariant> {
        bail!(ErrorKind::UnsupportedEnumType)
    }
}

#[doc(hidden)]
pub struct Compound<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
    buffer: Vec<u8>,
}

impl<'a, W> Compound<'a, W> {
    #[inline]
    fn new(ser: &'a mut Serializer<W>) -> Compound<'a, W> {
        Compound {
            ser: ser,
            buffer: Vec::new(),
        }
    }
}

impl<'a, W> ser::SerializeSeq for Compound<'a, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(&mut Serializer::new(&mut self.buffer))
    }

    #[inline]
    fn end(self) -> Result<()> {
        use serde::Serializer;
        self.ser.serialize_bytes(&self.buffer)
    }
}

impl<'a, W> ser::SerializeTuple for Compound<'a, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(&mut Serializer::new(&mut self.buffer))
    }

    #[inline]
    fn end(self) -> Result<()> {
        use serde::Serializer;
        self.ser.serialize_bytes(&self.buffer)
    }
}

impl<'a, W> ser::SerializeTupleStruct for Compound<'a, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(&mut Serializer::new(&mut self.buffer))
    }

    #[inline]
    fn end(self) -> Result<()> {
        use serde::Serializer;
        self.ser.serialize_bytes(&self.buffer)
    }
}

impl<'a, W> ser::SerializeStruct for Compound<'a, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<()>
        where T: ser::Serialize
    {
        value.serialize(&mut Serializer::new(&mut self.buffer))
    }

    #[inline]
    fn end(self) -> Result<()> {
        use serde::Serializer;
        self.ser.serialize_bytes(&self.buffer)
    }
}

#[doc(hidden)]
pub struct CompoundMap<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
    buffer: Serializer<Vec<u8>>,
    item: Vec<u8>,
}

impl<'a, W> CompoundMap<'a, W> {
    #[inline]
    fn new(ser: &'a mut Serializer<W>) -> CompoundMap<'a, W> {
        CompoundMap {
            ser: ser,
            buffer: Serializer::new(Vec::new()),
            item: Vec::new(),
        }
    }
}

impl<'a, W> ser::SerializeMap for CompoundMap<'a, W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
        where T: ser::Serialize
    {
        self.item = Vec::<u8>::new();
        let mut buffer = Vec::<u8>::new();
        key.serialize(&mut Serializer::new(&mut buffer))?;
        self.item.extend(buffer.into_iter().skip(4));
        self.item.push(b'=');
        Ok(())
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
        where T: ser::Serialize
    {
        use serde::Serializer as SerializerTrait;
        let mut buffer = Vec::<u8>::new();
        value.serialize(&mut Serializer::new(&mut buffer))?;
        self.item.extend(buffer.into_iter().skip(4));
        self.buffer.serialize_bytes(&self.item)
    }

    #[inline]
    fn end(self) -> Result<()> {
        use serde::Serializer;
        self.ser.serialize_bytes(&self.buffer.into_inner())
    }
}

impl ser::Error for Error {
    #[inline]
    fn custom<T: ::std::fmt::Display>(msg: T) -> Self {
        format!("{}", msg).into()
    }
}

/// Serialize the given data structure `T` as ROSMSG into the IO stream.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail. It can also fail if the structure contains unsupported elements.
///
/// Finally, it can also fail due to writer failure.
#[inline]
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
    where W: io::Write,
          T: ser::Serialize
{
    value.serialize(&mut Serializer::new(writer))
}

/// Serialize the given data structure `T` as a ROSMSG byte vector.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail. It can also fail if the structure contains unsupported elements.
#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
    where T: ser::Serialize
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn writes_u8() {
        assert_eq!(vec![150], to_vec(&150u8).unwrap());
    }

    #[test]
    fn writes_u16() {
        assert_eq!(vec![0x34, 0xA2], to_vec(&0xA234u16).unwrap());
    }

    #[test]
    fn writes_u32() {
        assert_eq!(vec![0x45, 0x23, 1, 0xCD], to_vec(&0xCD012345u32).unwrap());
    }

    #[test]
    fn writes_u64() {
        assert_eq!(vec![0xBB, 0xAA, 0x10, 0x32, 0x54, 0x76, 0x98, 0xAB],
                   to_vec(&0xAB9876543210AABBu64).unwrap());
    }

    #[test]
    fn writes_i8() {
        assert_eq!(vec![156], to_vec(&-100i8).unwrap());
    }

    #[test]
    fn writes_i16() {
        assert_eq!(vec![0xD0, 0x8A], to_vec(&-30000i16).unwrap());
    }

    #[test]
    fn writes_i32() {
        assert_eq!(vec![0x00, 0x6C, 0xCA, 0x88],
                   to_vec(&-2000000000i32).unwrap());
    }

    #[test]
    fn writes_i64() {
        assert_eq!(vec![0x00, 0x00, 0x7c, 0x1d, 0xaf, 0x93, 0x19, 0x83],
                   to_vec(&-9000000000000000000i64).unwrap());
    }

    #[test]
    fn writes_f32() {
        assert_eq!(vec![0x00, 0x70, 0x7b, 0x44], to_vec(&1005.75f32).unwrap());
    }

    #[test]
    fn writes_f64() {
        assert_eq!(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x6e, 0x8f, 0x40],
                   to_vec(&1005.75f64).unwrap());
    }

    #[test]
    fn writes_bool() {
        assert_eq!(vec![1], to_vec(&true).unwrap());
        assert_eq!(vec![0], to_vec(&false).unwrap());
    }

    #[test]
    fn writes_string() {
        assert_eq!(vec![0, 0, 0, 0], to_vec(&"").unwrap());
        assert_eq!(vec![13, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33],
                   to_vec(&"Hello, World!").unwrap());
    }

    #[test]
    fn writes_array() {
        assert_eq!(vec![8, 0, 0, 0, 7, 0, 1, 4, 33, 0, 57, 0],
                   to_vec(&[7i16, 1025, 33, 57]).unwrap());
    }

    #[test]
    fn writes_vector() {
        assert_eq!(vec![12, 0, 0, 0, 4, 0, 0, 0, 7, 0, 1, 4, 33, 0, 57, 0],
                   to_vec(&vec![7i16, 1025, 33, 57]).unwrap());
    }

    #[test]
    fn writes_tuple() {
        assert_eq!(vec![26, 0, 0, 0, 2, 8, 1, 7, 6, 0, 0, 0, 65, 66, 67, 48, 49, 50, 8, 0, 0, 0,
                        4, 0, 0, 0, 1, 0, 0, 1],
                   to_vec(&(2050i16, true, 7u8, "ABC012", vec![true, false, false, true]))
                       .unwrap());
    }

    #[derive(Serialize)]
    struct TestStructOne {
        a: i16,
        b: bool,
        c: u8,
        d: String,
        e: Vec<bool>,
    }

    #[test]
    fn writes_simple_struct() {
        let v = TestStructOne {
            a: 2050i16,
            b: true,
            c: 7u8,
            d: String::from("ABC012"),
            e: vec![true, false, false, true],
        };
        assert_eq!(vec![26, 0, 0, 0, 2, 8, 1, 7, 6, 0, 0, 0, 65, 66, 67, 48, 49, 50, 8, 0, 0, 0,
                        4, 0, 0, 0, 1, 0, 0, 1],
                   to_vec(&v).unwrap());
    }

    #[derive(Serialize)]
    struct TestStructPart {
        a: String,
        b: bool,
    }

    #[derive(Serialize)]
    struct TestStructBig {
        a: Vec<TestStructPart>,
        b: String,
    }

    #[test]
    fn writes_complex_struct() {
        let mut parts = Vec::new();
        parts.push(TestStructPart {
            a: String::from("ABC"),
            b: true,
        });
        parts.push(TestStructPart {
            a: String::from("1!!!!"),
            b: true,
        });
        parts.push(TestStructPart {
            a: String::from("234b"),
            b: false,
        });
        let v = TestStructBig {
            a: parts,
            b: String::from("EEe"),
        };
        assert_eq!(vec![54, 0, 0, 0, 43, 0, 0, 0, 3, 0, 0, 0, 8, 0, 0, 0, 3, 0, 0, 0, 65, 66, 67,
                        1, 10, 0, 0, 0, 5, 0, 0, 0, 49, 33, 33, 33, 33, 1, 9, 0, 0, 0, 4, 0, 0,
                        0, 50, 51, 52, 98, 0, 3, 0, 0, 0, 69, 69, 101],
                   to_vec(&v).unwrap());
    }

    #[test]
    fn writes_empty_string_string_map() {
        let data = HashMap::<String, String>::new();
        assert_eq!(vec![0, 0, 0, 0], to_vec(&data).unwrap());
    }

    #[test]
    fn writes_single_item_string_string_map() {
        let mut data = HashMap::<String, String>::new();
        data.insert(String::from("abc"), String::from("123"));
        assert_eq!(vec![11, 0, 0, 0, 7, 0, 0, 0, 97, 98, 99, 61, 49, 50, 51],
                   to_vec(&data).unwrap());
    }

    #[test]
    fn writes_multiple_item_string_string_map() {
        let mut data = HashMap::<String, String>::new();
        data.insert(String::from("abc"), String::from("123"));
        data.insert(String::from("AAA"), String::from("B0"));
        let answer = to_vec(&data).unwrap();
        assert!(vec![21, 0, 0, 0, 7, 0, 0, 0, 97, 98, 99, 61, 49, 50, 51, 6, 0, 0, 0, 65,
                     65, 65, 61, 66, 48] == answer ||
                vec![21, 0, 0, 0, 6, 0, 0, 0, 65, 65, 65, 61, 66, 48, 7, 0, 0, 0, 97, 98, 99,
                     61, 49, 50, 51] == answer);
    }
}
