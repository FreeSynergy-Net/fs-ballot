# FreeSynergy Ballot: an anonymous ballot box that keeps no log

**A building block. A vote goes in and cannot be traced back to the person, and each invited person can vote once.**

`fs-ballot` is a small Rust library for the box that collects anonymous ballots. Every invited person gets a one-time token. The box keeps only the hash of the token, in a sorted list apart from the ballots, and removes the hash when the token is used. A ballot goes in at a random position, with no person, no time and no id. Ballots come out only in shuffled batches of at least a minimum group size, so a group of one cannot reveal one person's choice.

## What it can do today

Latest tag: none yet. The version in this tree is `0.0.1`.

- `Lane`: the pure logic (pending ballots, released batches, unspent token hashes). `Meta` lets counts travel with a batch.
- `Urn`: the trait a storage backend implements: `issue_token`, `admit`, `cast`, `release`, `released`, `held_back`, `rows`. `rows` is the stored state as typed rows (no JSON, no blob): pending ballots, batches, token hashes.
- Tokens: 256 random bits from the operating system (`getrandom`), hashed with SHA-256.
- `fs-ballot-testdata` (a test double, not published): `MemoryUrn` and the conformance checks every `Urn` must pass: once only, unknown token, no link between token and ballot, random order, minimum group, stages. The tests also hold four mutants (a token that is not spent, an ignored minimum group, ballots in arrival order, a log of token hashes); each one is caught.
- Optional feature `serde`: `Lane` and `Batch` can be written by a caller that keeps them in a document store.

## What it does not do

- It is only the box. It does not know who is invited, who may vote, or what a ballot contains; the program decides the electorate, the method and the minimum group before the vote.
- Counting is `fs-tally`.
- A storage backend with typed columns is not here yet (planned as its own step). Today the caller keeps the `Lane` in memory or in its own store.
- Stated limits: whoever runs the machine can read memory while a ballot passes and sees the token when it is issued; the network address and the time a request arrives are the caller's to protect; a small group shows its members' choices (so choose the minimum group with care, at least 3). Two copies of the stored state taken around one vote (a backup, a snapshot) show which token hash went and which ballot came; keep such copies away from the issuer of the tokens.

## Part of FreeSynergy

FreeSynergy is a family of programs that are free for everyone, forever, and that keep your data yours. The documentation, the manifesto and the overall plan live in [fs-documentation](https://github.com/FreeSynergy/fs-documentation); the reading version is at <https://freesynergy.github.io/fs-documentation/>.

## For developers

- Workspace crates: `fs-ballot` (library) and `fs-ballot-testdata` (test double, `publish = false`). Patterns: Aggregate (`Lane`), Strategy behind a small trait (`Urn`), Facade (`random`, `token`).
- Gates: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings -W clippy::pedantic`, `cargo test --workspace --all-features --locked`. Minimum Rust 1.89.
- Licence: MIT OR Apache-2.0. Claude (Anthropic) helped write this code; the work is checked by tests and by an independent review.
- Origin: the box logic moved out of the survey program [fs-forms](https://github.com/FreeSynergy/fs-forms) (own code).

---

# FreeSynergy Ballot: eine anonyme Urne ohne Protokoll

**Ein Baustein. Eine Stimme geht hinein und lässt sich nicht auf die Person zurückführen; jede eingeladene Person kann einmal abstimmen.**

`fs-ballot` ist eine kleine Rust-Bibliothek für die Urne, die anonyme Stimmzettel sammelt. Jede eingeladene Person bekommt ein Einmal-Token. Die Urne behält nur den Hash des Tokens, in einer sortierten Liste getrennt von den Stimmzetteln, und entfernt ihn, sobald das Token benutzt ist. Ein Stimmzettel geht an einer Zufallsstelle hinein, ohne Person, ohne Zeit und ohne Kennung. Stimmzettel kommen nur in gemischten Stapeln von mindestens der Mindestgruppen-Größe heraus; so verrät eine Gruppe von einer Person nicht deren Wahl.

## Was heute geht

Letzter Tag: noch keiner. Die Version in diesem Baum ist `0.0.1`.

- `Lane`: die reine Logik (wartende Stimmzettel, freigegebene Stapel, unverbrauchte Token-Hashes). `Meta` lässt Zählwerte mit einem Stapel mitreisen.
- `Urn`: der Trait, den ein Speicher umsetzt: `issue_token`, `admit`, `cast`, `release`, `released`, `held_back`, `rows`. `rows` ist der gespeicherte Zustand als typisierte Zeilen (kein JSON, kein Blob).
- Tokens: 256 Zufallsbits vom Betriebssystem, mit SHA-256 gehasht.
- `fs-ballot-testdata` (Test-Attrappe, nicht veröffentlicht): `MemoryUrn` und die Konformitätstests: nur einmal, unbekanntes Token, kein Zusammenhang Token-Stimme, Zufallsreihenfolge, Mindestgruppe, Stufen. Dazu vier Mutanten, die alle erkannt werden.
- Optionales Feature `serde` für Aufrufer, die die `Lane` in einem Dokumentspeicher halten.

## Was es nicht tut

- Es ist nur die Urne. Wer eingeladen ist, wer abstimmen darf und was ein Stimmzettel enthält, weiß es nicht; das Programm legt Wahlvolk, Verfahren und Mindestgruppe vor der Abstimmung fest.
- Zählen macht `fs-tally`.
- Ein Speicher mit typisierten Spalten gibt es hier noch nicht (eigener Schritt geplant). Heute hält der Aufrufer die `Lane` im Speicher oder in seinem eigenen Speicher.
- Benannte Grenzen: wer die Maschine betreibt, kann den Speicher lesen, während ein Stimmzettel vorbeikommt, und sieht das Token bei der Ausgabe; Netzadresse und Ankunftszeit einer Anfrage zu schützen ist Sache des Aufrufers; eine kleine Gruppe zeigt die Wahl ihrer Mitglieder (Mindestgruppe mit Bedacht wählen, mindestens 3). Zwei Kopien des gespeicherten Zustands rund um eine Stimme (Sicherung, Snapshot) zeigen, welcher Token-Hash ging und welcher Stimmzettel kam; solche Kopien gehören nicht zu dem, der die Tokens ausgibt.

## Teil von FreeSynergy

FreeSynergy ist eine Familie von Programmen, die für alle frei und für immer frei sind und deine Daten bei dir lassen. Dokumentation, Manifest und Gesamtplan liegen in [fs-documentation](https://github.com/FreeSynergy/fs-documentation); die Leseversion ist <https://freesynergy.github.io/fs-documentation/>.

## Für Entwickelnde

- Workspace-Crates: `fs-ballot` und `fs-ballot-testdata`. Lizenz: MIT OR Apache-2.0. Claude (Anthropic) hat beim Schreiben geholfen; die Arbeit wird durch Tests und eine unabhängige Prüfung abgesichert.
- Herkunft: die Urnen-Logik stammt aus dem Umfrage-Programm [fs-forms](https://github.com/FreeSynergy/fs-forms) (eigener Code).
