using System.Collections;
using UnityEngine;

public class CamShaker : MonoBehaviour
{
	bool shakingCam;

	public void Shake(float duration, float amount, float intensity) // est-ce que la caméra secoue ?
	{
		if (!shakingCam) // la secouer
			StartCoroutine(ShakeCam(duration, amount, intensity));
	}

	IEnumerator ShakeCam(float dur, float amount, float intensity)
	{
		float t = dur;
		Vector3 originalPos = Camera.main.transform.localPosition;
		Vector3 targetPos = Vector3.zero;
		shakingCam = true;

		while (t > 0.0f)
		{
			if (targetPos == Vector3.zero)
			{
				targetPos = originalPos + (Random.insideUnitSphere * amount);
			}

			Camera.main.transform.localPosition = 
				Vector3.Lerp(Camera.main.transform.localPosition, targetPos, intensity * Time.deltaTime);

			if (Vector3.Distance(Camera.main.transform.localPosition, targetPos) < 0.02f)
			{
				targetPos = Vector3.zero;
			}

			t -= Time.deltaTime;
			yield return null;
		}

		Camera.main.transform.localPosition = originalPos; // le temps est écouler, la caméra reprend la place d'origine
		shakingCam = false; // et fonction avec la paramètre d'origine
	}

}
