using System;
using Gameapi;
using UnityEngine;

public class PlayersManager : MonoBehaviour {
    private Game.GameClient _client;
    private Grpc.Core.Channel _grpc_channel;

    string endpoint = "[::1]:50051";

    private void Start() {
        _grpc_channel = new Grpc.Core.Channel(
            endpoint, Grpc.Core.ChannelCredentials.Insecure);
        _client = new Game.GameClient(_grpc_channel);
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
