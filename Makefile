default:
	@cd mcp-server && make build-fts-graph

push:
	@cd mcp-server && make push

build:
	@cd mcp-server && make build-release-fts-graph

reindex: build
	@cd mcp-server && ./bin/music-theory-mcp index
	@cd mcp-server && ./bin/music-theory-mcp graph build

