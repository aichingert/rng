import { Component } from '@angular/core';
import {Router} from "@angular/router";
import {LobbyService} from "../../shared/lobby.service";

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [],
  templateUrl: './login.component.html',
  styleUrl: './login.component.css'
})
export class LoginComponent {
  public username: string = "";

  constructor (
    private router: Router,
    private lobbyService: LobbyService,
  ) {}

  public onKey(event: KeyboardEvent): void {
  }
}
