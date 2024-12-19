use advent_of_code::*;
#[derive(Debug)]
enum Entry {
    File(int),
    Dir(String)
}

fn dir_size(dirs: &HashMap<String, Vec<Entry>>, dir: &str) -> int {
    let mut acc = 0;
    for entry in dirs.get(dir).unwrap() {
        match entry {
            Entry::File(size) => acc += *size,
            Entry::Dir(subdir) => acc += dir_size(dirs, subdir)
        }
    }
    acc
}

fn main() {
    let input = get_input(2022, 7);

    let mut dirs = HashMap::<String, Vec<Entry>>::new();

    let mut current = "/".to_owned();

    for line in input.lines() {
        if line.starts_with("$ cd") {
            let arg = &line[5..];
            if arg == "/" {
                current = "/".to_owned();
            } else if arg == ".." {
                let last_slash = current[..current.len() - 1].rfind('/').unwrap();
                current = current[..=last_slash].to_owned();
            } else {
                current = format!("{}{}/", current, arg);
            }
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir ") {
            let name = &line[4..];
            dirs.entry(current.clone()).or_insert(vec![]).push(Entry::Dir(format!("{}{}/", current, name)));
        } else {
            let [size] = get_all_int(&line);
            dirs.entry(current.clone()).or_insert(vec![]).push(Entry::File(size));
        }
    }

    let total = 70000000;
    let needed = 30000000;

    let free = total - dir_size(&dirs, "/");

    let mut min = int::MAX;

    for dir in &dirs {
        let size =  dir_size(&dirs, &dir.0);
        if size + free >= needed {
            min = min.min(size);
        }
    }


    println!("{}", min);
}
