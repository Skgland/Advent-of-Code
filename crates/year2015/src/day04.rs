use md5::digest::Digest;

#[inline(always)]
fn find_md5_prefix(input: &str, prefix: &str) -> u32 {
    let input = input.trim();
    let md5_prefix = md5::Md5::new_with_prefix(input.as_bytes());
    (1..)
        .find(|suf| {
            let digest = md5_prefix.clone().chain_update(format!("{suf}")).finalize();
            let hash = format!("{:x}", digest);
            hash.starts_with(prefix)
        })
        .unwrap()
}

pub fn part1(input: &str) -> u32 {
    find_md5_prefix(input, "00000")
}

pub fn part2(input: &str) -> u32 {
    find_md5_prefix(input, "000000")
}

#[test]
fn part1_example1() {
    let input = "abcdef";
    assert_eq!(part1(input), 609043);
}

#[test]
fn part1_example2() {
    let input = "pqrstuv";
    assert_eq!(part1(input), 1048970);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2015/day04.txt"
    ));
    assert_eq!(part1(input), 282749);
}

#[ignore = "too slow"]
#[test]
fn part2_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2015/day04.txt"
    ));
    assert_eq!(part2(input), 9962624);
}
