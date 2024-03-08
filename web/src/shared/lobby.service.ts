import { Injectable } from '@angular/core';
import {GrpcWebFetchTransport} from "@protobuf-ts/grpcweb-transport";
import {LobbyClient} from "./generated/lobby.client";
import {AvailableChannels, ChannelState} from "./generated/lobby";
import {Subject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class LobbyService {
  private client = new LobbyClient(new GrpcWebFetchTransport({baseUrl: "http://localhost:9800"}));

  private channelUpdates: Subject<ChannelState> = new Subject<ChannelState>();

  public channels: number[] = [];

  constructor() {
    this.client.getAvailableChannels({}).response
      .then((available: AvailableChannels) => this.channels = available.ids)
      .catch(console.error);

    this.client.getChannelStates({}).responses
      .onNext((cs: ChannelState | undefined) => {
        if (!cs) {
          return;
        }

        if (cs.created) {
          this.channels.push(cs.id);
        } else {
          this.channels = this.channels.filter(e => e != cs.id);
        }
      });
  }
}
