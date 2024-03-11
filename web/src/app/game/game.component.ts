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
    BoardComponent,
  ],
  templateUrl: './game.component.html',
  styleUrl: './game.component.css'
})
export class GameComponent {
  public colors: string[] = [
    "coral", "coral", "coral",
    "coral", "coral", "coral",
    "coral", "coral", "coral",
  ];
  private normalColor: string = "aliceblue";
  private highlighted: string = "coral";

  constructor(private channelService: ChannelService) {

    this.channelService.getGameUpdates("").subscribe((gameMove) => {
      if (gameMove.infoCode === 10) {
        console.log("Someone won!");
        return;
      }

      const [p, z] = [gameMove.position % 9, Math.floor(gameMove.position / 9)];
      const [y, x] = [Math.floor(p / 3), p % 3];

      let elem = document.getElementById(`${z} ${y} ${x}`);
      if (!elem) return;

      for (let i: number = 0; i < 9; i++) {
        if (gameMove.infoCode == -1) {
          this.colors[i] = this.normalColor;
        } else {
          this.colors[i] = i == gameMove.infoCode ? this.highlighted : this.normalColor;
        }
      }

      elem.innerText = gameMove.isCross ? 'X' : 'O';
    })
  }

  makeMove(position: number): void {
    this.channelService.sendMove(position);
  }
}
