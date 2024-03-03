import { Component } from '@angular/core';
import {LobbyService} from "../../shared/lobby.service";
import {Message} from "../../shared/generated/subtac";

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
    this.lobbyService.getMessages.subscribe((msg: Message) => {
      this.messages.push(msg.content);
    })
  }

  public onKey(event: KeyboardEvent): void {
    if (event.key === "Enter") {
      this.lobbyService.sendMessage({content: this.message});
      this.message = "";
      return;
    }

    this.message = (event.target as HTMLInputElement).value;
  }
}
