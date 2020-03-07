use std::collections::HashMap;

fn main() {
    let text = "As an example, let’s say we want to implement Display on Vec<T>, which the orphan rule prevents us from doing directly because the Display trait and the Vec<T> type are defined outside our crate. We can make a Wrapper struct that holds an instance of Vec<T>; then we can implement Display on Wrapper and use the Vec<T> value, as shown in Listing 19-23. A type alias defines a new name for an existing type. Type aliases are declared with the keyword type. Every value has a single, specific type, but may implement several different traits, or be compatible with several different type constraints. The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding. We would have to implement all the methods of Vec<T> directly on Wrapper such that the methods delegate to self.0, which would allow us to treat Wrapper exactly like a Vec<T>. If we wanted the new type to have every method the inner type has, implementing the Deref trait (discussed in Chapter 15 in the “Treating Smart Pointers Like Regular References with the Deref Trait” section) on the Wrapper to return the inner type would be a solution. If we don’t want the Wrapper type to have all the methods of the inner type—for example, to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.";
    let mut word_counts = HashMap::new();
    let pairs = text.split(" ")
        .map(|x| { (x, 1) });
    pairs.for_each(|x| {
        let (word, count) = x;
        let tmp = word_counts.entry(word)
            .or_insert(0);
        *tmp += count;
    });
    let mut wc_vec = word_counts.iter().collect::<Vec<(&&str, &i32)>>();
    wc_vec.sort_by(|a, b| b.1.cmp(&a.1));
    wc_vec.iter().for_each(|x| {
        let (w, c) = x;
        println!("{}: {}", &w, &c);
    })
}
