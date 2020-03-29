
using UnityEngine;
using System.Collections;

// This script moves the character controller forward
// and sideways based on the arrow keys.
// Make sure to attach a character controller to the same game object.
// It is recommended that you make only one call to Move or SimpleMove per
// frame.

public class CharacterMove : MonoBehaviour {
    CharacterController characterController;

    public float speed = 6.0f;
    public float jumpSpeed = 8.0f;

    public string id;
    private Vector3 moveDirection = Vector3.zero;

    // These variables must be set in unity interface
    public Animator m_Animator = null;
    public string characterName;

    void Start() {
        characterController = GetComponent<CharacterController>();
    }

    void Update() {
        moveDirection = new Vector3(Input.GetAxis("Horizontal"), 0.0f,
                                    Input.GetAxis("Vertical"));

        m_Animator.SetBool("Walk", moveDirection != Vector3.zero);
        if (moveDirection != Vector3.zero)
            transform.rotation = Quaternion.Slerp(
                transform.rotation,
                Quaternion.LookRotation(moveDirection.normalized), 0.5f);

        moveDirection *= speed;
        characterController.Move(moveDirection * Time.deltaTime);
    }

    Vector3 GetPosition() {
        return transform.position;
    }

    Vector3 GetDirection() {
        return transform.rotation.eulerAngles;
    }
}

