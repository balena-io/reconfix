# file-disk
Handles reads / writes on disk image files.

## API

**Warning: The API exposed by this library is still forming and can change at
any time!**

### FileDisk

`new FileDisk(fd, readOnly, recordWrites, recordReads, discardIsZero=true)`

 - `fd` is a file descriptor returned by `fs.open`
 - `readOnly` a boolean (default `false`)
 - `recordWrites`, a boolean (default `false`); if you use `readOnly` without
 `recordWrites`, all write requests will be lost.
 - `recordReads`, a boolean (default `false`): cache reads in memory
 - `discardIsZero`, a boolean (default `true`): don't read discarded regions,
 return zero filled buffers instead.

`FileDisk.getCapacity()`: `Promise<Number>`

`FileDisk.read(buffer, bufferOffset, length, fileOffset)`: `Promise<{ bytesRead: Number, buffer: Buffer }>`

 - behaves like [fs.read](https://nodejs.org/api/fs.html#fs_fs_read_fd_buffer_offset_length_position_callback)

`FileDisk.write(buffer, bufferOffset, length, fileOffset)`: `Promise<{ bytesWritten: Number, buffer: Buffer }>`

 - behaves like [fs.write](https://nodejs.org/api/fs.html#fs_fs_write_fd_buffer_offset_length_position_callback)

`FileDisk.flush()`: `Promise<void>`

 - behaves like [fs.fdatasync](https://nodejs.org/api/fs.html#fs_fs_fdatasync_fd_callback)

`FileDisk.discard(offset, length)`: `Promise<void>`

`FileDisk.getStream([position, [length, [highWaterMark]]])`: `Promise<stream.Readable>`
 - `position` start reading from this offset (defaults to 0)
 - `length` read that amount of bytes (defaults to (disk capacity - position))
 - `highWaterMark` (defaults to 16384, minimum 16) is the size of chunks that
 will be read

`FileDisk.getDiscardedChunks()` returns the list of discarded chunks. Each chunk
has a `start` and `end` properties. `end` position is inclusive.

`FileDisk.getRanges(blockSize)`: `Promise<Range[]>`
 - using the disk's discarded chunks and the given blockSize, it returns a Promise
of an array of `Range`s: `{ offset: number, length: number }`.

### S3Disk

`S3Disk` has been moved to a [separate repository](https://github.com/balena-io-modules/s3-disk).

## Examples

### Read 1024 first bytes, write them starting at position 1024 then flush.

```javascript

const filedisk = require('file-disk');

await filedisk.withOpenFile('/path/to/some/file', 'r+', async (handle) => {
	const disk = new filedisk.FileDisk(handle)

	// get file size
	const size = await disk.getCapacity();
	console.log("size:", size);
	const buf = Buffer.alloc(1024);
	// read `buf.length` bytes starting at 0 from the file into `buf`
	const { bytesRead, buffer } = await disk.read(buf, 0, buf.length, 0);
	// write `buffer` into file starting at `buffer.length` (in the file)
	await disk.write(buf, 0, buf.length, buf.length);
	// flush
	await disk.flush();
});


```

### Open a file readOnly, use the recordWrites mode, then stream the contents somewhere.

```javascript

const filedisk = require('file-disk');

const BUF = Buffer.alloc(1024);

await filedisk.withOpenFile('/path/to/some/file', 'r', async (handle) => {
	const disk = new filedisk.FileDisk(handle, true, true);
	let bytesRead, bytesWritten, buffer;

	// read `BUF.length` bytes starting at 0 from the file into `BUF`
	{ bytesRead, buffer } = await disk.read(BUF, 0, BUF.length, 0);
	// write `buffer` into file starting at `buffer.length` (in the file)
	{ bytesWritten, buffer } = await disk.write(buffer, 0, buffer.length, buffer.length);
	const buf2 = Buffer.alloc(1024);
	// read what we've just written
	{ bytesRead, buffer } = await disk.read(buf2, 0, buffer.length, 0);
	// writes are stored in memory
	assert(BUF.equals(buffer));
	const stream = await disk.getStream();
	// pipe the stream somewhere
	await new Promise((resolve, reject) => {
		stream.pipe(someWritableStream)
		.on('close', resolve)
		.on('error', reject);
	});
});

```
