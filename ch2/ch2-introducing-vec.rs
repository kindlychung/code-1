
fn main() {
    // PARAMETERS
    let context_lines = 1;
    let needle = "rust";
    let haystack = "Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?
The taste of rust hangs
Heavy in my mouth
A copper color stains
My crooked teeth
I am awake now
Unlike how I used to be
Though my eyes were open
I was surely dreaming
The taste of rust hangs
Tangy on your lips
Blue twinges decorate
Your iced expression
Bound with veins
Of hope and smoke
We lie side by endless side
Entwined lost to the madness
Now the taste of rust hangs
Growing older still
Let me restore the flame
To the fire in your soul
 What is the Rust Belt?
Can we define it?
   - on a map, we mean -
Can we circle in black marker,
topographical green and brown, one mound,
from Canada on down to
Kentucky and say
well, there -
America’s sore fingers in old age
floating, separate, in the pond,
white and knobbed and wrapped around something
a lever, the haft of an oar,
the tuning dial to twist to Cavalcade,
the body of the eel which just keeps swimming away.

You said it in a message -
“Rust Belt” -
and a great blank region was filled
by old poets in corduroy
better than their surroundings
and if not better precisely
then at least when they drink
they drink in bars like smokestacks
with hubcaps on the walls, with weak plumbing,
listening to conversations, not having them.

Rust is something I know well:
I feel rust (but I don’t wear corduroy).
Rust like a signal ingredient
all through the cupboards.
Shot through, something you have too much of
and could never want to write about.
Rust in this message, too.
";

    // INITIALIZATION
    let mut tags: Vec<usize> = Vec::new(); // <1> `tags` holds line numbers where matches occur
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new(); // <2> `ctx` contains a vector per match to hold that match's context lines

    // PASS 1, find the matched lines
    for (i, line) in haystack.lines().enumerate() { // <3> iterate through the lines, recording line numbers where matches are encountered
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2*context_lines + 1); // <4> <5> `Vec::with_capacity(_n_)` reserves space for _n_ items
            ctx.push(v);
        }
    }

    if tags.len() == 0 { // <6> When there are no matches, exit early
        return;
    }

    // PASS 2, filling up the context vectors
    for (i, line) in haystack.lines().enumerate() { // <7> For each tag, at every line, check to see if we are nearby a match. When we are, add that line to the relevant `Vec<T>` within `ctx`.
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines); // <8> `usize.saturating_sub()` returns 0, rather than underflowing
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line); // <9> Copy `line` into a new `String` and store that locally for each match
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    let esc_char = vec![27];
    let esc = String::from_utf8(esc_char).unwrap();
    let reset: u8 = 0;
    let bright: u8 = 1;
    let green: u8 = 32;

    // OUTPUT
    for (ctx_i,local_ctx) in ctx.iter().enumerate() {
        println!(">>>>> Match {}", ctx_i + 1);
        let mid = local_ctx.len() / 2;
        let mid = local_ctx[mid].0;
        for &(i, ref line) in local_ctx.iter() { // <10> `ref line` informs the compiler that we wish to borrow this value, rather than move it. These two terms are explained fully later in later chapters.
            let line_num = i + 1;
            if i == mid {
                println!("{}: {}[{};{}m{}{}[{}m", line_num, esc, bright, green, line, esc, reset);
            } else {
                println!("{}: {}", line_num, line);
            }
        }
    }
}
