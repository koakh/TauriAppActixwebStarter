# NOTES

## Links

- [Setting up Actix Web in a Tauri App](https://blog.moonguard.dev/setting-up-actix-in-tauri)

## Install the Actix Web dependency

```toml
actix-web = "4"
```

## Create the folder structure

Create the folder structure for our server. Our folder can be set up within the src-tauri/src/ folder in the following manner:

```shell
src-tauri/
  src/
    server/
      mod.rs
      handlers/
        mod.rs
        example.rs
```

The `server/mod.rs` file is the **Rust module for our server**, while the `server/handler` folder is where we can add various endpoints for our API.

## Create Server Modules

`src-tauri/src/server/mod.rs`

## Create Mod Handlers

The mod handlers statement defines a submodule that includes a module named example. This module is used to handle incoming API requests.
Additionally, a struct called `TauriAppState` is defined, which holds an instance of an `AppHandle` from the Tauri library.
This `AppHandle` is _used to facilitate communication between the Rust web server and the Tauri desktop application_ it is designed to work with.

The `init` function is an asynchronous function that sets up and configures the web server to handle incoming requests.
It accepts an `AppHandle` as input, which is used to create the `TauriAppState` instance.
The `tauri_app` variable is created using the `web::Data` struct, which can be _used to store application data_ that can be accessed by different parts of the application. In this case, `tauri_app` is an instance of `TauriAppState`.

The `HttpServer` struct is then instantiated, and the new method is used to create a new instance of an `Actix-web` App.
This App instance is _configured with several middleware to handle web requests_, including the `Logger::default()` middleware.
The `.service()` method is called on the `App` instance, which includes the `handlers::example::handle` function as an argument.
This function is the request handler that actually processes incoming web requests.

Now, to use our example module in the Actix web server, navigate to `example.rs` and add the following code:

`src-tauri/src/server/handlers/example.rs`

The next and most critical step is to navigate to the `main.rs` file and _add the server module_ that we just created, along with a new standard module called `thread`. _This module will play an essential role in running the server and handling incoming requests_.

```rs
mod server;

use std::thread;
```

To _prevent the interruption of our main Tauri app thread_, we will use the `thread` module to _run our server in a separate thread_.
This allows us to keep our server running without interfering with the functionality of our main thread.
Finally, to achieve this, we will _chain our main method_ in the following way:

change `fn main()` from

```rs
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

to

```rs
fn main() {
    tauri::Builder::default()
        .setup(|app| {

            let handle = app.handle();
            let boxed_handle = Box::new(handle);

            thread::spawn(move || {
                server::init(*boxed_handle).unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

in this case we add

```rs
        .setup(|app| {

            let handle = app.handle();
            let boxed_handle = Box::new(handle);

            // closure captures ownership of the boxed_handle variable using the move keyword, which moves the boxed handle into the closure
            thread::spawn(move || {
                // pass boxed AppHandle to server init
                server::init(*boxed_handle).unwrap();
            });
            Ok(())
        })
```

In the `main` method, we uses the `tauri::Builder` struct to customize our configure and run the Tauri application.
The _setup method takes a closure that is executed before initializes the Tauri application_.
It creates a handle to the Tauri application using the `app.handle()` method, _stores it in a boxed variable_, and passes it to the `server::init` function.

Then, we _create a new thread_ and _starts the server_ using `thread::spawn` and passes a closure that initializes the Tauri server using the server::init function.
The closure captures ownership of the `boxed_handle` variable using the `move` keyword, which moves the boxed handle into the closure.
Finally, the `server::init` function takes the boxed handle as a dereferenced pointer (using the *operator) and initializes the Tauri server using it. The unwrap method is used to panic if there is an error during server initialization. If the closure executed successfully it will returns Ok.

We have completed our demo, and to ensure it's working correctly, we will run `npm run tauri dev`.
This command will launch our application, and we can then proceed to test it by sending a `GET` request to our endpoint located at `/api/test`.
If everything is working fine, we should see the "Hello World" message displayed in our terminal as a response to the request.

## Test App

```shell
$ curl localhost:4875/api/test
```

## Convert Tauri project to a Cargo Workspace

```shell
$ cargo new shared --lib
```

add `src-shared` to tauri `[dependencies]`

`src-tauri/Cargo.toml`

```toml
shared = { path = "../src-shared" }
```

now use it in `src-tauri/src/server/handlers/example.rs`

```rs
...
use src_shared::{add, rand};

/// example using shared library
#[get("/api/test-shared-add")]
pub async fn handle_shared_add() -> actix_web::Result<String> {
    let added = add(28, 14);
    println!("{}", added.to_string());
    Ok(added.to_string())
}

/// example using shared library
#[get("/api/test-shared-rand")]
pub async fn handle_rand_rand() -> actix_web::Result<String> {
    let rand = rand();
    println!("{}", rand.to_string());
    Ok(rand.to_string())
}
```

```shell
$ curl localhost:4875/api/test-shared-add
$ curl localhost:4875/api/test-shared-rand
$ curl localhost:4875/api/shared/test
```
