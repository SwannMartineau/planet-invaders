# planet-invaders

## Présentation

**planet-invaders** est un jeu de simulation en Rust où des robots explorent une planète, collectent des ressources (énergie, minéraux, science) et les ramènent à leur base. Le jeu utilise une interface en terminal pour afficher la carte, les robots et les ressources en temps réel.

## Fonctionnalités principales

- Génération procédurale de la carte avec différents types de tuiles (base, ressources, obstacles, terrain vide).
- Différents types de robots (explorateurs, mineurs, collecteurs d'énergie, scientifiques) avec des comportements spécifiques.
- Système de collecte et de gestion des ressources à la base.
- Interface utilisateur en terminal avec légende et affichage dynamique.

## Compilation et exécution

1. **Cloner le dépôt :**

   ```sh
   git clone https://github.com/SwannMartineau/planet-invaders.git
   cd planet-invaders
   ```

2. **Compiler le projet :**

   ```sh
   cargo build --release
   ```

3. **Lancer le jeu :**

   ```sh
   cargo run
   ```

   > Utilise `cargo run --release` pour de meilleures performances.

4. **Contrôles :**

   - `q` ou `Echap` : quitter le jeu

5. **Tests :**

   ```sh
   cargo test
   ```

## Collaborateurs

- Nicolas BIDET
- Swann MARTINEAU
- Bastien PROMPSY
- Marie RAMSSAMY

---
