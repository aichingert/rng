import {Component} from '@angular/core';
import {LobbyService} from "../../shared/lobby.service";
import {Router} from "@angular/router";
import {MatButton} from "@angular/material/button";
import {FormsModule} from "@angular/forms";
import {MatInput} from "@angular/material/input";

@Component({
  selector: 'app-lobby',
  standalone: true,
  imports: [
    MatButton,
    FormsModule,
    MatInput
  ],
  templateUrl: './lobby.component.html',
  styleUrl: './lobby.component.css'
})
export class LobbyComponent {
  public alias: string = "";

  constructor(
    private router: Router,
    public lobbyService: LobbyService,
  ) {}

  joinGame(_event: Event): void {
    if (this.alias.trim().length > 0) {
      this.router.navigate(["play"]).catch(console.error);
    }
  }
}
