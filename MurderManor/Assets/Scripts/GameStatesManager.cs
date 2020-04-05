using System;

using Gameapi;
using static Gameapi.GameProgress.Types;

using UnityEngine;

// It is supposed the GameStatesManager will be in the same GameObject as
// the GrpcManager
public class GameStatesManager: MonoBehaviour {
    private static float update_rate_ms = 10.0f;
    private float _time_to_next_update_ms = update_rate_ms;
    private Status _current_state = Status.WaitingForPlayers;
    private bool player_finished = false;

    // These attributes will be set in unity editor
    public GameObject gameHUD = null;

    public Status GetCurrentState() {
        return _current_state;
    }

    public void SetStatePlayerFinished() {
        player_finished = true;
    }

    private void Update() {
        // Update every update_rate_ms
        _time_to_next_update_ms -= Time.deltaTime * 1000;
        if(_time_to_next_update_ms > 0.0f)
            return;
        _time_to_next_update_ms = update_rate_ms;

        var progress = GetComponent<GrpcManager>().GetClient()
            .GetGameProgress(new GetGameProgressRequest{});

        _current_state = progress.Status;

        var text_to_display = "";
        switch(_current_state) {
            case Status.WaitingForPlayers:
                text_to_display = "Waiting for players";
                break;
            case Status.StartCountdown:
                text_to_display = "Game starting! Ready?";
                break;
            case Status.InGame:
                if(player_finished)
                    text_to_display = "Finished!";
                else
                    text_to_display = "Find the gramophone!";
                break;
            case Status.ScoreBoard:
                text_to_display = "And the winner is...";
                break;
            default:
                text_to_display = "Unknown state!";
                break;
        };

        gameHUD.GetComponent<TMPro.TMP_Text>().text = text_to_display;
    }
}
