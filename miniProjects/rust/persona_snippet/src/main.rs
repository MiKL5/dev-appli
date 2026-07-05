fn main() {
    let prenom = "Mickael";
    let mut age = 30;
    let money = 1505.9153547869521;
    let couleur_hex = "#00E9A5"; // vert émeraude en hexadécimal

    println!(
        "Bonjour 👋,\nJe suis {prenom}, j'ai {age} ans.\nJ'ai {money:.2} € 💶.\nMa couleur préférée est {couleur_hex}, le vert émeraude 💚."
    );

    let age = 31;
    println!("J'ai maintenant {age} ans.")
}