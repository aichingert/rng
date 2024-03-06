import { Injectable } from '@angular/core';
import {GrpcWebFetchTransport} from "@protobuf-ts/grpcweb-transport";
import {UnaryCall} from "@protobuf-ts/runtime-rpc";
import {LobbyClient} from "./generated/lobby.client";
import {AvailableChannels, Empty} from "./generated/lobby";

@Injectable({
  providedIn: 'root'
})
export class LobbyService {
  private client = new LobbyClient(new GrpcWebFetchTransport({baseUrl: "http://localhost:9800"}));
  // TODO: make stream for new and deleted channels
  // private channelStream: BehaviorSubject<Channels> = new BehaviorSubject<AvailableChannels>({ids: []});

  public channels: number[] = [];

  constructor() {
    const call: UnaryCall<Empty, AvailableChannels> = this.client.getAvailableChannels({/*Empty*/});

    call.response.then((available: AvailableChannels) => {
      this.channels = available.ids;
    }).catch(console.error);
  }
}
