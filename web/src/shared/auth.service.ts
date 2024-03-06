import { Injectable } from '@angular/core';
import {LobbyService} from "./lobby.service";
import {Router} from "@angular/router";

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  constructor(
    private router: Router,
    private lobbyService: LobbyService,
  ) { }

  canActivate(): boolean {
    return true;
  }
}
