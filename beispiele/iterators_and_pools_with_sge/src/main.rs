#[allow(non_upper_case_globals)]
const data: [&str; 3] = [
    "Peter Arbeitsloser",
    "Sandra Systemadministratorin",
    "Peter Koch",
];

fn loop_while() {
    title("loop_while");

    let mut index = 0;
    let length = data.len();
    while index < length {
        println!("{}: {}", index, data[index]);
        index += 1
    }
}

/*
fn for_index() {
    title("for_index");

    let length = data.len();
    for (let index = 0; index < length; index++) {
        let name = &data[index];
        println!("{}: {}", index, name);
    }
}
// */

fn foreach() {
    title("foreach");
    for name in &data {
        println!("{}", name);
    }
}

fn iterator() {
    title("iterator");

    let iterator = data.iter();
    iterator.for_each(|name| {
        println!("{}", name);
    });
}

fn title(t: &str) {
    println!("\n :: {} ::", t);
}

fn main() {
    loop_while();
    foreach();
    iterator();
}
