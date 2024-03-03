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
    if (this.lobbyService.hasUsername()) {
      return true;
    } else {
      this.router.navigate(["login"]).catch(console.error);
      return false;
    }
  }
}
