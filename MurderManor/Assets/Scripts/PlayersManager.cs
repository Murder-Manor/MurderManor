using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;

using Gameapi;
using UnityEngine;

public class PlayersManager : MonoBehaviour {
    private static float update_rate_ms = 10.0f;
    private Game.GameClient _client;
    private Grpc.Core.Channel _grpc_channel;

    private float _time_to_next_update_ms = update_rate_ms;
    private Dictionary<string, GameObject> _characters = new Dictionary<String, GameObject>();
    private Dictionary<string, CharacterMove> _controlled_characters = new Dictionary<String, CharacterMove>();

    public string endpoint = "[::1]:50051";
    public GameObject mainCharacter = null;
    public GameObject spawnedPrefab = null;

    private void Start() {
        _grpc_channel = new Grpc.Core.Channel(
            endpoint, Grpc.Core.ChannelCredentials.Insecure);
        _client = new Game.GameClient(_grpc_channel);
        NewPlayer(mainCharacter);
    }

    private void Update() {
        // Update every update_rate_ms
        _time_to_next_update_ms -= Time.deltaTime * 1000;
        if(_time_to_next_update_ms > 0.0f)
            return;
        _time_to_next_update_ms = update_rate_ms;

        // Synchronize map of characters
        using (var response = _client.ListPlayers(new ListPlayersRequest{})) {
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
                    Debug.Log("Adding " + currChar.Id);
                }
                // Update position of each character in case a change have been made
                var charMove = _characters[currChar.Id].GetComponent<CharacterMove>();
                charMove.MoveTo(new UnityEngine.Vector3(
                            currChar.Position.X, currChar.Position.Y, currChar.Position.Z));
                charMove.SetDirection(new UnityEngine.Vector3(
                            currChar.Direction.X, currChar.Direction.Y, currChar.Direction.Z));
            }
        }

        // Update controlled characters
        foreach(var entry in _controlled_characters) {
            MovePlayer(entry.Key, entry.Value.GetPosition(), entry.Value.GetDirection());
        }
    }

    public string NewPlayer(GameObject player) {
        var character = player.GetComponent<CharacterMove>();
        var returnedPlayer = _client.NewPlayer(new NewPlayerRequest{Name = character.name});
        _controlled_characters[returnedPlayer.Id] = character;
        character.id = returnedPlayer.Id;
        return returnedPlayer.Id;
    }

    public void MovePlayer(string id, UnityEngine.Vector3 position,
                           UnityEngine.Vector3 direction) {
        _client.MovePlayer(new MovePlayerRequest{
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

    private void OnDisable() { _grpc_channel.ShutdownAsync().Wait(); }
}
