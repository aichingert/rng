@0xd61729e8a8f619b6;

struct Player {
    id @0 :Uint16;
    lobby @1 :UInt16;
}

struct Game {
    players @0 :List(Player);
    board @1 :Board;
}

struct Board {
    union {
        fields @1 :List(UInt16);
        nested @2 :List(Board);
    }
}

interface Lobby {
    create @0 () -> (p:Player);
    join @1 (lobbyId :UInt16) -> (g:Game);
    delete @2 (lobbyId :UInt16) -> ();
}
