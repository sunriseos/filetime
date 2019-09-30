use crate::FileTime;
use std::fs::{self, File};
use std::io;
use std::path::Path;

// TODO: file time updates in Sunrise's filesystem
pub fn set_file_times(_p: &Path, _atime: FileTime, _mtime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn set_symlink_file_times(_p: &Path, _atime: FileTime, _mtime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn set_file_mtime(_p: &Path, _mtime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn set_file_atime(_p: &Path, _atime: FileTime) -> io::Result<()> {
    Ok(())
}

pub fn from_last_modification_time(meta: &fs::Metadata) -> FileTime {
    FileTime::from_system_time(meta.modified().unwrap())
}

pub fn from_last_access_time(meta: &fs::Metadata) -> FileTime {
    FileTime::from_system_time(meta.accessed().unwrap())
}

pub fn from_creation_time(meta: &fs::Metadata) -> Option<FileTime> {
    Some(FileTime::from_system_time(meta.created().unwrap()))
}

pub fn set_file_handle_times(
    _f: &File,
    _atime: Option<FileTime>,
    _mtime: Option<FileTime>,
) -> io::Result<()> {
    Ok(())
}
