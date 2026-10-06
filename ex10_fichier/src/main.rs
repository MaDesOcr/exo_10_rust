use ex10_fichier::{analyser_fichier, mot_le_plus_long};
use std::io::ErrorKind;
use std::process;

fn main() {
    // cargo run -p ex10_fichier -- chemin/vers/fichier.txt
    let chemin = match std::env::args().nth(1) {
        Some(c) => c,
        None => {
            eprintln!("Usage : ex10_fichier <fichier>");
            process::exit(2);
        }
    };

    match analyser_fichier(&chemin) {
        Ok(stats) => {
            println!("Fichier    : {chemin}");
            println!("Lignes     : {}", stats.lignes);
            println!("Mots       : {}", stats.mots);
            println!("Caractères : {}", stats.caracteres);
            // Relire pour le mot le plus long : exemple d'expect assumé (le fichier vient d'être lu)
            let texte = std::fs::read_to_string(&chemin).expect("fichier lisible juste avant");
            if let Some(mot) = mot_le_plus_long(&texte) {
                println!("Mot le plus long : {mot}");
            }
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            eprintln!("Erreur : le fichier « {chemin} » est introuvable.");
            process::exit(1);
        }
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            eprintln!("Erreur : accès refusé à « {chemin} ».");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Erreur inattendue : {e}");
            process::exit(1);
        }
    }
}
