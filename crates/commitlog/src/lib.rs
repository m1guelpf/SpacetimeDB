use std::{io, num::NonZeroU16, path::PathBuf, sync::RwLock};

use log::trace;

mod commit;
pub mod commitlog;
pub mod repo;
pub mod segment;
mod varchar;
mod varint;

pub use crate::{
    commit::Commit,
    payload::{Decoder, Encode},
    segment::{Transaction, DEFAULT_LOG_FORMAT_VERSION},
    varchar::Varchar,
};
pub mod error;
pub mod payload;

#[cfg(test)]
mod tests;

/// [`Commitlog`] options.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Set the log format version to write, and the maximum supported version.
    ///
    /// Choosing a payload format `T` of [`Commitlog`] should usually result in
    /// updating the [`DEFAULT_LOG_FORMAT_VERSION`] of this crate. Sometimes it
    /// may however be useful to set the version at runtime, e.g. to experiment
    /// with new or very old versions.
    ///
    /// Default: [`DEFAULT_LOG_FORMAT_VERSION`]
    pub log_format_version: u8,
    /// The maximum size in bytes to which log segments should be allowed to
    /// grow.
    ///
    /// Default: 1GiB
    pub max_segment_size: u64,
    /// The maximum number of records in a commit.
    ///
    /// If this number is exceeded, the commit is flushed to disk even without
    /// explicitly calling [`Commitlog::flush`].
    ///
    /// Default: 65,535
    pub max_records_in_commit: NonZeroU16,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            log_format_version: DEFAULT_LOG_FORMAT_VERSION,
            max_segment_size: 1024 * 1024 * 1024,
            max_records_in_commit: NonZeroU16::MAX,
        }
    }
}

/// The canonical commitlog, backed by on-disk log files.
///
/// Records in the log are of type `T`, which canonically is instantiated to
/// [`Txdata`].
pub struct Commitlog<T> {
    inner: RwLock<commitlog::Generic<repo::Fs, T>>,
}

impl<T> Commitlog<T> {
    /// Open the log at root directory `root` with [`Options`].
    ///
    /// The root directory must already exist.
    ///
    /// Note that opening a commitlog involves I/O: some consistency checks are
    /// performed, and the next writing position is determined.
    ///
    /// This is only necessary when opening the commitlog for writing. See the
    /// free-standing functions in this module for how to traverse a read-only
    /// commitlog.
    pub fn open(root: impl Into<PathBuf>, opts: Options) -> io::Result<Self> {
        let inner = commitlog::Generic::open(repo::Fs::new(root), opts)?;

        Ok(Self {
            inner: RwLock::new(inner),
        })
    }

    /// Determine the maximum transaction offset considered durable.
    ///
    /// The offset is `None` if the log hasn't been flushed to disk yet.
    pub fn max_committed_offset(&self) -> Option<u64> {
        self.inner.read().unwrap().max_committed_offset()
    }

    /// Sync all OS-buffered writes to disk.
    ///
    /// Note that this does **not** write outstanding records to disk.
    /// Use [`Self::flush_and_sync`] or call [`Self::flush`] prior to this
    /// method to ensure all data is on disk.
    ///
    /// Returns the maximum transaction offset which is considered durable after
    /// this method returns successfully. The offset is `None` if the log hasn't
    /// been flushed to disk yet.
    ///
    /// # Panics
    ///
    /// This method panics if syncing fails irrecoverably.
    pub fn sync(&self) -> Option<u64> {
        let mut inner = self.inner.write().unwrap();
        trace!("sync commitlog");
        inner.sync();

        inner.max_committed_offset()
    }

    /// Write all outstanding transaction records to disk.
    ///
    /// Note that this does **not** force the OS to sync the data to disk.
    /// Use [`Self::flush_and_sync`] or call [`Self::sync`] after this method
    /// to ensure all data is on disk.
    ///
    /// Returns the maximum transaction offset written to disk. The offset is
    /// `None` if the log is empty and no data was pending to be flushed.
    ///
    /// Repeatedly calling this method may return the same value.
    pub fn flush(&self) -> io::Result<Option<u64>> {
        let mut inner = self.inner.write().unwrap();
        trace!("flush commitlog");
        inner.commit()?;

        Ok(inner.max_committed_offset())
    }

    /// Write all outstanding transaction records to disk and flush OS buffers.
    ///
    /// Equivalent to calling [`Self::flush`] followed by [`Self::sync`], but
    /// without releasing the write lock in between.
    ///
    /// # Errors
    ///
    /// An error is returned if writing to disk fails due to an I/O error.
    ///
    /// # Panics
    ///
    /// This method panics if syncing fails irrecoverably.
    pub fn flush_and_sync(&self) -> io::Result<Option<u64>> {
        let mut inner = self.inner.write().unwrap();
        trace!("flush and sync commitlog");
        inner.commit()?;
        inner.sync();

        Ok(inner.max_committed_offset())
    }

    /// Obtain an iterator which traverses the log from the start, yielding
    /// [`Commit`]s.
    ///
    /// The returned iterator is not aware of segment rotation. That is, if a
    /// new segment is created after this method returns, the iterator will not
    /// traverse it.
    ///
    /// Commits appended to the log while it is being traversed are generally
    /// visible to the iterator. Upon encountering [`io::ErrorKind::UnexpectedEof`],
    /// however, a new iterator should be created using [`Self::commits_from`]
    /// with the last transaction offset yielded.
    ///
    /// Note that the very last [`Commit`] in a commitlog may be corrupt (e.g.
    /// due to a partial write to disk), but a subsequent `append` will bring
    /// the log into a consistent state.
    ///
    /// This means that, when this iterator yields an `Err` value, the consumer
    /// may want to check if the iterator is exhausted (by calling `next()`)
    /// before treating the `Err` value as an application error.
    pub fn commits(&self) -> impl Iterator<Item = Result<Commit, error::Traversal>> {
        self.commits_from(0)
    }

    /// Obtain an iterator starting from transaction offset `offset`, yielding
    /// [`Commit`]s.
    ///
    /// Similar to [`Self::commits`] but will skip until the offset is contained
    /// in the next [`Commit`] to yield.
    ///
    /// Note that the first [`Commit`] yielded is the first commit containing
    /// the given transaction offset, i.e. its `min_tx_offset` may be smaller
    /// than `offset`.
    pub fn commits_from(&self, offset: u64) -> impl Iterator<Item = Result<Commit, error::Traversal>> {
        self.inner.read().unwrap().commits_from(offset)
    }

    /// Remove all data from the log and reopen it.
    ///
    /// Log segments are deleted starting from the newest. As multiple segments
    /// cannot be deleted atomically, the log may not be completely empty if
    /// the method returns an error.
    ///
    /// Note that the method consumes `self` to ensure the log is not modified
    /// while resetting.
    pub fn reset(self) -> io::Result<Self> {
        let inner = self.inner.into_inner().unwrap().reset()?;
        Ok(Self {
            inner: RwLock::new(inner),
        })
    }

    /// Remove all data past the given transaction `offset` from the log and
    /// reopen it.
    ///
    /// Like with [`Self::reset`], it may happen that not all segments newer
    /// than `offset` can be deleted.
    ///
    /// If the method returns successfully, the most recent [`Commit`] in the
    /// log will contain the transaction at `offset`.
    ///
    /// Note that the method consumes `self` to ensure the log is not modified
    /// while resetting.
    pub fn reset_to(self, offset: u64) -> io::Result<Self> {
        let inner = self.inner.into_inner().unwrap().reset_to(offset)?;
        Ok(Self {
            inner: RwLock::new(inner),
        })
    }

    /// Determine the size on disk of this commitlog.
    pub fn size_on_disk(&self) -> io::Result<u64> {
        let inner = self.inner.read().unwrap();
        inner.repo.size_on_disk()
    }
}

impl<T: Encode> Commitlog<T> {
    /// Append the record `txdata` to the log.
    ///
    /// If the internal buffer exceeds [`Options::max_records_in_commit`], the
    /// argument is returned in an `Err`. The caller should [`Self::flush`] the
    /// log and try again.
    ///
    /// In case the log is appended to from multiple threads, this may result in
    /// a busy loop trying to acquire a slot in the buffer. In such scenarios,
    /// [`Self::append_maybe_flush`] is preferable.
    pub fn append(&self, txdata: T) -> Result<(), T> {
        let mut inner = self.inner.write().unwrap();
        inner.append(txdata)
    }

    /// Append the record `txdata` to the log.
    ///
    /// The `txdata` payload is buffered in memory until either:
    ///
    /// - [`Self::flush`] is called explicitly, or
    /// - [`Options::max_records_in_commit`] is exceeded
    ///
    /// In the latter case, [`Self::append`] flushes implicitly, _before_
    /// appending the `txdata` argument.
    ///
    /// I.e. the argument is not guaranteed to be flushed after the method
    /// returns. If that is desired, [`Self::flush`] must be called explicitly.
    ///
    /// # Errors
    ///
    /// If the log needs to be flushed, but an I/O error occurs, ownership of
    /// `txdata` is returned back to the caller alongside the [`io::Error`].
    ///
    /// The value can then be used to retry appending.
    pub fn append_maybe_flush(&self, txdata: T) -> Result<(), error::Append<T>> {
        let mut inner = self.inner.write().unwrap();

        if let Err(txdata) = inner.append(txdata) {
            if let Err(source) = inner.commit() {
                return Err(error::Append { txdata, source });
            }
            // `inner.commit.n` must be zero at this point
            let res = inner.append(txdata);
            debug_assert!(res.is_ok(), "failed to append while holding write lock");
        }

        Ok(())
    }

    /// Obtain an iterator which traverses the log from the start, yielding
    /// [`Transaction`]s.
    ///
    /// The provided `decoder`'s [`Decoder::decode_record`] method will be
    /// called [`Commit::n`] times per [`Commit`] to obtain the individual
    /// transaction payloads.
    ///
    /// Like [`Self::commits`], the iterator is not aware of segment rotation.
    /// That is, if a new segment is created after this method returns, the
    /// iterator will not traverse it.
    ///
    /// Transactions appended to the log while it is being traversed are
    /// generally visible to the iterator. Upon encountering [`io::ErrorKind::UnexpectedEof`],
    /// however, a new iterator should be created using [`Self::transactions_from`]
    /// with the last transaction offset yielded.
    ///
    /// Note that the very last [`Commit`] in a commitlog may be corrupt (e.g.
    /// due to a partial write to disk), but a subsequent `append` will bring
    /// the log into a consistent state.
    ///
    /// This means that, when this iterator yields an `Err` value, the consumer
    /// may want to check if the iterator is exhausted (by calling `next()`)
    /// before treating the `Err` value as an application error.
    pub fn transactions<'a, D>(&self, de: &'a D) -> impl Iterator<Item = Result<Transaction<T>, D::Error>> + 'a
    where
        D: Decoder<Record = T>,
        D::Error: From<error::Traversal>,
        T: 'a,
    {
        self.transactions_from(0, de)
    }

    /// Obtain an iterator starting from transaction offset `offset`, yielding
    /// [`Transaction`]s.
    ///
    /// Similar to [`Self::transactions`] but will skip until the provided
    /// `offset`, i.e. the first [`Transaction`] yielded will be the transaction
    /// with offset `offset`.
    pub fn transactions_from<'a, D>(
        &self,
        offset: u64,
        de: &'a D,
    ) -> impl Iterator<Item = Result<Transaction<T>, D::Error>> + 'a
    where
        D: Decoder<Record = T>,
        D::Error: From<error::Traversal>,
        T: 'a,
    {
        self.inner.read().unwrap().transactions_from(offset, de)
    }

    /// Traverse the log from the start and "fold" its transactions into the
    /// provided [`Decoder`].
    ///
    /// A [`Decoder`] is a stateful object due to the requirement to store
    /// schema information in the log itself. That is, a [`Decoder`] may need to
    /// be able to resolve transaction schema information dynamically while
    /// traversing the log.
    ///
    /// This is equivalent to "replaying" a log into a database state. In this
    /// scenario, it is not interesting to consume the [`Transaction`] payload
    /// as an iterator.
    ///
    /// This method allows the use of a [`Decoder`] which returns zero-sized
    /// data (e.g. `Decoder<Record = ()>`), as it will not allocate the commit
    /// payload into a struct.
    ///
    /// Note that, unlike [`Self::transaction`], this method will ignore a
    /// corrupt commit at the very end of the traversed log.
    pub fn fold_transactions<D>(&self, de: D) -> Result<(), D::Error>
    where
        D: Decoder,
        D::Error: From<error::Traversal>,
    {
        self.fold_transactions_from(0, de)
    }

    /// Traverse the log from the given transaction offset and "fold" its
    /// transactions into the provided [`Decoder`].
    ///
    /// Similar to [`Self::fold_transactions`] but will skip until the provided
    /// `offset`, i.e. the first `tx_offset` passed to [`Decoder::decode_record`]
    /// will be equal to `offset`.
    pub fn fold_transactions_from<D>(&self, offset: u64, de: D) -> Result<(), D::Error>
    where
        D: Decoder,
        D::Error: From<error::Traversal>,
    {
        self.inner.read().unwrap().fold_transactions_from(offset, de)
    }
}

/// Obtain an iterator which traverses the commitlog located at the `root`
/// directory from the start, yielding [`Commit`]s.
///
/// Starts the traversal without the upfront I/O imposed by [`Commitlog::open`].
/// See [`Commitlog::commits`] for more information.
pub fn commits(root: impl Into<PathBuf>) -> io::Result<impl Iterator<Item = Result<Commit, error::Traversal>>> {
    commits_from(root, 0)
}

/// Obtain an iterator which traverses the commitlog located at the `root`
/// directory starting from `offset` and yielding [`Commit`]s.
///
/// Starts the traversal without the upfront I/O imposed by [`Commitlog::open`].
/// See [`Commitlog::commits_from`] for more information.
pub fn commits_from(
    root: impl Into<PathBuf>,
    offset: u64,
) -> io::Result<impl Iterator<Item = Result<Commit, error::Traversal>>> {
    commitlog::commits_from(repo::Fs::new(root), DEFAULT_LOG_FORMAT_VERSION, offset)
}

/// Obtain an iterator which traverses the commitlog located at the `root`
/// directory from the start, yielding [`Transaction`]s.
///
/// Starts the traversal without the upfront I/O imposed by [`Commitlog::open`].
/// See [`Commitlog::transactions`] for more information.
pub fn transactions<'a, D, T>(
    root: impl Into<PathBuf>,
    de: &'a D,
) -> io::Result<impl Iterator<Item = Result<Transaction<T>, D::Error>> + 'a>
where
    D: Decoder<Record = T>,
    D::Error: From<error::Traversal>,
    T: 'a,
{
    transactions_from(root, 0, de)
}

/// Obtain an iterator which traverses the commitlog located at the `root`
/// directory starting from `offset` and yielding [`Transaction`]s.
///
/// Starts the traversal without the upfront I/O imposed by [`Commitlog::open`].
/// See [`Commitlog::transactions_from`] for more information.
pub fn transactions_from<'a, D, T>(
    root: impl Into<PathBuf>,
    offset: u64,
    de: &'a D,
) -> io::Result<impl Iterator<Item = Result<Transaction<T>, D::Error>> + 'a>
where
    D: Decoder<Record = T>,
    D::Error: From<error::Traversal>,
    T: 'a,
{
    commitlog::transactions_from(repo::Fs::new(root), DEFAULT_LOG_FORMAT_VERSION, offset, de)
}

/// Traverse the commitlog located at the `root` directory from the start and
/// "fold" its transactions into the provided [`Decoder`].
///
/// Starts the traversal without the upfront I/O imposed by [`Commitlog::open`].
/// See [`Commitlog::fold_transactions`] for more information.
pub fn fold_transactions<D>(root: impl Into<PathBuf>, de: D) -> Result<(), D::Error>
where
    D: Decoder,
    D::Error: From<error::Traversal> + From<io::Error>,
{
    fold_transactions_from(root, 0, de)
}

/// Traverse the commitlog located at the `root` directory starting from `offset`
/// and "fold" its transactions into the provided [`Decoder`].
///
/// Starts the traversal without the upfront I/O imposed by [`Commitlog::open`].
/// See [`Commitlog::fold_transactions_from`] for more information.
pub fn fold_transactions_from<D>(root: impl Into<PathBuf>, offset: u64, de: D) -> Result<(), D::Error>
where
    D: Decoder,
    D::Error: From<error::Traversal> + From<io::Error>,
{
    commitlog::fold_transactions_from(repo::Fs::new(root), DEFAULT_LOG_FORMAT_VERSION, offset, de)
}
