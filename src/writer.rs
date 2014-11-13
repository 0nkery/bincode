use std::io::{Writer, IoError, IoResult};
use serialize::Encoder;

type EwResult = IoResult<()>;

pub struct EncoderWriter<'a, W: 'a> {
    writer: &'a mut W
}

impl <'a, W: Writer> EncoderWriter<'a, W> {
    pub fn new(w: &'a mut W) -> EncoderWriter<'a, W> {
        EncoderWriter{ writer: w }
    }
}

impl<'a, W: Writer> Encoder<IoError> for EncoderWriter<'a, W> {
    fn emit_nil(&mut self) -> EwResult { Ok(()) }
    fn emit_uint(&mut self, v: uint) -> EwResult {
        self.emit_u64(v as u64)
    }
    fn emit_u64(&mut self, v: u64) -> EwResult {
        self.writer.write_be_u64(v)
    }
    fn emit_u32(&mut self, v: u32) -> EwResult {
        self.writer.write_be_u32(v)
    }
    fn emit_u16(&mut self, v: u16) -> EwResult {
        self.writer.write_be_u16(v)
    }
    fn emit_u8(&mut self, v: u8) -> EwResult {
        self.writer.write_u8(v)
    }
    fn emit_int(&mut self, v: int) -> EwResult {
        self.emit_i64(v as i64)
    }
    fn emit_i64(&mut self, v: i64) -> EwResult {
        self.writer.write_be_i64(v)
    }
    fn emit_i32(&mut self, v: i32) -> EwResult {
        self.writer.write_be_i32(v)
    }
    fn emit_i16(&mut self, v: i16) -> EwResult {
        self.writer.write_be_i16(v)
    }
    fn emit_i8(&mut self, v: i8) -> EwResult {
        self.writer.write_i8(v)
    }
    fn emit_bool(&mut self, v: bool) -> EwResult {
        self.writer.write_u8(if v {1} else {0})
    }
    fn emit_f64(&mut self, v: f64) -> EwResult {
        self.writer.write_be_f64(v)
    }
    fn emit_f32(&mut self, v: f32) -> EwResult {
        self.writer.write_be_f32(v)
    }
    fn emit_char(&mut self, v: char) -> EwResult {
        self.writer.write_char(v)
    }
    fn emit_str(&mut self, v: &str) -> EwResult {
        try!(self.emit_uint(v.len()));
        self.writer.write_str(v)
    }
    fn emit_enum(&mut self, _: &str,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_enum_variant(&mut self,
    _: &str, v_id: uint, _: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        try!(self.emit_uint(v_id));
        f(self)
    }
    fn emit_enum_variant_arg(&mut self, _: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_enum_struct_variant(&mut self, _: &str, _: uint,
    _: uint, f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_enum_struct_variant_field(&mut self, _: &str,
    _: uint, f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_struct(&mut self, _: &str, _: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_struct_field(&mut self, _: &str, _: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_tuple(&mut self, _: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_tuple_arg(&mut self, _: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_tuple_struct(&mut self, _: &str, len: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        self.emit_tuple(len, f)
    }
    fn emit_tuple_struct_arg(&mut self, f_idx: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        self.emit_tuple_arg(f_idx, f)
    }
    fn emit_option(&mut self,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_option_none(&mut self) -> EwResult {
        self.writer.write_u8(0)
    }
    fn emit_option_some(&mut self,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        try!(self.writer.write_u8(1));
        f(self)
    }
    fn emit_seq(&mut self, len: uint,
    f: |this: &mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        try!(self.emit_uint(len));
        f(self)
    }
    fn emit_seq_elt(&mut self, _: uint,
    f: |this: &mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_map(&mut self, len: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        try!(self.emit_uint(len));
        f(self)
    }
    fn emit_map_elt_key(&mut self, _: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
    fn emit_map_elt_val(&mut self, _: uint,
    f: |&mut EncoderWriter<'a, W>| -> EwResult) -> EwResult {
        f(self)
    }
}
