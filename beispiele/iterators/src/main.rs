
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

fn stacked_iterator() {
    title("stacked_iterator");

    let processed = data
        .iter()
        .map(|name| {
            let mut split = name.split(" ");

            let (vorname, nachname) = (split.next(), split.next());

            if vorname.is_none() || nachname.is_none() {
                return Err("Konnte namen nicht parsen: Zu wenige Teile")
            }

            Ok(Person {
                vorname: vorname.unwrap().into(),
                nachname: nachname.unwrap().into(),
            })
        })
        .collect::<Result<Vec<_>, _>>();

    println!("processed: {:#?}", processed);
}

#[derive(Debug)]
struct Person {
    vorname: String,
    nachname: String,
}

fn title(t: &str) {
    println!("\n :: {} ::", t);
}

fn main() {
    loop_while();
    foreach();
    iterator();
    stacked_iterator();
}
