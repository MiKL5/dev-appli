using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class bar : MonoBehaviour
{
    public bool isHumanPlayer = false; // humain ou IA ?
    public float speed;
    void Update()
    {
        float move; // movement du paddle
        // calcul du mouvement
        if(isHumanPlayer)
        {
            move = Input.GetAxis("Vertical") * speed * Time.deltaTime;
        }
        else
        {
            move = Input.GetAxis("horizontal") * speed * Time.deltaTime;
        }
        // application du mouvement
        transform.Translate(move * Vector2.up); // si Vector2 devient négatif ça fait Vector2.down donc les deux sont gèrer avec si peu de code
        // la raquette ne sort pas de l'écran
        if (transform.position.y > 5)
            transform.position = new Vector3(transform.position.x, 5, 0);
        if (transform.position.y < -3)
            transform.position = new Vector3(transform.position.x, -3, 0);
    }
}
