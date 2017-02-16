//! Serialize a Rust data structure into ROSMSG binary data.

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

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
    where W: io::Write
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
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
    fn serialize_i16(self, v: i16) -> SerializerResult {
        self.writer.write_i16::<LittleEndian>(v).map_err(|v| v.into())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> SerializerResult {
        self.writer.write_i32::<LittleEndian>(v).map_err(|v| v.into())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> SerializerResult {
        self.writer.write_i64::<LittleEndian>(v).map_err(|v| v.into())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> SerializerResult {
        self.writer.write_u8(v).map_err(|v| v.into())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> SerializerResult {
        self.writer.write_u16::<LittleEndian>(v).map_err(|v| v.into())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> SerializerResult {
        self.writer.write_u32::<LittleEndian>(v).map_err(|v| v.into())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> SerializerResult {
        self.writer.write_u64::<LittleEndian>(v).map_err(|v| v.into())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> SerializerResult {
        self.writer.write_f32::<LittleEndian>(v).map_err(|v| v.into())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> SerializerResult {
        self.writer.write_f64::<LittleEndian>(v).map_err(|v| v.into())
    }

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
        bail!(ErrorKind::UnsupportedMapType)
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

impl ser::Error for Error {
    #[inline]
    fn custom<T: ::std::fmt::Display>(msg: T) -> Self {
        format!("{}", msg).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std;
    use serde::Serialize;

    fn pull_data<T: Serialize>(data: &T) -> Vec<u8> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        data.serialize(&mut Serializer::new(&mut cursor)).unwrap();
        cursor.into_inner()
    }

    #[test]
    fn writes_u8() {
        assert_eq!(vec![150], pull_data(&150u8));
    }

    #[test]
    fn writes_u16() {
        assert_eq!(vec![0x34, 0xA2], pull_data(&0xA234u16));
    }

    #[test]
    fn writes_u32() {
        assert_eq!(vec![0x45, 0x23, 1, 0xCD], pull_data(&0xCD012345u32));
    }

    #[test]
    fn writes_u64() {
        assert_eq!(vec![0xBB, 0xAA, 0x10, 0x32, 0x54, 0x76, 0x98, 0xAB],
                   pull_data(&0xAB9876543210AABBu64));
    }

    #[test]
    fn writes_i8() {
        assert_eq!(vec![156], pull_data(&-100i8));
    }

    #[test]
    fn writes_i16() {
        assert_eq!(vec![0xD0, 0x8A], pull_data(&-30000i16));
    }

    #[test]
    fn writes_i32() {
        assert_eq!(vec![0x00, 0x6C, 0xCA, 0x88], pull_data(&-2000000000i32));
    }

    #[test]
    fn writes_i64() {
        assert_eq!(vec![0x00, 0x00, 0x7c, 0x1d, 0xaf, 0x93, 0x19, 0x83],
                   pull_data(&-9000000000000000000i64));
    }

    #[test]
    fn writes_f32() {
        assert_eq!(vec![0x00, 0x70, 0x7b, 0x44], pull_data(&1005.75f32));
    }

    #[test]
    fn writes_f64() {
        assert_eq!(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x6e, 0x8f, 0x40],
                   pull_data(&1005.75f64));
    }

    #[test]
    fn writes_bool() {
        assert_eq!(vec![1], pull_data(&true));
        assert_eq!(vec![0], pull_data(&false));
    }

    #[test]
    fn writes_string() {
        assert_eq!(vec![0, 0, 0, 0], pull_data(&""));
        assert_eq!(vec![13, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33],
                   pull_data(&"Hello, World!"));
    }

    #[test]
    fn writes_array() {
        assert_eq!(vec![8, 0, 0, 0, 7, 0, 1, 4, 33, 0, 57, 0],
                   pull_data(&[7i16, 1025, 33, 57]));
    }

    #[test]
    fn writes_vector() {
        assert_eq!(vec![12, 0, 0, 0, 4, 0, 0, 0, 7, 0, 1, 4, 33, 0, 57, 0],
                   pull_data(&vec![7i16, 1025, 33, 57]));
    }

    #[test]
    fn writes_tuple() {
        assert_eq!(vec![26, 0, 0, 0, 2, 8, 1, 7, 6, 0, 0, 0, 65, 66, 67, 48, 49, 50, 8, 0, 0, 0,
                        4, 0, 0, 0, 1, 0, 0, 1],
                   pull_data(&(2050i16, true, 7u8, "ABC012", vec![true, false, false, true])));
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
                   pull_data(&v));
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
                   pull_data(&v));
    }
}
