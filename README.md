# 🎲 GuessInt - Un jeu de devinette en Rust !

Bienvenue dans **GuessInt**, un petit projet écrit en Rust pour s'amuser avec les nombres aléatoires et l'interaction utilisateur via la console ! 🚀

## 📌 Objectif du jeu

Le but est simple :

1. Choisissez un **nombre minimum** et un **nombre maximum**.
2. L'ordinateur **choisit un nombre aléatoire** entre ces deux valeurs.
3. Essayez de **deviner ce nombre** avec des indices "C'est plus !" ou "C'est moins !".
4. Continuez jusqu'à trouver le bon nombre ! 🎉

---

## 🚀 Lancer le projet

### 1️⃣ **Pré-requis**

- Installer **Rust** via [rustup.rs](https://rustup.rs/).
- Installer la **crate `rand`** pour générer des nombres aléatoires :
  ```sh
  cargo add rand
  ```

### 2️⃣ **Compiler et exécuter**

```sh
cargo run
```

---

## 📜 Fonctionnalités

✅ Définition d'un **intervalle personnalisé** pour le jeu.  
✅ **Gestion des erreurs** : saisie invalide, min > max, etc.  
✅ **Génération aléatoire** d'un nombre à deviner.  
✅ **Interaction utilisateur** avec des messages clairs.

---

## 🛠 Code principal

### 🔢 **Génération du nombre aléatoire**

```rust
fn random_generator(nb_min: i32, nb_max: i32) -> Result<i32, String> {
    if nb_min > nb_max {
        return Err(format!("Erreur : min {} > max {}", nb_min, nb_max));
    }
    let mut rng = rand::rng();
    Ok(rng.random_range(nb_min..=nb_max))
}
```

### 🎤 **Demander un nombre à l'utilisateur**

```rust
fn user_number_choice(sentence: String) -> i32 {
    let mut good_choice = false;
    let mut new_nb: i32 = 0;
    let mut input = String::new();

    while !good_choice {
        println!("{}", sentence);
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        (good_choice, new_nb) = match_number(&input);
        if !good_choice {
            println!("Vous devez entrer un nombre valide !");
        }
        input.clear();
    }

    new_nb
}
```

---

## 🎯 Pourquoi ce projet ?

C'est **mon premier projet en Rust** ! 🎉 L'objectif était de :

- Découvrir **la syntaxe de Rust** 🦀
- Expérimenter **les types, les `Result` et les `loop`** 🔄
- Apprendre à **gérer l'entrée utilisateur et les erreurs** ⚠️
- Manipuler **les nombres aléatoires** 🎲

Si vous êtes débutant en Rust comme moi, j'espère que ce projet pourra vous être utile ! 💡

---

## 📌 Améliorations possibles

🛠 Ajouter un **compteur de tentatives** 📊  
🛠 Ajouter une option pour **rejouer** après une partie 🔄  
🛠 Ajouter des **niveaux de difficulté** 🎚️

N'hésitez pas à proposer vos idées et à forker le projet ! 🚀

---

### 📢 **Amusez-vous bien avec GuessInt !** 🎉
