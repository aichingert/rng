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

  onClick(_event: Event): void {

  }
}
