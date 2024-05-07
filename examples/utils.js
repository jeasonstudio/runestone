import { decodeHex, encodeHex } from '../runestone/index.js';

console.log(
  decodeHex(
    '51203b8b3ab1453eb47e2d4903b963776680e30863df3625d3e74292338ae7928da1'
  )
);

console.log(
  encodeHex(
    new Uint8Array([
      81, 32, 59, 139, 58, 177, 69, 62, 180, 126, 45, 73, 3, 185, 99, 119, 102,
      128, 227, 8, 99, 223, 54, 37, 211, 231, 66, 146, 51, 138, 231, 146, 141,
      161,
    ])
  )
);
