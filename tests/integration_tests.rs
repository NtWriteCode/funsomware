/// Integration tests for funsomware
/// Tests the full file encryption pipeline, config sanity, and cross-module interactions

use std::fs;
use std::path::PathBuf;

// ============================================================
// Helper: replicate the crypto module for integration testing
// (since we can't import private functions from the binary crate)
// ============================================================

fn expand_key(password: &[u8], length: usize) -> Vec<u8> {
    let mut key = Vec::with_capacity(length);
    let mut state: u64 = 0xcbf29ce484222325;
    for i in 0..length {
        state ^= password[i % password.len()] as u64;
        state = state.wrapping_mul(0x100000001b3);
        state = state.rotate_left(13) ^ (state >> 7);
        key.push((state & 0xFF) as u8);
    }
    key
}

fn xor_cipher(data: &mut [u8], password: &[u8]) {
    let keystream = expand_key(password, data.len());
    for (byte, key_byte) in data.iter_mut().zip(keystream.iter()) {
        *byte ^= key_byte;
    }
}

/// Create a temporary directory for test files
fn create_test_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("funsomware_test_{}", name));
    let _ = fs::remove_dir_all(&dir); // clean up from previous runs
    fs::create_dir_all(&dir).expect("Failed to create test directory");
    dir
}

/// Clean up a test directory
fn cleanup_test_dir(dir: &PathBuf) {
    let _ = fs::remove_dir_all(dir);
}

// ============================================================
// FILE ENCRYPTION PIPELINE TESTS
// ============================================================

#[test]
fn test_file_encrypt_and_decrypt_roundtrip() {
    let dir = create_test_dir("roundtrip");
    let file_path = dir.join("testfile.txt");
    let password = b"MySecretPassword123!";
    let original_content = b"This is the original file content that should survive encryption and decryption.";
    
    // Write original file
    fs::write(&file_path, original_content).unwrap();
    
    // Read, encrypt, write back (simulates process_file)
    let mut data = fs::read(&file_path).unwrap();
    xor_cipher(&mut data, password);
    fs::write(&file_path, &data).unwrap();
    
    // Verify the file on disk is now encrypted (different from original)
    let encrypted_on_disk = fs::read(&file_path).unwrap();
    assert_ne!(encrypted_on_disk, original_content.to_vec(), "File on disk should be encrypted");
    assert_eq!(encrypted_on_disk.len(), original_content.len(), "Encrypted file should be same size");
    
    // Read, decrypt, write back (simulates running the tool again)
    let mut data = fs::read(&file_path).unwrap();
    xor_cipher(&mut data, password);
    fs::write(&file_path, &data).unwrap();
    
    // Verify the file is restored
    let restored = fs::read(&file_path).unwrap();
    assert_eq!(restored, original_content.to_vec(), "File should be restored after decryption");
    
    cleanup_test_dir(&dir);
}

#[test]
fn test_multiple_files_encrypted_independently() {
    let dir = create_test_dir("multifile");
    let password = b"TestPassword";
    
    let file1_content = b"File one content";
    let file2_content = b"File two content - different!";
    let file3_content = b"Third file with yet another content";
    
    let file1 = dir.join("file1.txt");
    let file2 = dir.join("file2.dat");
    let file3 = dir.join("file3.bin");
    
    fs::write(&file1, file1_content).unwrap();
    fs::write(&file2, file2_content).unwrap();
    fs::write(&file3, file3_content).unwrap();
    
    // Encrypt all files
    for path in [&file1, &file2, &file3] {
        let mut data = fs::read(path).unwrap();
        xor_cipher(&mut data, password);
        fs::write(path, &data).unwrap();
    }
    
    // Verify all files are encrypted
    assert_ne!(fs::read(&file1).unwrap(), file1_content.to_vec());
    assert_ne!(fs::read(&file2).unwrap(), file2_content.to_vec());
    assert_ne!(fs::read(&file3).unwrap(), file3_content.to_vec());
    
    // Decrypt all files
    for path in [&file1, &file2, &file3] {
        let mut data = fs::read(path).unwrap();
        xor_cipher(&mut data, password);
        fs::write(path, &data).unwrap();
    }
    
    // Verify all restored
    assert_eq!(fs::read(&file1).unwrap(), file1_content.to_vec());
    assert_eq!(fs::read(&file2).unwrap(), file2_content.to_vec());
    assert_eq!(fs::read(&file3).unwrap(), file3_content.to_vec());
    
    cleanup_test_dir(&dir);
}

#[test]
fn test_binary_file_roundtrip() {
    let dir = create_test_dir("binary");
    let file_path = dir.join("binary.dat");
    let password = b"BinaryTestKey";
    
    // Create a binary file with all 256 byte values repeated
    let original_content: Vec<u8> = (0..=255).cycle().take(10_000).collect();
    fs::write(&file_path, &original_content).unwrap();
    
    // Encrypt
    let mut data = fs::read(&file_path).unwrap();
    xor_cipher(&mut data, password);
    fs::write(&file_path, &data).unwrap();
    
    // Verify encrypted
    assert_ne!(fs::read(&file_path).unwrap(), original_content);
    
    // Decrypt
    let mut data = fs::read(&file_path).unwrap();
    xor_cipher(&mut data, password);
    fs::write(&file_path, &data).unwrap();
    
    // Verify restored
    assert_eq!(fs::read(&file_path).unwrap(), original_content);
    
    cleanup_test_dir(&dir);
}

#[test]
fn test_empty_file_encryption() {
    let dir = create_test_dir("empty");
    let file_path = dir.join("empty.txt");
    let password = b"password";
    
    // Create empty file
    fs::write(&file_path, b"").unwrap();
    
    // Encrypt (should not crash)
    let mut data = fs::read(&file_path).unwrap();
    xor_cipher(&mut data, password);
    fs::write(&file_path, &data).unwrap();
    
    // Should still be empty
    assert_eq!(fs::read(&file_path).unwrap().len(), 0);
    
    cleanup_test_dir(&dir);
}

#[test]
fn test_large_file_roundtrip() {
    let dir = create_test_dir("largefile");
    let file_path = dir.join("large.bin");
    let password = b"MySecretPassword123!";
    
    // Create a 5MB file with pseudo-random content
    let size = 5 * 1024 * 1024;
    let original_content: Vec<u8> = (0..size).map(|i| ((i * 7 + 13) % 256) as u8).collect();
    fs::write(&file_path, &original_content).unwrap();
    
    // Encrypt
    let mut data = fs::read(&file_path).unwrap();
    xor_cipher(&mut data, password);
    fs::write(&file_path, &data).unwrap();
    
    let encrypted = fs::read(&file_path).unwrap();
    assert_ne!(encrypted, original_content, "Large file should be encrypted");
    assert_eq!(encrypted.len(), original_content.len(), "Size must be preserved");
    
    // Decrypt
    let mut data = fs::read(&file_path).unwrap();
    xor_cipher(&mut data, password);
    fs::write(&file_path, &data).unwrap();
    
    assert_eq!(fs::read(&file_path).unwrap(), original_content, "Large file must roundtrip");
    
    cleanup_test_dir(&dir);
}

#[test]
fn test_subdirectory_files() {
    let dir = create_test_dir("subdir");
    let sub1 = dir.join("documents");
    let sub2 = dir.join("documents").join("important");
    fs::create_dir_all(&sub2).unwrap();
    
    let password = b"SubdirTest";
    let file1 = dir.join("root.txt");
    let file2 = sub1.join("doc.txt");
    let file3 = sub2.join("secret.txt");
    
    let content1 = b"Root level file";
    let content2 = b"First subdirectory file";
    let content3 = b"Deeply nested file";
    
    fs::write(&file1, content1).unwrap();
    fs::write(&file2, content2).unwrap();
    fs::write(&file3, content3).unwrap();
    
    // Simulate walkdir + encryption
    let files: Vec<PathBuf> = walkdir::WalkDir::new(&dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();
    
    assert_eq!(files.len(), 3, "Should find 3 files across subdirectories");
    
    // Encrypt all
    for path in &files {
        let mut data = fs::read(path).unwrap();
        xor_cipher(&mut data, password);
        fs::write(path, &data).unwrap();
    }
    
    // Decrypt all
    for path in &files {
        let mut data = fs::read(path).unwrap();
        xor_cipher(&mut data, password);
        fs::write(path, &data).unwrap();
    }
    
    // Verify all restored
    assert_eq!(fs::read(&file1).unwrap(), content1.to_vec());
    assert_eq!(fs::read(&file2).unwrap(), content2.to_vec());
    assert_eq!(fs::read(&file3).unwrap(), content3.to_vec());
    
    cleanup_test_dir(&dir);
}

// ============================================================
// CRYPTO EDGE CASE TESTS
// ============================================================

#[test]
fn test_double_encryption_is_not_original() {
    let password = b"password";
    let original = b"double encrypt test".to_vec();
    let mut data = original.clone();
    
    // Encrypt once
    xor_cipher(&mut data, password);
    let single_encrypted = data.clone();
    
    // Encrypt again (not decrypt!)
    xor_cipher(&mut data, password);
    
    // Double encryption should produce original (XOR property: a ^ b ^ b = a)
    assert_eq!(data, original, "XOR double-encryption must equal original");
    assert_ne!(single_encrypted, original, "Single encryption must differ");
}

#[test]
fn test_encryption_changes_every_byte_for_zeroes() {
    let password = b"test_key";
    let mut data = vec![0u8; 1000];
    
    xor_cipher(&mut data, password);
    
    // With all-zero input, encrypted output should be the raw keystream
    // Verify it's not all zeroes anymore
    let zero_count = data.iter().filter(|&&b| b == 0).count();
    assert!(zero_count < 100, "Encrypting zeroes should produce mostly non-zero bytes, got {} zeroes", zero_count);
}

// ============================================================
// CONFIG SANITY TESTS
// ============================================================

#[test]
fn test_password_is_not_empty() {
    // The config password should never be empty, or encryption does nothing useful
    assert!(!funsomware::config_password().is_empty(), "PASSWORD must not be empty");
}

#[test]
fn test_password_minimum_length() {
    // A password shorter than 8 chars is too weak even for fun
    assert!(funsomware::config_password().len() >= 8, "PASSWORD should be at least 8 characters");
}

#[test]
fn test_thread_count_is_sane() {
    let tc = funsomware::config_thread_count();
    assert!(tc > 0, "THREAD_COUNT must be > 0");
    assert!(tc <= 1024, "THREAD_COUNT should be reasonable (<=1024)");
}
