
fn byte_parity(byte: u8) -> u8 {
    (byte ^
    (byte >> 1) ^
    (byte >> 2) ^
    (byte >> 3) ^
    (byte >> 4) ^
    (byte >> 5) ^
    (byte >> 6) ^
    (byte >> 7))
    & 0b00000001
}

fn shatter8(bytes: Vec<u8>) -> [Vec<u8>; 8] {
    let bit_vec_len = bytes.len()/8;
    assert_eq!(bit_vec_len*8,bytes.len(),"shatter8's input must have a length divisible by 8");
    let mut output = [
        Vec::with_capacity(bit_vec_len),
        Vec::with_capacity(bit_vec_len),
        Vec::with_capacity(bit_vec_len),
        Vec::with_capacity(bit_vec_len),
        Vec::with_capacity(bit_vec_len),
        Vec::with_capacity(bit_vec_len),
        Vec::with_capacity(bit_vec_len),
        Vec::with_capacity(bit_vec_len),
    ];
    for byte_chunk in bytes.chunks(8) {
        let byte_parities =
            byte_parity(byte_chunk[0]) |
            byte_parity(byte_chunk[1]) << 1 |
            byte_parity(byte_chunk[2]) << 2 |
            byte_parity(byte_chunk[3]) << 3 |
            byte_parity(byte_chunk[4]) << 4 |
            byte_parity(byte_chunk[5]) << 5 |
            byte_parity(byte_chunk[6]) << 6 |
            byte_parity(byte_chunk[7]) << 7;
        for bit_num in 0..8 {
            let mut bit_chunk = 0b00000000;
            for (byte_num,byte) in byte_chunk.iter().enumerate() {
                bit_chunk |= ((byte >> bit_num) & 0b00000001) << byte_num;
            }
            output[bit_num].push(byte_parities ^ bit_chunk)
        }
    }
    output
}

fn deshatter8(chunks: [Vec<u8>; 8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(chunks[0].len()*8);
    for i in 0..chunks[0].len() {
        let byte_chunk = [
            chunks[0][i],
            chunks[1][i],
            chunks[2][i],
            chunks[3][i],
            chunks[4][i],
            chunks[5][i],
            chunks[6][i],
            chunks[7][i],
        ];
        let byte_parities = 
            byte_chunk[0] ^
            byte_chunk[1] ^
            byte_chunk[2] ^
            byte_chunk[3] ^
            byte_chunk[4] ^
            byte_chunk[5] ^
            byte_chunk[6] ^
            byte_chunk[7];
        let mut output_byte_chunk = [0; 8];
        for (byte_num,byte) in byte_chunk.iter().enumerate() {
            let bit_chunk = byte ^ byte_parities;
            for bit_num in 0..8 {
                output_byte_chunk[bit_num] |= ((bit_chunk >> bit_num) & 0b00000001) << byte_num
            }
        }
        for byte in output_byte_chunk {
            output.push(byte);
        }
    }
    output
}

// rollsover
fn shift_l(bytes: &[u8],shift_num: usize) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    let real_shift = shift_num % bytes.len();
    for i in real_shift..bytes.len() {
        output.push(bytes[i]);
    }
    for i in 0..real_shift {
        output.push(bytes[i]);
    }
    output
}

// rollsover
fn shift_r(bytes: &[u8],shift_num: usize) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    let real_shift = bytes.len() - (shift_num % bytes.len());
    for i in real_shift..bytes.len() {
        output.push(bytes[i]);
    }
    for i in 0..real_shift {
        output.push(bytes[i]);
    }
    output
}


// rollsover
fn shift_even_l(bytes: &[u8],shift_num: usize) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    let num_even_bytes = bytes.len() - bytes.len()/2;
    let real_shift = shift_num % num_even_bytes;
    for i in 0..bytes.len() {
        if i % 2 == 0 {
            output.push(bytes[(i+2*real_shift)%bytes.len()])
        } else {
            output.push(bytes[i])
        }
    }
    output
}

// rollsover
fn shift_even_r(bytes: &[u8],shift_num: usize) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    let num_even_bytes = bytes.len() - bytes.len()/2;
    let real_shift = num_even_bytes - (shift_num % num_even_bytes);
    for i in 0..bytes.len() {
        if i % 2 == 0 {
            output.push(bytes[(i+2*real_shift)%bytes.len()])
        } else {
            output.push(bytes[i])
        }
    }
    output
}

fn shift8_l(chunks: [Vec<u8>; 8], shift_nums: [usize; 8]) -> [Vec<u8>; 8] {
    [
        shift_even_l(&chunks[0],shift_nums[0]),
        shift_l(&chunks[1],shift_nums[1]),
        shift_l(&chunks[2],shift_nums[2]),
        shift_l(&chunks[3],shift_nums[3]),
        shift_l(&chunks[4],shift_nums[4]),
        shift_l(&chunks[5],shift_nums[5]),
        shift_l(&chunks[6],shift_nums[6]),
        shift_l(&chunks[7],shift_nums[7]),
    ]
}

fn shift8_r(chunks: [Vec<u8>; 8], shift_nums: [usize; 8]) -> [Vec<u8>; 8] {
    [
        shift_even_r(&chunks[0],shift_nums[0]),
        shift_r(&chunks[1],shift_nums[1]),
        shift_r(&chunks[2],shift_nums[2]),
        shift_r(&chunks[3],shift_nums[3]),
        shift_r(&chunks[4],shift_nums[4]),
        shift_r(&chunks[5],shift_nums[5]),
        shift_r(&chunks[6],shift_nums[6]),
        shift_r(&chunks[7],shift_nums[7]),
    ]
}

fn fuse8(chunks: [Vec<u8>; 8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(chunks[0].len()*8);
    for chunk in chunks {
        for byte in chunk {
            output.push(byte);
        }
    }
    output
}

fn defuse8(bytes: Vec<u8>) -> [Vec<u8>; 8] {
    let chunk_len = bytes.len()/8;
    assert_eq!(chunk_len*8,bytes.len(),"defuse8's input must have a length divisible by 8");
    let mut output = [
        Vec::with_capacity(chunk_len),
        Vec::with_capacity(chunk_len),
        Vec::with_capacity(chunk_len),
        Vec::with_capacity(chunk_len),
        Vec::with_capacity(chunk_len),
        Vec::with_capacity(chunk_len),
        Vec::with_capacity(chunk_len),
        Vec::with_capacity(chunk_len),
    ];
    let mut bytes_iter = bytes.iter();
    for chunk in output.iter_mut() {
        for _ in 0..chunk_len {
            chunk.push(*bytes_iter.next().expect("Skibidi Ohio"));
        }
    }
    output
}

fn shatter8_encode(bytes: Vec<u8>,shift_nums: [usize; 8]) -> Vec<u8> {
    fuse8(shift8_l(shatter8(bytes),shift_nums))
}

fn shatter8_decode(bytes: Vec<u8>,shift_nums: [usize; 8]) -> Vec<u8> {
    deshatter8(shift8_r(defuse8(bytes),shift_nums))
}


fn pad(bytes: &mut Vec<u8>, len_align: usize) {
    while bytes.len() % len_align != 0 {
        bytes.push(0);
    }
}

fn encrypt_file(function: String, mut key: Vec<u8>, mut path: std::path::PathBuf) {
    pad(&mut key, 16);
    let byte_key = shatter8_encode(shatter8_encode(key, [1,2,3,4,5,6,7,8]),[8,7,6,5,4,3,2,1]).chunks(8).fold(0, |accum,byte_chunk| {accum ^ u64::from_le_bytes(<[u8;8]>::try_from(byte_chunk).expect("chunk should be exactly 8 long"))}).to_le_bytes().map(|byte| byte as usize);
    let input_path;
    let output_path;
    match &*function {
        "encrypt" => {
            input_path = path.clone();
            path.set_extension("shtr");
            output_path = path;
        }
        "decrypt" => {
            output_path = path.clone();
            path.set_extension("shtr");
            input_path = path;
        }
        _ => panic!("shatter function was not encrypt or decrypt")
    }
    let mut bytes = std::fs::read(input_path).expect("Couldn't read file");
    pad(&mut bytes,16);
    let processed_bytes = match &*function {
        "encrypt" => shatter8_encode(bytes, byte_key),
        "decrypt" => shatter8_decode(bytes, byte_key),
        _ => panic!("shatter function was not encrypt or decrypt")
    };
    std::fs::write(output_path, processed_bytes).expect("Failed to write encrypted data");
}

fn main() {
    let mut inputs = std::env::args();
    let _ = inputs.next(); //fst arg is the command itself
    let function = inputs.next().expect("shatter function not specified (encrypt or decrypt)");
    let key = inputs.next().expect("No key provided").into_bytes();

    for path in inputs {
        let path = std::path::PathBuf::from(path);
        encrypt_file(function.clone(), key.clone(), path);
    }
}
