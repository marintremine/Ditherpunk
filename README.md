# Ditherpunk - Application console

Le sujet porte sur la réalisation d'une application en ligne de commande en Rust pour transformer des images, par exemple en versions monochromes ou avec des palettes de couleurs. L'application utilise la bibliothèque Rust image pour manipuler les images et argh pour créer une interface utilisateur via la ligne de commande.

## Développeurs

- **TREMINE Marin**: [GitHub](https://github.com/marintremine)
- **BOURREAU Quentin**: [GitHub](https://github.com/BOURREAUQuentin)

## Réponses aux questions

### Question 2 : Ouvrir une image depuis un fichier

Pour ouvrir une image depuis un fichier, nous avons utilisé la méthode suivante :

```rust
use image::io::Reader as ImageReader;
use image::DynamicImage;

let img = ImageReader::open("image.png")?.decode()?;
```

DynamicImage

Le type DynamicImage est une énumération de la crate image qui peut représenter différentes formes d'images. Il peut contenir des images en niveaux de gris, en RGB, en RGBA, etc. Cela permet de manipuler des images sans se soucier de leur format interne initial.

Convertir en RGB8

Pour convertir une DynamicImage en une image en mode RGB8, nous avons utilisé la méthode to_rgb8() :

```rust
let rgb_image = img.to_rgb8();
```

### Question 3 : Sauvegarder une image

Nous avons choisi de créer un fichier utils.rs qui centralise toutes les fonctions utilitaires nécessaires à la manipulation et à la transformation des images, ce qui permet de rendre le code plus modulaire et réutilisable, en séparant clairement les différentes opérations logiques. Cela améliore également la lisibilité pour les réponses aux différentes questions par la suite.

Pour sauver l'image obtenue au format PNG, nous avons utilisé la méthode save :

```rust
rgb_image.save("output.png")?;
```

Ainsi, nous avons créé la fonction :

```rust
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

utils::sauvegarder_image_rgb8(&image_rgb8, &path_out); // Question 3
```

Si l'image d'entrée contient un canal alpha (mode RGBA), la conversion en RGB8 supprime ce canal. La transparence est perdue, et les zones transparentes deviennent opaques, affichant leur couleur RGB. L'image sauvegardée au format PNG est entièrement opaque, car le type RgbImage ne prend pas en charge la transparence. Cela signifie que toute information de transparence sera perdue dans l'image convertie. Ainsi, si l'on souhaite garder le canal alpha, il faudrait garder l'image sans conversion en RGB8.

### Question 4 : Affichage de la couleur du pixel (32,52)

Pour afficher la couleur d'un pixel, nous utilisons simplement la fonction **get_pixel** accessible à partir de l'image au format RGB8. Nous avons donc créé cette fonction générique prennant l'image et les coordonnées du pixel que l'on souhaite récupérer la couleur : 

```rust
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

let pixel = utils::recuperer_pixel(&image_rgb8, 32, 52); // Question 4
println!("La couleur du pixel (32, 52) est : {:?}", pixel);
```

### Question 5 : Pixels alternés en blanc

Nous avons implémenté la fonction **transformer_pixels_un_sur_deux** qui remplace un pixel sur deux par du blanc (RGB = [255, 255, 255]). L'image obtenue reste globalement reconnaissable, mais son apparence est fortement altérée, surtout pour les images avec beaucoup de détails :

```rust
/// Passer un pixel sur deux en blanc dans une image RGB8
pub fn transformer_pixels_un_sur_deux(image_rgb8: &mut RgbImage) {
    for (x, y, pixel) in image_rgb8.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = Rgb([255, 255, 255]); // pixel en blanc
        }
    }
}

utils::transformer_pixels_un_sur_deux(&mut image_rgb8); // Question 5
```

### Question 6 : Récupérer la luminosité d’un pixel

Pour trouver le calcul, nous avons été sur google pour trouver le calcul de la luminosité d'un pixel. Nous avons implémenté la fonction **luminosite_pixel** qui calcule la luminosité perçue d'un pixel en utilisant une pondération des composantes RGB basée sur leur contribution relative à la perception humaine :

Voici notre formule utilisée : ![alt text](formule_luminosite.png)

```rust
/// Récupérer la luminosité d’un pixel
pub fn luminosite_pixel(pixel: &Rgb<u8>) -> f32 {
    0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32
}
```

### Question 7 : Implémenter le traitement

On parcourt chaque pixel et on regarde si sa luminosité est supérieure à 50% (128.0), on le remplace par du blanc; inférieure, on le remplace par du noir. Nous avons donc créé la fonction suivante :

```rust
// Fonction de seuillage monochrome
pub fn monochrome_par_seuillage(image_rgb8: &mut RgbImage) {
    // Parcourir tous les pixels de l'image
    for (_x, _y, pixel) in image_rgb8.enumerate_pixels_mut() {
        // Calculer la luminosité du pixel
        let luminosite = luminosite_pixel(pixel);
        
        // Si la luminosité est supérieure à 128 (seuillage à 50%), mettre en blanc, sinon en noir
        if luminosite > 128.0 {
            *pixel = Rgb([255, 255, 255]);
        }
        else {
            *pixel = Rgb([0, 0, 0]);
        }
    }
}
```

### Question 8 : Permettre à l’utilisateurice de remplacer “noir” et “blanc” par une paire de couleurs au choix.

Cette fonctionnalité permet à l'utilisateur ou l'utilisatrice de personnaliser les couleurs utilisées dans le processus de seuillage monochrome. Par défaut, les couleurs sont définies comme "noir" et "blanc" si l'utilisateur ne saisit aucune option de couleur. Cependant, l'utilisateur·rice peut désormais spécifier une paire de couleurs personnalisées via les options en ligne de commande suivant les couleurs prédéfinis dans la palette : 

**[NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]**

Nous avons géré les erreurs :
- Dans le cas où l'utilisateur ne passe aucune couleur, cela passe les couleurs par défaut noir (couleur 1) et blanc (couleur 2).
- Dans le cas où l'utilisateur passe la couleur 1 mais pas la couleur 2, cela va prendre la couleur 1 et la couleur 2 va être celle par défaut (soit noir).
- Dans le cas où l'utilisateur passe la couleur 2 mais pas la couleur 1, cela va prendre la couleur 2 et la couleur 1 va être celle par défaut (soit blanc).
- Dans le cas où l'utilisateur passe une couleur n'existant pas parmi la palette, cela retourne une erreur.

Dans un premier temps, nous avons modifié le struct **OptsSeuil** pour inclure 2 options de personnalisation dans les arguments : 
- couleur_1: La première couleur personnalisée (optionnelle).
- couleur_2: La deuxième couleur personnalisée (optionnelle).

```rust
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
```

Ensuite, nous avons créé une fonction **creer_liste_couleurs** pour créer la liste de couleurs qui génère une liste fixe de couleurs principales associées à leur nom. Chaque couleur est définie à l'aide d'une chaîne de caractères (le nom de la couleur) et d'une valeur RGB correspondante (triplet [R, G, B]). Nous avons choisi de pas faire en sorte que l'utilisateur saisissse directement les couleurs en hexadécimal et utiliser par la suite un convertisseur en RGB pour simplifier l'utilisation pour l'utilisateur et car le mode palette allait avoir besoin de ces couleurs par la suite.

```rust
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
```

Nous avons créé aussi une fonction **obtenir_couleur_par_nom** qui permet de rechercher et d'obtenir la valeur RGB d'une couleur à partir de son nom. Elle parcourt une liste de couleurs prédéfinies et renvoie la valeur correspondante si le nom est trouvé. Si la couleur n'existe pas, la fonction affiche un message d'erreur et termine le programme avec un code d'échec.

```rust
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
```

Puis, nous avons créé la fonction principale qui fait le traitement **monochrome_par_seuillage** et qui applique un seuillage monochrome à une image en fonction de la luminosité des pixels. Chaque pixel de l'image est remplacé par l'une des deux couleurs spécifiées en fonction de sa luminosité. Cette transformation crée une image binaire où chaque pixel est soit de la couleur couleur_1, soit de la couleur couleur_2.

```rust
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
```

Enfin, dans main.rs, nous avons dû gérer suivant le mode d'entrée (Seuil ou Palette) gérer dans le mode **Seuil** la récupération des couleurs saisies par l'utilisateur et faire le traitement :

```rust
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
        }
}
```

### Question 9 : Calculer la distance entre deux couleurs

Pour mesurer la différence entre deux couleurs en représentation RGB, nous utilisons la distance euclidienne. Cette méthode permet de quantifier à quel point deux couleurs sont similaires ou différentes en les considérant comme des points dans un espace tridimensionnel (rouge, vert, bleu).

La distance euclidienne est une mesure courante pour calculer la différence entre deux points dans un espace à plusieurs dimensions. Dans le cas des couleurs RGB, chaque composante (rouge, vert, bleu) correspond à une dimension.

Notre formule utilisée est : ![alt text](formule_distance_2_couleurs.png)

- R1​,G1​,B1​ sont les composantes RGB de la première couleur.
- R2,G2,B2​ sont les composantes RGB de la deuxième couleur.
- d est la distance entre les deux couleurs.


### Question 10 : Implémenter le traitement

Pour implémenter le traitement, nous avons implémenté la fonction suivante utilisant la formule :

```rust
/// Calculer la distance euclidienne entre deux couleurs RGB
pub fn distance_couleurs(couleur1: &Rgb<u8>, couleur2: &Rgb<u8>) -> f32 {
    let r_diff = couleur1[0] as f32 - couleur2[0] as f32;
    let g_diff = couleur1[1] as f32 - couleur2[1] as f32;
    let b_diff = couleur1[2] as f32 - couleur2[2] as f32;

    (r_diff.powi(2) + g_diff.powi(2) + b_diff.powi(2)).sqrt()
}
```

Ensuite, nous avons fait la fonction **monochrome_par_palette** qui transforme une image en une version monochrome basée sur une palette de couleurs donnée.

```rust
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
```

Puis, nous gérons l'entrée de l'utilisateur sur le nombre de couleurs à utiliser dans main.rs. La fonction **creer_liste_couleurs** génère une liste de couleurs. Nous contruisons ensuite une palette avec n_couleurs (saisi par l'utilisateur) couleurs suivant la palette d'origine. Puis, nous appliquons le mode monochrome avec la palette :

```rust
let couleurs = utils::creer_liste_couleurs();
let mut couleurs_palette = vec![];
for i in 0..opts_palette.n_couleurs {
    couleurs_palette.push(couleurs[i].1);
}
println!("Les couleurs de la palette sont : {:?}", couleurs_palette);

utils::monochrome_par_palette(&mut image_rgb8, couleurs_palette); // Question 9
```

### Question 11 : Cas d'une palette vide ou palette trop importante

Si l'on passe une palette vide (l'option n_couleurs = 0), nous avons choisi de renvoyer l'image classique car nous ne voulions pas renvoyer une erreur à l'utilisateur car ce n'en est pas réellement une.

Néanmoins, pour le cas où l'utilisateur a saisi un nombre de couleurs trop important par rapport à la taille de la palette (soit supérieur à 8 actuellement mais on pourrait modifier la taille de la palette), nous avons décidé de renvoyer une erreur à l'utilisateur :

```rust
let couleurs = utils::creer_liste_couleurs();

if opts_palette.n_couleurs > couleurs.len() {
    eprintln!(
        "Erreur : Le nombre de couleurs demandé ({}) dépasse le nombre total de couleurs disponibles ({}).",
        opts_palette.n_couleurs,
        couleurs.len()
    );
    std::process::exit(1);
}
```

### Question 12 : Implémenter le tramage aléatoire des images

Nous avons commencé par créer une fonction **tramage_aleatoire** qui applique un tramage aléatoire (ou "random dithering") sur une image au format RgbImage. Elle transforme chaque pixel en noir ou blanc en fonction de sa luminosité et d'un seuil aléatoire.

```rust
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
```

Ensuite au niveau du main.rs, nous avons dû gérer le nouveau mode Dithering. Nous avons donc ajouté un mode Dithering pour permettre le rendu d'images en utilisant différentes techniques de tramage. Ce mode offre une flexibilité supplémentaire en permettant de choisir parmi des méthodes de tramage (actuellement que aléatoire mais nous prévoyons les questions d'après).

```rust
#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
    Dithering(OptsDithering), // ajouté
}
```

Nous avons ensuite défini des options pour ce dithering avec le choix d'une méthode de tramage à l'aide de l'option --tramage. La méthode choisie est spécifiée par un nouvel enum Methode que nous avons créé. Nous avons ajouté également implémenter **FromStr** pour **Methode** ce qui permet à l'utilisateur de spécifier une méthode de dithering sous forme de texte (via la ligne de commande ou du code), et d’obtenir la valeur correspondante de l’énumération Methode.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Methode {
    Aleatoire,
}

// Implémentation de FromStr pour Enum
impl FromStr for Methode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aleatoire" => Ok(Methode::Aleatoire),
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
```

Puis, nous avons ensuite géré dans le nouveau mode dans la fonction principale pour appliquer le dithering aléatoire :

```rust
Mode::Dithering(_opts_dithering) => {
            println!("Mode dithering");
            match _opts_dithering.tramage {
                Methode::Aleatoire => {
                    println!("Méthode de dithering : Aleatoire");
                    utils::tramage_aleatoire(&mut image_rgb8); // Question 12
                },
            }
        }
```

### Question 13 : Déterminer 𝐵3

Grâce à notre fonction implémentée **generer_matrice_bayer** qui génère une matrice de Bayer d'ordre 2^order. Une matrice de Bayer est utilisée dans les algorithmes de tramage (dithering), notamment pour réduire la profondeur de couleur dans les images tout en minimisant les artefacts visuels. Puis, notre fonction **afficher_matrice** qui prend une matrice (un vecteur de vecteurs) en paramètre et affiche son contenu dans la console sous forme de grille.

```rust
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

let matrice = utils::generer_matrice_bayer(3);
utils::afficher_matrice(&matrice);
```

Ainsi, nous obtenons B3 = ![alt text](b3.png)

### Question 14 : Quel type de données utiliser pour représenter la matrice de Bayer? Comment créer une matrice de Bayer d’ordre arbitraire?

**Quel type de données utiliser pour représenter la matrice de Bayer?**

Nous avons représenté la matrice de Bayer par un vecteur de vecteurs (Vec<Vec<f32>>).

- Vec : Ce type est dynamique, ce qui permet nous de gérer des tailles arbitraires de matrice sans connaître à l'avance leur dimension.
- f32 : Les valeurs de la matrice sont des nombres en virgule flottante sur 32 bits, ce qui est adapté pour notre traitement, car cela permet de représenter des indices avec une plus grande précision ou pour des calculs nécessitant des fractions. Même si les matrices utilisaient uniquement des entiers, on a décidé d'utiliser des flottants pour nous simplifier la suite du projet.

**Comment créer une matrice de Bayer d’ordre arbitraire?**

Comme montré précédemment, nous avons implémenté la méthode avec la fonction récursive **generer_matrice_bayer**.

- Cas de base (order == 0) :
    - Une matrice 1×1 contenant uniquement la valeur 0 est retournée.

- Récursion :
    - Si order > 0, la fonction appelle generer_matrice_bayer(order - 1) pour générer une matrice de l'ordre précédent.

    - Expansion de la matrice
        - À partir de la matrice précédente, nous contruisons une nouvelle matrice en la dupliquant et en appliquant les règles suivantes pour les 4 quadrants :
            - Quadrant supérieur gauche : valeurs multipliées par 4.
            - Quadrant supérieur droit : valeurs multipliées par 4 + 2.
            - Quadrant inférieur gauche : valeurs multipliées par 4 + 3.
            - Quadrant inférieur droit : valeurs multipliées par 4 + 1.

- Retour de la matrice complète
    - Une fois remplie, nous retournons la matrice.

### Question 15 : Implémenter le tramage par matrice de Bayer.

Pour implémenter le tramage par la matrice de Bayer, une fois avoir les fonctions de génération de la matrice de Bayer, il suffisait juste de créer une fonction **tramage_ordonne**. La fonction que nous avons implémentée applique donc un tramage ordonné sur une image en utilisant une matrice de Bayer.

```rust
/// Appliquer un tramage ordonné sur une image RGB8 en utilisant une matrice de Bayer
pub fn tramage_ordonne(image_rgb8: &mut RgbImage, matrice_bayer: &Vec<Vec<u32>>) {
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

let matrice = utils::generer_matrice_bayer(2);
utils::afficher_matrice(&matrice);
utils::tramage_ordonne(&mut image_rgb8, &matrice);
```

### Question 16 : Implémenter un mécanisme de diffusion d’erreur suivant la matrice donnée dans le sujet pour les images en noir et blanc

Pour répondre à la question, nous avons modifié la matrice de diffusion d'erreur donnée dans le sujet pour placer l'étoile (le pixel courant) au centre de la matrice. Cette approche a nécessité l’ajout de zéros dans les zones où aucune erreur n’est propagée. Voici notre méthodologie et les étapes suivies :
Représentation de la matrice avec l’étoile au centre

La matrice donnée dans le sujet :
```rust
[ *  0.5 ]
[0.5   0 ]
```

A été modifiée pour aligner l'étoile au centre :
```rust
[ 0.0   0.0   0.0 ]
[ 0.0   *    0.5 ]
[ 0.0  0.5   0.0 ]
```

Cette matrice est représentée en Rust comme suit :
```rust
pub fn simple_2_d() -> Vec<Vec<f32>> {
    vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.5],
        vec![0.0, 0.5, 0.0],
    ]
}
```

Pour appliquer l'erreur sur les pixels voisins en respectant cette matrice :

- Pour chaque pixel, nous déterminons la différence entre la couleur initiale du pixel et la couleur binaire choisie (noir ou blanc).
- Nous parcourons chaque élément de la matrice et appliquons l’erreur pondérée aux pixels voisins en tenant compte de la position centrale de l'étoile.
- Comme l’étoile est au centre de la matrice, les indices des voisins sont calculés en ajoutant les décalages relatifs (par rapport à la position centrale de la matrice).

```rust
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
```


### Question 17 : Pour une palette de couleurs, comment vous représentez l’erreur commise à chaque pixel, comment vous la diffusez

#### Représentation de l’erreur pour chaque pixel

Lors de la conversion d’une image vers une palette de couleurs, nous calculons l’erreur comme suit :

- Chaque pixel de l'image est comparé à la couleur de la palette qui lui est la plus proche.
- Une fois que la couleur la plus proche est déterminée, l'erreur est représentée par la différence entre les valeurs des canaux (R, G, B) du pixel d'origine et de la couleur choisie.
    - Erreur par canal :

        erreur = [
            ancien_pixel[0] as f32 - nouveau_pixel[0] as f32,
            ancien_pixel[1] as f32 - nouveau_pixel[1] as f32,
            ancien_pixel[2] as f32 - nouveau_pixel[2] as f32,
        ];

L’erreur est donc un vecteur de trois composantes flottantes (R, G, B) représentant la différence à répartir sur les pixels voisins.

#### Diffusion de l’erreur

La diffusion d'erreur consiste à redistribuer l'erreur calculée sur les pixels voisins qui n'ont pas encore été traités. Cela permet de corriger l'approximation effectuée lors de la conversion du pixel courant. Voici les étapes détaillées de comment nous diffusons l'erreur :

- Choix de la matrice de diffusion :
    - Une matrice spécifique (par exemple, Floyd-Steinberg, Atkinson, etc.) est utilisée pour répartir l’erreur.

- Application de l'erreur sur les pixels voisins :
    - Pour chaque voisin défini par la matrice, on applique une fraction de l'erreur calculée au pixel concerné :

    ```rust
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
    ```

- Répétition sur l’image :
    - Cette opération est répétée pour chaque pixel de l'image, en parcourant l'image ligne par ligne, afin que l'erreur se propage de manière cohérente.

### Question 18 : Implémenter la diffusion d’erreur pour la palettisation d’images

Pour implémenter la diffusion d'erreur lors de la palettisation d'images, nous avons créé la fonction **diffusion_erreur_generique** dans utils.rs. Cette fonction permet d'appliquer la diffusion d'erreur en utilisant une palette de couleurs spécifiée et une matrice de diffusion choisie :

- Si le nombre demandé dépasse la taille de la palette, un message d’erreur est affiché, et le programme se termine :
```rust
if opts_diffusion_erreur.n_couleurs > couleurs.len() {
    eprintln!(
        "Erreur : Le nombre de couleurs demandé ({}) dépasse le nombre total de couleurs disponibles ({}).",
        opts_diffusion_erreur.n_couleurs,
        couleurs.len()
    );
    std::process::exit(1);
}
```

- La palette est générée à partir des couleurs disponibles :
```rust
let mut couleurs_palette = vec![];
for i in 0..opts_diffusion_erreur.n_couleurs {
    couleurs_palette.push(couleurs[i].1);
}
```

- L'utilisateur sélectionne une matrice de diffusion (ex. Floyd-Steinberg, Atkinson) :
```rust
let matrice = match opts_diffusion_erreur.matrice {
    MatriceDiffusionErreur::Simple2D => utils::simple_2_d(),
    MatriceDiffusionErreur::FloydSteinberg => utils::floyd_steinberg(),
    MatriceDiffusionErreur::JarvisJudiceNinke => utils::jarvis_judice_ninke(),
    MatriceDiffusionErreur::Atkinson => utils::atkinson(),
};
```

- La fonction diffusion_erreur_generique applique la diffusion à l’image en répartissant l’erreur sur les pixels voisins :
```rust
utils::diffusion_erreur_generique(&mut image_rgb8, couleurs_palette, matrice);
```


### Question 19 : Implémenter la diffusion d’erreur pour la matrice de Floyd-Steinberg

Pour implémenter la diffusion d’erreur avec la matrice de Floyd-Steinberg, nous avons suivi une approche similaire à celle de la question précédente, en adaptant la matrice pour que l'étoile (le pixel courant) soit située au centre.
Matrice de Floyd-Steinberg avec l’étoile centrée

La matrice de diffusion d'erreur de Floyd-Steinberg originale :
```rust
[ *  7/16 ]
[3/16 5/16 1/16 ]
```

A été adaptée comme suit pour positionner l’étoile au centre :
```rust
[ 0.0   0.0    0.0 ]
[ 0.0    *    7/16 ]
[3/16  5/16   1/16 ]
```

La représentation en code que nous avons :
```rust
pub fn floyd_steinberg() -> Vec<Vec<f32>> {
    vec![
        vec![0.0, 0.0, 0.0],          // Ligne au-dessus (pas de propagation)
        vec![0.0, 0.0, 7.0 / 16.0],  // Ligne du pixel courant (étoile au centre)
        vec![3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0], // Ligne en dessous
    ]
}
```

Pour appliquer la matrice de Floyd-Steinberg :

- L'erreur est la différence entre la couleur originale du pixel et la couleur de la palette la plus proche.
- L'erreur est répartie sur les pixels voisins en utilisant les coefficients de la matrice.
- Les indices des voisins sont calculés en fonction de la position de l’étoile dans la matrice.

- Nous utilisons enfin la fonction générique **diffusion_erreur_generique**, qui applique la matrice choisie pour répartir l'erreur sur les pixels voisins :
```rust
utils::diffusion_erreur_generique(&mut image_rgb8, couleurs_palette, utils::floyd_steinberg());
```

### Question 20 : Comment représenter une matrice de diffusion d’erreur arbitraire? Permettre de changer de matrice de diffusion d’erreurs, et tester les matrices de diffusion de Jarvis-Judice-Ninke et Atkinson

Pour représenter une matrice de diffusion d'erreur arbitraire, nous avons utilisé un tableau 2D de type Vec<Vec<f32>>. Chaque élément de ce tableau correspond au poids d'erreur propagé à un voisin. L'étoile (pixel courant) est placée au centre de la matrice. Cela simplifie les calculs et rend le système plus extensible.

Jarvis-Judice-Ninke (J-J-N) adaptée avec étoile au centre :
```rust
pub fn jarvis_judice_ninke() -> Vec<Vec<f32>> {
    vec![
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 7.0 / 48.0, 5.0 / 48.0],
        vec![3.0 / 48.0, 5.0 / 48.0, 7.0 / 48.0, 5.0 / 48.0, 3.0 / 48.0],
        vec![1.0 / 48.0, 3.0 / 48.0, 5.0 / 48.0, 3.0 / 48.0, 1.0 / 48.0]
    ]
}
```

Atkinson adaptée avec étoile au centre :
```rust
pub fn atkinson() -> Vec<Vec<f32>> {
    vec![
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0 / 8.0, 1.0 / 8.0],
        vec![0.0, 1.0 / 8.0, 1.0 / 8.0, 1.0 / 8.0, 0.0],
        vec![0.0, 0.0, 1.0 / 8.0, 0.0, 0.0]
    ]
}
```

Pour permettre de changer dynamiquement la matrice de diffusion, nous avons défini une énumération MatriceDiffusionErreur. Cette énumération permet de représenter les différentes matrices disponibles, et leur sélection est réalisée à l'exécution : 
```rust
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
```

Dans la fonction principale, l'utilisateur peut spécifier la matrice souhaitée via une option en ligne de commande. La matrice sélectionnée est ensuite utilisée dans la fonction générique **diffusion_erreur_generique** :
```rust
let matrice = match opts_diffusion_erreur.matrice {
    MatriceDiffusionErreur::Simple2D => utils::simple_2_d(),
    MatriceDiffusionErreur::FloydSteinberg => utils::floyd_steinberg(),
    MatriceDiffusionErreur::JarvisJudiceNinke => utils::jarvis_judice_ninke(),
    MatriceDiffusionErreur::Atkinson => utils::atkinson(),
};

utils::diffusion_erreur_generique(&mut image_rgb8, couleurs_palette, matrice); // Question 20
```

### Question 21 : Donner une spécification de votre interface sous forme d’un projet d’écran d’aide, tel que celui qui sera obtenu par cargo run -- --help

Voici notre spécification de notre interface sous forme d'un projet d'écran d'aide : 

Elle est accessible depuis la commande :

```bash
cargo run -- --help
```

```bash
cargo run [OPTIONS] <INPUT> [OUTPUT] <SUBCOMMAND>

Positional Arguments:
  input              le fichier d’entrée
  output             le fichier de sortie (optionnel, par défaut : 'output/out.png')

Options:
  --help, help       affiche l’aide pour la commande.

Commands:
  seuil              Rendu de l’image par seuillage monochrome.
                     Options :
                       --couleur-1 <STRING>  la couleur 1 personnalisée (optionnelle, par défaut : blanc)
                       --couleur-2 <STRING>  la couleur 2 personnalisée (optionnelle, par défaut : noir)

  palette            Rendu de l’image avec une palette contenant un nombre limité de couleurs.
                     Options :
                       --n-couleurs <NUMBER> le nombre de couleurs à utiliser (obligatoire).

  dithering          Rendu de l’image par dithering.
                     Options :
                       --tramage <aleatoire|ordonne> méthode de tramage (obligatoire, valeurs possibles : aleatoire, ordonne).

  diffusion-erreur   Rendu de l’image par diffusion d’erreur.
                     Options :
                       --n-couleurs <NUMBER> le nombre de couleurs à utiliser dans la palette.
                       --matrice <simple2d|floydsteinberg|jarvisjudiceninke|atkinson> la matrice de diffusion d’erreur à utiliser (obligatoire).

```

#### Exemples d'utilisation

Seuillage monochrome avec couleurs personnalisées
```bash
cargo run -- images/defaut.jpg output/out.jpg seuil --couleur-1 rouge --couleur-2 bleu
```

Réduction à une palette de 4 couleurs
```bash
cargo run -- images/defaut.jpg output/out.jpg palette --n-couleurs 4
```

Dithering avec méthode aléatoire
```bash
cargo run -- images/defaut.jpg output/out.jpg dithering --tramage aleatoire
```

Dithering avec méthode ordonnée
```bash
cargo run -- images/defaut.jpg output/out.jpg dithering --tramage ordonne
```

Diffusion d’erreur avec Floyd-Steinberg et 5 couleurs
```bash
cargo run -- images/defaut.jpg output/out.jpg diffusion-erreur --n-couleurs 5 --matrice floydsteinberg
```

Diffusion d’erreur avec la matrice Atkinson et 8 couleurs
```bash
cargo run -- images/defaut.jpg output/out.jpg diffusion-erreur --n-couleurs 8 --matrice atkinson
```

#### Nos choix

Chaque traitement (seuil, palette, dithering, diffusion d’erreur) est isolé pour clarifier leur usage. Les paramètres pertinents (comme les couleurs, le nombre de couleurs, et la matrice de diffusion) sont associés uniquement aux sous-commandes concernées.


### Question 22 : Déterminer le type Rust correspondant à une sélection d’options fournies par l’utilisateur

Le type Rust que nous avons choisi pour représenter les options de la ligne de commande est défini par la structure principale DitherArgs. Cette structure regroupe toutes les options fournies par l'utilisateur ainsi que les sous-commandes associées, ce qui permet une gestion claire et typée des arguments en ligne de commande.

Voici la définition complète du type :

```rust
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
```

Voici quelques explications pour mieux comprendre la structure de notre projet :

1. DitherArgs
    - La structure principale pour gérer les options de la ligne de commande.
    - Champs :
        - input : Argument positionnel obligatoire représentant le chemin du fichier d'entrée.
        - output : Argument positionnel optionnel pour le fichier de sortie.
        - mode : Un Mode (enum) qui contient les sous-commandes disponibles.

2. Enumération Mode
    - Représente les sous-commandes disponibles :
        - Seuil : Pour le mode de seuillage monochrome.
        - Palette : Pour limiter l'image à une palette de couleurs.
        - Dithering : Pour effectuer un tramage aléatoire ou ordonné.
        - DiffussionErreur : Pour appliquer la diffusion d'erreur avec différentes matrices.

3. Sous-structures (OptsSeuil, OptsPalette, OptsDithering, OptsDiffusionErreur)
    - Chaque sous-commande a ses options spécifiques :
        - OptsSeuil :
            - Deux couleurs personnalisées, facultatives.
        - OptsPalette :
            - Nombre de couleurs à inclure dans la palette (obligatoire).
        - OptsDithering :
            - Méthode de tramage (aléatoire ou ordonné).
        - OptsDiffusionErreur :
            - Nombre de couleurs à utiliser et matrice de diffusion d'erreur sélectionnée.

4. Enumérations (Methode, MatriceDiffusionErreur)
    - Methode :
        - Représente les deux approches possibles pour le tramage : Aleatoire et Ordonne.
    - MatriceDiffusionErreur :
        - Contient les matrices disponibles : Simple2D, FloydSteinberg, JarvisJudiceNinke, et Atkinson.

### Question 23 : Implémenter votre interface en ligne de commande à l’aide de la directive #[derive(FromArgs)] sur votre type

Nous avons mis en place l'interface en ligne de commande tout au long du projet à l'aide de la bibliothèque argh. Grâce à la directive **#[derive(FromArgs)]**, nous avons défini un type Rust structurant les options et les sous-commandes disponibles pour l'utilisateur comme expliqué précédemment à la question 21 et 22.
