#ifndef PROTO_H
#define PROTO_H

// Our C++ needs to be exported through the C ABI, since that's the only stable ABI for C++.
// This is done by wrapping the C++ code in extern "C" { ... } blocks, which tells 
// the compiler to use the C ABI for the code inside the block.

// Even though we use C ABI, we can still compile and use C++ code since we're linking the C++ standard library, 
// and C is only used as the 'bridge' between Zig and C++.

#ifdef __cplusplus
extern "C"
{
#endif

    void helloWorld(void);

#ifdef __cplusplus
}
#endif

#endif // PCAPPLUSPLUS_WRAPPER_H
