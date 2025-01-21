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


Question 2 : Ouvrir une image depuis un fichier

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

Question 3 : Sauver l'image au format PNG

Pour sauver l'image obtenue au format PNG, vous pouvez utiliser la méthode save :

```rust
rgb_image.save("output.png")?;
```

Si l'image de départ avait un canal alpha (par exemple, une image en RGBA), la conversion en RGB8 via to_rgb8() va ignorer le canal alpha et ne conserver que les canaux rouge, vert et bleu. Cela signifie que toute information de transparence sera perdue dans l'image convertie.

Question 4 : Afficher dans le terminal la couleur du pixel (32, 52)

```rust
println!("La couleur du pixel (32, 52) est : {:?}", pixel);
```

Question 5 : Passer un pixel sur deux d’une image en blanc

Oui, l'image est reconnaissable.

```rust
// Parcourir tous les pixels de l'image
for (x, y, pixel) in rgb_image.enumerate_pixels_mut() {
    // Passer un pixel sur deux en blanc
    if (x + y) % 2 == 0 {
        *pixel = Rgb([255, 255, 255]);
    }
}
```

Question 6 : Récupérer la luminosité d’un pixel

Pour trouver le calcul, nous avons été sur google pour...

```rust
// Récupérer la luminosité d’un pixel
    let luminosite = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
    println!("La luminosité du pixel (32, 52) est : {:?}", luminosite);
```

Question 7 : Implémenter le traitement

On parcourt chaque pixel et on regarde si sa luminosité est supérieure à 50% (128.0), on le remplace par du blanc; inférieure, on le remplace par du noir.

```rust
// Parcourir tous les pixels de l'image
for (x, y, pixel) in rgb_image.enumerate_pixels_mut() {
    let luminosite = 0.2126 * (pixel[0] as f32) + 0.7152 * (pixel[1] as f32) + 0.0722 * (pixel[2] as f32);
    // Passer un pixel sur deux en blanc si sa luminosité est supérieure à 50%
    if luminosite > 128.0 {
        *pixel = Rgb([255, 255, 255]);
    }
    else {
        *pixel = Rgb([0, 0, 0]);
    }
}
```

Question 8 : Permettre à l’utilisateurice de remplacer “noir” et “blanc” par une paire de couleurs au choix.

expliquer question 6