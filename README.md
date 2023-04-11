# pretty_fen

Format Forsyth–Edwards Notation into pretty human-readable form:
```
 r  n  b  q  k  b  n  r  /
 p  p  p  p  p  p  p  p  /
 .  .  .  .  .  .  .  .  /
 .  .  .  .  .  .  .  .  /
 .  .  .  .  .  .  .  .  /
 .  .  .  .  .  .  .  .  /
 P  P  P  P  P  P  P  P  /
 R  N  B  Q  K  B  N  R
```

This preserves only the board position and discards castling etc. information.

## Example usage
compile:
```
rustc -O -o pretty_fen src/main.rs
```
```
./pretty_fen "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
```
```
./pretty_fen "rnbqkbnr/pppppppp/8/8/4(P)3/8/PPPP(.)PPP/RNBQKBNR w KQkq - 0 1"
```
```
 r  n  b  q  k  b  n  r  /
 p  p  p  p  p  p  p  p  /
 .  .  .  .  .  .  .  .  /
 .  .  .  .  .  .  .  .  /
 .  .  .  . (P) .  .  .  /
 .  .  .  .  .  .  .  .  /
 P  P  P  P (.) P  P  P  /
 R  N  B  Q  K  B  N  R
 ```
Supports highlighting using parentheses:
```
./pretty_fen "rnbqkbnr/pppppppp/8/2(4)2/2(4)2/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
```
```
 r  n  b  q  k  b  n  r  /
 p  p  p  p  p  p  p  p  /
 .  .  .  .  .  .  .  .  /
 .  . (.  .  .  .) .  .  /
 .  . (.  .  .  .) .  .  /
 .  .  .  .  .  .  .  .  /
 P  P  P  P  P  P  P  P  /
 R  N  B  Q  K  B  N  R
```

### TODO:
- Error handling
- Input validation
- Help
- Read PGN
    - Highlight the latest move
- Flip board
- Add option to print coordinates on the edge of the board
