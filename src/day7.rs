use std::cell::RefCell;
use std::{fs, collections::HashMap};
use std::rc::Rc;
use std::time::Instant;
use std::cmp::min;

const FILE_PATH: &str = "inputs/day7_input.txt";
const MAX_DIR_SIZE: u32 = 100_000;
const TOTAL_DISK_SPACE: u32 = 70_000_000;
const MINIMUM_REQUIRED_UNUSED_SPACE: u32 = 30000000;

trait GetSize {
    fn get_size(&self) -> u32;
}

#[derive(Debug)]
enum Command<'a> {
    CD(&'a str),
    LS
}

impl<'a> Command<'a> {
    fn from(text: &'a str) -> Self {
        use Command::*;
        let mut split = text.split_whitespace();
        let first = split.next().unwrap();
        match first {
            "ls" => LS,
            "cd" => CD(split.next().unwrap()),
            _ => panic!("Failed to parse command"),
        }
    }
}

#[derive(Debug)]
struct File<'a> {
    #[allow(dead_code)]
    name: &'a str,
    size: u32,
}

#[derive(Debug)]
enum DirectoryContents<'a> {
    File(File<'a>),
    Directory(Rc<Directory<'a>>),
}

impl<'a> DirectoryContents<'a> {
    fn from_str_and_pwd(str: &'a str, parent: Rc<Directory<'a>>) -> Self {
        let mut split = str.split_whitespace();
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        match first {
            "dir" => {
                DirectoryContents::Directory::<'a>(
                    Directory::new(second, HashMap::new(), Some(parent)).into()
                )
            },
            _ => {
                DirectoryContents::File(
                    File { size: first.parse().unwrap(), name: second}
                )
            }
        }
    }
}

#[derive(Debug)]
struct Directory<'a> {
    #[allow(dead_code)]
    name: &'a str,
    parent: RefCell<Option<Rc<Directory<'a>>>>,
    contents: RefCell<HashMap<&'a str, DirectoryContents<'a>>>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str, contents: HashMap<&'a str, DirectoryContents<'a>>, parent: Option<Rc<Directory<'a>>>) -> Self {
        Directory {
            name,
            contents: RefCell::new(contents),
            parent: RefCell::new(parent),
        }
    }
}

impl GetSize for File<'_> {
    fn get_size(&self) -> u32 {
        self.size
    }
}

impl GetSize for Directory<'_> {
    fn get_size(&self) -> u32 {
        self.contents.borrow().values().map(|c| c.get_size()).sum()
    }
}

impl GetSize for DirectoryContents<'_> {
    fn get_size(&self) -> u32 {
        match self {
            DirectoryContents::Directory(dir) => dir.get_size(),
            DirectoryContents::File(file) => file.get_size(),
        }
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day7");
    let root = parse_input(&contents);

    let p1 = part_one(&root);
    let p2 = part_two(&root);
    println!("Elapsed: {:?}", start.elapsed());
    println!("D7P1: {p1:?}");
    println!("D7P2: {p2:?}");
}

fn parse_input(input: &str) -> Rc<Directory> {
    let root = Rc::new(Directory::new("/", HashMap::new(), None));
    let mut next = root.clone();
    let mut lines = input.lines().peekable();
    // Skip cd to root command
    lines.next();
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let pwd = next.clone();
        let line = line.unwrap();
        let trailer = line.split_once(' ').unwrap().1;

        match Command::from(trailer) {
            Command::LS => {
                while let Some(peek) = lines.peek() {
                    if peek.starts_with('$') {
                        break;
                    }
                    let line = lines.next().unwrap();
                    let dir_contents = DirectoryContents::from_str_and_pwd(line, pwd.clone());
                    let name = line.split_whitespace().nth(1).unwrap();
                    pwd.contents.borrow_mut().insert(name, dir_contents);
                }
            }

            Command::CD(x) => {
                match x {
                    ".." => next = pwd.parent.borrow().as_ref().unwrap().clone(),
                    _ => {
                        if let DirectoryContents::Directory(dir) = pwd.contents.borrow().get(x).unwrap() {
                            next = dir.clone()
                        };
                    }
                };
            }
        };
    }

    root
}

fn part_one(root: &Directory) -> u32 {
    fn helper(node: &Directory, total_so_far: u32) -> u32 {
        let mut total = total_so_far;
        for c in node.contents.borrow().values() {
            if let DirectoryContents::Directory(dir) = c {
                total += helper(dir, total_so_far);
            }
        }
        let size = node.get_size();

        if size <= MAX_DIR_SIZE {
            total += size;
        }

        total
    }
    helper(root, 0)
}

fn part_two(root:&Directory) -> u32 {
    let root_size = root.get_size();
    let available_space = TOTAL_DISK_SPACE - root_size;
    let min_delete_size = MINIMUM_REQUIRED_UNUSED_SPACE - available_space;

    fn helper(node: &Directory, min_to_delete: u32, comparator: u32) -> u32 {
        let size = node.get_size();
        if size < comparator {
            return min_to_delete;
        }

        let mut min_size = min(min_to_delete, size);

        for c in node.contents.borrow().values() {
            if let DirectoryContents::Directory(dir) = c {
                min_size = min(min_size, helper(dir, min_size, comparator));
            }
        }
        min_size
    }

    helper(root, root_size, min_delete_size)
}
