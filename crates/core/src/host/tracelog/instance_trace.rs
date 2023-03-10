#![allow(clippy::too_many_arguments)]

use crate::protobuf::instance_db_trace_log::instance_event::Type;
use crate::protobuf::instance_db_trace_log::{
    CreateTable, DeleteEq, DeletePk, DeleteRange, DeleteValue, GetTableId, Insert, InstanceEvent,
};
use flate2::write::GzEncoder;
use flate2::Compression;
use prost::Message;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct TraceLog {
    trace_writer: BufWriter<File>,
    write_error_count: usize,
}

impl TraceLog {
    pub fn new() -> Result<Self, anyhow::Error> {
        let file_name = format!("/tmp/db-trace-events-log-{}.bin", Uuid::new_v4());

        let trace_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(file_name.clone())?;

        let trace_writer = BufWriter::new(trace_file);
        log::info!("Opened trace log: {}", file_name);
        Ok(Self {
            trace_writer,
            write_error_count: 0,
        })
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.trace_writer.flush()
    }

    /// Retrieve the current contents of the trace log.
    pub fn retrieve(&mut self) -> Result<bytes::Bytes, anyhow::Error> {
        self.flush()?;
        let mut reader = self.trace_writer.get_ref();
        reader.rewind()?;
        let mut buf_vec = vec![];
        let _read_bytes = reader.read_to_end(&mut buf_vec)?;
        Ok(buf_vec.into())
    }

    fn write_event(&mut self, start_time: SystemTime, duration: Duration, event: Type) {
        let epoch_time = start_time.duration_since(UNIX_EPOCH).unwrap();
        let msg = InstanceEvent {
            event_start_epoch_micros: epoch_time.as_micros() as u64,
            duration_micros: duration.as_micros() as u64,
            r#type: Some(event),
        };
        let compressed = {
            let mut out_buf = Vec::with_capacity(msg.encoded_len());
            msg.encode(&mut out_buf).unwrap();

            // compress it
            let mut e = GzEncoder::new(Vec::new(), Compression::default());
            match e.write_all(&out_buf[..]) {
                Ok(_) => {}
                Err(e) => {
                    // Don't spam the log.
                    if self.write_error_count < 16 {
                        log::warn!("Failure to compress instance event in trace log: {}", e);
                    }
                    self.write_error_count += 1;

                    return;
                }
            }
            match e.finish() {
                Ok(b) => b,
                Err(e) => {
                    // Don't spam the log.
                    if self.write_error_count < 16 {
                        log::warn!("Failure to compress instance event in trace log: {}", e);
                    }
                    self.write_error_count += 1;
                    return;
                }
            }
        };

        // Prefix with msg bytes length.
        let len_bytes = compressed.len().to_le_bytes();

        // Just eat write fails so as not to cause problems in the running process.
        match self.trace_writer.write(&len_bytes[..]) {
            Ok(_) => {
                self.trace_writer
                    .write_all(compressed.as_slice())
                    .expect("Unable to write event to file");
                self.flush().unwrap()
            }
            Err(e) => {
                // Don't spam the log.
                if self.write_error_count < 16 {
                    log::warn!("Failure to write instance event trace log: {}", e);
                }
                self.write_error_count += 1;
            }
        }
    }

    pub fn insert(&mut self, start_time: SystemTime, duration: Duration, table_id: u32, buffer: Vec<u8>) {
        let event = Type::Insert(Insert { table_id, buffer });
        self.write_event(start_time, duration, event)
    }

    pub fn delete_pk(
        &mut self,
        start_time: SystemTime,
        duration: Duration,
        table_id: u32,
        buffer: Vec<u8>,
        success: bool,
    ) {
        let event = Type::DeletePk(DeletePk {
            table_id,
            buffer,
            result_success: success,
        });
        self.write_event(start_time, duration, event)
    }

    pub fn delete_value(
        &mut self,
        start_time: SystemTime,
        duration: Duration,
        table_id: u32,
        buffer: Vec<u8>,
        success: bool,
    ) {
        let event = Type::DeleteValue(DeleteValue {
            table_id,
            buffer,
            result_success: success,
        });
        self.write_event(start_time, duration, event)
    }

    pub fn delete_eq(
        &mut self,
        start_time: SystemTime,
        duration: Duration,
        table_id: u32,
        col_id: u32,
        buffer: Vec<u8>,
        deleted_count: u32,
    ) {
        let event = Type::DeleteEq(DeleteEq {
            table_id,
            col_id,
            buffer,
            result_deleted_count: deleted_count,
        });
        self.write_event(start_time, duration, event)
    }

    pub fn delete_range(
        &mut self,
        start_time: SystemTime,
        duration: Duration,
        table_id: u32,
        col_id: u32,
        start_buffer: Vec<u8>,
        end_buffer: Vec<u8>,
        deleted_count: u32,
    ) {
        let event = Type::DeleteRange(DeleteRange {
            table_id,
            col_id,
            start_buffer,
            end_buffer,
            result_deleted_count: deleted_count,
        });
        self.write_event(start_time, duration, event)
    }

    pub fn create_table(
        &mut self,
        start_time: SystemTime,
        duration: Duration,
        table_name: String,
        schema_buffer: Vec<u8>,
        table_id: u32,
    ) {
        let event = Type::CreateTable(CreateTable {
            table_name,
            schema_buffer,
            result_table_id: table_id,
        });
        self.write_event(start_time, duration, event)
    }

    pub fn get_table_id(&mut self, start_time: SystemTime, duration: Duration, table_name: String, table_id: u32) {
        let event = Type::GetTableId(GetTableId {
            table_name,
            result_table_id: table_id,
        });
        self.write_event(start_time, duration, event)
    }
}
