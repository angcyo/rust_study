///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2026-5-21
///
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    static TEST_FILE_NAME1: &str = "FaceQ.png";

    static TEST_FILE_NAME2: &str = "image 1-image-stucki-1779331603635_300kb.ydd";
    static TEST_FILE_NAME3: &str = "image 1-image-stucki-1779331405815_5mb.ydd";

    static TEST_FILE_NAME4: &str = "image 1-image-stucki-1779332274220_147kb.ydd.nc";

    static TEST_FILE_NAME5: &str = "image 1-image-stucki-1779332354279_16mb.ydd.nc";

    //MARK: - lz4

    /// 测试 lz4 压缩
    /// - 速度快
    /// - 对图片的压缩无效?
    /// - 能压缩到原来的60%
    ///
    /// [TEST_FILE_NAME1] 耗时: 590.9µs 93115 -> 93193 (100%)
    /// [TEST_FILE_NAME2] 耗时: 11.7476ms 306828 -> 177846 (57%)
    /// [TEST_FILE_NAME3] 耗时: 218.2694ms 5581188 -> 3233522 (57%)
    /// [TEST_FILE_NAME4] 耗时: 35.6718ms 1507233 -> 453642 (30%)
    /// [TEST_FILE_NAME5] 耗时: 406.4583ms 17294691 -> 5634031 (32%)
    ///
    /// https://github.com/PSeitz/lz4_flex/blob/main/examples/compress_block.rs
    #[test]
    fn test_lz4_compression() {
        use lz4_flex::block::compress_prepend_size;
        use rc_test::*;
        let bytes = read_test_file_bytes(TEST_FILE_NAME5, false);
        let input = bytes.as_slice();
        let compressed = measure_time(|| compress_prepend_size(input));
        write_test_file_bytes(&format!("{}.lz4", TEST_FILE_NAME5), &compressed);
        println!(
            "{} -> {} ({}%)",
            input.len(),
            compressed.len(),
            compressed.len() * 100 / input.len()
        )
    }

    /// 测试 lz4 解压
    ///
    /// [TEST_FILE_NAME1] 耗时: 120.6µs
    /// [TEST_FILE_NAME2] 耗时: 4.7661ms
    /// [TEST_FILE_NAME3] 耗时: 86.2499ms
    /// [TEST_FILE_NAME4] 耗时: 16.3574ms
    /// [TEST_FILE_NAME5] 耗时: 216.2536ms
    ///
    /// https://github.com/PSeitz/lz4_flex/blob/main/examples/decompress_block.rs
    #[test]
    fn test_lz4_decompression() {
        use lz4_flex::decompress_size_prepended;
        use rc_test::*;
        let bytes = read_test_file_bytes(&format!("{}.lz4", TEST_FILE_NAME5), true);
        let input = bytes.as_slice();
        let uncompressed = measure_time(|| decompress_size_prepended(&input).unwrap());
        write_test_file_bytes(TEST_FILE_NAME5, &uncompressed);
    }

    //MARK: - zlib

    /// 测试 zlib 压缩
    ///
    /// [TEST_FILE_NAME1] 耗时: 8.2093ms 93115 -> 90187 (96%)
    /// [TEST_FILE_NAME2] 耗时: 73.5666ms 306828 -> 69267 (22%)
    /// [TEST_FILE_NAME3] 耗时: 1.4295227s 5581188 -> 1178578 (21%)
    /// [TEST_FILE_NAME4] 耗时: 261.5602ms 1507233 -> 234227 (15%)
    /// [TEST_FILE_NAME5] 耗时: 2.7459241s 17294691 -> 2892321 (16%)
    ///
    /// https://github.com/rust-lang/flate2-rs/blob/main/examples/compress_file.rs
    #[test]
    fn test_zlib_compression() {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        use rc_test::*;
        let bytes = read_test_file_bytes(TEST_FILE_NAME1, false);
        let mut input = bytes.as_slice();
        let input_len = input.len();
        let mut compressed = Vec::new();
        let mut encoder = GzEncoder::new(&mut compressed, Compression::default());
        measure_time(|| std::io::copy(&mut input, &mut encoder).unwrap());
        let output = encoder.finish().unwrap();
        write_test_file_bytes(&format!("{}.zlib", TEST_FILE_NAME1), &output);
        println!(
            "{} -> {} ({}%)",
            input_len,
            output.len(),
            output.len() * 100 / input_len
        )
    }

    /// 测试 zlib 解压
    ///
    /// [TEST_FILE_NAME1] 耗时: 1.9125ms
    /// [TEST_FILE_NAME2] 耗时: 4.2778ms
    /// [TEST_FILE_NAME3] 耗时: 76.5786ms
    /// [TEST_FILE_NAME1] 耗时: 11.7878ms
    /// [TEST_FILE_NAME1] 耗时: 142.4359ms
    ///
    /// https://github.com/rust-lang/flate2-rs/blob/main/examples/decompress_file.rs
    #[test]
    fn test_zlib_decompression() {
        use flate2::bufread::GzDecoder;
        use rc_test::*;
        let bytes = read_test_file_bytes(&format!("{}.zlib", TEST_FILE_NAME5), true);
        let input = bytes.as_slice();
        let mut decoder = GzDecoder::new(input);
        let mut decompressed = Vec::new();
        measure_time(|| std::io::copy(&mut decoder, &mut decompressed).unwrap());
        write_test_file_bytes(TEST_FILE_NAME5, &decompressed);
    }

    //MARK: - lzma

    /// 测试 LZMA (7-Zip) 压缩
    /// - LZMA (7-Zip)
    ///
    /// [TEST_FILE_NAME1] 耗时: 62.0853ms 93115 -> 85945 (92%)
    /// [TEST_FILE_NAME2] 耗时: 748.6024ms 306828 -> 38227 (12%)
    /// [TEST_FILE_NAME3] 耗时: 15.2975353s 5581188 -> 663511 (11%)
    /// [TEST_FILE_NAME4] 耗时: 5.7839049s 1507233 -> 130309 (8%)
    /// [TEST_FILE_NAME5] 耗时: 71.60947s 17294691 -> 1764380 (10%)
    ///
    /// https://github.com/hasenbanck/lzma-rust2/blob/master/examples/lzma_writer.rs
    #[test]
    fn test_7zip_compression() {
        use lzma_rust2::{LzmaOptions, LzmaWriter};
        use rc_test::*;
        let bytes = read_test_file_bytes(TEST_FILE_NAME5, false);
        let mut input = bytes.as_slice();
        let input_len = input.len();
        let mut compressed = Vec::new();
        let mut writer = LzmaWriter::new_use_header(
            &mut compressed,
            &LzmaOptions::default(),
            Some(input_len as u64),
        )
        .unwrap();
        measure_time(|| std::io::copy(&mut input, &mut writer).unwrap());
        writer.finish().unwrap();
        write_test_file_bytes(&format!("{}.7zip", TEST_FILE_NAME5), &compressed);
        println!(
            "{} -> {} ({}%)",
            input_len,
            compressed.len(),
            compressed.len() * 100 / input_len
        )
    }

    /// 测试 LZMA (7-Zip) 解压
    /// [TEST_FILE_NAME1] 耗时: 9.0969ms
    /// [TEST_FILE_NAME2] 耗时: 10.373ms
    /// [TEST_FILE_NAME3] 耗时: 182.6222ms
    /// [TEST_FILE_NAME4] 耗时: 31.2794ms
    /// [TEST_FILE_NAME5] 耗时: 486.8894ms
    ///
    /// https://github.com/hasenbanck/lzma-rust2/blob/master/examples/lzma_reader.rs
    #[test]
    fn test_7zip_decompression() {
        use lzma_rust2::LzmaReader;
        use rc_test::*;
        use std::io::Read;
        let bytes = read_test_file_bytes(&format!("{}.7zip", TEST_FILE_NAME5), true);
        let input = bytes.as_slice();
        let mut reader = LzmaReader::new_mem_limit(input, u32::MAX, None).unwrap();
        let mut decompressed = Vec::new();
        //measure_time(|| reader.read_to_end(&mut decompressed).unwrap());
        measure_time(|| std::io::copy(&mut reader, &mut decompressed).unwrap());
        write_test_file_bytes(TEST_FILE_NAME5, &decompressed);
    }

    /// 测试 LZMA (7-Zip) 压缩
    /// - LZMA (7-Zip)
    ///
    /// [TEST_FILE_NAME1] 耗时: 62.0853ms 93115 -> 85945 (92%)
    /// [TEST_FILE_NAME2] 耗时: 746.9976ms 306828 -> 38227 (12%)
    /// [TEST_FILE_NAME3] 耗时: 15.2975353s 5581188 -> 663511 (11%)
    /// [TEST_FILE_NAME4] 耗时: 5.7839049s 1507233 -> 130309 (8%)
    /// [TEST_FILE_NAME5] 耗时: 71.2794638s 17294691 -> 1764373 (10%)
    ///
    /// https://github.com/hasenbanck/lzma-rust2/blob/master/examples/lzip_writer.rs
    #[test]
    fn test_lzip_compression() {
        use lzma_rust2::{LzipOptions, LzipWriter};
        use rc_test::*;
        let bytes = read_test_file_bytes(TEST_FILE_NAME1, false);
        let mut input = bytes.as_slice();
        let input_len = input.len();
        let mut compressed = Vec::new();
        let mut writer = LzipWriter::new(&mut compressed, LzipOptions::default());
        measure_time(|| std::io::copy(&mut input, &mut writer).unwrap());
        writer.finish().unwrap();
        write_test_file_bytes(&format!("{}.lzip", TEST_FILE_NAME1), &compressed);
        println!(
            "{} -> {} ({}%)",
            input_len,
            compressed.len(),
            compressed.len() * 100 / input_len
        )
    }

    /// 测试 LZMA (7-Zip) 解压
    ///
    /// https://github.com/hasenbanck/lzma-rust2/blob/master/examples/lzip_reader.rs
    #[test]
    fn test_lzip_decompression() {
        use lzma_rust2::LzipReader;
        use rc_test::*;
        use std::io::Read;
        let bytes = read_test_file_bytes(&format!("{}.lzip", TEST_FILE_NAME4), true);
        let input = bytes.as_slice();
        let mut reader = LzipReader::new(input);
        let mut decompressed = Vec::new();
        //measure_time(|| reader.read_to_end(&mut decompressed).unwrap());
        measure_time(|| std::io::copy(&mut reader, &mut decompressed).unwrap());
        write_test_file_bytes(TEST_FILE_NAME4, &decompressed);
    }

    /// 测试 LZMA (7-Zip) 压缩
    /// - LZMA (7-Zip)
    ///
    /// [TEST_FILE_NAME1] 耗时: 49.5575ms 93115 -> 65517 (70%)
    /// [TEST_FILE_NAME2] 耗时: 738.2204ms 306828 -> 0 (0%)
    /// [TEST_FILE_NAME3] 耗时: 1.1948406s 5581188 -> 65517 (1%)
    /// [TEST_FILE_NAME4] 耗时: 2.5475552s 1507233 -> 65518 (4%)
    /// [TEST_FILE_NAME5] 耗时: 57.6427953s 17294691 -> 1375844 (7%)
    ///
    /// https://github.com/hasenbanck/lzma-rust2/blob/master/examples/lzma2_writer.rs
    #[test]
    fn test_7zip_compression2() {
        use lzma_rust2::{Lzma2Options, Lzma2Writer};
        use rc_test::*;
        let bytes = read_test_file_bytes(TEST_FILE_NAME2, false);
        let mut input = bytes.as_slice();
        let input_len = input.len();
        let mut compressed = Vec::new();
        let mut writer = Lzma2Writer::new(&mut compressed, Lzma2Options::default());
        measure_time(|| std::io::copy(&mut input, &mut writer).unwrap());
        writer.finish().unwrap();
        write_test_file_bytes(&format!("{}.7zip2", TEST_FILE_NAME2), &compressed);
        println!(
            "{} -> {} ({}%)",
            input_len,
            compressed.len(),
            compressed.len() * 100 / input_len
        )
    }

    /// 测试 LZMA (7-Zip) 解压
    ///
    /// https://github.com/hasenbanck/lzma-rust2/blob/master/examples/lzma2_reader.rs
    #[test]
    fn test_7zip_decompression2() {
        use lzma_rust2::{Lzma2Reader, LzmaOptions};
        use rc_test::*;
        use std::io::Read;
        let bytes = read_test_file_bytes(&format!("{}.7zip2", TEST_FILE_NAME4), true);
        let input = bytes.as_slice();
        let mut reader = Lzma2Reader::new(input, LzmaOptions::DICT_SIZE_DEFAULT, None);
        let mut decompressed = Vec::new();
        //measure_time(|| reader.read_to_end(&mut decompressed).unwrap());
        measure_time(|| std::io::copy(&mut reader, &mut decompressed).unwrap());
        write_test_file_bytes(TEST_FILE_NAME4, &decompressed);
    }
}
