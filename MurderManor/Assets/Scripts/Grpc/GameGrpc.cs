// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: game.proto
// </auto-generated>
#pragma warning disable 0414, 1591
#region Designer generated code

using grpc = global::Grpc.Core;

namespace Gameapi {
  public static partial class Extra
  {
    static readonly string __ServiceName = "gameapi.Extra";

    static readonly grpc::Marshaller<global::Gameapi.ServiceInfoRequest> __Marshaller_gameapi_ServiceInfoRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.ServiceInfoRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Gameapi.ServiceInfoReply> __Marshaller_gameapi_ServiceInfoReply = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.ServiceInfoReply.Parser.ParseFrom);

    static readonly grpc::Method<global::Gameapi.ServiceInfoRequest, global::Gameapi.ServiceInfoReply> __Method_ServiceInfo = new grpc::Method<global::Gameapi.ServiceInfoRequest, global::Gameapi.ServiceInfoReply>(
        grpc::MethodType.Unary,
        __ServiceName,
        "ServiceInfo",
        __Marshaller_gameapi_ServiceInfoRequest,
        __Marshaller_gameapi_ServiceInfoReply);

    /// <summary>Service descriptor</summary>
    public static global::Google.Protobuf.Reflection.ServiceDescriptor Descriptor
    {
      get { return global::Gameapi.GameReflection.Descriptor.Services[0]; }
    }

    /// <summary>Base class for server-side implementations of Extra</summary>
    [grpc::BindServiceMethod(typeof(Extra), "BindService")]
    public abstract partial class ExtraBase
    {
      public virtual global::System.Threading.Tasks.Task<global::Gameapi.ServiceInfoReply> ServiceInfo(global::Gameapi.ServiceInfoRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

    }

    /// <summary>Client for Extra</summary>
    public partial class ExtraClient : grpc::ClientBase<ExtraClient>
    {
      /// <summary>Creates a new client for Extra</summary>
      /// <param name="channel">The channel to use to make remote calls.</param>
      public ExtraClient(grpc::ChannelBase channel) : base(channel)
      {
      }
      /// <summary>Creates a new client for Extra that uses a custom <c>CallInvoker</c>.</summary>
      /// <param name="callInvoker">The callInvoker to use to make remote calls.</param>
      public ExtraClient(grpc::CallInvoker callInvoker) : base(callInvoker)
      {
      }
      /// <summary>Protected parameterless constructor to allow creation of test doubles.</summary>
      protected ExtraClient() : base()
      {
      }
      /// <summary>Protected constructor to allow creation of configured clients.</summary>
      /// <param name="configuration">The client configuration.</param>
      protected ExtraClient(ClientBaseConfiguration configuration) : base(configuration)
      {
      }

      public virtual global::Gameapi.ServiceInfoReply ServiceInfo(global::Gameapi.ServiceInfoRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return ServiceInfo(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Gameapi.ServiceInfoReply ServiceInfo(global::Gameapi.ServiceInfoRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_ServiceInfo, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.ServiceInfoReply> ServiceInfoAsync(global::Gameapi.ServiceInfoRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return ServiceInfoAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.ServiceInfoReply> ServiceInfoAsync(global::Gameapi.ServiceInfoRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_ServiceInfo, null, options, request);
      }
      /// <summary>Creates a new instance of client from given <c>ClientBaseConfiguration</c>.</summary>
      protected override ExtraClient NewInstance(ClientBaseConfiguration configuration)
      {
        return new ExtraClient(configuration);
      }
    }

    /// <summary>Creates service definition that can be registered with a server</summary>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static grpc::ServerServiceDefinition BindService(ExtraBase serviceImpl)
    {
      return grpc::ServerServiceDefinition.CreateBuilder()
          .AddMethod(__Method_ServiceInfo, serviceImpl.ServiceInfo).Build();
    }

    /// <summary>Register service method with a service binder with or without implementation. Useful when customizing the  service binding logic.
    /// Note: this method is part of an experimental API that can change or be removed without any prior notice.</summary>
    /// <param name="serviceBinder">Service methods will be bound by calling <c>AddMethod</c> on this object.</param>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static void BindService(grpc::ServiceBinderBase serviceBinder, ExtraBase serviceImpl)
    {
      serviceBinder.AddMethod(__Method_ServiceInfo, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Gameapi.ServiceInfoRequest, global::Gameapi.ServiceInfoReply>(serviceImpl.ServiceInfo));
    }

  }
  public static partial class Game
  {
    static readonly string __ServiceName = "gameapi.Game";

    static readonly grpc::Marshaller<global::Gameapi.NewPlayerRequest> __Marshaller_gameapi_NewPlayerRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.NewPlayerRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Gameapi.Player> __Marshaller_gameapi_Player = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.Player.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Gameapi.GetPlayerRequest> __Marshaller_gameapi_GetPlayerRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.GetPlayerRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Gameapi.ListPlayersRequest> __Marshaller_gameapi_ListPlayersRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.ListPlayersRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Gameapi.MovePlayerRequest> __Marshaller_gameapi_MovePlayerRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.MovePlayerRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Gameapi.TakeObjectRequest> __Marshaller_gameapi_TakeObjectRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.TakeObjectRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Gameapi.ObjectStatus> __Marshaller_gameapi_ObjectStatus = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.ObjectStatus.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Gameapi.GetObjectTakersRequest> __Marshaller_gameapi_GetObjectTakersRequest = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.GetObjectTakersRequest.Parser.ParseFrom);
    static readonly grpc::Marshaller<global::Gameapi.GetObjectTakersResponse> __Marshaller_gameapi_GetObjectTakersResponse = grpc::Marshallers.Create((arg) => global::Google.Protobuf.MessageExtensions.ToByteArray(arg), global::Gameapi.GetObjectTakersResponse.Parser.ParseFrom);

    static readonly grpc::Method<global::Gameapi.NewPlayerRequest, global::Gameapi.Player> __Method_NewPlayer = new grpc::Method<global::Gameapi.NewPlayerRequest, global::Gameapi.Player>(
        grpc::MethodType.Unary,
        __ServiceName,
        "NewPlayer",
        __Marshaller_gameapi_NewPlayerRequest,
        __Marshaller_gameapi_Player);

    static readonly grpc::Method<global::Gameapi.GetPlayerRequest, global::Gameapi.Player> __Method_GetPlayer = new grpc::Method<global::Gameapi.GetPlayerRequest, global::Gameapi.Player>(
        grpc::MethodType.Unary,
        __ServiceName,
        "GetPlayer",
        __Marshaller_gameapi_GetPlayerRequest,
        __Marshaller_gameapi_Player);

    static readonly grpc::Method<global::Gameapi.ListPlayersRequest, global::Gameapi.Player> __Method_ListPlayers = new grpc::Method<global::Gameapi.ListPlayersRequest, global::Gameapi.Player>(
        grpc::MethodType.ServerStreaming,
        __ServiceName,
        "ListPlayers",
        __Marshaller_gameapi_ListPlayersRequest,
        __Marshaller_gameapi_Player);

    static readonly grpc::Method<global::Gameapi.MovePlayerRequest, global::Gameapi.Player> __Method_MovePlayer = new grpc::Method<global::Gameapi.MovePlayerRequest, global::Gameapi.Player>(
        grpc::MethodType.Unary,
        __ServiceName,
        "MovePlayer",
        __Marshaller_gameapi_MovePlayerRequest,
        __Marshaller_gameapi_Player);

    static readonly grpc::Method<global::Gameapi.TakeObjectRequest, global::Gameapi.ObjectStatus> __Method_TakeObject = new grpc::Method<global::Gameapi.TakeObjectRequest, global::Gameapi.ObjectStatus>(
        grpc::MethodType.Unary,
        __ServiceName,
        "TakeObject",
        __Marshaller_gameapi_TakeObjectRequest,
        __Marshaller_gameapi_ObjectStatus);

    static readonly grpc::Method<global::Gameapi.GetObjectTakersRequest, global::Gameapi.GetObjectTakersResponse> __Method_GetObjectTakers = new grpc::Method<global::Gameapi.GetObjectTakersRequest, global::Gameapi.GetObjectTakersResponse>(
        grpc::MethodType.Unary,
        __ServiceName,
        "GetObjectTakers",
        __Marshaller_gameapi_GetObjectTakersRequest,
        __Marshaller_gameapi_GetObjectTakersResponse);

    /// <summary>Service descriptor</summary>
    public static global::Google.Protobuf.Reflection.ServiceDescriptor Descriptor
    {
      get { return global::Gameapi.GameReflection.Descriptor.Services[1]; }
    }

    /// <summary>Base class for server-side implementations of Game</summary>
    [grpc::BindServiceMethod(typeof(Game), "BindService")]
    public abstract partial class GameBase
    {
      public virtual global::System.Threading.Tasks.Task<global::Gameapi.Player> NewPlayer(global::Gameapi.NewPlayerRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Gameapi.Player> GetPlayer(global::Gameapi.GetPlayerRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task ListPlayers(global::Gameapi.ListPlayersRequest request, grpc::IServerStreamWriter<global::Gameapi.Player> responseStream, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Gameapi.Player> MovePlayer(global::Gameapi.MovePlayerRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Gameapi.ObjectStatus> TakeObject(global::Gameapi.TakeObjectRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

      public virtual global::System.Threading.Tasks.Task<global::Gameapi.GetObjectTakersResponse> GetObjectTakers(global::Gameapi.GetObjectTakersRequest request, grpc::ServerCallContext context)
      {
        throw new grpc::RpcException(new grpc::Status(grpc::StatusCode.Unimplemented, ""));
      }

    }

    /// <summary>Client for Game</summary>
    public partial class GameClient : grpc::ClientBase<GameClient>
    {
      /// <summary>Creates a new client for Game</summary>
      /// <param name="channel">The channel to use to make remote calls.</param>
      public GameClient(grpc::ChannelBase channel) : base(channel)
      {
      }
      /// <summary>Creates a new client for Game that uses a custom <c>CallInvoker</c>.</summary>
      /// <param name="callInvoker">The callInvoker to use to make remote calls.</param>
      public GameClient(grpc::CallInvoker callInvoker) : base(callInvoker)
      {
      }
      /// <summary>Protected parameterless constructor to allow creation of test doubles.</summary>
      protected GameClient() : base()
      {
      }
      /// <summary>Protected constructor to allow creation of configured clients.</summary>
      /// <param name="configuration">The client configuration.</param>
      protected GameClient(ClientBaseConfiguration configuration) : base(configuration)
      {
      }

      public virtual global::Gameapi.Player NewPlayer(global::Gameapi.NewPlayerRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return NewPlayer(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Gameapi.Player NewPlayer(global::Gameapi.NewPlayerRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_NewPlayer, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.Player> NewPlayerAsync(global::Gameapi.NewPlayerRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return NewPlayerAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.Player> NewPlayerAsync(global::Gameapi.NewPlayerRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_NewPlayer, null, options, request);
      }
      public virtual global::Gameapi.Player GetPlayer(global::Gameapi.GetPlayerRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetPlayer(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Gameapi.Player GetPlayer(global::Gameapi.GetPlayerRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_GetPlayer, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.Player> GetPlayerAsync(global::Gameapi.GetPlayerRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetPlayerAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.Player> GetPlayerAsync(global::Gameapi.GetPlayerRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_GetPlayer, null, options, request);
      }
      public virtual grpc::AsyncServerStreamingCall<global::Gameapi.Player> ListPlayers(global::Gameapi.ListPlayersRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return ListPlayers(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncServerStreamingCall<global::Gameapi.Player> ListPlayers(global::Gameapi.ListPlayersRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncServerStreamingCall(__Method_ListPlayers, null, options, request);
      }
      public virtual global::Gameapi.Player MovePlayer(global::Gameapi.MovePlayerRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return MovePlayer(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Gameapi.Player MovePlayer(global::Gameapi.MovePlayerRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_MovePlayer, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.Player> MovePlayerAsync(global::Gameapi.MovePlayerRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return MovePlayerAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.Player> MovePlayerAsync(global::Gameapi.MovePlayerRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_MovePlayer, null, options, request);
      }
      public virtual global::Gameapi.ObjectStatus TakeObject(global::Gameapi.TakeObjectRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return TakeObject(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Gameapi.ObjectStatus TakeObject(global::Gameapi.TakeObjectRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_TakeObject, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.ObjectStatus> TakeObjectAsync(global::Gameapi.TakeObjectRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return TakeObjectAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.ObjectStatus> TakeObjectAsync(global::Gameapi.TakeObjectRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_TakeObject, null, options, request);
      }
      public virtual global::Gameapi.GetObjectTakersResponse GetObjectTakers(global::Gameapi.GetObjectTakersRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetObjectTakers(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual global::Gameapi.GetObjectTakersResponse GetObjectTakers(global::Gameapi.GetObjectTakersRequest request, grpc::CallOptions options)
      {
        return CallInvoker.BlockingUnaryCall(__Method_GetObjectTakers, null, options, request);
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.GetObjectTakersResponse> GetObjectTakersAsync(global::Gameapi.GetObjectTakersRequest request, grpc::Metadata headers = null, global::System.DateTime? deadline = null, global::System.Threading.CancellationToken cancellationToken = default(global::System.Threading.CancellationToken))
      {
        return GetObjectTakersAsync(request, new grpc::CallOptions(headers, deadline, cancellationToken));
      }
      public virtual grpc::AsyncUnaryCall<global::Gameapi.GetObjectTakersResponse> GetObjectTakersAsync(global::Gameapi.GetObjectTakersRequest request, grpc::CallOptions options)
      {
        return CallInvoker.AsyncUnaryCall(__Method_GetObjectTakers, null, options, request);
      }
      /// <summary>Creates a new instance of client from given <c>ClientBaseConfiguration</c>.</summary>
      protected override GameClient NewInstance(ClientBaseConfiguration configuration)
      {
        return new GameClient(configuration);
      }
    }

    /// <summary>Creates service definition that can be registered with a server</summary>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static grpc::ServerServiceDefinition BindService(GameBase serviceImpl)
    {
      return grpc::ServerServiceDefinition.CreateBuilder()
          .AddMethod(__Method_NewPlayer, serviceImpl.NewPlayer)
          .AddMethod(__Method_GetPlayer, serviceImpl.GetPlayer)
          .AddMethod(__Method_ListPlayers, serviceImpl.ListPlayers)
          .AddMethod(__Method_MovePlayer, serviceImpl.MovePlayer)
          .AddMethod(__Method_TakeObject, serviceImpl.TakeObject)
          .AddMethod(__Method_GetObjectTakers, serviceImpl.GetObjectTakers).Build();
    }

    /// <summary>Register service method with a service binder with or without implementation. Useful when customizing the  service binding logic.
    /// Note: this method is part of an experimental API that can change or be removed without any prior notice.</summary>
    /// <param name="serviceBinder">Service methods will be bound by calling <c>AddMethod</c> on this object.</param>
    /// <param name="serviceImpl">An object implementing the server-side handling logic.</param>
    public static void BindService(grpc::ServiceBinderBase serviceBinder, GameBase serviceImpl)
    {
      serviceBinder.AddMethod(__Method_NewPlayer, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Gameapi.NewPlayerRequest, global::Gameapi.Player>(serviceImpl.NewPlayer));
      serviceBinder.AddMethod(__Method_GetPlayer, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Gameapi.GetPlayerRequest, global::Gameapi.Player>(serviceImpl.GetPlayer));
      serviceBinder.AddMethod(__Method_ListPlayers, serviceImpl == null ? null : new grpc::ServerStreamingServerMethod<global::Gameapi.ListPlayersRequest, global::Gameapi.Player>(serviceImpl.ListPlayers));
      serviceBinder.AddMethod(__Method_MovePlayer, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Gameapi.MovePlayerRequest, global::Gameapi.Player>(serviceImpl.MovePlayer));
      serviceBinder.AddMethod(__Method_TakeObject, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Gameapi.TakeObjectRequest, global::Gameapi.ObjectStatus>(serviceImpl.TakeObject));
      serviceBinder.AddMethod(__Method_GetObjectTakers, serviceImpl == null ? null : new grpc::UnaryServerMethod<global::Gameapi.GetObjectTakersRequest, global::Gameapi.GetObjectTakersResponse>(serviceImpl.GetObjectTakers));
    }

  }
}
#endregion
