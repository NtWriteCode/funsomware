#!/usr/bin/env python3
"""
Quick script to generate test files for funsomware.
No external dependencies required.
"""

import os
import random

# Configuration
TARGET_DIR = "/tmp/target"
NUM_FILES = 300
MIN_SIZE_MB = 1
MAX_SIZE_MB = 50

def generate_random_data(size_bytes):
    """Generate random binary data of specified size."""
    # Use os.urandom for better randomness
    return os.urandom(size_bytes)

def create_test_files():
    """Create test files with random sizes between MIN_SIZE_MB and MAX_SIZE_MB."""
    # Create target directory if it doesn't exist
    os.makedirs(TARGET_DIR, exist_ok=True)
    
    print(f"Generating {NUM_FILES} test files in {TARGET_DIR}...")
    print(f"File sizes: {MIN_SIZE_MB}MB - {MAX_SIZE_MB}MB")
    
    total_size = 0
    
    for i in range(NUM_FILES):
        # Random size in MB, convert to bytes
        size_mb = random.randint(MIN_SIZE_MB, MAX_SIZE_MB)
        size_bytes = size_mb * 1024 * 1024
        
        # Generate filename
        filename = f"test_file_{i:04d}_{size_mb}mb.bin"
        filepath = os.path.join(TARGET_DIR, filename)
        
        # Generate and write random data
        print(f"[{i+1}/{NUM_FILES}] Creating {filename} ({size_mb}MB)...", end=" ")
        
        # Write in chunks to avoid memory issues with large files
        chunk_size = 1024 * 1024  # 1MB chunks
        with open(filepath, 'wb') as f:
            remaining = size_bytes
            while remaining > 0:
                chunk = min(chunk_size, remaining)
                f.write(generate_random_data(chunk))
                remaining -= chunk
        
        total_size += size_bytes
        print("✓")
    
    print(f"\nDone! Created {NUM_FILES} files")
    print(f"Total size: {total_size / (1024 * 1024 * 1024):.2f} GB")
    print(f"Location: {TARGET_DIR}")

if __name__ == "__main__":
    create_test_files()
