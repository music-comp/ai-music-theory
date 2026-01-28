default:
	@cd mcp-server && make build-fts-graph

push:
	@cd mcp-server && make push
