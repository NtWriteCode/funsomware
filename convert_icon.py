#!/usr/bin/env python3
"""
Convert icon.png to icon.ico for Windows executable
"""
from PIL import Image

# Open the PNG
img = Image.open('rsrc/icon.png')

# Convert to RGBA if not already
img = img.convert('RGBA')

# Save as ICO with multiple sizes (Windows standard)
# ICO format supports multiple resolutions in one file
img.save('rsrc/icon.ico', format='ICO', sizes=[(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])

print("✓ Converted rsrc/icon.png to rsrc/icon.ico")
