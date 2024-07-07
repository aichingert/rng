#include <capnp/serialize.h>
#include <capnp/message.h>
#include <capnp/ez-rpc.h>
#include <iostream>

#include "hello.capnp.h"

class HelloService final : public Hello::Server {
public:
    kj::Promise<void> sayHello(SayHelloContext context) override {
        std::cout << "Client says: " << context.getParams().getMsg().cStr() << std::endl;
        return kj::READY_NOW;
    }
};

int main(int argc, const char* argv[]) {
    if (argc != 2) {
        std::cerr << "usage: " << argv[0] << " ADDRESS[:PORT]\n";
        return 1;
    }

    capnp::EzRpcServer server(kj::heap<HelloService>(), argv[1]);
    auto& waitScope = server.getWaitScope();

    kj::NEVER_DONE.wait(waitScope);
}
