import {Component, OnInit} from '@angular/core';
import {GreetingClient, ServiceError} from "../shared/generated/subtac_pb_service";
import {HelloRequest, HelloResponse} from "../shared/generated/subtac_pb";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  title = 'web';

  ngOnInit() {
    const client = new GreetingClient('http://localhost:9800');
    const req = new HelloRequest();
    req.setName("Thomas!");
    client.greet(req, (err: ServiceError | null, response: HelloResponse | null) => {
      if (err) {
        this.title = `Error: ${err.message}`;
        return;
      }
      this.title = response!.getGreeting();
    });
  }
}
