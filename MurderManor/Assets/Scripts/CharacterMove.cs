
using UnityEngine;
using System.Collections;

// This script moves the character controller forward
// and sideways based on the arrow keys.
// Make sure to attach a capsule collider to the same game object.
// It is recommended that you make only one call to Move per frame.

public class CharacterMove : MonoBehaviour {
    public float speed = 1.0f;
    public string id;

    // These variables must be set in unity interface
    public Animator m_Animator = null;

    private float lastUpdatedTime = 0.0f;
    private string characterName;
    private uint score = 0;

    public void SetCharacterName(string characterName) {
        this.characterName = characterName;
        Debug.Log("New char: " + characterName);
        gameObject.transform.Find("NameDisplay").GetComponent<TextMesh>().text = characterName;
    }

    public string GetCharacterName() {
        return characterName;
    }

    public void SetScore(uint score) {
        this.score = score;
    }

    public uint GetScore() {
        return score;
    }

    void Update() {
        // Normalization also ensures a "small" vector will be set to vector3.zero
        var moveDirection = Vector3.Normalize(
               new Vector3(Input.GetAxis("Horizontal"), 0.0f, Input.GetAxis("Vertical")));
        MoveTo(moveDirection);
        RotateTo(moveDirection);
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

    public void MoveTo(Vector3 dir) {
        lastUpdatedTime = Time.time;
        m_Animator.SetBool("Walk", dir != Vector3.zero);
        GetComponent<Rigidbody>().AddForce(dir * speed);
    }

    public void SetPosition(Vector3 pos) {
        m_Animator.SetBool("Walk", GetComponent<Rigidbody>().position != pos);
        lastUpdatedTime = Time.time;
        GetComponent<Rigidbody>().position = pos;
    }

    public void SetDirection(Vector3 dir) {
        lastUpdatedTime = Time.time;
        GetComponent<Rigidbody>().rotation = Quaternion.Euler(dir);
    }
}

