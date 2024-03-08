import { Injectable } from '@angular/core';
import {GrpcWebFetchTransport} from "@protobuf-ts/grpcweb-transport";
import {ChannelClient} from "./generated/channel.client";
import {Subject} from "rxjs";
import {GameMove} from "./generated/channel";

@Injectable({
  providedIn: 'root'
})
export class ChannelService {
  private client = new ChannelClient(new GrpcWebFetchTransport({baseUrl: "http://localhost:9800"}));

  private gameUpdate = new Subject<GameMove>();

  constructor() {
  }

  getGameUpdates(alias: string) {
    const call = this.client.joinQueue({alias});

    call.responses
      .onNext((move: GameMove | undefined) => {
        if (!move) return;
        this.gameUpdate.next(move);
      })

    return this.gameUpdate.asObservable();
  }
}
