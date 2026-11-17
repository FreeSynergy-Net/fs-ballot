# fs-ballot: Design

`fs-ballot` is the anonymous urn of FreeSynergy: the box that collects ballots without a log. It is used by `fs-forms` (anonymous surveys) and `fs-decisions` (anonymous decisions), and can be used by anyone. Counting is `fs-tally`; who may vote and the smallest group are decided by the program before the vote (decision b7, 17.11.).

## Place in the family (whose data?)

A library has no data of its own. The urn holds ballots and token hashes of a program that owns the question; it does not know persons. Which person got which token is the caller's secret and must not be kept next to the ballots.

## What the box keeps

| Rows | Content | Not in it |
|---|---|---|
| `pending` | ballots not released, in random order | person, time, id, token |
| `batch` | `(stage, ballots)`, shuffled at release | person, time, id, token |
| `token` | `sha256(token)` hex, sorted, unspent only | the token itself, the ballot, who got it |

`Lane` is the pure logic; `Urn` is the trait a backend implements; `Urn::rows` is the typed view of the stored state. There is no JSON column: a backend writes these three kinds of rows as typed columns (ADR 0086; memory "typed storage").

## Promises of `Urn` (checked by `fs-ballot-testdata::conformance`)

1. Once: a token buys one ballot; a second cast and a never-issued token are refused alike (`TokenUnknownOrSpent`), and a refusal changes nothing.
2. No link: a spent token leaves no trace; hashes are sorted (the order tells nothing about who was invited when); `cast` gives no receipt; ballots enter at a random position and leave shuffled.
3. Minimum group: release only with at least `min_group` pending ballots (0 counts as 1); the rest stays hidden and is only counted.

Mutants (in `tests/mutants.rs`): spend forgotten, minimum ignored, arrival order kept, hash log kept. Each is caught by its own check.

## Stored state today (honest)

`fs-forms` keeps its box as ONE JSON document in `fs-objects` (`application/json`, written unlogged, envelope time fixed at the Unix epoch). That is a blob and not what the user decided for typed storage. This library does not add to it: it has no JSON in it; the optional feature `serde` only lets a caller write a `Lane` as it does today. The change to typed columns is its own step (D2b): an `Urn` backend on `fs-objects` with typed tables for the three kinds of rows, moving the box of fs-forms over, with a one-time read of the old JSON document. Until it is done, fs-forms keeps its JSON box (named in its CHANGELOG), now holding `fs_ballot::Lane` values.

## Limits (stated, not hidden)

- The operator sees the token when it is issued and can read memory while a ballot passes. If the creator is the operator, "nobody sees" is not met; the user must be told. Stage A2 (blind signatures, RFC 9474, Privacy Pass RFC 9578) removes this and is a later step.
- A group of fewer than the minimum is never shown; a group of one voter, even above a minimum of 1, shows that voter's ballot. Minimum group at least 3 for decisions.
- Time of arrival and network address belong to the caller (batch release, random waiting).
- Order of rows in a store (insertion order of a database) may leak arrival order; a backend must write pending ballots at a random position or sort them at read (conformance `random_order` reads `rows`).

## Not here

Who is invited, who may vote, methods (`fs-tally`), storage backend (D2b), blind signatures (A2).
