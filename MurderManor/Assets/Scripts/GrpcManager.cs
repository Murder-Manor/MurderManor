using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;

using Gameapi;
using UnityEngine;

public class GrpcManager : MonoBehaviour {
    private Game.GameClient _client;
    private Grpc.Core.Channel _grpc_channel;
    private bool is_connected = false;

    private void Start() {
    }

    // Connect instantiates a new server connexion and instatiantes the main
    // character.
    public bool Connect(string endpoint) {
        if (endpoint == "") endpoint = "[::1]:50051";
        Debug.Log(endpoint);

        _grpc_channel = new Grpc.Core.Channel(
            endpoint, Grpc.Core.ChannelCredentials.Insecure);
        _client = new Game.GameClient(_grpc_channel);
        is_connected = true;

        return true;
    }

    public bool IsConnected() {
        return is_connected;
    }

    // GetClient gives a gRPC client to call RPC methods.
    public Game.GameClient GetClient() {
        return _client;
    }

    // OnDisable shuts down the gRPC channel
    private void OnDisable() {
        if(_grpc_channel != null)
            _grpc_channel.ShutdownAsync().Wait();
    }
}
