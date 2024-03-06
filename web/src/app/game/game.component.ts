import { Component } from '@angular/core';
import {LobbyService} from "../../shared/lobby.service";

@Component({
  selector: 'app-game',
  standalone: true,
  imports: [],
  templateUrl: './game.component.html',
  styleUrl: './game.component.css'
})
export class GameComponent {
  public messages: string[] = [];
  public message: string = "";

  constructor(private lobbyService: LobbyService) {

  }

  public onKey(event: KeyboardEvent): void {
  }
}
