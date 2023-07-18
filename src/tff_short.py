import fontforge
import itertools

# Create a new font
font = fontforge.font()

# Define the block size and grid size
grid_size = 3

# Set glyph boundaries
glyph_top = 800
glyph_bottom = -200
glyph_left = 0
glyph_right = 1000

# Compute the block size to fill the character space
block_size = (glyph_right - glyph_left) // grid_size

# Generate all possible combinations of blocks in a 3x3 grid
for i in range(33, 2**(grid_size*grid_size)):
    # If more than 5 bits are set in i, skip this combination
    if bin(i).count("1") > 4: # Skip the "0b" prefix in the binary representation
        continue

    # Create a new glyph with Unicode value i
    glyph = font.createChar(i)

    # Set the glyph width
    glyph.width = glyph_right

    # Create a pen to draw the glyph
    pen = glyph.glyphPen()

    # Create the contours for the glyph
    for j in range(grid_size*grid_size):
        # If the j-th bit of i is set, fill the corresponding cell
        if (i & (1 << j)) != 0:
            x = glyph_left + (j % grid_size) * block_size
            y = glyph_top - ((j // grid_size) * block_size)  # Reverse the rows to match the bit order
            pen.moveTo((x, y))
            pen.lineTo((x + block_size, y))
            pen.lineTo((x + block_size, y - block_size))
            pen.lineTo((x, y - block_size))
            pen.closePath()

# Generate the TTF file
font.generate("text_short.ttf")
