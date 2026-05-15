use std::path::Path;

use napi::Error;
use napi_derive::napi;
use simulang_rs::Directory as SimulangDirectory;
use simulang_rs::traits::DirectoryTrait;

use crate::file::File;

#[napi]
/// Represents a directory handle.
pub struct Directory {
  inner: std::cell::RefCell<Option<SimulangDirectory>>,
}

#[napi]
impl Directory {
  #[napi(constructor)]
  /// Creates a handle to a directory at the given path.
  ///
  /// If `create_missing` is true, the directory (and all missing
  /// ancestors) is created when it does not already exist. If false,
  /// the call fails when the directory does not exist.
  #[allow(clippy::needless_pass_by_value)]
  pub fn new(path: String, create_missing: bool) -> napi::Result<Self> {
    SimulangDirectory::new(Path::new(&path), create_missing)
      .map(|inner| Self {
        inner: std::cell::RefCell::new(Some(inner)),
      })
      .map_err(Error::from_reason)
  }

  #[napi(factory)]
  /// Creates a directory with a unique name inside the system's temp
  /// directory, returning a handle to it. The directory is **not**
  /// automatically removed; call `delete()` when done.
  pub fn temp() -> napi::Result<Self> {
    SimulangDirectory::temp()
      .map(|inner| Self {
        inner: std::cell::RefCell::new(Some(inner)),
      })
      .map_err(Error::from_reason)
  }

  fn with_inner<T>(
    &self,
    f: impl FnOnce(&SimulangDirectory) -> Result<T, String>,
  ) -> napi::Result<T> {
    let borrow = self.inner.borrow();
    let inner = borrow.as_ref().ok_or_else(|| {
      Error::from_reason("Directory handle has been consumed (deleted or moved)".to_string())
    })?;
    f(inner).map_err(Error::from_reason)
  }

  fn with_inner_mut<T>(
    &self,
    f: impl FnOnce(&mut SimulangDirectory) -> Result<T, String>,
  ) -> napi::Result<T> {
    let mut borrow = self.inner.borrow_mut();
    let inner = borrow.as_mut().ok_or_else(|| {
      Error::from_reason("Directory handle has been consumed (deleted or moved)".to_string())
    })?;
    f(inner).map_err(Error::from_reason)
  }

  #[napi]
  /// Returns the resolved absolute path.
  pub fn path(&self) -> napi::Result<String> {
    self.with_inner(|d| Ok(d.path().to_string_lossy().into_owned()))
  }

  #[napi]
  /// Renames the directory in place (same parent directory)
  #[allow(clippy::needless_pass_by_value)]
  pub fn rename(&mut self, new_name: String) -> napi::Result<()> {
    self.with_inner_mut(|d| d.rename(&new_name))
  }

  #[napi]
  /// Copies the directory recursively to a new location.
  #[allow(clippy::needless_pass_by_value)]
  pub fn copy_to(&self, dest: String) -> napi::Result<Directory> {
    self
      .with_inner(|d| d.copy_to(Path::new(&dest)))
      .map(|inner| Directory {
        inner: std::cell::RefCell::new(Some(inner)),
      })
  }

  #[napi]
  /// Moves the directory to a new location
  /// Falls back to copy + delete for cross-device moves.
  #[allow(clippy::needless_pass_by_value)]
  pub fn move_to(&mut self, dest: String) -> napi::Result<()> {
    self.with_inner_mut(|d| d.move_to(Path::new(&dest)))
  }

  #[napi]
  /// Deletes the directory recursively, invalidating the handle.
  pub fn delete(&self) -> napi::Result<()> {
    let inner = self.inner.borrow_mut().take().ok_or_else(|| {
      Error::from_reason("Directory handle has been consumed (deleted or moved)".to_string())
    })?;
    inner.delete().map_err(Error::from_reason)
  }

  #[napi]
  /// Returns all files in this directory (non-recursive).
  ///
  /// Throws if any individual directory entry fails to read.
  pub fn list_files(&self) -> napi::Result<Vec<File>> {
    let borrow = self.inner.borrow();
    let inner = borrow
      .as_ref()
      .ok_or_else(|| Error::from_reason("Directory handle has been consumed".to_string()))?;
    inner
      .list_files()
      .map(|r| {
        r.map(|f| File {
          inner: std::cell::RefCell::new(Some(f)),
        })
        .map_err(Error::from_reason)
      })
      .collect::<napi::Result<Vec<_>>>()
  }

  #[napi]
  /// Returns all subdirectories in this directory (non-recursive).
  ///
  /// Throws if any individual directory entry fails to read.
  pub fn list_dirs(&self) -> napi::Result<Vec<Directory>> {
    let borrow = self.inner.borrow();
    let inner = borrow
      .as_ref()
      .ok_or_else(|| Error::from_reason("Directory handle has been consumed".to_string()))?;
    inner
      .list_dirs()
      .map(|r| {
        r.map(|d| Directory {
          inner: std::cell::RefCell::new(Some(d)),
        })
        .map_err(Error::from_reason)
      })
      .collect::<napi::Result<Vec<_>>>()
  }

  #[napi]
  /// Returns the directory name (last component of the path).
  pub fn name(&self) -> napi::Result<String> {
    self.with_inner(|d| Ok(d.name().to_owned()))
  }

  #[napi]
  /// Returns the last modification time of the directory itself as
  /// milliseconds since the Unix epoch.
  #[allow(clippy::cast_precision_loss)]
  pub fn modified(&self) -> napi::Result<f64> {
    self.with_inner(|d| {
      d.modified().map(|t| {
        t.duration_since(std::time::UNIX_EPOCH)
          .unwrap_or_default()
          .as_millis() as f64
      })
    })
  }

  #[napi]
  /// Returns whether the directory is read-only.
  pub fn is_readonly(&self) -> napi::Result<bool> {
    self.with_inner(DirectoryTrait::is_readonly)
  }
}
