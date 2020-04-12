using System;
using System.Collections;
using System.Collections.Generic;

using Gameapi;
using static Gameapi.GameProgress.Types;

using UnityEngine;

// It is supposed the GameStatesManager will be in the same GameObject as
// the GrpcManager
public class GameStatesManager: MonoBehaviour {
    private static float update_rate_ms = 10.0f;
    private float _time_to_next_update_ms = update_rate_ms;
    private Status _current_state = Status.WaitingForPlayers;
    private bool player_finished_round = false;
    private uint _current_round = 0;
    private string object_to_take = "";
    private Dictionary<String, FindableObject> _takable_objects =
        new Dictionary<String, FindableObject>();

    // These attributes will be set in unity editor
    public GameObject mainText = null;
    public GameObject scoreBoard = null;
    public GameObject findableItems = null;

    private void Start() {
        foreach(Transform child in findableItems.transform) {
            var fo = child.gameObject.GetComponent<FindableObject>();
            _takable_objects[fo.itemId] = fo;
        }
    }

    public Status GetCurrentState() {
        return _current_state;
    }

    public void NotifyObjectTaken(string object_id) {
        if (!player_finished_round && (object_id == object_to_take))
            player_finished_round = true;
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

        if(_current_state == Status.InGame) {
            object_to_take = progress.ObjectToTake;
            _current_round = progress.CurrentRound;
        }

        if(_current_state == Status.WaitingForNextRound) {
            player_finished_round = false;
        }


        var text_to_display = HUDTextFromState();
        mainText.GetComponent<TMPro.TMP_Text>().text = text_to_display;

        var score_text = "";
        foreach(var score in GetComponent<PlayersManager>().GetScoreBoard()) {
            score_text += score.Item1 + " -> " + score.Item2 + "\n";
        }
        scoreBoard.GetComponent<TMPro.TMP_Text>().text = score_text;
    }

    private string HUDTextFromState() {
        var text_to_display = "";
        switch(_current_state) {
            case Status.WaitingForPlayers:
                text_to_display = "Waiting for players";
                break;
            case Status.StartCountdown:
                text_to_display = "Game starting! Ready?";
                break;
            case Status.InGame:
                if(player_finished_round)
                    text_to_display = "Round " + _current_round + " finished!";
                else {
                    var object_name = _takable_objects[object_to_take].itemName;
                    text_to_display = "Find " + object_name;
                }
                break;
            case Status.WaitingForNextRound:
                text_to_display = "Next round starting... Ready?";
                break;
            case Status.ScoreBoard:
                text_to_display = "And the winner is...\n";
                foreach(var score in GetComponent<PlayersManager>().GetScoreBoard()) {
                    text_to_display += score.Item1 + " -> " + score.Item2 + "\n";
                }
                break;
            default:
                text_to_display = "Unknown state!";
                break;
        };
        return text_to_display;
    }
}
