# tile-images

Utility tool for tiling a collection of images in a list


### Usage

```
tile-images [OPTIONS] [IMAGE]...
```

### Options

* `-c, --columns <columns>     [default: 0]`
* `-o, --output <output>       [default: output.png]`
* `-r, --rows <rows>           [default: 0]`
* `-s, --scale <scale>         [default: 1.0]`


## Examples

### Tile images in a square (default)

```
tile-images image1.png image2.png ...
```

### Tile 3 images horizontally

```
tile-images -c3 image1.png image2.png image3.png
```

### Tile 3 images vertically

```
tile-images -r3 image1.png image2.png image3.png
```

### Tile 6 images in a 3x2 grid

```
tile-images -c3 image11.png image21.png image31.png image12.png image22.png image32.png
```

