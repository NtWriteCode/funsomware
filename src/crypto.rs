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
        
        // Decrypt (same operation since XOR is symmetric)
        xor_cipher(&mut data, password);
        assert_eq!(data, original, "Data should be decrypted back to original");
    }

    #[test]
    fn test_empty_data() {
        let password = b"password";
        let mut data: Vec<u8> = Vec::new();
        
        // Should not panic on empty data
        xor_cipher(&mut data, password);
        assert!(data.is_empty());
    }

    #[test]
    fn test_single_byte() {
        let password = b"key";
        let mut data = vec![0x42];
        let original = data.clone();
        
        xor_cipher(&mut data, password);
        assert_ne!(data, original, "Single byte should be encrypted");
        
        xor_cipher(&mut data, password);
        assert_eq!(data, original, "Single byte should decrypt back");
    }

    #[test]
    fn test_large_data() {
        let password = b"MySecretPassword123!";
        let mut data: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
        let original = data.clone();
        
        xor_cipher(&mut data, password);
        assert_ne!(data, original, "Large data should be encrypted");
        
        xor_cipher(&mut data, password);
        assert_eq!(data, original, "Large data should decrypt back");
    }

    #[test]
    fn test_preserves_length() {
        let password = b"password";
        let mut data = b"Some data of specific length".to_vec();
        let original_len = data.len();
        
        xor_cipher(&mut data, password);
        assert_eq!(data.len(), original_len, "Encryption must not change data length");
        
        xor_cipher(&mut data, password);
        assert_eq!(data.len(), original_len, "Decryption must not change data length");
    }

    #[test]
    fn test_different_passwords_produce_different_output() {
        let mut data1 = b"Hello, World!".to_vec();
        let mut data2 = data1.clone();
        
        xor_cipher(&mut data1, b"password_one");
        xor_cipher(&mut data2, b"password_two");
        
        assert_ne!(data1, data2, "Different passwords must produce different ciphertext");
    }

    #[test]
    fn test_wrong_password_does_not_decrypt() {
        let original = b"Sensitive data here".to_vec();
        let mut data = original.clone();
        
        xor_cipher(&mut data, b"correct_password");
        xor_cipher(&mut data, b"wrong_password");
        
        assert_ne!(data, original, "Wrong password must not recover original data");
    }

    #[test]
    fn test_deterministic() {
        let password = b"deterministic_key";
        let mut data1 = b"Same input data".to_vec();
        let mut data2 = data1.clone();
        
        xor_cipher(&mut data1, password);
        xor_cipher(&mut data2, password);
        
        assert_eq!(data1, data2, "Same input + same key must produce same output");
    }

    #[test]
    fn test_all_byte_values() {
        let password = b"test";
        let mut data: Vec<u8> = (0..=255).collect();
        let original = data.clone();
        
        xor_cipher(&mut data, password);
        assert_ne!(data, original);
        
        xor_cipher(&mut data, password);
        assert_eq!(data, original, "All 256 byte values must round-trip correctly");
    }

    #[test]
    fn test_key_expansion_deterministic() {
        let key1 = expand_key(b"password", 100);
        let key2 = expand_key(b"password", 100);
        
        assert_eq!(key1, key2, "Key expansion must be deterministic");
    }

    #[test]
    fn test_key_expansion_length() {
        let key = expand_key(b"password", 5000);
        assert_eq!(key.len(), 5000, "Expanded key must match requested length");
    }

    #[test]
    fn test_key_expansion_different_passwords() {
        let key1 = expand_key(b"alpha", 64);
        let key2 = expand_key(b"bravo", 64);
        
        assert_ne!(key1, key2, "Different passwords must produce different keystreams");
    }

    #[test]
    fn test_binary_data() {
        let password = b"binkey";
        let mut data: Vec<u8> = vec![0x00, 0xFF, 0x00, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF];
        let original = data.clone();
        
        xor_cipher(&mut data, password);
        assert_ne!(data, original);
        
        xor_cipher(&mut data, password);
        assert_eq!(data, original, "Binary data must round-trip correctly");
    }

    #[test]
    fn test_password_longer_than_data() {
        let password = b"this_is_a_very_long_password_that_exceeds_data_length";
        let mut data = b"short".to_vec();
        let original = data.clone();
        
        xor_cipher(&mut data, password);
        assert_ne!(data, original);
        
        xor_cipher(&mut data, password);
        assert_eq!(data, original, "Password longer than data must still work");
    }

    #[test]
    fn test_config_password_roundtrip() {
        // Test with the actual config password to catch config issues
        let password = crate::config::PASSWORD.as_bytes();
        let mut data = b"Test with the real configured password".to_vec();
        let original = data.clone();
        
        xor_cipher(&mut data, password);
        assert_ne!(data, original);
        
        xor_cipher(&mut data, password);
        assert_eq!(data, original);
    }
}
