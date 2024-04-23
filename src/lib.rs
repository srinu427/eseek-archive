mod appendix_generated;

use std::collections::HashMap;
use crate::appendix_generated::{
  Appendix, AppendixArgs, AppendixEntry, AppendixEntryArgs, ArchiveEntryType,
};
use lzma::LzmaWriter;
use std::fs;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

enum ArchiveInputEntryType {
  File = 0,
  Folder = 1,
}

#[repr(u8)]
enum CompressionType {
  LZMA = 0,
  DEFLATE = 1,
}

struct ArchiveInputEntryDetails {
  name: PathBuf,
  path: PathBuf,
  archive_file_type: ArchiveInputEntryType,
}

impl ArchiveInputEntryDetails {
  pub fn get_details(path: PathBuf, name: PathBuf) -> Result<Vec<Self>, String> {
    if path.is_file() {
      return Ok(vec![Self {
        name,
        path,
        archive_file_type: ArchiveInputEntryType::File,
      }]);
    }
    if path.is_dir() {
      let mut arc_inp_entry_list = vec![];
      for dir_entry in fs::read_dir(&path).map_err(|e| format!("can't read dir: {e}"))? {
        if let Ok(dir_entry) = dir_entry {
          if let Ok(entry_type) = dir_entry.file_type() {
            let entry_name = PathBuf::from(dir_entry.file_name());
            if entry_type.is_file() {
              arc_inp_entry_list.push(Self {
                name: name.join(&entry_name),
                path: path.join(&entry_name),
                archive_file_type: ArchiveInputEntryType::File,
              });
            }
            if entry_type.is_dir() {
              if let Ok(mut dir_details) =
                Self::get_details(path.join(&entry_name), name.join(&entry_name))
              {
                arc_inp_entry_list.append(&mut dir_details);
              }
            }
          }
        }
      }
      if arc_inp_entry_list.len() == 0 {
        arc_inp_entry_list.push({
          Self {
            name,
            path,
            archive_file_type: ArchiveInputEntryType::Folder,
          }
        })
      }
      return Ok(arc_inp_entry_list);
    }
    Ok(vec![])
  }
}

fn compress_and_append<T>(
  src: &Path,
  dst: &impl Write,
  tmp_file_path: &Path,
) -> Result<usize, String> {
  let write_len = match fs::File::open(src) {
    Ok(mut fr) => {
      let mut f_read_buffer = [0u8; 4 * 1024 * 1024];
      match fs::File::create(tmp_file_path) {
        Ok(fw_tmp) => {
          let mut lzma_writer = LzmaWriter::new_compressor(fw_tmp, 9)
            .map_err(|e| format!("can't initialize compressor: {e}"))?;
          let mut compressed_size = 0;
          loop {
            let read_len = fr
              .read(&mut f_read_buffer)
              .map_err(|e| format!("can't read file {src:?}: {e}"))?;
            if read_len == 0 {
              break;
            }
            compressed_size += lzma_writer
              .write(&f_read_buffer[0..read_len])
              .map_err(|e| format!("error while compressing {src:?}: {e}"))?;
          }
          lzma_writer
            .finish()
            .map_err(|e| format!("can't write compressed data: {e}"))?;
          Ok(compressed_size)
        }
        Err(e) => Err(format!("unable to create {tmp_file_path:?}: {e}"))
      }
    }
    Err(e) => Err(format!("unable to open {src:?}: {e}")),
  }?;
  match fs::File::open(tmp_file_path){
    Ok(mut fr_tmp) => {
      let mut tmp_read_buffer = [0u8; 4 * 1024 * 1024];
      loop{
        let read_len = fr_tmp
          .read(&mut tmp_read_buffer)
          .map_err(|e| format!("can't read file {tmp_file_path:?}: {e}"))?;
        if read_len == 0 {
          break;
        }
      }
    }
    Err(e) => Err(format!("unable to open {tmp_file_path:?}: {e}"))
  }?;
}

pub fn archive_dir(
  in_dir: &Path,
  out_path: &Path,
  // prefix: &str,
) -> Result<(), String> {
  if in_dir.is_dir() {
    let out_dir = out_path
      .parent()
      .ok_or(format!("Error getting parent dir of {out_path:?}"))?;
    fs::create_dir_all(out_dir).map_err(|e| format!("unable to create: {out_dir:?}"))?;
    let out_path_tmp = PathBuf::from(
      out_path.to_str()
        .ok_or("invalid output file path")?.to_string() + ".temp"
    );
    match fs::File::create(&out_path_tmp) {
      Ok(mut fwb) => {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(4 * 1024 * 1024);
        let mut appendix_data = vec![];
        let mut fwb_bw = BufWriter::new(fwb);
        fwb_bw
          .write(&[0;16])
          .map_err(|e| format!("can't write to {out_path:?}: {e}"))?;
        let mut current_offset = 16u64;

        for input_entry in
          ArchiveInputEntryDetails::get_details(PathBuf::from(in_dir), PathBuf::from(""))?.iter()
        {
          let entry_name_str = input_entry
            .name
            .to_str()
            .ok_or(format!("Invalid file_name: {:?}", &input_entry.name))?;
          let entry_tmp_path = PathBuf::from(
            out_dir.to_str().ok_or("invalid output parent path")?.to_string() +
              &entry_name_str.replace("/", ":").replace("\\", ":")
          );
          let entry_name_fb = builder.create_string(entry_name_str);

          match input_entry.archive_file_type {
            ArchiveInputEntryType::File => {
              fwb_bw
                .write(&[CompressionType::LZMA as u8])
                .map_err(|e| format!("can't write to {out_path:?}: {e}"))?;
              println!("processing: {:?}", &input_entry.path);
              let write_size = compress_and_append(&input_entry.path, &fwb_bw, &entry_tmp_path)?;

              appendix_data.push(AppendixEntry::create(
                &mut builder,
                &AppendixEntryArgs {
                  type_: if write_size == 0 {
                    ArchiveEntryType::EmptyFile
                  } else {
                    ArchiveEntryType::File
                  },
                  offset: current_offset,
                  size_: write_size as u64,
                  name: Some(entry_name_fb),
                },
              ));
              current_offset += write_size as u64;
            }
            ArchiveInputEntryType::Folder => {
              appendix_data.push(AppendixEntry::create(
                &mut builder,
                &AppendixEntryArgs {
                  type_: ArchiveEntryType::EmptyFolder,
                  offset: current_offset,
                  size_: 0u64,
                  name: Some(entry_name_fb),
                },
              ));
            }
          }
        }
        let appendix_entries_fb = builder.create_vector(&appendix_data);
        let appendix_wip = Appendix::create(
          &mut builder,
          &AppendixArgs {
            entries: Some(appendix_entries_fb),
          },
        );
        builder.finish(appendix_wip, None);
        let compressed_appendix = lzma::compress(builder.finished_data(), 9)
          .map_err(|e| format!("error compressing appendix: {e}"))?;
        fwb_bw
          .write(&compressed_appendix)
          .map_err(|e| format!("error writing appendix: {e}"))?;
        fwb_bw
          .seek(SeekFrom::Start(0))
          .map_err(|e| format!("error seeking: {e}"))?;
        fwb_bw
          .write(&current_offset.to_le_bytes())
          .map_err(|e| format!("error writing appendix offset: {e}"))?;
        fwb_bw
          .seek(SeekFrom::Start(8))
          .map_err(|e| format!("error seeking: {e}"))?;
        fwb_bw
          .write(&(compressed_appendix.len() as u64).to_le_bytes())
          .map_err(|e| format!("error writing appendix size: {e}"))?;
        Ok(())
      }
      Err(e) => Err(format!("failed opening {out_path_tmp:?}: {e}, skipping it")),
    }
  } else {
    Err(format!("{in_dir:?} is not a directory"))
  }
}

#[derive(Debug)]
struct ArchiveEntryDetails{
  name: String,
  type_: ArchiveEntryType,
  offset: u64,
  size_: u64,
}

impl ArchiveEntryDetails{
  pub fn hashmap_from_appendix(appendix: Appendix) -> Result<HashMap<String, Self>, String>{
    let appendix_entries = appendix
      .entries()
      .ok_or("error getting appendix entries")?;
    let mut archive_entry_map = HashMap::new();
    for entry in appendix_entries{
      let tmp_self = Self{
        name: entry.name().ok_or("error getting appendix entry's name")?.to_string(),
        type_: entry.type_(),
        offset: entry.offset(),
        size_: entry.size_(),
      };
      archive_entry_map.insert(tmp_self.name.clone(), tmp_self);
    }
    Ok(archive_entry_map)
  }
}

pub fn extract(
  in_path: &Path,
  out_dir: &Path,
) -> Result<(), String>{
  match fs::File::open(in_path) {
    Ok(mut fr) => {
      let mut appendix_offset = [0u8; 8];
      let mut appendix_size = [0u8; 8];
      fr.read_exact(&mut appendix_offset)
        .map_err(|e| format!("error reading appendix offset: {e}"))?;
      fr.read_exact(&mut appendix_size)
        .map_err(|e| format!("error reading appendix size: {e}"))?;
      let appendix_offset = u64::from_le_bytes(appendix_offset);
      let appendix_size = u64::from_le_bytes(appendix_size);
      let mut compressed_appendix = vec![0; appendix_size as usize];
      fr.seek(SeekFrom::Start(appendix_offset))
        .map_err(|e| format!("error seeking to appendix: {e}"))?;
      fr.read_exact(&mut compressed_appendix)
        .map_err(|e| format!("error reading appendix: {e}"))?;
      let appendix_bytes = lzma::decompress(&compressed_appendix)
        .map_err(|e| format!("error decompressing appendix: {e}"))?;
      let appendix = appendix_generated::root_as_appendix(&appendix_bytes)
        .map_err(|e| format!("error parsing appendix: {e}"))?;
      let entry_hashmap = ArchiveEntryDetails::hashmap_from_appendix(appendix)?;
      println!("{:#?}", entry_hashmap);
      Ok(())
    }
    Err(e) => { Err(format!("failed opening {in_path:?}: {e}")) }
  }
}
