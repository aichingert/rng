import {Component} from '@angular/core';
import {LobbyService} from "../../shared/lobby.service";
import {Router} from "@angular/router";
import {MatButton} from "@angular/material/button";

@Component({
  selector: 'app-lobby',
  standalone: true,
  imports: [
    MatButton
  ],
  templateUrl: './lobby.component.html',
  styleUrl: './lobby.component.css'
})
export class LobbyComponent {
  constructor(
    private router: Router,
    public lobbyService: LobbyService,
  ) {}

  joinQueue(_event: Event): void {
    if (!this.lobbyService.hasUsername()) {
      this.router.navigate((["login"])).catch(e => console.error(e));
      return;
    }

    this.lobbyService.joinQueue();
    this.router.navigate(["play"]).then(r => {});
  }
}
