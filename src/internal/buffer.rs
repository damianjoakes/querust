use std::cmp;
use std::io::Read;
use std::mem::MaybeUninit;

/// A buffer object for performing buffered reads to the database.
#[derive(Debug)]
pub struct Buffer {
    /// The buffer, contains bytes that are possibly uninitialized.
    pub buf: Box<[MaybeUninit<u8>]>,

    /// The current position in the buffer. Anything between `pos` and `filled` is new data that
    /// has not been read.
    pos: usize,

    /// The amount of bytes in the buffer that currently have data. Anything under `filled`,
    /// after `pos` is unread data.
    end: usize,

    /// The number of bytes in the buffer that have been initialized previously.
    ///
    /// The usage of a variable that has the number of initialized bytes available
    /// offers a substantial performance increase. This is because we can make calls to
    /// reader operations by providing a `BorrowedBuf` the number of initialized and uninitialized
    /// bytes.
    initialized: usize,
}

impl Buffer {
    pub fn new(capacity: usize) -> Self {
        Buffer {
            buf: Box::new_uninit_slice(capacity),
            pos: 0,
            end: 0,
            initialized: 0,
        }
    }

    /// Returns the slice of the buffer containing unread data.
    pub fn buffer(&self) -> &[u8] {
        // SAFETY: The bytes from `self.pos` to `self.filled` will always be initialized.
        //
        // We return a slice from the range `[self.pos..self.filled]` here, as any reads made
        // to this buffer are always made from the current position of the cursor, to the length
        // of the bytes read into this buffer.
        unsafe { MaybeUninit::slice_assume_init_ref(&self.buf[self.pos..self.end]) }
    }

    /// Get the current cursor position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Get the amount of filled bytes.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Get the amount of initialized bytes.
    pub fn initialized(&self) -> usize {
        self.initialized
    }

    /// Gets the length of the internal buffer. This includes initialized bytes as well as
    /// uninitialized bytes.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Sets the cursor position and the number of bytes filled back to 0.
    ///
    /// This doesn't empty the buffer, this only resets the cursor positions, which enable
    /// read operations to overwrite unneeded bytes.
    pub fn discard_buffer(&mut self) {
        self.pos = 0;
        self.end = 0;
    }

    /// Sets the cursor position either to `self.pos + amount`, or `self.filled`, depending on
    /// which is smaller.
    ///
    /// This function is used to set the cursor to an earlier position, allowing for unneeded
    /// data to be overwritten.
    pub fn reposition(&mut self, amount: usize) {
        use std::cmp::min;

        self.pos = min(self.pos + amount, self.end);
    }

    /// Identical to `reposition`, but if the provided amount surpasses `self.filled`, the
    /// cursor position will remain the same, and the function will return `None`.
    pub fn try_reposition(&mut self, amount: usize) -> Option<()> {
        match self.pos + amount > self.end {
            true => {
                self.pos = self.pos + amount;
                Some(())
            }
            false => {
                None
            }
        }
    }

    /// Moves the cursor backwards by the specified amount.
    ///
    /// This uses `saturating_sub` on `self.pos`, so if overflow were to occur, this would
    /// set `self.pos` to `0`.
    pub fn unposition(&mut self, amount: usize) {
        self.pos = self.pos.saturating_sub(amount);
    }

    /// Identical to `unposition`, but if the provided amount were to overflow, the
    /// cursor position will remain the same, and the function will return `None`.
    pub fn try_unposition(&mut self, amount: usize) -> Option<()> {
        match self.pos - amount > 0 {
            true => {
                self.pos = self.pos - amount;
                Some(())
            }
            false => {
                None
            }
        }
    }

    /// If there are enough bytes available in the buffer, this function will call `f`
    /// with a slice of the buffer of `[0..amount]`, and return `Some(())`. If there
    /// are not enough bytes in the buffer, this function will return `None`.
    pub fn consume<F: FnMut(&[u8])>(&mut self, amount: usize, mut f: F) -> Option<()> {
        if let Some(buffer) = self.buffer().get(..amount) {
            f(buffer);
            self.pos += amount;

            Some(())
        } else {
            None
        }
    }

    /// Reads some data into this buffer using an object that implements `Read`. Returns the
    /// amount of bytes read into the buffer.
    ///
    /// Returns `Ok(0)` according to the specification for `std::io::Read`, where if the reader hits
    /// `EOF`, or this buffer has a length of `0`, then `Ok(0)` is returned.
    pub fn read_some<R: Read>(&mut self, reader: &mut R) -> std::io::Result<usize> {
        // Create a slice from the cursor position onwards.
        let slice = &mut self.buf[self.pos..];

        // **SAFETY**: `reader.read()` is always going to insert data into our buffer, even when
        //             the data at any position is uninitialized. The data after `slice.len()` is
        //             not a problem, since external sources can only access between `self.pos` and
        //             `self.filled`, which will always be an initialized slice of unread data.
        //
        //             `reader.read` will also never exceed the length of `slice` (as defined within
        //             the fundamental contract of `std::io::Read`.
        let bytes = unsafe {
            let ptr = slice.as_mut_ptr();
            let buffer = std::slice::from_raw_parts_mut(ptr as *mut u8, slice.len());

            reader.read(buffer)?
        };

        if bytes == 0 {
            return Ok(0);
        }

        // The amount of bytes there are available to read is equal to the amount of bytes read from
        // `reader.read()`.
        self.end += bytes;

        // If `self.pos + bytes` is less than `self.initialized`, then `self.initialized` bytes
        // remains correct. If `self.pos + bytes` is greater than `self.initialized`, then this
        // function has initialized more bytes of information, and we need to adjust
        // `self.initialized` accordingly.
        self.initialized = cmp::max(self.pos + bytes, self.initialized);

        Ok(bytes)
    }

    /// Resets the cursor position back to 0, reads as much data from `reader` as possible,
    /// and then sets `self.filled` to the length of the read data. This resets the cursor
    /// and data positions back to the start of the internal buffer.
    pub fn fill_buf<R: Read>(&mut self, reader: &mut R) -> std::io::Result<usize> {
        self.pos = 0;

        let bytes = unsafe {
            let ptr = self.buf.as_mut_ptr();
            let buffer = std::slice::from_raw_parts_mut(ptr as *mut u8, self.len());

            reader.read(buffer)?
        };

        if bytes == 0 {
            return Ok(0);
        }

        self.end = bytes;

        // Calculate `self.initialized`, setting it to either `self.pos + bytes` or
        // `self.initialized`, depending on which is greater.
        self.initialized = cmp::max(self.pos + bytes, self.initialized);

        Ok(bytes)
    }
}