mod appendix_generated;
mod file_iter_helpers;

use std::collections::HashMap;
use crate::appendix_generated::{
  Appendix, AppendixArgs, AppendixEntry, AppendixEntryArgs, ArchiveEntryType,
};
use std::fs;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
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

fn compress_and_append(
  src: &Path,
  dst: &mut impl Write,
  tmp_file_path: &Path,
) -> Result<usize, String> {
  file_iter_helpers::compress_file_lzma(src, tmp_file_path, 9)?;
  let write_len = file_iter_helpers::append_file_contents(tmp_file_path, dst)?;
  fs::remove_file(tmp_file_path)
    .map_err(|e| format!("unable to delete {tmp_file_path:?}: {e}"))?;
  Ok(write_len)
}

pub fn archive_dir(
  in_dir: &Path,
  out_path: &Path,
  // prefix: &str,
) -> Result<(), String> {
  if in_dir.is_dir() {
    let out_file_name = out_path
      .file_name()
      .ok_or(format!("invalid output file name: {out_path:?}"))?
      .to_str()
      .ok_or(format!("invalid output file name: {out_path:?}"))?;
    let out_dir = out_path
      .parent()
      .ok_or(format!("error getting parent dir of {out_path:?}"))?;
    let tmp_dir = out_dir.join(".eseek_temp");
    fs::create_dir_all(&tmp_dir)
      .map_err(|e| format!("unable to create: {:?}: {e}", &tmp_dir))?;
    let out_path_tmp = tmp_dir.join(out_file_name.to_string() + ".temp");
    match fs::File::create(&out_path_tmp) {
      Ok(fwb) => {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(4 * 1024 * 1024);
        let mut appendix_data = vec![];
        let mut fwb_bw = BufWriter::new(fwb);
        fwb_bw
          .write(&[0;16])
          .map_err(|e| format!("can't write to {out_path:?}: {e}"))?;
        let mut current_offset = 16u64;

        let archive_input_entries =
          ArchiveInputEntryDetails::get_details(PathBuf::from(in_dir), PathBuf::from(""))?;
        for input_entry in archive_input_entries {
          let entry_name_str = input_entry
            .name
            .to_str()
            .ok_or(format!("Invalid file_name: {:?}", &input_entry.name))?;
          let entry_tmp_path = tmp_dir
            .join(entry_name_str.replace("/", ":").replace("\\", ":") + ".temp");
          let entry_name_fb = builder.create_string(entry_name_str);

          match input_entry.archive_file_type {
            ArchiveInputEntryType::File => {
              fwb_bw
                .write(&[CompressionType::LZMA as u8])
                .map_err(|e| format!("can't write to {out_path:?}: {e}"))?;
              let write_size = compress_and_append(&input_entry.path, &mut fwb_bw, &entry_tmp_path)?;

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
              current_offset += write_size as u64 + 1u64;
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
      }
      Err(e) => return Err(format!("failed opening {out_path_tmp:?}: {e}, skipping it")),
    }
    fs::rename(&out_path_tmp, out_path)
      .map_err(|e| format!("error moving temp file to output location: {e}"))?;
    fs::remove_dir(&tmp_dir)
      .map_err(|e| format!("error removing temp directory: {e}"))?;
    Ok(())
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

  pub fn extract_to_directory(
    &self,
    archive: &mut (impl Read + Seek),
    out_dir: &Path
  ) -> Result<(), String> {
    let entry_path = PathBuf::from(&self.name);
    if entry_path.is_absolute(){
      return Err(format!("SECURITY ERROR: absolute path in archive entry"))
    }
    let out_file_path = out_dir.join(&self.name);
    let temp_file_path = out_dir.join(self.name.clone() + ".xz");
    match self.type_{
      ArchiveEntryType::File => {
        let out_file_dir = out_file_path.parent().ok_or(format!("invalid parent dir"))?;
        fs::create_dir_all(out_file_dir)
          .map_err(|e| format!("error creating directory {out_file_dir:?}: {e}"))?;
        file_iter_helpers::extract_file_contents(
          archive,
          self.offset + 1,
          self.size_,
          &temp_file_path
        )?;
        file_iter_helpers::extract_file_lzma(
          &temp_file_path,
          &out_file_path
        )?;
        fs::remove_file(&temp_file_path)
          .map_err(|e| format!("error deleting temp file {:?}: {e}", &temp_file_path))?;
      }
      ArchiveEntryType::EmptyFile => {
        let out_file_dir = out_file_path.parent().ok_or(format!("invalid parent dir"))?;
        fs::create_dir_all(out_file_dir)
          .map_err(|e| format!("error creating directory {out_file_dir:?}: {e}"))?;
        fs::File::create(&out_file_path)
          .map_err(|e| format!("error creating file {out_file_path:?}: {e}"))?;
      }
      ArchiveEntryType::EmptyFolder => {
        fs::create_dir_all(&out_file_path)
          .map_err(|e| format!("error creating directory {out_file_path:?}: {e}"))?;
      }
      _ => {}
    }
    Ok(())
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
      for (_, entry) in entry_hashmap.iter(){
        entry.extract_to_directory(&mut fr, out_dir)?;
      }
      Ok(())
    }
    Err(e) => Err(format!("failed opening {in_path:?}: {e}"))
  }
}
