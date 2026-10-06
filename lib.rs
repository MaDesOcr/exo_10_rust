//! Exercice 10 — Lecture de fichier et gestion des erreurs d'entrée/sortie.

use std::fs;
use std::io;

#[derive(Debug, PartialEq, Default)]
pub struct Statistiques {
    pub lignes: usize,
    pub mots: usize,
    pub caracteres: usize,
}

/// Compte lignes, mots et caractères (Unicode) d'un texte.
pub fn compter(texte: &str) -> Statistiques {
    Statistiques {
        lignes: texte.lines().count(),
        mots: texte.split_whitespace().count(),
        caracteres: texte.chars().count(),
    }
}

/// Lit le fichier puis calcule ses statistiques ; l'erreur d'E/S est propagée avec ?.
pub fn analyser_fichier(chemin: &str) -> Result<Statistiques, io::Error> {
    let contenu = fs::read_to_string(chemin)?;
    Ok(compter(&contenu))
}

/// Mot le plus long du texte (le premier en cas d'égalité), None si texte vide.
pub fn mot_le_plus_long(texte: &str) -> Option<&str> {
    let mut meilleur: Option<&str> = None;
    for mot in texte.split_whitespace() {
        match meilleur {
            Some(m) if m.chars().count() >= mot.chars().count() => {}
            _ => meilleur = Some(mot),
        }
    }
    meilleur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comptage() {
        let s = compter("Bonjour le monde\nRust c'est chouette\n");
        assert_eq!(
            s,
            Statistiques {
                lignes: 2,
                mots: 6,
                caracteres: 37
            }
        );
        assert_eq!(compter(""), Statistiques::default());
    }

    #[test]
    fn fichier_existant() {
        let chemin = std::env::temp_dir().join("ex10_test_rust.txt");
        std::fs::write(&chemin, "un deux\ntrois").unwrap();
        let stats = analyser_fichier(chemin.to_str().unwrap()).unwrap();
        assert_eq!(stats.lignes, 2);
        assert_eq!(stats.mots, 3);
        std::fs::remove_file(chemin).ok();
    }

    #[test]
    fn fichier_absent() {
        let erreur = analyser_fichier("/chemin/qui/n/existe/pas.txt").unwrap_err();
        assert_eq!(erreur.kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn plus_long() {
        assert_eq!(mot_le_plus_long("le chat mange"), Some("mange"));
        assert_eq!(mot_le_plus_long("   "), None);
    }
}
