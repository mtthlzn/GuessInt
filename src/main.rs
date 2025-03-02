use std::io; // Module pour intéragir avec la console

fn main() {
    println!("Bienvenu dans GuessInt !");
    println!("Tout d'abord tu vas devoir choisir un nombre minimum et maximum pour définir l'étalonnage.");

    let nb_min = user_number_choice("Choisis le chiffre minimum".to_string());

    let mut nb_max = 0;

    while nb_min > (nb_max + 1) {
        nb_max = user_number_choice("Choisis le chiffre maximum".to_string());

        if nb_min > (nb_max + 1) {
            println!("Attention, le chiffre max. doit être plus grand que le chiffre min. !");
        }
    }

    println!("Le jeu va commencer ! \n Valeur min = {nb_min} \n Valeur max = {nb_max}");
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

    return new_nb;
}
