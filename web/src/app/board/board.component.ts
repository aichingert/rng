import {Component, EventEmitter, Input, Output} from '@angular/core';

@Component({
  selector: 'app-board',
  standalone: true,
  imports: [],
  templateUrl: './board.component.html',
  styleUrl: './board.component.css'
})
export class BoardComponent {
  @Input() index: number | undefined = undefined;
  @Output() newMove = new EventEmitter<number>;

  makeMove(event: Event): void {
    if (this.index === undefined) {
      console.error("ERROR: board does not have an index");
      return;
    }

    if (!event.target || (event.target as HTMLDivElement).id === "board") return;
    const [_, y, x]= (event.target as HTMLDivElement).id.split(' ');
    this.newMove.next(9 * this.index + 3 * parseInt(y) + parseInt(x));
  }
}
