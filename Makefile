OS := linux
ARCH := x64
CLIENT_DIR := MurderManor/Assets/Scripts/Grpc 
CLIENT_ASSETS_DIR := MurderManor/Assets/
PROTOC_PATH := tools/$(OS)_$(ARCH)

# https://packages.grpc.io/archive/2019/12/a02d6b9be81cbadb60eed88b3b44498ba27bcba9-edd81ac6-e3d1-461a-a263-2b06ae913c3f/index.xml
BASE_URL := https://packages.grpc.io/archive/2019/12/a02d6b9be81cbadb60eed88b3b44498ba27bcba9-edd81ac6-e3d1-461a-a263-2b06ae913c3f

.PHONY: client-tools
client-tools:
	wget $(BASE_URL)/csharp/Grpc.Tools.2.26.0-dev201912021138.nupkg -O grpc-tools.zip
	unzip -o grpc-tools.zip "$(PROTOC_PATH)/*"
	find $(PROTOC_PATH) -type f -exec chmod +x {} \;
	$(RM) -f grpc-tools.zip

.PHONY: client-unity
client-unity:
	wget $(BASE_URL)/csharp/grpc_unity_package.2.26.0-dev.zip \
	-O grpc-unity-package.zip && \
	unzip -o grpc-unity-package.zip -d $(CLIENT_ASSETS_DIR) && \
	$(RM) -f grpc-unity-package.zip

.PHONY: proto
proto: server/proto/game.proto
	mkdir -p $(CLIENT_DIR)
	$(PROTOC_PATH)/protoc $^ \
	-I server/proto \
	--csharp_out=$(CLIENT_DIR) \
	--grpc_out=$(CLIENT_DIR) \
	--plugin=protoc-gen-grpc=$(PROTOC_PATH)/grpc_csharp_plugin
