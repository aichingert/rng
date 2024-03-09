import { Component } from '@angular/core';
import {ChannelService} from "../../shared/channel.service";
import {GameMove} from "../../shared/generated/channel";
import {MatInput} from "@angular/material/input";
import {FormsModule} from "@angular/forms";
import {BoardComponent} from "../board/board.component";
import {of} from "rxjs";

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
      const [x, y] = [gameMove.position % 3, Math.floor(gameMove.position / 3)];

      let elem = document.getElementById(`${y} ${x}`);
      if (!elem) return;
      elem.innerText = "X";
    })
  }

  makeMove(position: number): void {
    this.channelService.sendMove(position);
  }

  protected readonly of = of;
}
