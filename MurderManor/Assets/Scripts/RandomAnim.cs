using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class RandomAnim : MonoBehaviour
{
    // Start is called before the first frame update
    void Start()
    {
        Animator anim = GetComponent<Animator>();

        float randomIdleStart = Random.Range(0, anim.GetCurrentAnimatorStateInfo(0).length); //Set a random part of the animation to start from
        anim.Play("BushIdle", 0, randomIdleStart);
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}
