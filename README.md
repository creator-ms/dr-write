# dr-write Actor

This project implements an actor of cosmonic platform.


It speed up essay writing, and enable students to finish their homework in an hour.
1. search your journal articles, as ChatGPT will never give correct citations.
2. collect the related articles in our Dr Write app.
3. get ChatGPT prompts, so you can copy and paste to any A.I. tools
4. after copy the summarized content to your homework, add the inline citiations. (We already prepared an easy to use button.) 
5. copy the A.P.A. citations.


## The implementation

To respond to http requests, the actor must implement the
`httpResponse` method of the
[HttpServer interface](https://github.com/wasmCloud/interfaces/tree/main/httpserver) interface.

The implementation is in the file [dr-write/src/lib.rs](./dr-write/src/lib.rs)

## See it in action

- Demo server at : "frosty-leaf-9865" (https://frosty-leaf-9865.cosmonic.app/)


- To compile the actor and generate a signed Webassembly module, type `make`.
- To load and start the actor you'll need to have a running OCI-compatible
registry. Check that `REG_URL` setting in Makefile is correct, and run
`make push` and `make start` to push the actor to the registry
and start the actor.
Alternately, you can load and start the actor from the host's web ui.
When prompted for the path,
select `dr-write/build/dr_write_s.wasm`.

The actor must be linked with an HttpServer capability
provider with the contract id `wasmcloud:httpserver`. You can start the
provider (TODO: need registry url and more specific instructions here)

Your actor can be invoked from a terminal command-line or from a web browser.
The following examples assume the http server is listening on localhost port 8000.


### In a browser

visit the url "http://localhost:8000" or "https://frosty-leaf-9865.cosmonic.app/"

## How do I customize this template to use other contracts & interfaces?

- You can change what contracts this actor claims in `wasmcloud.toml` and the `Makefile`. In the future this will just be in `wasmcloud.toml`.
- You will then need to change the dependencies in `Cargo.toml` to import the interfaces for the contracts you want. Delete the `wasmcloud-interface-httpserver` dep if you're not using that contract.
- Finally, change the `src/lib.rs` file, changing/deleting the current interface import and `impl` block, while adding a new import & `impl` for any contracts you added!


## Development environment

- this project is tested with MacOS.
- docker-compose environment is underway to speed up dev environment setup.