import openapiTS, { astToString } from 'openapi-typescript'
import { expect, test } from "vitest"

const SNAPSHOT_PATH = "../../tests/snapshots/openapi__open_api_snapshot.snap"

const HEADER = `/**
 * This file is auto-generated. Do not edit manually.
 *
 * Run \`bun --bun run -F @shiori/api-client gen\` to update this file.
 */
`

async function genSchema() {
  let content = await Bun.file(Bun.resolveSync(SNAPSHOT_PATH, import.meta.dirname)).text()

  let jsonStart = content.indexOf("{")
  let json = content.slice(jsonStart)

  let schema = JSON.parse(json)
  let ast = await openapiTS(schema)
  return HEADER + astToString(ast)
}

test("schema.d.ts is up to date", async () => {
  let generated = await genSchema()
  let schemaPath = Bun.resolveSync("./schema.d.ts", import.meta.dirname)
  await expect(generated).toMatchFileSnapshot(schemaPath)
})
