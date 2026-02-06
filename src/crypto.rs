/// Expands a password into a keystream of the specified length
/// using an FNV-1a-inspired mixing function for fast, position-dependent bytes
fn expand_key(password: &[u8], length: usize) -> Vec<u8> {
    let mut key = Vec::with_capacity(length);
    let mut state: u64 = 0xcbf29ce484222325; // FNV offset basis
    
    for i in 0..length {
        state ^= password[i % password.len()] as u64;
        state = state.wrapping_mul(0x100000001b3); // FNV prime
        state = state.rotate_left(13) ^ (state >> 7);
        key.push((state & 0xFF) as u8);
    }
    
    key
}

/// XOR cipher that encrypts/decrypts data in place using the password
/// This is symmetric - the same function encrypts and decrypts
pub fn xor_cipher(data: &mut [u8], password: &[u8]) {
    let keystream = expand_key(password, data.len());
    
    for (byte, key_byte) in data.iter_mut().zip(keystream.iter()) {
        *byte ^= key_byte;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_encryption() {
        let password = b"test_password";
        let mut data = b"Hello, World! This is a test.".to_vec();
        let original = data.clone();
        
        // Encrypt
        xor_cipher(&mut data, password);
        assert_ne!(data, original, "Data should be encrypted");
        
        // Decrypt
        xor_cipher(&mut data, password);
        assert_eq!(data, original, "Data should be decrypted back to original");
    }
}
