# grpc-tutorial

## Reflection

1. **What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?**
    The key differences are:

    Unary: Only allows the client to send a single request and receive a single response. For example, banning a user on Discord.

    Server streaming: Allows the client to send one request and receive a stream of responses from the server. For example, purging a chat history on Discord with a bot that provides percentage updates sequentially, instead of waiting until the task is complete.

    Bi-directional streaming: Allows both the client and server to send and receive streams of messages at the same time. For example, collaborating in real time using Google Docs.
2. **What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?**

    When designing authentication, we need to verify the client’s identity and ensure that we know who is making each request. This is usually done using JWT or OAuth tokens to authenticate users. For authorization, we check whether the user is allowed to perform the actions they are requesting. The security considerations here typically involve checking the user’s permissions and scopes. For encryption, our goal is to prevent data from being read by unauthorized parties. End-to-end encryption can be used to ensure that communication remains confidential and secure, even if it is intercepted by unauthorized users.
3. **What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?**

    There are several challenges that can arise when handling bidirectional streaming in Rust gRPC. For example, it is important to properly handle situations when connections are interrupted or dropped, ensuring that message delivery status is accurately reflected. Maintaining correct message ordering is also crucial to user. Additionally, improperly managed open sockets can lead to security vulnerabilities.
4. **What are the advantages and disadvantages of using the `tokio_stream::wrappers::ReceiverStream` for streaming responses in Rust gRPC services?**

    The advantage of using `ReceiverStream` to stream responses is that it integrates seamlessly with the Tokio async runtime, making it an excellent choice for channel-based message streaming. However, a potential disadvantage is that resource leaks can occur if streams are not managed properly by the developer.
5. **In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?**

    We can use Rust modules to separate different logic components and gRPC service definitions, as well as to keep protobuf files distinct from business logic. Encapsulating reusable logic as middleware can also help in reusability and modularity in the codebase over time.
6. **In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?**

    To handle more complex payment processing, we can add validation to the payment request data to ensure that the payment amount is positive and provided. We should also ensure that transactions are atomic to prevent duplicate requests and payments. It’s also important to verify that the requests are coming from users who are authorized to make the payment.
7. **What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?**

    gRPC helps because it's a language-agnostic model and, compared to REST, it ensures better compatibility because of the schemas from Protobuf. It also forces HTTP/2 support and helps improve communication performance. However, for legacy systems, there may need to be some changes to make it compatible.
8. **What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?**

    The advantages of HTTP/2 are the ability for multiple streams in a single connection and much better performance overall including lower latency and header compression. However, the disadvantages include increased complexity and less support for old environments which requires some legacy systems to create custom gateways to handle the requests better.
9. **How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?**

    REST is typically used for one request, one response interactions. While you can use WebSockets for real-time needs, gRPC natively supports real-time communication, making it more natural for live updates and streaming with low latency.

10. **What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?**

    When using Protocol Buffers, we can enforce a much stricter structure, which helps catch mismatches early. However, because of this strict structure, changing data types can be less flexible. In comparison, JSON is easier to extend and change but it is not as efficient in terms of performance.