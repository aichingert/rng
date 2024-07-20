@0xd61729e8a8f619b6;

struct Player {
    id @0 :Uint16;
    lobby @1 :UInt16;
}

struct Game {
    players @0 :List(Player);

    union {
        board @1 :List(UInt16);
        nested @2 :List(Game);
    }
}

interface Lobby {
    create @0 () -> (p:Player);
    join @1 (lobbyId :UInt16) -> (g:Game);
    delete @2 (lobbyId :UInt16) -> ();
}

