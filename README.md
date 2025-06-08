## Repertoire

Tool to help me memorize chess openings. Built as a CLI application that loads and manages `.opening` files. Contains some tools for interacting with and annotating an opening you are exploring.

### The `.opening` file format

This is a JSON-like format for saving an opening study. Save your openings using this file format. Here's an example for the Caro Kann:

```
{   
    "author": "Shreyas",
    "date_modified": "1749349708",
    "name": "Caro Kann",
    "description": "Collection of common Caro Kann ideas. Collected from a bunch of different YouTube videos",
    "moves": {
        "e4": {
            "note": "The move that black responds most often to with the Caro Kann."
        },
        "e4-c6-d4-d5-e5": {
            "note": "The advanced Caro Kann. White fights for space and black must quickly develop."
        }
    }
}
```

### Commands

```
Usage: repertoire [OPTIONS]

Options:
  -s, --study <FILENAME>  Load and study an existing opening file
  -n, --new               Start a new opening study session
  -h, --help              Print help
  -V, --version           Print version
```

### Example Study Session

```
❯ ./repertoire --study ../../../samples/caro_kann.opening

Studying: Caro Kann
Author: Shreyas
Description: Collection of common Caro Kann ideas. Collected from a bunch of different YouTube videos

Available commands:
  list    - List all move sequences
  show <sequence> - Show notes for a specific sequence
  play    - Start a practice game
  quit    - Exit the study session

> list

Move sequences:
  e4
  e4-c6

> show e4

Notes for e4:
The move that black responds most often to with the Caro Kann.

> play

Starting practice game. Available commands:
  move <chess move> - Make a move (e.g., 'move e4')
  explore          - Open current position in LiChess
  stop            - End practice game

(practice) > move e4

Your move: e4
Note: The move that black responds most often to with the Caro Kann.

Computer plays: c6
Note: The starting move for black. Looking to push d5 and strike in the center.

(practice) > explore

Open this URL in your browser to explore the position:
https://lichess.org/analysis/pgn/1.%20e4%20c6

(practice) > stop
> quit

```