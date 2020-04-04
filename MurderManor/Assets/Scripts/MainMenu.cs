using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;

using UnityEngine;

public class MainMenu: MonoBehaviour {
    // These values will be filled in unity
    public GameObject networkManager = null;
    public GameObject serverEndpoint = null;
    public GameObject nickname = null;

    public void OnPlay() {
        var endpoint = serverEndpoint.GetComponent<TMPro.TMP_InputField>().text;
        var nick = nickname.GetComponent<TMPro.TMP_InputField>().text;

        networkManager.GetComponent<GrpcManager>().Connect(endpoint);
        networkManager.GetComponent<PlayersManager>().SetMainCharacter(nick);
    }

    public void OnQuit() {
        Application.Quit();
    }
}
