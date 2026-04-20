using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class P4AI : MonoBehaviour
{
    public P4Game jeu;

    void Start()
    {
        InvokeRepeating("AiPlay", 2, 2);
    }

    public void AiPlay()
    {
        if(jeu.actualPlayer == 2 && jeu.canPlay)
        {
            jeu.canPlay = false;
            int choix;
            // Faire un choix
            if (jeu.nextChoice != -1 && !jeu.choiceToAvoid.Contains(jeu.nextChoice))
            {
                choix = jeu.nextChoice;
                jeu.choiceToAvoid.Add(jeu.nextChoice);
                jeu.nextChoice = -1;
            }
            else
            {
                choix = Random.Range(0, 7);
            }

            // Si le choix se porte sur une colonne déjà pleine
            // Il faut faire un autre choix
            if(jeu.cols[choix] >= 6)
            {
                // La colonne est déjà pleine
                print("IA : Je fais un autre choix que " + choix);
                jeu.canPlay = true;
                AiPlay();
                return;
            }

            jeu.PlayPawn(choix);
        }
    }
}
