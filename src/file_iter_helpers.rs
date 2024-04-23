use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use lzma::{LzmaReader, LzmaWriter};

pub fn append_file_contents(
  src: &Path,
  dst: &mut impl Write
) -> Result<usize, String>{
  match fs::File::open(src) {
    Ok(mut fr) => {
      let mut fr_buffer = [0u8; 4 * 1024 * 1024];
      let mut write_size = 0;
      loop {
        let read_len = fr.read(fr_buffer.as_mut_slice())
          .map_err(|e| format!("error reading src: {e}"))?;
        if read_len == 0{
          break;
        }
        dst.write(&fr_buffer[0..read_len])
          .map_err(|e| format!("error writing to dst: {e}"))?;
        write_size += read_len;
      }
      Ok(write_size)
    },
    Err(e) => return Err(format!("error opening {src:?}: {e}")),
  }
}

pub fn compress_file_lzma(
  src: &Path,
  dst: &Path,
  preset: u32
) -> Result<usize, String>{
  match fs::File::open(src) {
    Ok(mut fr) => {
      match fs::File::create(dst) {
        Ok(fw) => {
          let mut lzma_writer = LzmaWriter::new_compressor(fw, preset)
            .map_err(|e| format!("error initializing lzma compressor: {e}"))?;
          let mut fr_buffer = [0u8; 4 * 1024 * 1024];
          let mut write_size = 0;
          loop {
            let read_len = fr.read(fr_buffer.as_mut_slice())
              .map_err(|e| format!("error reading src: {e}"))?;
            if read_len == 0{
              break;
            }
            let write_len = lzma_writer.write(&fr_buffer[0..read_len])
              .map_err(|e| format!("error compressing {src:?}: {e}"))?;
            write_size += write_len;
          }
          lzma_writer
            .finish()
            .map_err(|e| format!("error compressing {src:?}: {e}"))?;
          Ok(write_size)
        },
        Err(e) => return Err(format!("error opening {dst:?}: {e}")),
      }
    },
    Err(e) => return Err(format!("error opening {src:?}: {e}")),
  }
}

pub fn extract_file_contents(
  src: &mut (impl Read + Seek),
  offset: u64,
  size: u64,
  dst: &Path
) -> Result<(), String>{
  match fs::File::create(dst){
    Ok(mut fw) => {
      src.seek(SeekFrom::Start(offset))
        .map_err(|e| format!("error seeking src: {e}"))?;
      let mut fr_buffer = [0u8; 4 * 1024 * 1024];
      let mut remaining_size = size;
      while remaining_size != 0 {
        let read_len = if remaining_size < 4 * 1024 * 1024 {
          src
            .read(&mut fr_buffer[0..remaining_size as usize])
            .map_err(|e| format!("error reading src: {e}"))?
        } else {
          src
            .read(&mut fr_buffer)
            .map_err(|e| format!("error reading src: {e}"))?
        };
        fw.write(&fr_buffer[0..read_len])
          .map_err(|e| format!("error writing to {dst:?}: {e}"))?;
        remaining_size -= read_len as u64;
      }
      Ok(())
    },
    Err(e) => return Err(format!("error opening {dst:?}: {e}")),
  }
}

pub fn extract_file_lzma(
  src: &Path,
  dst: &Path
) -> Result<(), String>{
  match fs::File::open(src){
    Ok(fr) => {
      match fs::File::create(dst) {
        Ok(mut fw) => {
          let mut lzma_reader = LzmaReader::new_decompressor(fr)
            .map_err(|e| format!("error initializing lzma decompressor: {e}"))?;
          let mut fr_buffer = [0u8; 4 * 1024 * 1024];
          loop {
            let read_len = lzma_reader
                .read(&mut fr_buffer)
                .map_err(|e| format!("error reading {src:?}: {e}"))?;
            if read_len == 0{
              break;
            }
            fw.write(&fr_buffer[0..read_len])
              .map_err(|e| format!("error writing to {dst:?}: {e}"))?;
          }
          Ok(())
        },
        Err(e) => return Err(format!("error opening {dst:?}: {e}")),
      }
    },
    Err(e) => return Err(format!("error opening {src:?}: {e}")),
  }
}
