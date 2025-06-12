# RustRPG

## 1 - Lancement du jeu

### 1.1 - Prérequis

Pour utiliser et lancer notre jeu, vous devez avoir **Cargo** installé sur votre machine.  
Si vous ne l’avez pas encore, vous pouvez l’installer en suivant la documentation officielle de Rust :  
[https://doc.rust-lang.org/cargo/getting-started/installation.html](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### 1.2 - Lancement

1. Placez-vous dans le dossier du projet **RustRPG**.  
2. Compilez le projet avec la commande suivante :  
   ```bash
   cargo build
   ```
Maintenant que le projet est compilé, vous pouvez le lancer avec la commande :

```bash
   cargo run
```

Une fois le jeu lancé, vous arriverez dans le choix des personnages où vous pourrez soit en créer un, soit en choisir un déjà existant.
Une fois le choix du personnage effectué, il ne vous restera plus qu'à apprécier notre jeu !


## 2 - Documentation

Pour avoir accès à la documentation de notre projet, placez-vous dans le dossier RustRPG et effectuez la commande :

```bash
cargo doc
```
Cela permettra de générer la documentation de chacun des modules dans le dossier target/doc/RustRPG/affichage/index.html.

Vous pouvez également ajouter l'option --open à la commande pour ouvrir la documentation directement dans votre navigateur :
```bash
cargo doc --open
```
