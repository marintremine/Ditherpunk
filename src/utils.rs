use image::io::Reader as ImageReader;
use image::{Pixel, Rgb, RgbImage};
use rand::Rng;


/// Lit une image à partir d'un chemin et la convertit en mode RGB8
pub fn charger_image_rgb8(path: &str) -> RgbImage {
    match ImageReader::open(path) {
        Ok(reader) => match reader.decode() {
            Ok(img) => img.to_rgb8(),
            Err(err) => {
                eprintln!("Erreur lors de la conversion de l'image : {}", err);
                std::process::exit(1); // Quitte le programme avec un code d'erreur
                
                // Return a default RgbImage to satisfy the return type
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

pub fn monochrome_par_palette(image_rgb8: &mut RgbImage, couleurs_palette: Vec<Rgb<u8>>) {
    // Parcourir tous les pixels de l'image
    for (_x, _y, pixel) in image_rgb8.enumerate_pixels_mut() {
        let mut distance_min = std::f32::MAX;
        let mut couleur_plus_proche = *pixel;
        for couleur in &couleurs_palette {
            let distance = distance_couleurs(pixel, couleur);
            if distance < distance_min {
                distance_min = distance;
                couleur_plus_proche = *couleur;
            }
        }
        // Appliquer la couleur la plus proche au pixel correspondant dans l'image monochrome
        *pixel = couleur_plus_proche;
    }
}

/// Calculer la distance euclidienne entre deux couleurs RGB
pub fn distance_couleurs(couleur1: &Rgb<u8>, couleur2: &Rgb<u8>) -> f32 {
    let r_diff = couleur1[0] as f32 - couleur2[0] as f32;
    let g_diff = couleur1[1] as f32 - couleur2[1] as f32;
    let b_diff = couleur1[2] as f32 - couleur2[2] as f32;

    (r_diff.powi(2) + g_diff.powi(2) + b_diff.powi(2)).sqrt()
}

/// Créer une liste avec des couleurs principales
pub fn creer_liste_couleurs() -> Vec<(&'static str, Rgb<u8>)> {
    vec![
        ("noir", Rgb([0, 0, 0])),
        ("blanc", Rgb([255, 255, 255])),
        ("rouge", Rgb([255, 0, 0])),
        ("vert", Rgb([0, 255, 0])),
        ("bleu", Rgb([0, 0, 255])),
        ("jaune", Rgb([255, 255, 0])),
        ("magenta", Rgb([255, 0, 255])),
        ("cyan", Rgb([0, 255, 255])),
    ]
}

/// Obtenir une couleur par son nom à partir de la liste
pub fn obtenir_couleur_par_nom(nom: &str, liste_couleurs: &Vec<(&'static str, Rgb<u8>)>) -> Rgb<u8> {
    for (nom_couleur, rgb) in liste_couleurs {
        if *nom_couleur == nom {
            return *rgb;
        }
    }
    eprintln!("Erreur : La couleur '{}' n'est pas disponible.", nom);
    std::process::exit(1); // Quitte le programme avec un code d'erreur
}

/// Appliquer un tramage aléatoire sur une image RGB8
pub fn tramage_aléatoire(image_rgb8: &mut RgbImage) {
    for (_x, _y, pixel) in image_rgb8.enumerate_pixels_mut() {
        let luminosite = luminosite_pixel(pixel);
        let seuil: f32 = rand::thread_rng().gen();
        if luminosite / 255.0 > seuil {
            *pixel = Rgb([255, 255, 255]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    }
}
