import { Component } from '@angular/core';
import {Router} from "@angular/router";
import {LobbyService} from "../../shared/lobby.service";
import {FormsModule} from "@angular/forms";
import {MatButton} from "@angular/material/button";
import {UserService} from "../../shared/user.service";
import {MatCard, MatCardContent} from "@angular/material/card";
import {MatError, MatFormField} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {Subscription} from "rxjs";

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [
    FormsModule,
    MatButton,
    MatCard,
    MatCardContent,
    MatError,
    MatFormField,
    MatInput
  ],
  templateUrl: './login.component.html',
  styleUrl: './login.component.css'
})
export class LoginComponent {
  public username: string = "";
  public password: string = "";
  public invalidLogin: boolean = false;

  private loginSubscription: Subscription = new Subscription();

  constructor (private user: UserService,) {}

  ngOnDestroy() {
    this.loginSubscription.unsubscribe();
  }

  onSubmit(_event: Event): void {
    this.loginSubscription = this.user
      .login(this.username, this.password)
      .subscribe((_) => this.invalidLogin = true);
  }
}
