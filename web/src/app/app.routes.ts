import { Routes } from '@angular/router';
import {GameComponent} from "./game/game.component";
import {LobbyComponent} from "./lobby/lobby.component";
import {LoginComponent} from "./login/login.component";
import {AuthService} from "../shared/auth.service";

export const routes: Routes = [
  { path: "lobby", component: LobbyComponent },
  { path: "login", component: LoginComponent },
  { path: "play", component: GameComponent, canActivate: [AuthService] },
  { path: "", redirectTo: "lobby", pathMatch: "full" }
];
