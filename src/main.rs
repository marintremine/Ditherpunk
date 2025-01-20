use argh::FromArgs;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::{Rgb, RgbImage};

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {

    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}


#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {

    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
}

fn main() {
    let args: DitherArgs = argh::from_env();
    let path_in = args.input;
    println!("path_in: {}", path_in);
    let path_out = args.output.unwrap_or("output/out.png".to_string());
    println!("path_out: {}", path_out);
    // sandbox
    
    let img = match ImageReader::open(path_in) {
        Ok(reader) => reader.decode(),
        Err(err) => {
            eprintln!("Failed to open image: {}", err);
            return;
        }
    };

    // Convertir l'image en mode RGB8
    let mut rgb_image = img.unwrap().to_rgb8();

    // Parcourir tous les pixels de l'image
    for (x, y, pixel) in rgb_image.enumerate_pixels_mut() {
        // Passer un pixel sur deux en blanc
        if (x + y) % 2 == 0 {
            *pixel = Rgb([255, 255, 255]);
        }
    }

    // Sauvegarder l'image convertie
    rgb_image.save(path_out).unwrap();

    //rgb_image.save("output_image.png")?;
    println!("L'image a été convertie en mode RGB8 avec succès.");

    // Accéder au pixel à la position (32, 52)
    let pixel = rgb_image.get_pixel(32, 51);

    // Afficher la couleur du pixel dans le terminal
    println!("La couleur du pixel (32, 52) est : {:?}", pixel);


}
