using System;

using Gameapi;
using UnityEngine;

// It is supposed the PlayersManager will be in the same GameObject as
// the GrpcManager
public class ObjectsManager : MonoBehaviour {
    private static float update_rate_ms = 10.0f;
    private float _time_to_next_update_ms = update_rate_ms;
    private string player_id;

    public void SetPlayerID(string ID) {
        player_id = ID;
    }

    private void Update() {
        // Update every update_rate_ms
        _time_to_next_update_ms -= Time.deltaTime * 1000;
        if(_time_to_next_update_ms > 0.0f)
            return;
        _time_to_next_update_ms = update_rate_ms;
    }

    public void TakeObject(string object_id) {
        GetComponent<GrpcManager>().GetClient().TakeObject(new TakeObjectRequest{
                PlayerId = player_id,
                ObjectId = object_id,
                });
    }
}
