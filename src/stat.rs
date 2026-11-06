//! stat(2) result structures and file-type decoding.

/// File type bits — stat(2) `st_mode` analogue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FileType {
    Fifo = 0o010000,
    Chr = 0o020000,
    Dir = 0o040000,
    Blk = 0o060000,
    Reg = 0o100000,
    Lnk = 0o120000,
    Sock = 0o140000,
}

impl FileType {
    pub fn from_mode(mode: u32) -> Option<Self> {
        Some(match mode & 0o170000 {
            0o010000 => FileType::Fifo,
            0o020000 => FileType::Chr,
            0o040000 => FileType::Dir,
            0o060000 => FileType::Blk,
            0o100000 => FileType::Reg,
            0o120000 => FileType::Lnk,
            0o140000 => FileType::Sock,
            _ => return None,
        })
    }
}

/// stat(2) result structure (subset of fields, layout-stable).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stat {
    pub dev: u64,
    pub ino: u64,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: i64,
    pub atime: i64,
    pub mtime: i64,
    pub ctime: i64,
}

impl Stat {
    pub fn file_type(&self) -> Option<FileType> {
        FileType::from_mode(self.mode)
    }
    pub fn is_dir(&self) -> bool {
        self.file_type() == Some(FileType::Dir)
    }
    pub fn permissions(&self) -> u32 {
        self.mode & 0o7777
    }
}
