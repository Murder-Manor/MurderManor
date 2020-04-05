using System;

using Gameapi;
using UnityEngine;

// It is supposed the GameStatesManager will be in the same GameObject as
// the GrpcManager
public class GameStatesManager: MonoBehaviour {
    private static float update_rate_ms = 10.0f;
    private float _time_to_next_update_ms = update_rate_ms;

    // These attributes will be set in unity editor
    public GameObject gameHUD = null;

    private void Update() {
        // Update every update_rate_ms
        _time_to_next_update_ms -= Time.deltaTime * 1000;
        if(_time_to_next_update_ms > 0.0f)
            return;
        _time_to_next_update_ms = update_rate_ms;
        
        var progress = GetComponent<GrpcManager>().GetClient()
            .GetGameProgress(new GetGameProgressRequest{});
        var text_to_display = "";
        switch(progress.Status) {
            case GameProgress.Types.Status.WaitingForPlayers:
                text_to_display = "Waiting for players";
                break;
            case GameProgress.Types.Status.StartCountdown:
                text_to_display = "Game starting! Ready?";
                break;
            case GameProgress.Types.Status.InGame:
                text_to_display = "Find the gramophone!";
                break;
        };

        gameHUD.GetComponent<TMPro.TMP_Text>().text = text_to_display;
    }
}
