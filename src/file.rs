use std::path::Path;

use napi::Error;
use napi_derive::napi;
use simulang_rs::File as SimulangFile;
use simulang_rs::traits::FileTrait;

#[napi]
#[allow(clippy::needless_pass_by_value)]
/// Reads a file and returns its trimmed contents.
///
/// If the path is relative, it is resolved against the default cache
/// location.
pub fn read_file(path: String) -> napi::Result<String> {
  let file = SimulangFile::new(Path::new(&path), false).map_err(Error::from_reason)?;
  file.read().map_err(Error::from_reason)
}

#[napi]
#[allow(clippy::needless_pass_by_value)]
/// Writes content to a file, returning the absolute path written to.
///
/// If the path is relative, it is resolved against the default cache location.
/// When `append` is true and the file already has content, a newline is
/// inserted before appending the new content. When `append` is false, the file
/// is overwritten.
pub fn write_file(path: String, content: String, append: bool) -> napi::Result<String> {
  let file = SimulangFile::new(Path::new(&path), true).map_err(Error::from_reason)?;
  file.write(&content, append).map_err(Error::from_reason)?;
  Ok(file.path().to_string_lossy().into_owned())
}

#[napi]
/// Represents a file handle.
pub struct File {
  pub(crate) inner: std::cell::RefCell<Option<SimulangFile>>,
}

#[napi]
impl File {
  #[napi(constructor)]
  /// Creates a handle to a file at the given path.
  ///
  /// If `create_missing` is true, the file (and all missing parent
  /// directories) is created when it does not already exist. If false,
  /// the call fails when the file does not exist.
  #[allow(clippy::needless_pass_by_value)]
  pub fn new(path: String, create_missing: bool) -> napi::Result<Self> {
    SimulangFile::new(Path::new(&path), create_missing)
      .map(|inner| Self {
        inner: std::cell::RefCell::new(Some(inner)),
      })
      .map_err(Error::from_reason)
  }

  fn with_inner<T>(&self, f: impl FnOnce(&SimulangFile) -> Result<T, String>) -> napi::Result<T> {
    let borrow = self.inner.borrow();
    let inner = borrow.as_ref().ok_or_else(|| {
      Error::from_reason("File handle has been consumed (deleted or moved)".to_string())
    })?;
    f(inner).map_err(Error::from_reason)
  }

  fn with_inner_mut<T>(
    &self,
    f: impl FnOnce(&mut SimulangFile) -> Result<T, String>,
  ) -> napi::Result<T> {
    let mut borrow = self.inner.borrow_mut();
    let inner = borrow.as_mut().ok_or_else(|| {
      Error::from_reason("File handle has been consumed (deleted or moved)".to_string())
    })?;
    f(inner).map_err(Error::from_reason)
  }

  #[napi]
  /// Returns the resolved absolute path.
  pub fn path(&self) -> napi::Result<String> {
    self.with_inner(|f| Ok(f.path().to_string_lossy().into_owned()))
  }

  #[napi]
  /// Reads the file and returns its trimmed contents.
  pub fn read(&self) -> napi::Result<String> {
    self.with_inner(FileTrait::read)
  }

  #[napi]
  /// Writes content to the file.
  ///
  /// When `append` is true and the file already has content, a newline is
  /// inserted before appending. When `append` is false, the file is
  /// overwritten. Parent directories are created if needed.
  #[allow(clippy::needless_pass_by_value)]
  pub fn write(&self, content: String, append: bool) -> napi::Result<()> {
    self.with_inner(|f| f.write(&content, append))
  }

  #[napi]
  /// Renames the file in place (same parent directory).
  #[allow(clippy::needless_pass_by_value)]
  pub fn rename(&mut self, new_name: String) -> napi::Result<()> {
    self.with_inner_mut(|f| f.rename(&new_name))
  }

  #[napi]
  /// Copies the file to a new location, returning a handle to the copy.
  #[allow(clippy::needless_pass_by_value)]
  pub fn copy_to(&self, dest: String) -> napi::Result<File> {
    self
      .with_inner(|f| f.copy_to(Path::new(&dest)))
      .map(|inner| File {
        inner: std::cell::RefCell::new(Some(inner)),
      })
  }

  #[napi]
  /// Moves the file to a new location.
  /// Falls back to copy + delete for cross-device moves.
  #[allow(clippy::needless_pass_by_value)]
  pub fn move_to(&mut self, dest: String) -> napi::Result<()> {
    self.with_inner_mut(|f| f.move_to(Path::new(&dest)))
  }

  #[napi]
  /// Deletes the file, invalidating the handle.
  pub fn delete(&self) -> napi::Result<()> {
    let inner = self.inner.borrow_mut().take().ok_or_else(|| {
      Error::from_reason("File handle has been consumed (deleted or moved)".to_string())
    })?;
    inner.delete().map_err(Error::from_reason)
  }

  #[napi]
  /// Returns the file name (last component of the path).
  pub fn name(&self) -> napi::Result<String> {
    self.with_inner(|f| Ok(f.name().to_owned()))
  }

  #[napi]
  /// Returns the file extension, if any.
  pub fn extension(&self) -> napi::Result<Option<String>> {
    self.with_inner(|f| Ok(f.extension().map(ToOwned::to_owned)))
  }

  #[napi]
  /// Returns the file size in bytes.
  pub fn size(&self) -> napi::Result<i64> {
    #[allow(clippy::cast_possible_wrap)]
    self.with_inner(|f| f.size().map(|s| s as i64))
  }

  #[napi]
  /// Returns the last modification time as milliseconds since the Unix
  /// epoch.
  #[allow(clippy::cast_precision_loss)]
  pub fn modified(&self) -> napi::Result<f64> {
    self.with_inner(|f| {
      f.modified().map(|t| {
        t.duration_since(std::time::UNIX_EPOCH)
          .unwrap_or_default()
          .as_millis() as f64
      })
    })
  }

  #[napi]
  /// Returns whether the file is read-only.
  pub fn is_readonly(&self) -> napi::Result<bool> {
    self.with_inner(FileTrait::is_readonly)
  }
}
