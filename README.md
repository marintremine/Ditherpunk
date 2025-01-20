# Ditherpunk - Application console

TODO : Description

## Installation

TODO 

## Développeurs

- Quentin BOURREAU
- Marin TREMINE

## Fonctionnalités

TODO

## Utilisation

ditherpunk [OPTIONS] <INPUT> <OUTPUT> <SUBCOMMAND>

ditherpunk.exe -- --help
```bash
Convertit une image en monochrome ou vers une palette réduite de couleurs.

Positional Arguments:
  input             le fichier d’entrée
  output            le fichier de sortie (optionnel)

Options:
  --help, help      display usage information

Commands:
  seuil             Rendu de l’image par seuillage monochrome.  
  palette           Rendu de l’image avec une palette contenant un nombre limité de couleurs
```
Exemple 
    
```bash
cargo run -- input.png seuil
```

```bash
cargo run -- input_image.png output_image.png palette --n-couleurs 4
```


Question 1 :
Ouvrir une image depuis un fichier
Pour ouvrir une image depuis un fichier, vous pouvez utiliser la méthode suivante :

```rust
use image::io::Reader as ImageReader;
use image::DynamicImage;

let img = ImageReader::open("image.png")?.decode()?;
```

DynamicImage
Le type DynamicImage est une énumération de la crate image qui peut représenter différentes formes d'images. Il peut contenir des images en niveaux de gris, en RGB, en RGBA, etc. Cela permet de manipuler des images sans se soucier de leur format interne initial.

Convertir en RGB8
Pour convertir une DynamicImage en une image en mode RGB8, vous pouvez utiliser la méthode to_rgb8() :

```rust
let rgb_image = img.to_rgb8();
```

Question 2 :

Sauver l'image au format PNG
Pour sauver l'image obtenue au format PNG, vous pouvez utiliser la méthode save :

```rust
rgb_image.save("output.png")?;
```

Si l'image de départ avait un canal alpha (par exemple, une image en RGBA), la conversion en RGB8 via to_rgb8() va ignorer le canal alpha et ne conserver que les canaux rouge, vert et bleu. Cela signifie que toute information de transparence sera perdue dans l'image convertie.