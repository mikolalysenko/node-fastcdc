# fastcdc

**fastcdc:** NodeJS bindings for [@nlfielder](https://github.com/nlfiedler)'s [fastcdc-rs](https://github.com/nlfiedler/fastcdc-rs) package.

This module implements the fast content defined chunking algorithm (FastCDC) described in the following paper:

W. Xia et al. "[FastCDC: a Fast and Efficient Content-Defined Chunking Approach for Data Deduplication](https://www.usenix.org/system/files/conference/atc16/atc16-paper-xia.pdf)" Usenix 2016

Content defined chunking takes a blob of bytes (like a file for example) and cuts it into more or less uniform sized chunks along content boundaries.  Because these chunk boundaries are determined by the contents of a file, not regular intervals they are robust to local modifications like insertions and deletions.  This makes content defined chunking useful when deduplicating date, comparing files and synchronizing files over remote network storage.

## Example

```javascript
const fastCDC = require('./index.js')

const buffer = new Uint8Array(10 * 1024)
for (let i = 0; i < buffer.length; ++i) {
    buffer[i] = Math.random() * 256
}

console.log(fastCDC(buffer, {
    min: 1024,
    avg: 4096,
    max: 65536
}))
```

**Output**

```
[ 0, 2594, 6454, 8106, 9244, 10240 ]
```

## Install

If your system has an up-to-date build of nodejs and rust you should be able to install this package using

```
npm install fastcdc
```

## API

### `require('fastcdc')(bytes:ArrayBuffer|Uint8Array, opts?)`

Computes a list of content defined chunk boundaries using fastcdc-rs. 

**Arguments**

* **`bytes`** is either an `ArrayBuffer` or a `Uint8Array` containing the bytes to be hashed
* **`opts`** is a set of optional arguments to the chunker.  If it is a number, then it assumed to be the average chunk size in bytes (default is `1024`).  Otherwise, if an object is passed then it is assumed to be a configuration with the following properties:
    * `min` the minimum chunk size
    * `avg` the average chunk size
    * `max` the maximum chunk size

**Returns** An array of chunk boundaries (0 and `bytes.length` inclusive) representing the chunks of the file

## License
(c) 2021 Mikola Lysenko.  ISC License