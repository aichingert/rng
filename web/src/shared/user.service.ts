import { Injectable } from '@angular/core';
import {AuthClient} from "./generated/auth.client";
import {GrpcWebFetchTransport} from "@protobuf-ts/grpcweb-transport";
import {LoginRequest, RegisterRequest, Token} from "./generated/auth";
import {UnaryCall} from "@protobuf-ts/runtime-rpc";
import {BehaviorSubject, Observable} from "rxjs";
import {Router} from "@angular/router";

@Injectable({
  providedIn: 'root'
})
export class UserService {
  private client = new AuthClient(new GrpcWebFetchTransport({baseUrl: "http://localhost:9800"}));
  private authResponse = new BehaviorSubject<string>("");

  constructor(private router: Router) {
    /*
    const call: UnaryCall<RegisterRequest, Token> = this.client.register({username: "Tobias", password: "12345"});

    call.response
      .then(console.log)
      .catch(console.error);
     */
  }

  login(username: string, password: string): Observable<string> {
    this.client.login({username: username.toLowerCase(), password}).response
      .then((token: Token) => {
        localStorage.setItem("access_token", token.accessToken);
        this.router.navigate(["lobby"]).catch(console.error);
      })
      .catch((error) => this.authResponse.next(error));

    return this.authResponse.asObservable();
  }

  register(username: string, password: string): Observable<string> {
    this.client.register({username: username.toLowerCase(), password}).response
      .then((token: Token) => {
        localStorage.setItem("access_token", token.accessToken);
        this.router.navigate(["lobby"]).catch(console.error);
      })
      .catch((error) => this.authResponse.next(error));

    return this.authResponse.asObservable();
  }
}
