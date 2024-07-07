#include <capnp/serialize.h>
#include <capnp/message.h>
#include <capnp/ez-rpc.h>
#include <iostream>
#include <kj/async.h>

#include "hello.capnp.h"

int main(int argc, const char* argv[]) {
    if (argc != 2) {
        std::cerr << "usage: " << argv[0] << " HOST[:PORT]" << std::endl;
        return 1;
    }

    capnp::EzRpcClient client(argv[1], 5923);
    auto& waitScope = client.getWaitScope();

    Hello::Client cap = client.getMain<Hello>();

    auto request = cap.sayHelloRequest();
    request.setMsg("hello, world!\n");
    auto promise = request.send();

    auto response = promise.wait(waitScope);

    return 0;
}
