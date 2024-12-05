static CHROMA_SCALE_S : [&str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
static CHROMA_SCALE_F : [&str; 12] = ["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];

fn concat_arrays_stable<T, const A: usize, const B: usize, const C: usize>(
    a: [T; A], b: [T; B]
) -> [T; C]
where
    T: Default,
{
    assert_eq!(A+B, C);
    let mut ary: [T; C] = std::array::from_fn(|_| Default::default());
    for (idx, val) in a.into_iter().chain(b.into_iter()).enumerate() {
        ary[idx] = val;
    }
    ary
}

fn main() {
    println!("Hello, world!");
    let combined: [&str; 24 ] = concat_arrays_stable(CHROMA_SCALE_S, CHROMA_SCALE_F);
    for symbol in combined {
        println!("{symbol}");
    }
}
