import { Component } from '@angular/core';
import {ChannelService} from "../../shared/channel.service";
import {GameMove} from "../../shared/generated/channel";

@Component({
  selector: 'app-game',
  standalone: true,
  imports: [],
  templateUrl: './game.component.html',
  styleUrl: './game.component.css'
})
export class GameComponent {
  public gameMoves: GameMove[] = [];

  constructor(private channelService: ChannelService) {
    this.channelService.getGameUpdates("").subscribe((gameMove) => {
      this.gameMoves.push(gameMove);
    })
  }
}
