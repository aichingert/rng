import {Component, OnInit} from '@angular/core';
import { RouterOutlet } from '@angular/router';
import {GrpcWebFetchTransport} from "@protobuf-ts/grpcweb-transport";
import {LobbyClient} from "../shared/subtac.client";
import {JoinRequest, Message} from "../shared/subtac";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent implements OnInit {
  title = 'web';

  ngOnInit() {
    const transport = new GrpcWebFetchTransport({baseUrl: "http://localhost:9800"});
    const client = new LobbyClient(transport);

    const req: JoinRequest = {
      user: "Tobias",
    };

    const call = client.joinLobby(req);

    call.responses.onMessage((msg: Message) => {
      console.log(msg);
      this.title = msg.content;
    });
  }
}
