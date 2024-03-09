import { Component } from '@angular/core';
import {ChannelService} from "../../shared/channel.service";
import {MatInput} from "@angular/material/input";
import {FormsModule} from "@angular/forms";
import {BoardComponent} from "../board/board.component";

@Component({
  selector: 'app-game',
  standalone: true,
  imports: [
    MatInput,
    FormsModule,
    BoardComponent
  ],
  templateUrl: './game.component.html',
  styleUrl: './game.component.css'
})
export class GameComponent {
  constructor(private channelService: ChannelService) {
    this.channelService.getGameUpdates("").subscribe((gameMove) => {
      const [p, z] = [gameMove.position % 9, Math.floor(gameMove.position / 9)];
      const [y, x] = [Math.floor(p / 3), p % 3];

      let elem = document.getElementById(`${z} ${y} ${x}`);
      if (!elem) return;
      elem.innerText = "X";
    })
  }

  makeMove(position: number): void {
    this.channelService.sendMove(position);
  }
}
