import assert from "node:assert/strict";
import { test } from "node:test";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { InMemoryTransport } from "@modelcontextprotocol/sdk/inMemory.js";
import { SERVER_ID, TOOLS, createServer } from "../src/server.js";

test("every declared tool is registered and described", async () => {
  const server = createServer({ version: "0.0.0" });
  const client = new Client({ name: "test-client", version: "0.0.0" });
  const [clientTransport, serverTransport] = InMemoryTransport.createLinkedPair();

  await Promise.all([
    server.connect(serverTransport),
    client.connect(clientTransport),
  ]);

  assert.equal(client.getServerVersion().name, "template");

  const { tools } = await client.listTools();
  assert.deepEqual(
    tools.map((tool) => tool.name).sort(),
    TOOLS.map((tool) => tool.name).sort()
  );

  for (const tool of tools) {
    assert.ok(tool.description, `${tool.name} needs a description`);
  }

  await server.close();
  void SERVER_ID;
});
