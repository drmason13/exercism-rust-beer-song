pub fn verse(n: u32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottle of beer on the wall.\n", n, n-1),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    assert!(start > end, "Must drink beer, not stockpile it!");
    let mut song = verse(start);
    let mut n = start;
    while n > end {
        n -= 1;
        song.push_str("\n");
        song.push_str(&verse(n));
    }
    song
}
