#!/usr/bin/env python3
"""
Generate a simple wallpaper image for funsomware.
Creates a red background with warning text.
"""

from PIL import Image, ImageDraw, ImageFont
import os

# Configuration
WIDTH = 1920
HEIGHT = 1080
OUTPUT_PATH = "rsrc/wallpaper.png"

# Create image with red background
img = Image.new('RGB', (WIDTH, HEIGHT), color='#8B0000')  # Dark red
draw = ImageDraw.Draw(img)

# Try to use a system font, fallback to default
try:
    # Try common system fonts
    title_font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 120)
    text_font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 60)
    small_font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 40)
except:
    try:
        # Windows fonts
        title_font = ImageFont.truetype("C:\\Windows\\Fonts\\Arial.ttf", 120)
        text_font = ImageFont.truetype("C:\\Windows\\Fonts\\Arial.ttf", 60)
        small_font = ImageFont.truetype("C:\\Windows\\Fonts\\Arial.ttf", 40)
    except:
        # Fallback to default
        title_font = ImageFont.load_default()
        text_font = ImageFont.load_default()
        small_font = ImageFont.load_default()

# Draw warning symbol
symbol_size = 200
symbol_x = WIDTH // 2
symbol_y = 150
draw.regular_polygon((symbol_x, symbol_y, symbol_size), 3, rotation=180, fill='#FFD700', outline='#000000', width=5)
draw.text((symbol_x, symbol_y - 20), "!", font=title_font, fill='#000000', anchor="mm")

# Draw main text
y_offset = 400
texts = [
    ("⚠ YOUR FILES HAVE BEEN ENCRYPTED ⚠", title_font, '#FFFFFF'),
    ("", None, None),  # Spacing
    ("This is a DEMONSTRATION of funsomware", text_font, '#FFD700'),
    ("", None, None),  # Spacing
    ("Your files are encrypted but can be decrypted", small_font, '#FFFFFF'),
    ("by running the program again with the same password.", small_font, '#FFFFFF'),
    ("", None, None),  # Spacing
    ("For educational purposes only! 😈", text_font, '#FFD700'),
]

for text, font, color in texts:
    if text:
        draw.text((WIDTH // 2, y_offset), text, font=font, fill=color, anchor="mm")
    y_offset += 80 if font == title_font else (60 if font == text_font else 50)

# Save image
os.makedirs(os.path.dirname(OUTPUT_PATH), exist_ok=True)
img.save(OUTPUT_PATH)
print(f"Wallpaper created: {OUTPUT_PATH}")
print(f"Size: {WIDTH}x{HEIGHT}")
print(f"File size: {os.path.getsize(OUTPUT_PATH) / 1024:.2f} KB")
