import { Component } from '@angular/core';
import {ChannelService} from "../../shared/channel.service";
import {GameMove} from "../../shared/generated/channel";
import {MatInput} from "@angular/material/input";
import {FormsModule} from "@angular/forms";

@Component({
  selector: 'app-game',
  standalone: true,
  imports: [
    MatInput,
    FormsModule
  ],
  templateUrl: './game.component.html',
  styleUrl: './game.component.css'
})
export class GameComponent {
  public position: string = "";
  public gameMoves: GameMove[] = [];


  constructor(private channelService: ChannelService) {
    this.channelService.getGameUpdates("").subscribe((gameMove) => {
      this.gameMoves.push(gameMove);
    })
  }

  onSubmit(_event: Event): void {
    this.channelService.sendMove(parseInt(this.position));
  }
}
