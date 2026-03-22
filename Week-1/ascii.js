const x  = 0;
console.log(x);

function bytesToAscii(byteArray){
    return byteArray.map(byte => String.fromCharCode(byte)).join('');
}

const bytes = [72, 101, 108, 108, 111]
const asciiString = bytesToAscii(bytes)
console.log("Ascii String", asciiString)

function asciiToBytes(asciiString) {
  const byteArray = [];
  for (let i = 0; i < asciiString.length; i++) {
    byteArray.push(asciiString.charCodeAt(i));
  }
  return byteArray;
}

// Example usage:
const ascii = "5";
const byteArray = asciiToBytes(ascii);
console.log(byteArray); // Output: [72, 101, 108, 108, 111]

let uint8Arr = new Uint8Array([0, 255, 127, 128]); // this takes only 1 byte meanwhile array takes 8 bytes
console.log(uint8Arr);
uint8Arr[1] = 300; // 300 % 256
console.log(uint8Arr); //Values ≥ 256 wrap around (e.g., 256 → 0, 257 → 1, 300 → 44).