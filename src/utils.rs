use image::io::Reader as ImageReader;
use image::{Rgb, RgbImage};
use std::collections::HashMap;

/// Lit une image à partir d'un chemin et la convertit en mode RGB8
pub fn charger_image_rgb8(path: &str) -> RgbImage {
    match ImageReader::open(path) {
        Ok(reader) => match reader.decode() {
            Ok(img) => img.to_rgb8(),
            Err(err) => {
                eprintln!("Erreur lors de la conversion de l'image : {}", err);
                std::process::exit(1); // Quitte le programme avec un code d'erreur
            }
        },
        Err(err) => {
            eprintln!("Erreur lors de l'ouverture du fichier : {}", err);
            std::process::exit(1); // Quitte le programme avec un code d'erreur
        }
    }
}

/// Sauvegarder une image RGB8 dans un fichier au format PNG
pub fn sauvegarder_image_rgb8(image_rgb8: &RgbImage, path_out: &str) {
    match image_rgb8.save(path_out) {
        Ok(_) => println!("Image sauvegardée avec succès à l'emplacement : {}", path_out),
        Err(err) => {
            eprintln!("Erreur lors de la sauvegarde de l'image : {}", err);
            std::process::exit(1); // Quitte le programme avec un code d'erreur
        }
    }
}

/// Récupérer un pixel à partir de ses coordonnées
pub fn recuperer_pixel(image_rgb8: &RgbImage, x: u32, y: u32) -> Rgb<u8> {
    // Vérifie si les coordonnées sont valides
    if x < image_rgb8.width() && y < image_rgb8.height() {
        *image_rgb8.get_pixel(x, y)
    } 
    else {
        eprintln!("Erreur de récupération du pixel car les coordonnées sont hors des limites de l'image");
        std::process::exit(1); // Quitte le programme avec un code d'erreur
    }
}

/// Passer un pixel sur deux en blanc dans une image RGB8
pub fn transformer_pixels_un_sur_deux(image_rgb8: &mut RgbImage) {
    for (x, y, pixel) in image_rgb8.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = Rgb([255, 255, 255]); // pixel en blanc
        }
    }
}

/// Récupérer la luminosité d’un pixel
pub fn luminosite_pixel(pixel: &Rgb<u8>) -> f32 {
    0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32
}

// Fonction de seuillage monochrome
pub fn monochrome_par_seuillage(image_rgb8: &mut RgbImage, couleur_1: Rgb<u8>, couleur_2: Rgb<u8>) {
    // Parcourir tous les pixels de l'image
    for (_x, _y, pixel) in image_rgb8.enumerate_pixels_mut() {
        // Calculer la luminosité du pixel
        let luminosite = luminosite_pixel(pixel);
        
        // Si la luminosité est supérieure à 128 (seuillage à 50%), appliquer couleur_1, sinon couleur_2
        if luminosite > 128.0 {
            *pixel = couleur_1;
        }
        else {
            *pixel = couleur_2;
        }
    }
}

/// Créer un dictionnaire avec des couleurs principales
pub fn creer_dictionnaire_couleurs() -> HashMap<String, Rgb<u8>> {
    let mut couleurs = HashMap::new();
    couleurs.insert("rouge".to_string(), Rgb([255, 0, 0]));
    couleurs.insert("vert".to_string(), Rgb([0, 255, 0]));
    couleurs.insert("bleu".to_string(), Rgb([0, 0, 255]));
    couleurs.insert("jaune".to_string(), Rgb([255, 255, 0]));
    couleurs.insert("cyan".to_string(), Rgb([0, 255, 255]));
    couleurs.insert("magenta".to_string(), Rgb([255, 0, 255]));
    couleurs.insert("noir".to_string(), Rgb([0, 0, 0]));
    couleurs.insert("blanc".to_string(), Rgb([255, 255, 255]));
    
    couleurs
}

/// Convertir le nom de la couleur en une valeur `Rgb`
/// Si la couleur n'existe pas dans le dictionnaire, cela quitte le programme avec une erreur
pub fn obtenir_couleur_par_nom(couleur: &str, couleurs: &HashMap<String, Rgb<u8>>) -> Rgb<u8> {
    match couleurs.get(&couleur.to_lowercase()) {
        Some(couleur_rgb) => *couleur_rgb, // Retourne la couleur trouvée
        None => {
            eprintln!("Erreur : La couleur '{}' n'est pas valide", couleur);
            std::process::exit(1); // Quitte le programme avec un code d'erreur
        }
    }
}
