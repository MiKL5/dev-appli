using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.UI;

public class P4Game : MonoBehaviour
{
    public GameObject[] pawns; // Pions 
    public int actualPlayer = 1; // Joueur actuel
    public Text playerNameTxt; // Nom du joueur actuel
    public int[] cols; // Les colonnes
    int nbPlayedPawns = 0; // Nombre de poins joués au total, 42 = max
    RaycastHit2D[] hits;
    public LayerMask layerToIgnore;
    string line;
    bool winner = false;
    // Ajouts pour IA
    public bool canPlay = false;
    public int nextChoice = -1;
    public List<int> choiceToAvoid;

    public void PlayPawn(int col) // Jouer un jeton dans la colonne
    {
        if (!winner)
        {
            cols[col] += 1;
            nbPlayedPawns++;
            if (nbPlayedPawns >= 42)
            {
                // Plateau plein ! Fin de partie.
            }
            if (cols[col] >= 6) // Si on a 6 jetons ou + dans la col
            {
                // On rend le btn inactif
                EventSystem.current.currentSelectedGameObject.GetComponent<Button>().interactable = false;
            }
            // Instanciation du jeton
            Vector2 instantiatePos = GameObject.Find("Pos" + col).transform.position;
            GameObject go = Instantiate(pawns[actualPlayer - 1], instantiatePos, Quaternion.identity);
            go.name = actualPlayer.ToString();
            // Changement de joueur
            actualPlayer = actualPlayer == 1 ? 2 : 1;
            playerNameTxt.text = "Player " + actualPlayer;
            if (actualPlayer == 2) // Si c'est à l'IA de jouer
                canPlay = true;
            Invoke("GetWinner", 2);
        }
    }

    public void NewGame() // Relancer le jeu
    {
        Application.LoadLevel(Application.loadedLevel);
    }

    public int GetWinner()
    {
        // Boucle sur les lignes
        for (float y = -3.9f; y <= 4.1; y += 1.6f)
        {
            hits = Physics2D.RaycastAll(new Vector2(-8, y), Vector2.right, 20, ~layerToIgnore);
            line = "";
            foreach (RaycastHit2D rh in hits)
            {
                line += rh.transform.gameObject.name;
            }
            if (line.Contains("1111"))
            {
                winner = true;
                playerNameTxt.text = "Winner : P1";
                return 1;
            }
            if (line.Contains("2222"))
            {
                winner = true;
                playerNameTxt.text = "Winner : P2";
                return 2;
            }
        }

        // Faire une boucle pour les colonnes
        int actualCol = -1;
        for (float x = -4.9f; x < 4.7f; x += 1.6f) // Pour chaque col
        {
            actualCol++;
            hits = Physics2D.RaycastAll(new Vector2(x, -5.5f), Vector2.up, 20, ~layerToIgnore);
            line = "";
            foreach (RaycastHit2D rh in hits)
            {
                line += rh.transform.gameObject.name;
            }
            if (line.Contains("1111"))
            {
                winner = true;
                playerNameTxt.text = "Winner : P1";
                return 1;
            }
            if (line.Contains("2222"))
            {
                winner = true;
                playerNameTxt.text = "Winner : P2";
                return 2;
            }
            // Empêcher le joueur de gagner ou gagner la partie
            if (line.Contains("111") && !choiceToAvoid.Contains(actualCol)) // Le joueur est sur le point de gagner, attention !!!
            {
                nextChoice = actualCol;
            }
            if (line.Contains("222")) // Si nous sommes sur le point de gagner
            {
                nextChoice = actualCol;
            }
        }

        // Faire une boucle pour la diagonale

        return 0; // Personne n'a gagné 
    }
}
