import sys
from pathlib import Path

import cv2 as cv

IMAGE_PATH = str(
    Path(sys.path[0]) / ".." / "images" / "Game_Boy_Camera_Pokémon_sticker.png"
)

img = cv.imread(IMAGE_PATH)
print(img.shape)
img = img.reshape(img.shape[1], img.shape[0], 3)
print(img.shape)
with open("src/image.rs", "w") as f:
    f.write("pub const IMAGE:[(u8,u8,u8);23040] = [\n")
    for elem in img.reshape(img.shape[0] * img.shape[1], 3).tolist():
        elem = tuple(elem)
        f.write(f"{elem},\n")
    f.write("];")
