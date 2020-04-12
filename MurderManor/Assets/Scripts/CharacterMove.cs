
using UnityEngine;
using System.Collections;

// This script moves the character controller forward
// and sideways based on the arrow keys.
// Make sure to attach a capsule collider to the same game object.
// It is recommended that you make only one call to Move per frame.

public class CharacterMove : MonoBehaviour {
    public string id;

    // These variables must be set in unity interface
    public Animator m_Animator = null;
    public float speed = 1.0f;

    private float lastUpdatedTime = 0.0f;
    private string characterName;
    private Vector3 moveDirection = Vector3.zero;

    public void SetCharacterName(string characterName) {
        this.characterName = characterName;
        Debug.Log("New char: " + characterName);
        gameObject.transform.Find("NameDisplay").GetComponent<TextMesh>().text = characterName;
    }

    public string GetCharacterName() {
        return characterName;
    }

    void Update() {
        // Normalization also ensures a "small" vector will be set to vector3.zero
        moveDirection = Vector3.Normalize(
               new Vector3(Input.GetAxis("Horizontal"), 0.0f, Input.GetAxis("Vertical")));
        RotateTo(moveDirection);
    }

    void FixedUpdate() {
        // Move the player using a MovePosition instead of adding a force and let
        // the physics engine do stuff for us, thus avoiding different speeds
        // between players depending on local physics engine resolution.
        // Doing it in FixedUpdate lets the move be done before physics engine
        // resolution.
        var rb = GetComponent<Rigidbody>();
        SetPosition(rb.position + moveDirection * speed * Time.fixedDeltaTime);
    }

    public Vector3 GetPosition() {
        return transform.position;
    }

    public Vector3 GetDirection() {
        return transform.rotation.eulerAngles;
    }

    public float GetLastUpdatedTime() {
        return lastUpdatedTime;
    }

    public void RotateTo(Vector3 dir) {
        if(dir == Vector3.zero)
            return;
        GetComponent<Rigidbody>().rotation = Quaternion.Slerp(
                transform.rotation,
                Quaternion.LookRotation(dir.normalized), 0.5f);
    }

    public void SetPosition(Vector3 pos) {
        m_Animator.SetBool("Walk", GetComponent<Rigidbody>().position != pos);
        lastUpdatedTime = Time.time;
        GetComponent<Rigidbody>().MovePosition(pos);
    }

    public void SetDirection(Vector3 dir) {
        lastUpdatedTime = Time.time;
        GetComponent<Rigidbody>().rotation = Quaternion.Euler(dir);
    }
}

