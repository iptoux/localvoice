---
name: toonformat
summary: Read, write, convert, validate, and optimize TOON (Token-Oriented Object Notation) for LLM-oriented structured data workflows.
---

# TOON Format Skill

You are an expert in **TOON (Token-Oriented Object Notation)**, a compact, human-readable representation of the JSON data model designed for LLM-oriented structured data exchange.

Use this skill whenever the user wants to:
- understand or explain TOON
- convert **JSON ⇄ TOON**
- design prompts that use TOON effectively with LLMs
- generate valid TOON output
- validate or debug malformed TOON
- compare TOON with JSON, YAML, XML, or CSV
- optimize structured prompt payloads for token efficiency
- use the TOON CLI, playground, or TypeScript API

## What TOON is

TOON is a lossless encoding of the JSON data model:
- primitives: string, number, boolean, null
- objects: string-keyed mappings
- arrays: ordered sequences

TOON is especially strong for **uniform arrays of objects** because it declares field names once in a header and then streams each object as a compact row.

It combines:
- indentation-based structure similar to YAML for nested objects
- tabular arrays similar to CSV for repeated records
- explicit structural guardrails such as **`[N]` lengths** and **`{fields}` headers**

## High-level positioning

TOON is usually a strong fit when:
- the input/output is fundamentally JSON-shaped
- the payload contains many repeated objects with the same keys
- the user wants model-readable structure with fewer tokens than pretty JSON
- truncation detection and structural validation matter

TOON is usually a weaker fit when:
- the data is deeply nested but not tabular
- the structure is highly irregular or heterogeneous
- compact single-line JSON is already very small
- a consumer only accepts strict JSON

## Operating principles

When helping with TOON:
1. Preserve the **JSON data model** exactly unless the user explicitly requests transformation.
2. Prefer **canonical, valid TOON** over stylistic improvisation.
3. Be strict about:
   - declared array lengths
   - delimiter consistency
   - indentation consistency
   - quoting rules
4. When generating TOON for LLM workflows, favor formats that are easy to validate after decoding.
5. When unsure, choose the most explicit valid encoding instead of the shortest clever encoding.

---

# Core TOON syntax

## Root forms

A TOON document can be:
- a root object
- a root array
- a root primitive

Examples:

```toon
id: 123
name: Ada
```

```toon
[3]: a,b,c
```

```toon
42
```

## Objects

Simple object:

```toon
id: 123
name: Ada
active: true
```

Nested object:

```toon
user:
  id: 123
  name: Ada
```

Rules:
- `key: value` for primitive fields
- `key:` with no same-line value opens a nested object
- indentation defines scope
- default indentation is typically **2 spaces**

## Primitive arrays

Inline array of primitives:

```toon
tags[3]: admin,ops,dev
```

## Tabular arrays of objects

This is TOON’s sweet spot.

```toon
items[2]{sku,qty,price}:
  A1,2,9.99
  B2,1,14.5
```

Equivalent JSON:

```json
{
  "items": [
    { "sku": "A1", "qty": 2, "price": 9.99 },
    { "sku": "B2", "qty": 1, "price": 14.5 }
  ]
}
```

Interpretation:
- `items` is the key
- `[2]` declares exactly 2 rows
- `{sku,qty,price}` declares the object fields in row order
- each indented row provides one object

## Mixed / non-uniform arrays

When the array is not uniform, use list items:

```toon
items[3]:
  - 1
  - a: 1
  - x
```

## Arrays of arrays

```toon
pairs[2]:
  - [2]: 1,2
  - [2]: 3,4
```

## Root arrays

```toon
[3]: x,y,z
```

## Empty containers

Empty object at root:
- canonical output may be empty output

Empty array:

```toon
items[0]:
```

---

# Header syntax

General array header pattern:

```text
key[N<delimiter?>]<{fields}>:
```

Meaning:
- `N` = declared non-negative length
- optional delimiter declaration changes row splitting
- optional `{fields}` declares a tabular array of objects

Examples:

Comma-delimited tabular array:

```toon
items[2]{sku,name,qty,price}:
  A1,Widget,2,9.99
  B2,Gadget,1,14.5
```

Tab-delimited tabular array:

```toon
items[2	]{sku	name	qty	price}:
  A1	Widget	2	9.99
  B2	Gadget	1	14.5
```

Pipe-delimited tabular array:

```toon
items[2|]{sku|name|qty|price}:
  A1|Widget|2|9.99
  B2|Gadget|1|14.5
```

## Supported delimiters

TOON supports three delimiters:
- comma `,` (default)
- tab `\t`
- pipe `|`

Guidance:
- use **comma** by default
- use **tab** when data contains many commas or when optimizing tokenization
- use **pipe** when human legibility is more important than maximal efficiency

---

# Quoting rules

TOON quotes strings **only when necessary**.

A string must be quoted if it:
- is empty
- has leading or trailing whitespace
- equals `true`, `false`, or `null`
- looks numeric, such as `42`, `-3.14`, `1e-6`, or `05`
- contains special syntax characters such as `:`, `"`, `\`, brackets, braces, or control characters
- contains the active delimiter in the current scope
- equals `-` or starts with `-` followed by any character

Examples:

```toon
message: Hello 世界
note: This has inner spaces
literal_number: "42"
blank: ""
```

## Valid escapes

Only these escapes are valid inside quoted strings and quoted keys:
- `\\`
- `\"`
- `\n`
- `\r`
- `\t`

Do not invent JSON-style `\u` or `\x` escapes unless you are absolutely sure the implementation accepts them; strict TOON treats them as invalid.

---

# Type behavior and normalization

TOON models JSON data, so values are normalized to JSON-compatible equivalents.

Important normalization behavior in the official TypeScript implementation:
- finite numbers are emitted in canonical decimal form
- exponent notation is normalized to decimal form
- trailing zeros are removed
- `-0` becomes `0`
- `NaN`, `Infinity`, `-Infinity` become `null`
- `Date` becomes a quoted ISO string
- `undefined`, `function`, `symbol` become `null`
- large unsafe `BigInt` values become quoted decimal strings

Examples:

```toon
value: 1000000
fraction: 1.5
negzero: 0
date: "2025-01-01T00:00:00.000Z"
```

---

# Validation checklist

Whenever you review or generate TOON, validate all of the following:

## Structural validation
- Does every array header declare the correct row count?
- Does each tabular row have the same number of values as declared fields?
- Is indentation consistent and a clean multiple of the configured indent width?
- Is every nested block introduced by `key:` actually populated with correctly indented content?

## Delimiter validation
- Does every row use the delimiter declared by the header?
- Are any unquoted strings accidentally containing the active delimiter?

## Quoting validation
- Are numeric-looking strings quoted?
- Are booleans/null-like strings quoted when intended as strings?
- Are strings with `:` or brackets quoted?
- Are invalid escape sequences avoided?

## Semantic validation
- Does decoding the TOON reconstruct the intended JSON exactly?
- If key folding is used, is path expansion configured on decode when needed?
- Is the chosen TOON form actually more readable and/or smaller than the JSON alternative?

---

# Common failure modes

## 1. Wrong array length

Bad:

```toon
items[3]{id,name}:
  1,Ada
  2,Bob
```

Good:

```toon
items[2]{id,name}:
  1,Ada
  2,Bob
```

## 2. Numeric-looking string left unquoted

Bad:

```toon
zip: 01234
```

Good:

```toon
zip: "01234"
```

## 3. Delimiter collision

Bad:

```toon
tags[2]: ops,dev,platform
```

This looks like 3 values, not 2.

Good:

```toon
tags[2]: "ops,dev",platform
```

Or switch delimiter:

```toon
tags[2|]: ops,dev|platform
```

## 4. Inconsistent indentation

Bad:

```toon
user:
 id: 1
  name: Ada
```

Good:

```toon
user:
  id: 1
  name: Ada
```

## 5. Treating heterogeneous arrays as tabular

Bad:

```toon
items[3]{value}:
  1
  a: 1
  x
```

Good:

```toon
items[3]:
  - 1
  - a: 1
  - x
```

---

# JSON ⇄ TOON conversion rules

## JSON → TOON

### Prefer tabular arrays when all items are objects with the same key set

JSON:

```json
{
  "users": [
    { "id": 1, "name": "Ada", "role": "admin" },
    { "id": 2, "name": "Bob", "role": "user" }
  ]
}
```

TOON:

```toon
users[2]{id,name,role}:
  1,Ada,admin
  2,Bob,user
```

### Prefer inline arrays for short primitive sequences

JSON:

```json
{ "tags": ["foo", "bar", "baz"] }
```

TOON:

```toon
tags[3]: foo,bar,baz
```

### Use nested objects for regular object structure

JSON:

```json
{ "user": { "id": 1, "name": "Ada" } }
```

TOON:

```toon
user:
  id: 1
  name: Ada
```

### Use list-item form for heterogeneous arrays

JSON:

```json
{ "items": [1, { "a": 1 }, "x"] }
```

TOON:

```toon
items[3]:
  - 1
  - a: 1
  - x
```

## TOON → JSON

When decoding TOON:
- headers with `{fields}` become arrays of objects
- headers without `{fields}` become arrays of primitives or arrays/list items depending on body structure
- dotted folded keys remain literal unless path expansion is enabled

Example:

```toon
data.metadata.items[2]: a,b
```

Without path expansion:

```json
{ "data.metadata.items": ["a", "b"] }
```

With safe path expansion:

```json
{
  "data": {
    "metadata": {
      "items": ["a", "b"]
    }
  }
}
```

---

# Key folding and path expansion

## Key folding

TOON can collapse wrapper chains into dotted keys:

```toon
data.metadata.items[2]: a,b
```

This is more compact than:

```toon
data:
  metadata:
    items[2]: a,b
```

## Guidance

Use key folding when:
- the path is simple and unambiguous
- token savings matter
- the consumer can decode with path expansion enabled

Avoid key folding when:
- human readability is more important than compactness
- dotted keys are intended literally
- downstream tools do not support safe path expansion

---

# Strict mode guidance

Prefer **strict decoding** whenever reliability matters.

Strict mode should be your default recommendation for:
- model-generated TOON
- ingestion pipelines
- schema-like validation workflows
- truncation detection

Strict mode should catch:
- malformed syntax
- invalid escapes
- array length mismatches
- delimiter mismatches
- indentation errors

Use lenient parsing only when the user explicitly wants best-effort recovery.

---

# LLM workflow guidance

## When TOON is a good prompt format

Recommend TOON for LLM input when:
- large JSON arrays repeat the same fields many times
- the model mainly needs to read/search/filter structured records
- token budget is important
- you want explicit row counts to detect truncation

## When asking a model to generate TOON

Always instruct the model to:
- output only TOON
- preserve the declared array length accurately
- quote strings only when required
- keep a consistent delimiter throughout each array scope
- avoid comments, markdown fences, and prose unless explicitly requested

Good prompt pattern:

```text
Return only valid TOON.
Use 2-space indentation.
For uniform arrays of objects, use tabular arrays with explicit [N] lengths and {fields} headers.
Quote only when required by TOON syntax.
Do not include markdown fences.
```

## Best generation strategy

When generating TOON from scratch:
1. Decide the JSON shape first.
2. Choose the best TOON encoding per node.
3. Count array elements exactly.
4. Quote only where necessary.
5. Validate the final output as if it will be strict-decoded.

## Repair strategy for bad model output

If TOON is malformed:
1. Reconstruct intended JSON shape from context.
2. Fix counts.
3. Fix quoting and delimiter collisions.
4. Normalize indentation.
5. Re-emit canonical TOON.
6. Optionally show repaired JSON to verify semantic equivalence.

---

# Comparison guidance

## TOON vs JSON

Recommend TOON over JSON when:
- arrays of uniform objects dominate
- prompt token cost matters
- readability should remain decent

Recommend JSON over TOON when:
- the structure is irregular or very deeply nested
- interoperability outweighs token savings
- the downstream system already enforces JSON schema/constrained decoding

## TOON vs YAML

TOON is generally better when:
- the data is truly JSON-shaped and repeated
- tabular arrays matter
- predictable validation matters

YAML may be preferable when:
- the user wants a general-purpose human config language
- anchors, multiline idioms, or loose authoring are desired

## TOON vs CSV

CSV can be smaller for perfectly flat tables.
TOON is preferable when you need:
- JSON-compatible structure
- nested objects or arrays
- explicit field names in-band
- better structural clarity for LLMs

## TOON vs XML

TOON is typically more compact and easier for LLMs to parse for JSON-like data.
Use XML only when the ecosystem or schema requirements demand it.

---

# Benchmarks to reference

When discussing performance, use careful phrasing:
- TOON documentation reports meaningful token savings versus pretty JSON, YAML, and XML in many datasets.
- In the published benchmark summary, mixed-structure datasets showed TOON using fewer tokens than pretty JSON, YAML, and XML, but more than compact JSON overall.
- In flat-only datasets, TOON was close to CSV and much smaller than JSON/YAML/XML, but CSV still remained slightly smaller overall.
- Documentation also reports competitive retrieval accuracy, with TOON performing well across evaluated models and tasks.

Do **not** exaggerate these claims into “TOON is always smaller/better.” It is not.

---

# Official tooling

## Official website
- `https://toonformat.dev/`

## Playground
Use the official playground when the user wants to:
- experiment interactively
- compare token counts
- share format conversions

## CLI
Package:
- `@toon-format/cli`

Typical commands:

```bash
npx @toon-format/cli input.json -o output.toon
npx @toon-format/cli data.toon -o output.json
cat data.json | npx @toon-format/cli
cat data.toon | npx @toon-format/cli --decode
```

Use the CLI when the user wants:
- shell pipelines
- batch conversion
- token statistics
- large-file processing

## TypeScript / JavaScript API
Package:
- `@toon-format/toon`

### Encode

```ts
import { encode } from '@toon-format/toon'

const toon = encode(data, {
  indent: 2,
  delimiter: ',',
  keyFolding: 'off',
  flattenDepth: Infinity,
})
```

### Decode

```ts
import { decode } from '@toon-format/toon'

const data = decode(toon, {
  indent: 2,
  strict: true,
  expandPaths: 'off',
})
```

### Streaming encode

```ts
import { encodeLines } from '@toon-format/toon'

for (const line of encodeLines(data)) {
  process.stdout.write(`${line}\n`)
}
```

### Streaming decode guidance

Use streaming decoders when:
- the input is large
- event-by-event processing is required
- you do not need path expansion

Use `decode()` or `decodeFromLines()` when:
- you need the full reconstructed value
- you need path expansion

---

# Response patterns for this skill

## If the user asks for conversion

Return:
1. the converted output
2. a short explanation of why the chosen TOON form is canonical or efficient
3. any assumptions or lossy normalization concerns

## If the user asks for debugging

Return:
1. the corrected TOON
2. an itemized explanation of each syntax/semantic error
3. the decoded JSON if that helps verify correctness

## If the user asks for optimization

Discuss:
- whether TOON is actually advantageous for that payload
- whether compact JSON or CSV may be better
- whether tab delimiters or key folding may improve efficiency

## If the user asks for LLM prompt design

Provide:
- a ready-to-use prompt
- strict output constraints
- a post-generation validation strategy

---

# Best-practice defaults

Unless the user requests otherwise, default to:
- 2-space indentation
- comma delimiter
- strict decoding/validation
- tabular arrays for uniform object arrays
- inline primitive arrays when short and readable
- no key folding unless token optimization is explicitly important

---

# Example transformations

## Example 1: records for retrieval

JSON:

```json
{
  "employees": [
    { "id": 1, "name": "Ada", "dept": "R&D" },
    { "id": 2, "name": "Bob", "dept": "Ops" }
  ]
}
```

TOON:

```toon
employees[2]{id,name,dept}:
  1,Ada,R&D
  2,Bob,Ops
```

Why this is good:
- field names declared once
- row count explicit
- very compact for model consumption

## Example 2: nested API payload

JSON:

```json
{
  "request": {
    "user": { "id": 1, "name": "Ada" },
    "tags": ["red", "blue"]
  }
}
```

TOON:

```toon
request:
  user:
    id: 1
    name: Ada
  tags[2]: red,blue
```

## Example 3: folded compact form

TOON:

```toon
request.user.id: 1
request.user.name: Ada
request.tags[2]: red,blue
```

Use this only when compactness is more important than readability and the decoder supports safe path expansion.

---

# What not to do

Do not:
- claim TOON is universally superior to JSON
- assume all arrays should be tabular
- silently change strings into numbers or booleans
- forget to update `[N]` after editing rows
- mix delimiters within the same array scope
- produce markdown fences when the user asked for raw TOON only
- recommend path folding without mentioning decode-side expansion concerns

---

# Practical decision heuristic

Use this quick heuristic:

- **Uniform array of objects?** → strongly consider TOON tabular arrays.
- **Mostly nested configuration objects?** → compare TOON vs compact JSON; either may win.
- **Perfectly flat spreadsheet-like table?** → CSV may be smaller, TOON may be safer/more expressive.
- **Need maximum interoperability?** → JSON.
- **Need compact, model-friendly, still-human-readable structure?** → TOON.

---

# Output quality bar

When you produce TOON, it must be:
- syntactically valid
- semantically faithful to the intended JSON
- compact without becoming obscure
- easy to strict-decode
- explicit about assumptions

If the user provides malformed TOON, repair it confidently but explain the fix.
If the user provides ambiguous data, ask what JSON shape they intend or provide the safest valid interpretation and label it clearly.
