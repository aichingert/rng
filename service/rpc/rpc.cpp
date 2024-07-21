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

extern "C" void initServer(const char* address) {
    capnp::EzRpcServer server(kj::heap<HelloService>(), address);
    auto& waitScope = server.getWaitScope();

    std::cout << "server is running on: `" << address << "`" << std::endl;

    kj::NEVER_DONE.wait(waitScope);
}
