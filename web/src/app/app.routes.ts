import { Routes } from '@angular/router';
import {GameComponent} from "./game/game.component";
import {LobbyComponent} from "./lobby/lobby.component";
import {LoginComponent} from "./login/login.component";
import {RegisterComponent} from "./register/register.component";

export const routes: Routes = [
  { path: "lobby", component: LobbyComponent },
  { path: "login", component: LoginComponent },
  { path: "register", component: RegisterComponent },
  { path: "play", component: GameComponent },
  { path: "", redirectTo: "lobby", pathMatch: "full" }
];
