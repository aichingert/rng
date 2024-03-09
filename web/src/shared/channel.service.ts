import { Injectable } from '@angular/core';
import {GrpcWebFetchTransport} from "@protobuf-ts/grpcweb-transport";
import {ChannelClient} from "./generated/channel.client";
import {Subject} from "rxjs";
import {GameMove, JoinRequest} from "./generated/channel";
import {ServerStreamingCall} from "@protobuf-ts/runtime-rpc";

@Injectable({
  providedIn: 'root'
})
export class ChannelService {
  private client = new ChannelClient(new GrpcWebFetchTransport({baseUrl: "http://localhost:9800"}));
  private channelId: number = -1;

  private gameUpdate = new Subject<GameMove>();

  constructor() {
  }

  getGameUpdates(alias: string) {
    const call: ServerStreamingCall<JoinRequest, GameMove> = this.client.joinQueue({alias});

    call.responses
      .onNext((move: GameMove | undefined) => {
        if (!move) return;
        if (this.channelId == -1) this.channelId = move.channel;
        else this.gameUpdate.next(move);
      })

    return this.gameUpdate.asObservable();
  }

  sendMove(position: number): void {
    if (this.channelId == -1) {
      console.error("ERROR: you are not in a channel");
      return;
    }

    const move: GameMove = {
      position,
      isCross: false,
      channel: this.channelId,
    };

    this.client.sendMove(move).response.catch(console.error);
  }
}
