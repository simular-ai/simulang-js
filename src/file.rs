use napi::Error;
use napi_derive::napi;
use simulang_rs::File as SimulangFile;

#[napi]
/// A file on a [`Machine`]'s filesystem.
///
/// Obtain via [`Machine.file`]. The handle stays bound to the machine it
/// came from: paths are meaningful only on that machine, and all operations
/// run there. Content access is text-oriented ([`File.read`] trims,
/// [`File.write`] takes strings).
pub struct File {
  inner: std::cell::RefCell<Option<SimulangFile>>,
}

#[napi]
impl File {
  pub(crate) fn new(inner: SimulangFile) -> Self {
    Self {
      inner: std::cell::RefCell::new(Some(inner)),
    }
  }

  fn with_inner<T>(&self, f: impl FnOnce(&SimulangFile) -> Result<T, String>) -> napi::Result<T> {
    let borrow = self.inner.borrow();
    let inner = borrow
      .as_ref()
      .ok_or_else(|| Error::from_reason("File handle has been consumed (deleted)".to_string()))?;
    f(inner).map_err(Error::from_reason)
  }

  fn with_inner_mut<T>(
    &self,
    f: impl FnOnce(&mut SimulangFile) -> Result<T, String>,
  ) -> napi::Result<T> {
    let mut borrow = self.inner.borrow_mut();
    let inner = borrow
      .as_mut()
      .ok_or_else(|| Error::from_reason("File handle has been consumed (deleted)".to_string()))?;
    f(inner).map_err(Error::from_reason)
  }

  #[napi]
  /// The file's path on its machine, as a string.
  pub fn path(&self) -> napi::Result<String> {
    self.with_inner(|f| Ok(f.path()))
  }

  #[napi]
  /// Reads the file and returns its trimmed contents.
  pub fn read(&self) -> napi::Result<String> {
    self.with_inner(SimulangFile::read)
  }

  #[napi]
  /// Writes content to the file, overwriting it — or appending, with a
  /// newline inserted first when the file already has content. Parent
  /// directories are created if needed.
  #[allow(clippy::needless_pass_by_value)]
  pub fn write(&self, content: String, append: bool) -> napi::Result<()> {
    self.with_inner(|f| f.write(&content, append))
  }

  #[napi]
  /// Renames the file in place (same parent directory). `newName` must
  /// be a single filename component.
  #[allow(clippy::needless_pass_by_value)]
  pub fn rename(&mut self, new_name: String) -> napi::Result<()> {
    self.with_inner_mut(|f| f.rename(&new_name))
  }

  #[napi]
  /// Copies the file to `dest` on the same machine, returning a handle
  /// to the copy. Fails when `dest` already exists.
  ///
  /// Local: an absolute `dest` is used as-is. A relative `dest` is joined
  /// to the `SimularFiles` root (`..` is kept — this is a default base,
  /// not a sandbox).
  /// Android: `dest` must be absolute.
  #[allow(clippy::needless_pass_by_value)]
  pub fn copy_to(&self, dest: String) -> napi::Result<File> {
    self.with_inner(|f| f.copy_to(&dest)).map(Self::new)
  }

  #[napi]
  /// Moves the file to `dest` on the same machine. Fails when `dest`
  /// already exists. Path resolution as in [`File.copyTo`].
  #[allow(clippy::needless_pass_by_value)]
  pub fn move_to(&mut self, dest: String) -> napi::Result<()> {
    self.with_inner_mut(|f| f.move_to(&dest))
  }

  #[napi]
  /// Deletes the file, invalidating the handle.
  pub fn delete(&self) -> napi::Result<()> {
    let inner = self
      .inner
      .borrow_mut()
      .take()
      .ok_or_else(|| Error::from_reason("File handle has been consumed (deleted)".to_string()))?;
    inner.delete().map_err(Error::from_reason)
  }

  #[napi]
  /// The file name (last component of the path).
  pub fn name(&self) -> napi::Result<String> {
    self.with_inner(|f| Ok(f.name().to_owned()))
  }

  #[napi]
  /// The file extension, if any.
  pub fn extension(&self) -> napi::Result<Option<String>> {
    self.with_inner(|f| Ok(f.extension().map(ToOwned::to_owned)))
  }

  #[napi]
  /// The file size in bytes.
  pub fn size(&self) -> napi::Result<i64> {
    #[allow(clippy::cast_possible_wrap)]
    self.with_inner(|f| f.size().map(|s| s as i64))
  }

  #[napi]
  /// The last modification time as milliseconds since the Unix epoch.
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
  /// Whether the file is read-only.
  pub fn is_readonly(&self) -> napi::Result<bool> {
    self.with_inner(SimulangFile::is_readonly)
  }
}
