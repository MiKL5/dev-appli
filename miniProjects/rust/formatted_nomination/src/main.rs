fn main() {
    let prenom = "Mickael";
    let age = 31;
    let money = 1502.2123547869521;
    let couleur_hex = "#00E9A5"; // vert émeraude en hexadécimal

    let phrase = format!(
        "Bonjour\nJe suis {}, j'ai {} ans, J'ai {:.2} € et ma couleur préférée en hexadécimal est le vert émeraude : {}.",
        prenom, age, money, couleur_hex
    );

    println!("{}", phrase);
}