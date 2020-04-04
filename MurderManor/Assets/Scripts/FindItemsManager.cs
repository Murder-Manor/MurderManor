using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class FindItemsManager : MonoBehaviour
{
    public GameObject[] itemList = null;
    public Dictionary<string, GameObject> itemDictionary = null;

    // Start is called before the first frame update
    void Start()
    {
        itemDictionary = new Dictionary<string, GameObject>();

        for (int i = 0; i < itemList.Length; i++)
        {
            itemDictionary.Add(itemList[i].GetComponent<FindableObject>().itemId, itemList[i]); 
        }
        Debug.Log("There are" + itemDictionary.Count + "findable items");
    }

    // Update is called once per frame
    void Update()
    {
        
    }
}
