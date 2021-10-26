const fastCDC = require('./index.js')

const buffer = new Uint8Array(10 * 1024)
for (let i = 0; i < buffer.length; ++i) {
    buffer[i] = Math.random() * 256
}

console.log(fastCDC(buffer))