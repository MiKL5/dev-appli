using UnityEngine;

public class AI : MonoBehaviour
{
    public Transform ball;
    [Range(1,8)]
    public float latency = 3.25f;
    float originalLatency;
    [Range(1, 20)]
    public float reactionField = 7; // Si la balle est dans ce champ l'ordi peut dépalacer la barre
    public float returnToCenter;

    private void Start()
    {
        originalLatency = latency;
        InvokeRepeating("ChangeVal", 10, 10);
    }

    void FixedUpdate() // FixedUpdate car c'est un objet physique
    {
        if (Vector2.Distance(transform.position, ball.position) < reactionField)
        {
            transform.position = 
            Vector2.MoveTowards(
            transform.position,
            new Vector2(transform.position.x, ball.position.y), 
            latency * Time.deltaTime);
            // transform.position = new Vector2(transform.position.x, ball.position.y); // INVICIBLE!
        }
        else // quand la balle est hors de portée
        { // la replacer au centre
            if (Random.Range(1, 100) < returnToCenter)
            {
                transform.position = Vector2.MoveTowards(
                    transform.position,
                    new Vector2(transform.position.x, 1), // centre de l'axe Y
                    latency * Time.deltaTime);
            }
        }
    }

    void ChangeVal()
    {
        if (Random.Range(0, 100) > 50) // 50% de probabilité que le comportement change
        {
            latency = (Random.Range(1, 10) > 5) ? (originalLatency + 0.5f) : (originalLatency - 0.5f);
            // if(Random.Range(1, 10) > 5)
            // {
            //     latency += 0.5f;
            // }
            // else
            // {
            //     latency -= 0.5f;
            // }
        }
    }
}