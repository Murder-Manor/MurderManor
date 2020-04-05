using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class FindableObject : MonoBehaviour
{
    private Collider mainCharacterCollider = null;
    private bool taken = false;

    // These fields will be filled in unity editor
    public string itemId;
    public string itemName = "Item Name";
    public GameObject objectsManager = null;
    public GameObject mainCharacter = null;

    // Start is called before the first frame update
    void Start() {
        mainCharacterCollider = mainCharacter.GetComponent<CapsuleCollider>();
    }

    // Update is called once per frame
    void Update() {
    }

    void OnTriggerEnter(Collider collider) {
        if(taken) return;
        if(collider != mainCharacterCollider)
            return;
        objectsManager.GetComponent<ObjectsManager>().TakeObject(itemId);
        taken = true;
    }
}
