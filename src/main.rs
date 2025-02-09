mod utils;
use std::str;
use std::str::FromStr;

use argh::FromArgs;
//use image::io::Reader as ImageReader;
//use image::DynamicImage;
//use image::{Rgb, RgbImage};

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
    mode: Mode,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
    Dithering(OptsDithering),
    DiffussionErreur(OptsDiffusionErreur),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {
    /// la couleur 1 personnalisée (optionnelle)
    #[argh(option)]
    couleur_1: Option<String>,

    /// la couleur 2 personnalisée (optionnelle)
    #[argh(option)]
    couleur_2: Option<String>,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {

    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "diffusion-erreur")]
/// Rendu de l’image par diffusion d’erreur.
struct OptsDiffusionErreur {
    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize,
    /// la matrice de diffusion d’erreur à utiliser
    #[argh(option)]
    matrice: MatriceDiffusionErreur,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatriceDiffusionErreur {
    Simple2D,
    FloydSteinberg,
    JarvisJudiceNinke,
    Atkinson,
}

// Implémentation de FromStr pour Enum
impl FromStr for MatriceDiffusionErreur {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "simple2d" => Ok(MatriceDiffusionErreur::Simple2D),
            "floydsteinberg" => Ok(MatriceDiffusionErreur::FloydSteinberg),
            "jarvisjudiceninke" => Ok(MatriceDiffusionErreur::JarvisJudiceNinke),
            "atkinson" => Ok(MatriceDiffusionErreur::Atkinson),
            _ => Err(format!("Matrice de diffusion d'erreur invalide: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Methode {
    Aleatoire,
    Ordonne,
}

// Implémentation de FromStr pour Enum
impl FromStr for Methode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aleatoire" => Ok(Methode::Aleatoire),
            "ordonne" => Ok(Methode::Ordonne),
            _ => Err(format!("Méthode de dithering invalide: {}", s)),
        }
    }
}



#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "dithering")]
/// Rendu de l'image par dithering.
pub struct OptsDithering {
    /// la méthode de tramage à utiliser
    #[argh(option)]
    tramage: Methode,
}


fn main() {
    let args: DitherArgs = argh::from_env();
    let path_in = args.input;
    println!("path_in: {}", path_in);
    let path_out = args.output.unwrap_or("output/out.png".to_string());
    println!("path_out: {}", path_out);
    
    let mut image_rgb8 = utils::charger_image_rgb8(&path_in); // Question 2

    let pixel = utils::recuperer_pixel(&image_rgb8, 32, 52); // Question 4
    println!("La couleur du pixel (32, 52) est : {:?}", pixel);

    //utils::transformer_pixels_un_sur_deux(&mut image_rgb8); // Question 5

    let luminosite_pixel = utils::luminosite_pixel(&pixel); // Question 6
    println!("La luminosité du pixel (32, 52) est : {:?}", luminosite_pixel);

    //utils::monochrome_par_seuillage(&mut image_rgb8); // Question 7

    match &args.mode {
        Mode::Seuil(opts_seuil) => {
            let couleurs = utils::creer_liste_couleurs();
            let couleur_1_rgb = if let Some(couleur) = &opts_seuil.couleur_1 {
                utils::obtenir_couleur_par_nom(couleur, &couleurs)
            }
            else {
                utils::obtenir_couleur_par_nom("blanc", &couleurs) // valeur par défaut
            };
            println!("La couleur 1 est : {:?}", couleur_1_rgb);

            let couleur_2_rgb = if let Some(couleur) = &opts_seuil.couleur_2 {
                utils::obtenir_couleur_par_nom(couleur, &couleurs)
            }
            else {
                utils::obtenir_couleur_par_nom("noir", &couleurs) // valeur par défaut
            };
            println!("La couleur 2 est : {:?}", couleur_2_rgb);

            utils::monochrome_par_seuillage(&mut image_rgb8, couleur_1_rgb, couleur_2_rgb); // Question 8
        },
        Mode::Palette(opts_palette) => {
            println!("Mode palette avec {} couleurs", opts_palette.n_couleurs);

            let couleurs = utils::creer_liste_couleurs();

            if opts_palette.n_couleurs > couleurs.len() {
                eprintln!(
                    "Erreur : Le nombre de couleurs demandé ({}) dépasse le nombre total de couleurs disponibles ({}).",
                    opts_palette.n_couleurs,
                    couleurs.len()
                );
                std::process::exit(1);
            }

            let mut couleurs_palette = vec![];
            for i in 0..opts_palette.n_couleurs {
                couleurs_palette.push(couleurs[i].1);
            }
            println!("Les couleurs de la palette sont : {:?}", couleurs_palette);

            utils::monochrome_par_palette(&mut image_rgb8, couleurs_palette); // Question 10
        },
        Mode::Dithering(_opts_dithering) => {
           
            println!("Mode dithering");
            match _opts_dithering.tramage {
                Methode::Aleatoire => {
                    println!("Méthode de dithering : Aleatoire");
                    utils::tramage_aleatoire(&mut image_rgb8); // Question 12
                },
                Methode::Ordonne => {
                    println!("Méthode de dithering : Ordonne");
                    let matrice = utils::generer_matrice_bayer(2);
                    utils::afficher_matrice(&matrice);
                    utils::tramage_ordonne(&mut image_rgb8, &matrice); // Question 13
                },
            }
        }
        Mode::DiffussionErreur(opts_diffusion_erreur) => {
            println!("Mode diffusion d'erreur");
            let couleurs = utils::creer_liste_couleurs();

            if opts_diffusion_erreur.n_couleurs > couleurs.len() {
                eprintln!(
                    "Erreur : Le nombre de couleurs demandé ({}) dépasse le nombre total de couleurs disponibles ({}).",
                    opts_diffusion_erreur.n_couleurs,
                    couleurs.len()
                );
                std::process::exit(1);
            }

            let mut couleurs_palette = vec![];
            for i in 0..opts_diffusion_erreur.n_couleurs {
                couleurs_palette.push(couleurs[i].1);
            }
            println!("Les couleurs de la palette sont : {:?}", couleurs_palette);
            
            let matrice = match opts_diffusion_erreur.matrice {
                MatriceDiffusionErreur::Simple2D => utils::simple_2_d(),
                MatriceDiffusionErreur::FloydSteinberg => utils::floyd_steinberg(),
                MatriceDiffusionErreur::JarvisJudiceNinke => utils::jarvis_judice_ninke(),
                MatriceDiffusionErreur::Atkinson => utils::atkinson(),
            };

            println!("Matrice de diffusion d'erreur : {:?}", opts_diffusion_erreur.matrice);
            utils::afficher_matrice(&matrice);
            //utils::diffusion_erreur(&mut image_rgb8); //question 16
            utils::diffusion_erreur_generique(&mut image_rgb8, couleurs_palette, matrice); // Question 20
        }
    }

    utils::sauvegarder_image_rgb8(&image_rgb8, &path_out); // Question 3
}
