import {Component, OnDestroy} from '@angular/core';
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {MatButton} from "@angular/material/button";
import {MatCard, MatCardContent} from "@angular/material/card";
import {MatError, MatFormField} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {UserService} from "../../shared/user.service";
import {Subscription} from "rxjs";

@Component({
  selector: 'app-register',
  standalone: true,
    imports: [
        FormsModule,
        MatButton,
        MatCard,
        MatCardContent,
        MatError,
        MatFormField,
        MatInput,
        ReactiveFormsModule
    ],
  templateUrl: './register.component.html',
  styleUrl: './register.component.css'
})
export class RegisterComponent implements OnDestroy {
  public username: string = "";
  public password: string = "";
  public invalidRegistration: boolean = false;

  private registerSubscription: Subscription = new Subscription();

  constructor (private user: UserService,) {}

  ngOnDestroy() {
    this.registerSubscription.unsubscribe();
  }

  onSubmit(_event: Event): void {
    this.registerSubscription = this.user
      .register(this.username, this.password)
      .subscribe((_) => this.invalidRegistration = true);
  }
}
