# Changelog

Version = tag; all crates of the workspace carry the same version (ADR 0064, 0.0.N). No tag is set by the work branch.

## 0.0.1

### English

- First version as a library of its own: `Lane` (the pure logic of the box: random insertion, shuffled release in batches of a minimum group, sorted token hashes), `Meta`, the trait `Urn` with `Rows` (the stored state as typed rows), one-time tokens (`random::token`, `hash_token`), `Error`.
- The logic moved out of the box of fs-forms (own code), made generic over the ballot type; the stored field names of `Lane` and `Batch` are kept for the feature `serde`.
- New crate `fs-ballot-testdata` (not published): `MemoryUrn` and the conformance checks (once only, unknown token, no link between token and ballot, random order, minimum group, zero minimum, stages); mutants of four promises are caught.
- Not here: a storage backend with typed columns (own step; the caller keeps the lane), anonymity against the operator (blind signatures, a later stage), counting (`fs-tally`).

### Deutsch

- Erste Version als eigene Bibliothek: `Lane` (reine Urnen-Logik: Einfügen an Zufallsstelle, gemischte Freigabe in Stapeln ab Mindestgruppe, sortierte Token-Hashes), `Meta`, der Trait `Urn` mit `Rows` (gespeicherter Zustand als typisierte Zeilen), Einmal-Tokens (`random::token`, `hash_token`), `Error`.
- Die Logik ist aus der Urne von fs-forms umgezogen (eigener Code) und über den Stimmzettel-Typ verallgemeinert; die gespeicherten Feldnamen von `Lane` und `Batch` bleiben für das Feature `serde` gleich.
- Neues Crate `fs-ballot-testdata` (nicht veröffentlicht): `MemoryUrn` und die Konformitätstests (nur einmal, unbekanntes Token, kein Zusammenhang Token-Stimme, Zufallsreihenfolge, Mindestgruppe, Mindestgruppe 0, Stufen); Mutanten von vier Zusagen werden erkannt.
- Nicht hier: ein Speicher mit typisierten Spalten (eigener Schritt; der Aufrufer hält die Lane), Anonymität gegenüber dem Betreiber (blinde Signaturen, spätere Stufe), Zählen (`fs-tally`).
