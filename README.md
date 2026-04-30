# libp2p-ping

Esercizio minimale per prendere confidenza con `libp2p` in Rust.

Il programma avvia un peer che:

- genera una nuova identita` a ogni esecuzione
- apre un listener TCP su una porta casuale
- usa `noise` per la cifratura della connessione
- usa `yamux` per il multiplexing
- abilita il behaviour `ping`
- se riceve un multiaddr da riga di comando, prova a connettersi a quel peer
- stampa a terminale gli eventi principali: indirizzi in ascolto, nuove connessioni, chiusure e ping

## Avvio

Serve avere Rust e Cargo installati.

Per avviare un peer:

```bash
cargo run
```

All'avvio vedrai una riga simile a questa:

```text
Listening on /ip4/127.0.0.1/tcp/52642
```

Quello e` l'indirizzo da usare per collegare un altro peer.

## Prova con 2 terminali

Nel primo terminale:

```bash
cargo run
```

Copia l'indirizzo `Listening on ...` con `127.0.0.1`.

Nel secondo terminale:

```bash
cargo run -- /ip4/127.0.0.1/tcp/52642
```

Ovviamente sostituisci `52642` con la porta reale stampata dal primo peer.

Se tutto va bene, i due peer mostreranno messaggi simili a:

```text
New connection with ...
Event { ..., result: Ok(...) }
```

## Prova con piu` terminali

Puoi aprire un terzo terminale e collegarlo a uno dei peer gia` attivi:

```bash
cargo run -- /ip4/127.0.0.1/tcp/52642
```

Ogni nuova istanza:

- ascolta a sua volta su una porta casuale
- puo` ricevere connessioni
- puo` essere usata come nuovo target per altri peer

## Nota utile

L'identita` del peer non e` persistita: ogni `cargo run` crea un peer nuovo.

Per fermare un peer basta `Ctrl+C`.
