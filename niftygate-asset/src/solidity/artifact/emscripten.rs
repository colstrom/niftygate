// SPDX-License-Identifier: MIT
//
// Ported from the LZ4 decompression code embedded by Emscripten,
// which is in turn based on node-lz4, and licensed accordingly.
//
// https://github.com/pierrec/node-lz4
//
// Copyright (c) 2012 Pierre Curto
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

pub(crate) mod lz4 {
  /// uncompress an LZ4-compressed WASM blob produced by Emscripten.
  ///
  // In an ideal world, this wouldn't be needed. But for whatever reason, the
  // LZ4-compressed payloads produced by Emscripten fail to decompress with the
  // usual approach (lz4_flex). Maybe it's missing some padding, an end marker,
  // frame headers, or something like that. I'm not actually sure.
  //
  // What I do know, is that the JS decompression program embedded by Emscripten
  // works, for the payloads it creates. So I ported it to Rust. This code is
  // certainly not idiomatic Rust, and there's likely room for optimization.
  //
  // But it works. The code is written in an excessively verbose style,
  // entirely because it made it easier to compare to the JS implementation.
  //
  pub(crate) fn uncompress(source: &[u8], uncompressed_size: usize) -> Vec<u8> {
    let mut result = vec![0_u8; uncompressed_size];
    let mut source_index = 0;
    let mut dest_index = 0;
    let mut block_size;

    loop {
      let b1 = source[source_index] as usize;
      let b2 = (source[source_index + 1] as usize) << 8;
      let b3 = (source[source_index + 2] as usize) << 16;
      let b4 = (source[source_index + 3] as usize) << 24;

      block_size = b1 | b2 | b3 | b4;
      if block_size > 0 {
        source_index += 4;
        if (block_size & 0x80000000) != 0 {
          block_size &= 0x7ffffffff;
          for _ in 0..block_size {
            result[dest_index] = source[source_index];
            dest_index += 1;
            source_index += 1;
          }
        } else {
          let length = uncompressed_size - dest_index;
          let output = &mut result[dest_index..(dest_index + length)];
          let e_idx = source_index + block_size;
          dest_index += uncompress_block(source, output, source_index, e_idx) as usize;
          source_index += block_size;
        }
      } else {
        break;
      }
    }
    result
  }

  fn uncompress_block(input: &[u8], output: &mut [u8], s_idx: usize, e_idx: usize) -> isize {
    let e_idx = if e_idx == 0 {
      input.len() - s_idx
    } else {
      e_idx
    };

    let mut j = 0;
    let n = e_idx;
    let mut i = s_idx;
    while i < n {
      let token = input[i];
      i += 1;
      let mut literals_length: usize = token as usize >> 4;
      if literals_length > 0 {
        let mut l = literals_length + 240;
        while l == 255 {
          l = input[i] as usize;
          i += 1;
          literals_length += l;
        }

        let end = i + literals_length as usize;
        while i < end {
          output[j] = input[i];
          i += 1;
          j += 1;
        }
        if i == n {
          return j as isize;
        }
      }

      let x = i;
      i += 1;
      let a = input[x] as usize;
      let y = i;
      i += 1;
      let z = input[y];
      let b = (z as usize) << 8;
      let offset = a | b;
      if offset == 0 {
        return j as isize;
      }
      if offset > j {
        let r = -(i as isize - 2);
        return r;
      }
      let mut match_length: usize = token as usize & 0xf;
      let mut l = match_length + 240;
      while l == 255 {
        l = input[i] as usize;
        i += 1;
        match_length += l;
      }
      let mut pos = j - offset;
      let end = j + match_length as usize + 4;
      while j < end {
        output[j] = output[pos];
        j += 1;
        pos += 1;
      }
    }
    return j as isize;
  }

  /// JS implementation for validation purposes only.
  //
  // To support porting Emscripten's LZ4 decompression code to Rust, this
  // module was written. It contains the JS implementation that Emscripten
  // embeds, with modifications to make it easier to compare to the Rust
  // implementation. It's excessively verbose for the same reasons.
  //
  // This is feature-gated specifically because it should not be used for any
  // reason other than comparing to the Rust version. It's slow, and not by a
  // small margin. It's somewhere in the range of 400-1500x slower than the
  // Rust implementation.
  //
  // Yes, this embeds an entire JavaScript runtime (QuickJS), just to execute
  // four functions. There's a reason it was ported to Rust.
  //
  // But it works.
  #[cfg(feature = "js-runtime")]
  mod js {
    use quick_js::console::LogConsole;
    use quick_js::{Context, ContextError, ExecutionError, JsValue};

    #[derive(Debug, thiserror::Error)]
    pub enum JavaScriptError {
      #[error("context error: {0}")]
      ContextError(#[from] ContextError),
      #[error("execution error: {0}")]
      ExecutionError(#[from] ExecutionError),
    }

    fn uncompress(source: &str, uncompressed_size: usize) -> Result<Vec<u8>, JavaScriptError> {
      let js = Context::builder().console(LogConsole).build()?;
      js.eval(IMPLEMENTATION)?;
      js.set_global("source", source)?;
      js.set_global("uncompressedSize", uncompressed_size.to_string().as_str())?;
      let result = js.eval("uncompress(base64DecToArr(source), uncompressedSize)")?;

      if let JsValue::Object(object) = result {
        let mut out = vec![0 as u8; uncompressed_size];
        for (key, value) in object {
          if let JsValue::Int(value) = value {
            let index = key.parse::<usize>().unwrap();
            out[index] = value as u8;
          }
        }

        Ok(out)
      } else {
        dbg!(uncompressed_size);
        match result {
          JsValue::Array(_value) => println!("Array"),
          JsValue::Bool(_value) => println!("Bool"),
          JsValue::Float(_value) => println!("Float"),
          JsValue::Int(_value) => println!("Int"),
          JsValue::Null => println!("Null"),
          JsValue::Object(object) => {
            println!("Object(len={})", object.len());

            // for (index, (key, value)) in object.iter().enumerate() {
            //   println!("{key} {value:?}")
            // }
          }
          JsValue::String(_value) => println!("String"),
          JsValue::Undefined => println!("Undefined"),
          _ => println!("OTHER"),
        }

        Ok(Vec::new())
      }
    }

    const IMPLEMENTATION: &str = "
        function b64ToUint6 (nChr) {
        return nChr > 64 && nChr < 91 ?
            nChr - 65
            : nChr > 96 && nChr < 123 ?
            nChr - 71
            : nChr > 47 && nChr < 58 ?
            nChr + 4
            : nChr === 43 ?
            62
            : nChr === 47 ?
            63
            :
            0;
        }
        
        function base64DecToArr (sBase64) {
        var
            nInLen = sBase64.length,
            nOutLen = nInLen * 3 + 1 >> 2,
            taBytes = new Uint8Array(nOutLen);
        
        for (var nMod3, nMod4, nUint24 = 0, nOutIdx = 0, nInIdx = 0; nInIdx < nInLen; nInIdx++) {
            nMod4 = nInIdx & 3;
            nUint24 |= b64ToUint6(sBase64.charCodeAt(nInIdx)) << 6 * (3 - nMod4);
            if (nMod4 === 3 || nInLen - nInIdx === 1) {
            for (nMod3 = 0; nMod3 < 3 && nOutIdx < nOutLen; nMod3++, nOutIdx++) {
                taBytes[nOutIdx] = nUint24 >>> (16 >>> nMod3 & 24) & 255;
            }
            nUint24 = 0;
        
            }
        }
        
        return taBytes;
        }
    
        function uncompress(source, uncompressedSize) {
        var result = new ArrayBuffer(uncompressedSize);
        var sourceIndex = 0;
        var destIndex = 0;
        var blockSize;
        var sourceLength = source.length;
        
        var logged = 0;
        
        while (true) {
            logged += 1;
            var b1 = source[sourceIndex];
            var b2 = source[sourceIndex + 1] << 8;
            var b3 = source[sourceIndex + 2] << 16;
            var b4 = source[sourceIndex + 3] << 24;
        
            blockSize = b1 | b2 | b3 | b4;
        
            if (blockSize > 0) {
            sourceIndex += 4;
            if ((blockSize & 0x80000000) != 0) {
                blockSize &= 0x7ffffffff;
                for (var i = 0; i < blockSize; i++) {
                // result[destIndex++] = source[sourceIndex++];
                result[destIndex] = source[sourceIndex];
                destIndex += 1;
                sourceIndex += 1;
                }
            } else {
                var length = uncompressedSize - destIndex;
                var output = new Uint8Array(result, destIndex, length);
                var size = output.length;
                var eIdx = sourceIndex + blockSize;
                destIndex += uncompressBlock(source, output, sourceIndex, eIdx);
                sourceIndex += blockSize;
            }
            } else {
            break;
            }
        }
        return new Uint8Array(result, 0, uncompressedSize);
        }
        
        function uncompressBlock(input, output, sIdx, eIdx) {
        sIdx = sIdx || 0;
        eIdx = eIdx || input.length - sIdx;
        
        for (var i = sIdx, n = eIdx, j = 0; i < n; ) {
            var token = input[i];
            i += 1;
            var literals_length = token >> 4;
            if (literals_length > 0) {
            var l = literals_length + 240;
            while (l == 255) {
                l = input[i];
                i += 1;
                literals_length += l;
            }
        
            var end = i + literals_length;
            while (i < end) {
                output[j] = input[i];
                i += 1;
                j += 1;
            }
            if (i == n) {
                return j;
            }
            }
        
            var x = i;
            i += 1;
            var a = input[x];
            var y = i;
            i += 1;
            var z = input[y];
            var b = z << 8;
            var offset = a | b;
            if (offset == 0) {
            return j;
            }
            if (offset > j) {
            var r = -(i - 2);
            return r;
            }
            var match_length = token & 0xf;
            var l = match_length + 240;
            while (l == 255) {
            l = input[i];
            i += 1;
            match_length += l;
            }
            var pos = j - offset;
            var end = j + match_length + 4;
            while (j < end) {
            var copied = output[pos];
            output[j] = output[pos];
            j += 1;
            pos += 1;
            }
        }
        
        return j;
        }
    ";
  }
}
