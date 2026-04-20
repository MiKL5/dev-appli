using System.Collections;
using System.Collections.Generic;
using Unity.VisualScripting;
using UnityEngine;
using UnityEngine.UI;

public class ball : MonoBehaviour
{
    public float speed;
    Vector2 dir;
    CamShaker cs;
    public Text score;
    public int[] scores;

    // la direction par défaut
    void Start()
    {
        dir = Vector2.one.normalized;
        cs = Camera.main.GetComponent<CamShaker>();
    }

    // le mouvement
    void Update()
    {
        transform.Translate(dir * speed * Time.deltaTime);
        // la balle se cogne en haut et en bas
        if ( (transform.position.y < -3.9f && dir.y < 0)
        ||   (transform.position.y > 5.9f && dir.y > 0) )
        {
            cs.Shake(0.15f, 0.2f, 20f); // secousse
            dir.y *= -1; // inverse la direction
        }
        // vérifier si la balle sor de l'écran
        if (transform.position.x < -7.5f && dir.x < 0)
        {
            // le joueur gange
            transform.position = Vector2.zero;
            scores[0]++;
            score.text = scores[1] + " - " + scores[0];
            cs.Shake(0.55f, 0.6f, 40f);
        }
        if (transform.position.x > 7.5f && dir.x > 0)
        {
            // l'IA gagne
            transform.position = Vector2.zero;
            scores[1]++;
            score.text = scores[1] + " - " + scores[0];
            cs.Shake(0.75f, 0.6f, 40f);
        }
    }
    // gérer les collisions avec les paddles
    private void OnTriggerEnter2D(Collider2D collision)
    {
        if (collision.tag == "paddle") 
        {
            bool isPlayer = collision.GetComponent<bar>().isHumanPlayer;
            if ( (isPlayer && dir.x > 0 ) || (!isPlayer && dir.x < 0 ) )
            {
                cs.Shake(0.4f, 0.25f, 30f);
                dir.x *= -1;
            }
        }
    }

    // quitter le jeu
    /* public void QuitGame()
    {
        if (Input.GetKeyDown(KeyCode.Escape) || Input.GetKeyDown(KeyCode.Q))
        {
            // Close the game
            Application.Quit();
        }
    } */
}
