
use std::collections::HashMap;


enum DirOrFile<'a>{
    Dir(Dir<'a>),
    File(usize),
}

struct Dir <'a>{
    content: HashMap<&'a [u8], DirOrFile<'a>>
}

impl<'a> Dir<'a> {
    fn find_folder(&mut self, input: &[&'a [u8]]) -> &mut Dir<'a> {
        let mut cwd = self;
        let mut tmp = input;
        while let [parent,rem@..] = tmp {
            tmp = rem;
            match cwd.content.get_mut(parent).unwrap() {
                DirOrFile::Dir(dir) => cwd = dir,
                DirOrFile::File(_) => panic!(),
            }
        }
        cwd
    }
    fn mkdir(&mut self, input: &[&'a [u8]], child: &'a [u8]) {
        self.find_folder(input).content.insert(child, DirOrFile::Dir(Dir{ content: Default::default() }));
    }

    fn touch(&mut self, input: &[&'a [u8]], child: &'a [u8], size: usize) {
        self.find_folder(input).content.insert(child, DirOrFile::File(size));
    }

    fn recursive_size(&self) -> usize {
        self.content.iter().map(|(_,elem)| match elem {
            DirOrFile::Dir(dir) => dir.recursive_size(),
            DirOrFile::File(size) => *size,
        }).sum()
    }

    fn walk_dirs(&self, walk_dir: &mut impl FnMut(&Dir<'a>) ) {
        walk_dir(self);
        for (name, entry) in &self.content {
            match entry {
                DirOrFile::Dir(dir) => dir.walk_dirs(walk_dir),
                DirOrFile::File(size) => {},
            }
        }
    }
}





fn parse(input: &str) -> Dir
{
    let mut cwd = vec![];
    let mut fs = Dir{content: Default::default()};

    for line in input.lines() {
        match line.as_bytes() {
            [b'$', b' ', b'c', b'd', b' ', b'/'] => {
                cwd.clear();
            }
            [b'$', b' ', b'c', b'd', b' ', b'.', b'.'] => {
                cwd.pop();
            }
            [b'$', b' ', b'c', b'd', b' ', path@..] => {
                cwd.push(path);
            }
            [b'$', b' ', b'l', b's'] => {}
            [b'd', b'i', b'r', b' ', path@..] => {
                fs.mkdir(&cwd, path)
            }
            file => {
                let (size, name) =  line.split_once(' ').unwrap();
                fs.touch(&cwd, name.as_bytes(), size.parse().unwrap())
            }
        }
    }
    fs
}

pub fn part1(input: &str) -> usize {
    let fs = parse(input);
    let mut total = 0;
    fs.walk_dirs(&mut |dir| {
        let size =dir.recursive_size();
        if size <= 100_000 {
            total += size
        }});
    total
}

pub fn part2(input: &str) -> usize {
    let fs = parse(input);
    const TOTAL_SPACE : usize= 70000000;
    const REQUIRED_SPACE : usize = 30000000;
    let used_space = fs.recursive_size();
    let free_space = TOTAL_SPACE - used_space;
    let missing_space = REQUIRED_SPACE - free_space;

    let mut min = TOTAL_SPACE;
    fs.walk_dirs(&mut  |dir| {
        let size = dir.recursive_size();
        if size >= missing_space && size < min {
            min = size;
        }
    });

    min
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day07.example.txt");
    assert_eq!(part1(input), 584 + 94853);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day07.txt"));
    assert_eq!(part1(input), 2104783);
}


#[test]
fn part2_example() {
    let input = include_str!("../input/day07.example.txt");
    assert_eq!(part2(input), 24933642);
}


#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day07.txt"));
    assert_eq!(part2(input), 5883165);
}
