import { Injectable } from '@angular/core';
import {GrpcWebFetchTransport} from "@protobuf-ts/grpcweb-transport";
import {LobbyClient} from "./generated/subtac.client";
import {JoinRequest, Message} from "./generated/subtac";
import {BehaviorSubject, Observable} from "rxjs";
import {ServerStreamingCall} from "@protobuf-ts/runtime-rpc";

@Injectable({
  providedIn: 'root'
})
export class LobbyService {
  private username: string = "";
  private client = new LobbyClient(new GrpcWebFetchTransport({baseUrl: "http://localhost:9800"}));
  private messages: BehaviorSubject<Message> = new BehaviorSubject<Message>({content: "joined"});

  constructor() {}

  public hasUsername(): boolean {
    return this.username.trim().length != 0
  }

  public joinQueue(): void {
    const req: JoinRequest = {user: this.username};
    const call: ServerStreamingCall<JoinRequest, Message> = this.client.joinLobby(req);

    call.responses.onMessage((msg: Message) => {
      this.messages.next(msg);
    })
  }

  public sendMessage(msg: Message): void {
    this.client.sendMessage(msg).status.catch(console.error);
  }

  public setUsername(username: string): boolean {
    this.username = username;
    return this.hasUsername();
  }

  public get getUsername(): string {
    return this.username;
  }

  public get getMessages(): Observable<Message> {
    return this.messages.asObservable();
  }
}
