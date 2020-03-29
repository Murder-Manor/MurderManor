
using UnityEngine;
using System.Collections;

// This script moves the character controller forward
// and sideways based on the arrow keys.
// Make sure to attach a character controller to the same game object.
// It is recommended that you make only one call to Move or SimpleMove per
// frame.

public class CharacterMove : MonoBehaviour {
    public float speed = 6.0f;
    public string id;

    // These variables must be set in unity interface
    public Animator m_Animator = null;
    public string characterName;

    private float lastUpdatedTime = 0.0f;

    void Start() {
    }

    void Update() {
        var moveDirection = new Vector3(Input.GetAxis("Horizontal"), 0.0f,
                                    Input.GetAxis("Vertical"));
        MoveTo(transform.position + moveDirection * speed * Time.deltaTime);
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

    public void MoveTo(Vector3 pos) {
        lastUpdatedTime = Time.time;
        var movement = pos - transform.position;

        m_Animator.SetBool("Walk", movement != Vector3.zero);

        if(movement != Vector3.zero)
            transform.rotation = Quaternion.Slerp(
                transform.rotation,
                Quaternion.LookRotation(movement.normalized), 0.5f);

        GetComponent<CharacterController>().Move(movement);
    }

    public void SetPosition(Vector3 pos) {
        lastUpdatedTime = Time.time;
        transform.position = pos;
    }

    public void SetDirection(Vector3 dir) {
        lastUpdatedTime = Time.time;
        transform.rotation = Quaternion.Euler(dir);
    }
}

