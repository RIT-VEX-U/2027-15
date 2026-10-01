//! VDP Part types representing records, scalar numbers, and strings.
//!
//! Mirrors `core/device/vdb/types.hpp` and `types.cpp`.

use super::protocol::{DataType, PacketReader, PacketWriter};

/// Common trait for all VDP schema components and telemetry parts.
pub trait Part: Send + Sync {
    /// Returns the name of the part.
    fn get_name(&self) -> &str;

    /// Fetches live data from hardware or state and updates the stored value.
    fn fetch(&mut self) {}

    /// Applies stored or received data back to the hardware or state.
    fn response(&mut self) {}

    /// Writes the schema structure into a packet writer.
    fn write_schema(&self, writer: &mut PacketWriter);

    /// Writes the payload data into a packet writer.
    fn write_message(&self, writer: &mut PacketWriter);

    /// Reads incoming payload data from a packet reader.
    fn read_data_from_message(&mut self, reader: &mut PacketReader);
}

/// A compound record containing an ordered list of child parts.
pub struct Record {
    name: String,
    fields: Vec<Box<dyn Part>>,
}

impl Record {
    /// Creates a new empty record.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fields: Vec::new(),
        }
    }

    /// Creates a record populated with child fields.
    pub fn with_fields(name: impl Into<String>, fields: Vec<Box<dyn Part>>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }

    /// Adds a child field to this record.
    pub fn add_field(&mut self, field: Box<dyn Part>) {
        self.fields.push(field);
    }

    /// Returns a slice of the child fields.
    pub fn fields(&self) -> &[Box<dyn Part>] {
        &self.fields
    }
}

impl Part for Record {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn fetch(&mut self) {
        for field in &mut self.fields {
            field.fetch();
        }
    }

    fn response(&mut self) {
        for field in &mut self.fields {
            field.response();
        }
    }

    fn write_schema(&self, writer: &mut PacketWriter) {
        writer.write_type(DataType::Record);
        writer.write_string(&self.name);
        writer.write_i32(self.fields.len() as i32);
        for field in &self.fields {
            field.write_schema(writer);
        }
    }

    fn write_message(&self, writer: &mut PacketWriter) {
        for field in &self.fields {
            field.write_message(writer);
        }
    }

    fn read_data_from_message(&mut self, reader: &mut PacketReader) {
        for field in &mut self.fields {
            field.read_data_from_message(reader);
        }
    }
}

/// Float (32-bit floating point) part.
pub struct FloatPart {
    name: String,
    value: f32,
    fetcher: Option<Box<dyn Fn() -> f32 + Send + Sync>>,
}

impl FloatPart {
    /// Creates a new FloatPart.
    pub fn new(name: impl Into<String>, fetcher: Option<Box<dyn Fn() -> f32 + Send + Sync>>) -> Self {
        Self {
            name: name.into(),
            value: 0.0,
            fetcher,
        }
    }

    /// Returns the stored float value.
    pub fn get_value(&self) -> f32 {
        self.value
    }

    /// Sets the stored float value.
    pub fn set_value(&mut self, v: f32) {
        self.value = v;
    }
}

impl Part for FloatPart {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn fetch(&mut self) {
        if let Some(ref f) = self.fetcher {
            self.value = f();
        }
    }

    fn write_schema(&self, writer: &mut PacketWriter) {
        writer.write_type(DataType::Float);
        writer.write_string(&self.name);
    }

    fn write_message(&self, writer: &mut PacketWriter) {
        writer.write_f32(self.value);
    }

    fn read_data_from_message(&mut self, reader: &mut PacketReader) {
        self.value = reader.get_f32();
    }
}

/// Double (64-bit floating point) part.
pub struct DoublePart {
    name: String,
    value: f64,
    fetcher: Option<Box<dyn Fn() -> f64 + Send + Sync>>,
}

impl DoublePart {
    /// Creates a new DoublePart.
    pub fn new(name: impl Into<String>, fetcher: Option<Box<dyn Fn() -> f64 + Send + Sync>>) -> Self {
        Self {
            name: name.into(),
            value: 0.0,
            fetcher,
        }
    }

    /// Returns the stored double value.
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Sets the stored double value.
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }
}

impl Part for DoublePart {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn fetch(&mut self) {
        if let Some(ref f) = self.fetcher {
            self.value = f();
        }
    }

    fn write_schema(&self, writer: &mut PacketWriter) {
        writer.write_type(DataType::Double);
        writer.write_string(&self.name);
    }

    fn write_message(&self, writer: &mut PacketWriter) {
        writer.write_f64(self.value);
    }

    fn read_data_from_message(&mut self, reader: &mut PacketReader) {
        self.value = reader.get_f64();
    }
}

/// String part.
pub struct StringPart {
    name: String,
    value: String,
    fetcher: Option<Box<dyn Fn() -> String + Send + Sync>>,
}

impl StringPart {
    /// Creates a new StringPart.
    pub fn new(name: impl Into<String>, fetcher: Option<Box<dyn Fn() -> String + Send + Sync>>) -> Self {
        Self {
            name: name.into(),
            value: String::new(),
            fetcher,
        }
    }

    /// Returns a reference to the stored string.
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Sets the stored string.
    pub fn set_value(&mut self, s: impl Into<String>) {
        self.value = s.into();
    }
}

impl Part for StringPart {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn fetch(&mut self) {
        if let Some(ref f) = self.fetcher {
            self.value = f();
        }
    }

    fn write_schema(&self, writer: &mut PacketWriter) {
        writer.write_type(DataType::String);
        writer.write_string(&self.name);
    }

    fn write_message(&self, writer: &mut PacketWriter) {
        writer.write_string(&self.value);
    }

    fn read_data_from_message(&mut self, reader: &mut PacketReader) {
        self.value = reader.get_string();
    }
}
