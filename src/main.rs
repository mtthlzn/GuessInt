use rand::Rng;
use std::io; // Module pour intéragir avec la console // Rust n'a pas de générateur aléatoire par défaut, il faut utiliser rand 

fn main() {
    println!("Bienvenu dans GuessInt !");
    println!(
        "Tout d'abord tu vas devoir choisir un nombre minimum et maximum pour définir l'étalonnage."
    );

    // Choix nombre min
    let nb_min = user_number_choice("Choisis le chiffre minimum".to_string());

    // Choix nombre max
    let mut nb_max = -999999999;
    loop {
        nb_max = user_number_choice("Choisis le chiffre maximum".to_string());

        if nb_min > (nb_max + 1) {
            println!("Attention, le chiffre max. doit être plus grand que le chiffre min. !");
        }

        if nb_min < (nb_max + 1) {
            break;
        }
    }

    println!("Le jeu va commencer ! \n Valeur min = {nb_min} \n Valeur max = {nb_max}");

    // Choix du nombre à deviner
    let golden_nb: i32 = match random_generator(nb_min, nb_max) {
        Ok(n) => n,
        Err(e) => {
            println!("Erreur : {} \n Arrêt du programme ...", e);
            return;
        }
    };

    println!("Ok c'est bon ! J'ai choisis un nombre ... À vous de jouer ! Aide ({golden_nb})");

    let mut user_nb = -999999;

    while user_nb != golden_nb {
        user_nb = user_number_choice(String::from("À quel nombre je pense ?"));

        if user_nb > golden_nb {
            println!("C'est moins !");
        }
        if user_nb < golden_nb {
            println!("C'est plus !");
        }
    }

    println!("Félicitation ! Vous avez trouvé le bon nombre : {golden_nb} !");
}

// Permet de voir si un String peut être cast en nombre et assigne une variable en même temps
fn match_number(value: &String) -> (bool, i32) {
    match value.trim().parse::<i32>() {
        Ok(nb) => (true, nb), // Le parsing a réussi, c'est un nombre

        Err(_) => (false, -1), // Échec du parsing, ce n'est pas un nombre valide
    }
}

// Permet de demander un nombre à l'utilisateur et de le stocker dans une variable
fn user_number_choice(sentence: String) -> i32 {
    let mut good_choice = false;
    let mut new_nb: i32 = 0;
    let mut input = String::new();

    while !good_choice {
        // Choix valeur max
        println!("{sentence}");
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture de la saisie");

        (good_choice, new_nb) = match_number(&input);

        if !good_choice {
            println!("Vous devez choisir un nombre !");
        }

        input.clear(); // Réinitialisation de l'input pour la prochaine sortie
    }

    new_nb
}

fn random_generator(nb_min: i32, nb_max: i32) -> Result<i32, String> {
    if nb_min > nb_max {
        return Err(format!("Erreur : min {} > max {}", nb_min, nb_max));
    }

    let mut rng = rand::rng();
    Ok(rng.random_range(nb_min..=nb_max)) // Un retour de fonction n'a pas de ';'
}
