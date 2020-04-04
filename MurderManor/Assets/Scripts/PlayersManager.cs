using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;

using Gameapi;
using UnityEngine;

// It is supposed the PlayersManager will be in the same GameObject as
// the GrpcManager
public class PlayersManager : MonoBehaviour {
    private static float update_rate_ms = 10.0f;

    private float _time_to_next_update_ms = update_rate_ms;
    private Dictionary<string, GameObject> _characters = new Dictionary<String, GameObject>();
    private Dictionary<string, CharacterMove> _controlled_characters = new Dictionary<String, CharacterMove>();

    // These attributes will be set in unity editor
    public GameObject mainCharacter = null;
    public GameObject spawnedPrefab = null;

    // SetMainCharacter includes the player's character in the manager.
    // Takes the name of the main character and returns its UUID.
    public string SetMainCharacter(string name) {
        if(name == "") name = "Arthur";

        var character = mainCharacter.GetComponent<CharacterMove>();
        character.SetCharacterName(name);
        var returnedPlayer = GetComponent<GrpcManager>()
            .GetClient()
            .NewPlayer(new NewPlayerRequest{
                    Name = character.GetCharacterName()});
        _controlled_characters[returnedPlayer.Id] = character;
        character.id = returnedPlayer.Id;
        character.enabled = true;
        return returnedPlayer.Id;
    }

    private void Update() {
        // PlayersManager is useless if the Grpc Channel is not ready
        if(!GetComponent<GrpcManager>().IsConnected())
            return;
        // Update every update_rate_ms
        _time_to_next_update_ms -= Time.deltaTime * 1000;
        if(_time_to_next_update_ms > 0.0f)
            return;
        _time_to_next_update_ms = update_rate_ms;

        // Synchronize map of characters
        UpdateCharsMap();

        // Cleanup by destroying all characters that did not respond for 2s
        var toDelete = new List<string>();
        foreach(var entry in _characters) {
            if(Time.time - entry.Value.GetComponent<CharacterMove>().GetLastUpdatedTime() < 2.0f)
                continue;
            Destroy(entry.Value);
            toDelete.Add(entry.Key);
        }
        foreach(var key in toDelete)
            _characters.Remove(key);

        // Update controlled characters
        SendCharsUpdate();
    }

    // SendCharsUpdate sends the status of the controllable characters to the server
    private void SendCharsUpdate() {
        foreach(var entry in _controlled_characters) {
            MovePlayer(entry.Key, entry.Value.GetPosition(), entry.Value.GetDirection());
        }
    }

    // MovePlayer notifies the server of a player's movement
    private void MovePlayer(string id, UnityEngine.Vector3 position,
                           UnityEngine.Vector3 direction) {
        GetComponent<GrpcManager>().GetClient().MovePlayer(
                new MovePlayerRequest{
            Id = id,
            Position =
                new Gameapi.Vector3{
                    X = position.x,
                    Y = position.y,
                    Z = position.z,
                },
            Direction =
                new Gameapi.Vector3{
                    X = direction.x,
                    Y = direction.y,
                    Z = direction.z,
                },
        });
    }

    // UpdateCharsMap queries the server for updates and updates the internal map
    // of characters with their new states.
    // It takes care of instantiating the new characters. Old characters are
    // cleaned up automatically in the Update method.
    private void UpdateCharsMap() {
        using (var response = GetComponent<GrpcManager>().GetClient().ListPlayers(new ListPlayersRequest{})) {
            var cancellationToken = default(System.Threading.CancellationToken);
            while(true) {
                var next = response.ResponseStream.MoveNext(cancellationToken);
                next.Wait();
                if(!next.Result)
                    break;
                var currChar = response.ResponseStream.Current;
                // Pass our turn if this is one of ours controller characters
                if(_controlled_characters.ContainsKey(currChar.Id))
                    continue;
                // Instantiate a new character if we didn't have it
                if(!_characters.ContainsKey(currChar.Id)) {
                    _characters[currChar.Id] = Instantiate(spawnedPrefab);
                    _characters[currChar.Id].GetComponent<CharacterMove>().SetCharacterName(currChar.Name);
                    // Teleport it at the beginning to avoid collision issues
                    Debug.Log("Adding " + currChar.Id);
                }
                // Update position of each character in case a change have been made
                var charMove = _characters[currChar.Id].GetComponent<CharacterMove>();
                charMove.SetPosition(new UnityEngine.Vector3(
                            currChar.Position.X, currChar.Position.Y, currChar.Position.Z));
                charMove.SetDirection(new UnityEngine.Vector3(
                            currChar.Direction.X, currChar.Direction.Y, currChar.Direction.Z));
            }
        }
    }
}
