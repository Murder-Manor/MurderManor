using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ObjectPicker : MonoBehaviour
{
    
    private List<Collider> activeObjects = new List<Collider>();
    public GameObject itemContainer;
    private bool isHoldingItem = false;
    private GameObject itemHeld;

    // Start is called before the first frame update
    void Start()
    {
        Collider m_ObjectCollider = GetComponent<Collider>();
    }

    // Update is called once per frame
    void Update()
    {
        if (activeObjects.Count <= 0) return;
       
        var closestObject = activeObjects[0].gameObject;
        var currentMinDist = Vector3.Distance(closestObject.gameObject.transform.position, transform.position);
        

        foreach (var activeObject in activeObjects)
        {
            float dist = Vector3.Distance(activeObject.gameObject.transform.position, transform.position);

            if (dist < currentMinDist)
            {
                closestObject = activeObject.gameObject;
                currentMinDist = dist;
            }
           
        }

        if (Input.GetButtonDown("Pickup"))
        {
            if (isHoldingItem == false)
            {
                isHoldingItem = true;
                itemHeld = closestObject;
                closestObject.transform.parent = transform.Find("ItemPosition").gameObject.transform;
                closestObject.transform.localPosition = new Vector3(0, 0, 0);
                //closestObject.transform.position = transform.position;
                //closestObject.transform.position = new Vector3(transform.position.x, transform.position.y + 0.5f, transform.position.z + 0.2f);
                closestObject.GetComponent<Rigidbody>().isKinematic = true;
                Debug.Log(closestObject);

            }

            else
            {
                isHoldingItem = false;
                itemHeld.transform.parent = itemContainer.transform;
                closestObject.GetComponent<Rigidbody>().isKinematic = false;

            }
        }   
    }

    private void OnTriggerEnter(Collider other)
    {
        if (other.gameObject.tag == "pickable")
        {
            activeObjects.Add(other);
            //Debug.Log(other + "added to list");
        }

    }
   
    private void OnTriggerExit(Collider other)
    {
        if (other.gameObject.tag == "pickable")
        {
            activeObjects.Remove(other);
            //Debug.Log(other + "removed from list");
        }
        
    }
}
