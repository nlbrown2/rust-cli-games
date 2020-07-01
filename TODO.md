# Program structure
## main.rs
- Parses arguments
- creates Game object
- calls Game.run()
- handles Game.run() errors?
    - Let Game.run() bubble errors up instead of assert falsing for testability?

## lib.rs
    - imports Game?

## game.rs
    - no idea where to put this yet
    - sets up game
    - owns a game board
    - given players from main?
        - not yet- for now, assume two human players
    - while game  not done, get next players move, make move, change next player
    - needs to know when game is done

## game-board.rs
    - owns game board memory
    - knows how a player makes a move
    - knows if the game is over or not
    - knows if a player can go or not
    - printable
        - scores per player?
        - current positions

## player.rs
    - Knows how to ask a player where to go
    - printable
