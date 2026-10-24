# Le blog d'Eros

Site écrit en Rust avec Yew, compilé en WebAssembly.

## Les fichiers du projet

| Fichier | Rôle |
|---|---|
| `Cargo.toml` | La fiche du projet Rust et ses dépendances (Yew). |
| `index.html` | La page de départ que Trunk complète avec le programme Rust. |
| `style.css` | Les couleurs, les polices et la mise en page. |
| `src/main.rs` | Le point d'entrée : il lance l'application dans la page. |
| `src/lib.rs` | Il range le code en deux modules et rend `App` disponible. |
| `src/composants.rs` | Les morceaux de la page : en-tête, couverture, étapes, chapitres, catégorie 2, pied de page. |
| `src/histoire.rs` | Tout le texte du blog : la fiche d'accueil et les chapitres. |
| `images/` | Les photos. |

## Remplacer une photo provisoire

Chaque photo provisoire affiche son nom de fichier. Remplace le fichier dans `images/` par ta vraie photo, en gardant exactement le même nom (par exemple `chapitre-1.jpg`). Il n'y a rien à changer dans le code.

## Modifier le texte ou ajouter un chapitre

Tout le texte est dans `src/histoire.rs`. Pour ajouter un chapitre, copie un bloc `Chapitre { ... }` à la fin de la liste `CHAPITRES` et change son numéro, son texte et le nom de sa photo.
