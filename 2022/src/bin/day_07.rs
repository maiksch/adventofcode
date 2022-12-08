use std::{cell::RefCell, rc::Rc, str::FromStr};

#[derive(Debug)]
enum Cmd {
    Cd(String),
    Ls,
}

impl FromStr for Cmd {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, cmd) = s.split_once(" ").expect("this split works");

        match cmd {
            x if x.starts_with("cd") => {
                let (_, target) = x.split_once(" ").expect("this works aswell");
                Ok(Cmd::Cd(String::from(target)))
            }
            x if x.starts_with("ls") => Ok(Cmd::Ls),
            _ => panic!("OH NO WE CANT BE HERE"),
        }
    }
}

#[derive(Debug)]
enum Item {
    Cmd(Cmd),
    Dir(String),
    File(usize),
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            x if x.starts_with("$") => {
                let cmd = x.parse::<Cmd>().expect("this cmd was parsed");
                Ok(Item::Cmd(cmd))
            }
            x if x.starts_with("dir") => {
                let (_, name) = x.split_once(" ").expect("dir split");
                Ok(Item::Dir(String::from(name)))
            }
            _ => {
                let (size, _) = s.split_once(" ").expect("file split");
                Ok(Item::File(size.parse().expect("file size")))
            }
        }
    }
}

struct Dir {
    name: String,
    size: usize,
    children: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn new(name: String) -> Self {
        Self {
            name,
            size: 0,
            children: vec![],
            parent: None,
        }
    }

    fn add_child(&mut self, child: Rc<RefCell<Dir>>) {
        self.children.push(child);
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day_07.test").expect("input is there");
    println!("{}", part_one(&input));
}

fn cd_cmd(dir: String, pwd: Rc<RefCell<Dir>>) -> Rc<RefCell<Dir>> {
    let mut pwd = pwd;

    if dir == "/".to_string() {
        println!("up to root");
        let pwd_clone = Rc::clone(&pwd);
        while let Some(x) = pwd_clone.borrow().parent.as_ref() {
            pwd = Rc::clone(x);
        }
    } else if dir == "..".to_string() {
        println!("Back");
        let pwd_clone = Rc::clone(&pwd);
        pwd = Rc::clone(pwd_clone.borrow().parent.as_ref().unwrap());
    } else {
        let child = pwd.borrow().children.iter().find_map(|x| {
            if x.borrow().name == dir {
                Some(Rc::clone(x))
            } else {
                None
            }
        });
        println!("open dir {}", dir);
        if child.is_some() {
            pwd = Rc::clone(&child.unwrap());
        } else {
            panic!("child not found");
        }
    }

    pwd
}

fn dir_cmd(dir_name: String, pwd: Rc<RefCell<Dir>>) {
    let mut mut_pwd = pwd.borrow_mut();
    let child = Rc::new(RefCell::new(Dir::new(dir_name)));
    mut_pwd.add_child(Rc::clone(&child));
    let mut mut_child = child.borrow_mut();
    mut_child.parent = Some(Rc::clone(&pwd));
}

fn part_one(input: &str) -> usize {
    let file_system = Rc::new(RefCell::new(Dir::new("/".to_string())));
    let mut pwd: Rc<RefCell<Dir>> = Rc::clone(&file_system);

    for item in input.lines() {
        let item = item.parse::<Item>().expect("Item parsed successfully");
        match item {
            Item::Cmd(Cmd::Cd(dir)) => {
                pwd = cd_cmd(dir, Rc::clone(&pwd));
            }
            Item::Dir(name) => {
                dir_cmd(name, Rc::clone(&pwd));
            }
            Item::File(file_size) => {
                let mut pwd = pwd.borrow_mut();
                pwd.size += file_size;
            }
            _ => (),
        };

        let pwd = pwd.borrow();
        println!("{:?}: {}", pwd.name, pwd.size);
    }

    0
}
