use napi::Error;
use napi_derive::napi;
use simulang_rs::Directory as SimulangDirectory;

use crate::file::File;

#[napi]
/// A directory on a [`Machine`]'s filesystem.
///
/// Obtain via [`Machine.dir`] or [`Machine.tempDir`]. The handle stays
/// bound to the machine it came from: paths are meaningful only on that
/// machine, and all operations run there.
pub struct Directory {
  inner: std::cell::RefCell<Option<SimulangDirectory>>,
}

#[napi]
impl Directory {
  pub(crate) fn new(inner: SimulangDirectory) -> Self {
    Self {
      inner: std::cell::RefCell::new(Some(inner)),
    }
  }

  fn with_inner<T>(
    &self,
    f: impl FnOnce(&SimulangDirectory) -> Result<T, String>,
  ) -> napi::Result<T> {
    let borrow = self.inner.borrow();
    let inner = borrow.as_ref().ok_or_else(|| {
      Error::from_reason("Directory handle has been consumed (deleted)".to_string())
    })?;
    f(inner).map_err(Error::from_reason)
  }

  fn with_inner_mut<T>(
    &self,
    f: impl FnOnce(&mut SimulangDirectory) -> Result<T, String>,
  ) -> napi::Result<T> {
    let mut borrow = self.inner.borrow_mut();
    let inner = borrow.as_mut().ok_or_else(|| {
      Error::from_reason("Directory handle has been consumed (deleted)".to_string())
    })?;
    f(inner).map_err(Error::from_reason)
  }

  #[napi]
  /// The directory's path on its machine, as a string.
  pub fn path(&self) -> napi::Result<String> {
    self.with_inner(|d| Ok(d.path()))
  }

  #[napi]
  /// Renames the directory in place (same parent directory). `newName`
  /// must be a single filename component.
  #[allow(clippy::needless_pass_by_value)]
  pub fn rename(&mut self, new_name: String) -> napi::Result<()> {
    self.with_inner_mut(|d| d.rename(&new_name))
  }

  #[napi]
  /// Copies the directory recursively to `dest` on the same machine,
  /// returning a handle to the copy. Fails when `dest` already exists.
  ///
  /// Local: an absolute `dest` is used as-is. A relative `dest` is joined
  /// to the `SimularFiles` root (`..` is kept — this is a default base,
  /// not a sandbox).
  /// Android: `dest` must be absolute.
  #[allow(clippy::needless_pass_by_value)]
  pub fn copy_to(&self, dest: String) -> napi::Result<Directory> {
    self.with_inner(|d| d.copy_to(&dest)).map(Self::new)
  }

  #[napi]
  /// Moves the directory to `dest` on the same machine. Fails when
  /// `dest` already exists. Path resolution as in [`Directory.copyTo`].
  #[allow(clippy::needless_pass_by_value)]
  pub fn move_to(&mut self, dest: String) -> napi::Result<()> {
    self.with_inner_mut(|d| d.move_to(&dest))
  }

  #[napi]
  /// Deletes the directory and everything in it, invalidating the handle.
  pub fn delete(&self) -> napi::Result<()> {
    let inner = self.inner.borrow_mut().take().ok_or_else(|| {
      Error::from_reason("Directory handle has been consumed (deleted)".to_string())
    })?;
    inner.delete().map_err(Error::from_reason)
  }

  #[napi]
  /// All files in this directory (non-recursive), hidden files included.
  pub fn list_files(&self) -> napi::Result<Vec<File>> {
    self
      .with_inner(SimulangDirectory::list_files)
      .map(|files| files.into_iter().map(File::new).collect())
  }

  #[napi]
  /// All subdirectories of this directory (non-recursive), hidden ones
  /// included.
  pub fn list_dirs(&self) -> napi::Result<Vec<Directory>> {
    self
      .with_inner(SimulangDirectory::list_dirs)
      .map(|dirs| dirs.into_iter().map(Self::new).collect())
  }

  #[napi]
  /// The directory name (last component of the path).
  pub fn name(&self) -> napi::Result<String> {
    self.with_inner(|d| Ok(d.name().to_owned()))
  }

  #[napi]
  /// The last modification time of the directory itself as milliseconds
  /// since the Unix epoch.
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
  /// Whether the directory is read-only.
  pub fn is_readonly(&self) -> napi::Result<bool> {
    self.with_inner(SimulangDirectory::is_readonly)
  }
}
