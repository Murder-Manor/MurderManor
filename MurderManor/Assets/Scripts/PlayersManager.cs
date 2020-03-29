using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;

using Gameapi;
using UnityEngine;

public class PlayersManager : MonoBehaviour {
    private static float update_rate_ms = 1000.0f;
    private Game.GameClient _client;
    private Grpc.Core.Channel _grpc_channel;

    private float _time_to_next_update_ms = update_rate_ms;
    private Dictionary<string, GameObject> _characters = new Dictionary<String, GameObject>();

    public string endpoint = "[::1]:50051";
    public GameObject mainPlayer = null;

    private void Start() {
        _grpc_channel = new Grpc.Core.Channel(
            endpoint, Grpc.Core.ChannelCredentials.Insecure);
        _client = new Game.GameClient(_grpc_channel);
        var mainPlayerCasted = mainPlayer.GetComponent<CharacterMove>();
        _characters[mainPlayerCasted.id] = mainPlayer;
    }

    private void Update() {
        // Update every update_rate_ms
        _time_to_next_update_ms -= Time.deltaTime * 1000;
        if(_time_to_next_update_ms > 0.0f)
            return;
        _time_to_next_update_ms = update_rate_ms;

        // Synchronize map of characters
        System.Threading.Tasks.Task.Run(async () => {
            using (var response = _client.ListPlayers(new ListPlayersRequest{})) {
                var cancellationToken = default(System.Threading.CancellationToken);
                while(await response.ResponseStream.MoveNext(cancellationToken)) {
                    var currChar = response.ResponseStream.Current;
                    if(!_characters.ContainsKey(currChar.Id)) {
                        _characters[currChar.Id] = Instantiate(mainPlayer);
                        Debug.Log("Adding " + currChar.Name);
                    }
                }
            }
        });

        // Update position of each character in case a change have been made
        Debug.Log("End of player sync");
    }

    public string NewPlayer(string name) {
        var player = _client.NewPlayer(new NewPlayerRequest{Name = name});
        return player.Id;
    }

    public void MovePlayer(string id, UnityEngine.Vector3 position,
                           UnityEngine.Vector3 direction) {
        _client.MovePlayer(new MovePlayerRequest{
            Id = id,
            NewCoordinates =
                new Gameapi.Vector2{
                    X = position.x,
                    Y = position.y,
                },
            NewDirection =
                new Gameapi.Vector2{
                    X = direction.x,
                    Y = direction.y,
                },
        });
    }

    private void OnDisable() { _grpc_channel.ShutdownAsync().Wait(); }
}
