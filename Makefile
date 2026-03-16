default:
	@cd mcp-server && make build-full

push:
	@cd mcp-server && make push

build:
	@cd mcp-server && make build-release-full

reindex: build
	@cd mcp-server && ./bin/music-theory-mcp index
	@cd mcp-server && ./bin/music-theory-mcp graph build

