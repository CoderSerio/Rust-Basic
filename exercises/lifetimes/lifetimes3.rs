// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// 思考：再想想一个引用该何时被释放呢——如果无法被自动推断，那很显然就需要人工注明，这就是 lifetime 的意义

struct Book<'life> {
    author: &'life str,
    title: &'life str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
