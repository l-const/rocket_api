#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world from REST_API"
}
#[get("/notes")]
fn get_notes() -> String {
    create_notes::<3>()
}

// const generics MVP usage
//link : https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta
fn combine_notes_to_str<const N: usize>(notes: [&str; N]) -> String {
    // let notes_iter = notes.iter();
    // let output = notes_iter.;
    let output = notes.join(" ");
    format!("Notes : \n{}", output)
}

// const generics MVP
// link : https://blog.rust-lang.org/2022/02/26/const-generics-mvp-beta
fn create_notes<const N: usize>() -> String {
    let mut array: [&str; N] = ["Note" ; N];
    for i in 0..N {
        array[i] = "A note!\n";
    }
    combine_notes_to_str(array)
}

fn main() {
    println!("Server is starting!");
    // try const generics
    let mut my_app = rocket::ignite();
    let my_router = routes![index, get_notes];
    my_app.mount( "/", my_router).launch();
}
