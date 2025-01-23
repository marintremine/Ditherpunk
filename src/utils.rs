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

// // Fonction de seuillage monochrome
// pub fn monochrome_par_seuillage(image_rgb8: &mut RgbImage) {
//     // Parcourir tous les pixels de l'image
//     for (_x, _y, pixel) in image_rgb8.enumerate_pixels_mut() {
//         // Calculer la luminosité du pixel
//         let luminosite = luminosite_pixel(pixel);
        
//         // Si la luminosité est supérieure à 128 (seuillage à 50%), couleur blanc, sinon couleur noir
//         if luminosite > 128.0 {
//             *pixel = Rgb([255, 255, 255]);
//         }
//         else {
//             *pixel = Rgb([0, 0, 0]);
//         }
//     }
// }

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

/// Calculer la distance euclidienne entre deux couleurs RGB
pub fn distance_couleurs(couleur1: &Rgb<u8>, couleur2: &Rgb<u8>) -> f32 {
    let r_diff = couleur1[0] as f32 - couleur2[0] as f32;
    let g_diff = couleur1[1] as f32 - couleur2[1] as f32;
    let b_diff = couleur1[2] as f32 - couleur2[2] as f32;

    (r_diff.powi(2) + g_diff.powi(2) + b_diff.powi(2)).sqrt()
}

// Fonction de palette monochrome
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

/// Appliquer un tramage aléatoire sur une image RGB8
pub fn tramage_aleatoire(image_rgb8: &mut RgbImage) {
    for (_x, _y, pixel) in image_rgb8.enumerate_pixels_mut() {
        let luminosite = luminosite_pixel(pixel);
        let seuil: f32 = rand::thread_rng().gen();
        if luminosite / 255.0 > seuil {
            *pixel = Rgb([255, 255, 255]);
        }
        else {
            *pixel = Rgb([0, 0, 0]);
        }
    }
}

/// Générer une matrice de Bayer de taille 2^order
pub fn generer_matrice_bayer(order: u32) -> Vec<Vec<f32>> {
    if order == 0 {
        return vec![vec![0.0]];
    }

    let matrice_precedente = generer_matrice_bayer(order - 1);
    let taille = matrice_precedente.len();
    let nouvelle_taille = taille * 2;
    let mut matrice = vec![vec![0.0; nouvelle_taille]; nouvelle_taille];

    for i in 0..taille {
        for j in 0..taille {
            let valeur_base = matrice_precedente[i][j] * 4.0;
            matrice[i][j] = valeur_base;
            matrice[i][j + taille] = valeur_base + 2.0;
            matrice[i + taille][j] = valeur_base + 3.0;
            matrice[i + taille][j + taille] = valeur_base + 1.0;
        }
    }

    matrice
}

/// Afficher une matrice d'entiers
pub fn afficher_matrice(matrice: &Vec<Vec<f32>>) {
    for ligne in matrice {
        for valeur in ligne {
            print!("{} ", valeur);
        }
        println!();
    }
}

/// Appliquer un tramage ordonné sur une image RGB8 en utilisant une matrice de Bayer
pub fn tramage_ordonne(image_rgb8: &mut RgbImage, matrice_bayer: &Vec<Vec<f32>>) {
    let taille = matrice_bayer.len();
    for (x, y, pixel) in image_rgb8.enumerate_pixels_mut() {
        let luminosite = luminosite_pixel(pixel);
        let i = x as usize % taille;
        let j = y as usize % taille;
        if luminosite / 255.0 > matrice_bayer[i][j] as f32 / (taille * taille) as f32 {
            *pixel = Rgb([255, 255, 255]);
        }
        else {
            *pixel = Rgb([0, 0, 0]);
        }
    }
}

fn couleur_la_plus_proche(pixel: &Rgb<u8>, couleurs_palette: &Vec<Rgb<u8>>) -> Rgb<u8>{
    let mut distance_min = std::f32::MAX;
    let mut couleur_plus_proche = *pixel;
    for couleur in couleurs_palette {
        let distance = distance_couleurs(pixel, couleur);
        if distance < distance_min {
            distance_min = distance;
            couleur_plus_proche = *couleur;
        }
    }
    couleur_plus_proche
}

pub fn diffusion_erreur(image_rgb8: &mut RgbImage){
    let width = image_rgb8.width() as i32;
    let height = image_rgb8.height() as i32;

    for y in 0..height{
        for x in 0..width{
            let pixel = image_rgb8.get_pixel_mut(x as u32, y as u32);
            let ancien_pixel = *pixel;
            let nouveau_pixel = couleur_la_plus_proche(&ancien_pixel, &vec![Rgb([0, 0, 0]), Rgb([255, 255, 255])]);
            *pixel = nouveau_pixel;
            let erreur = [
                ancien_pixel[0] as f32 - nouveau_pixel[0] as f32,
                ancien_pixel[1] as f32 - nouveau_pixel[1] as f32, 
                ancien_pixel[2] as f32 - nouveau_pixel[2] as f32
            ];

            //pixel de droite
            if x + 1 < width {
                let new_pixel = image_rgb8.get_pixel_mut((x + 1) as u32, y as u32);
                new_pixel[0] = (new_pixel[0] as f32 + erreur[0] * 0.5) as u8;
                new_pixel[1] = (new_pixel[1] as f32 + erreur[1] * 0.5) as u8;
                new_pixel[2] = (new_pixel[2] as f32 + erreur[2] * 0.5) as u8;
            }

            //pixel en dessous
            if y + 1 < height {
                let new_pixel = image_rgb8.get_pixel_mut(x as u32, (y + 1) as u32);
                new_pixel[0] = (new_pixel[0] as f32 + erreur[0] * 0.5) as u8;
                new_pixel[1] = (new_pixel[1] as f32 + erreur[1] * 0.5) as u8;
                new_pixel[2] = (new_pixel[2] as f32 + erreur[2] * 0.5) as u8;
            }
        }
    }
}

pub fn diffusion_erreur_generique(image_rgb8: &mut RgbImage, couleurs_palette: Vec<Rgb<u8>>, matrix: Vec<Vec<f32>>){
    let width = image_rgb8.width() as i32;
    let height = image_rgb8.height() as i32;

    let matrix_height = matrix.len() as i32;
    let matrix_width = matrix[0].len() as i32;

    for y in 0..height {
        for x in 0..width {
            let pixel = image_rgb8.get_pixel_mut(x as u32, y as u32);
            let ancien_pixel = *pixel;
            let nouveau_pixel = couleur_la_plus_proche(&ancien_pixel, &couleurs_palette);
            *pixel = nouveau_pixel;
            let erreur = [
                ancien_pixel[0] as f32 - nouveau_pixel[0] as f32,
                ancien_pixel[1] as f32 - nouveau_pixel[1] as f32, 
                ancien_pixel[2] as f32 - nouveau_pixel[2] as f32
            ];

            for i in 0..matrix_height {
                for j in 0..matrix_width {
                    let new_x = x + j - matrix_width / 2;
                    let new_y = y + i - matrix_height / 2;
                    if new_x >= 0 && new_y >= 0 && new_x < width && new_y < height {
                        let new_pixel = image_rgb8.get_pixel_mut(new_x as u32, new_y as u32);
                        new_pixel[0] = (new_pixel[0] as f32 + erreur[0] * matrix[i as usize][j as usize]) as u8;
                        new_pixel[1] = (new_pixel[1] as f32 + erreur[1] * matrix[i as usize][j as usize]) as u8;
                        new_pixel[2] = (new_pixel[2] as f32 + erreur[2] * matrix[i as usize][j as usize]) as u8;
                    }
                }
            }
        }
    } 
} 

pub fn simple_2_d() -> Vec<Vec<f32>> {
    vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.5],
        vec![0.0, 0.5, 0.0]
    ]
}

pub fn floyd_steinberg() -> Vec<Vec<f32>> {
    vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 7.0 / 16.0],
        vec![3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0]
    ]
}

pub fn jarvis_judice_ninke() -> Vec<Vec<f32>> {
    vec![
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 7.0 / 48.0, 5.0 / 48.0],
        vec![3.0 / 48.0, 5.0 / 48.0, 7.0 / 48.0, 5.0 / 48.0, 3.0 / 48.0],
        vec![1.0 / 48.0, 3.0 / 48.0, 5.0 / 48.0, 3.0 / 48.0, 1.0 / 48.0]
    ]
}

pub fn atkinson() -> Vec<Vec<f32>> {
    vec![
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0 / 8.0, 1.0 / 8.0],
        vec![0.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 0.0],
        vec![0.0, 0.0, 1.0 / 8.0, 0.0, 0.0]
    ]
}
