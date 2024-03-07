import { Routes } from '@angular/router';
import {GameComponent} from "./game/game.component";
import {LobbyComponent} from "./lobby/lobby.component";

export const routes: Routes = [
  { path: "lobby", component: LobbyComponent },
  { path: "play", component: GameComponent },
  { path: "", redirectTo: "lobby", pathMatch: "full" }
];
