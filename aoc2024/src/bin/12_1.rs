use advent_of_code::*;

fn main() {
    let input = get_input(2024, 12);

    let mut acc = 0;

    let grid = Grid::from_str(&input);

    let (clusters, cluster_num) = clusters(&grid);

    for i in 0..cluster_num {
        let area = clusters.count(i);
        let mut perimeter = 0;
        for pos in grid.positions() {
            if clusters.get(pos) != Some(i) {
                continue;
            }
            for dir in DIRS {
                let next = pos + dir;
                if clusters.get(next) != Some(i) {
                    perimeter += 1;
                }
            }
        }
        acc += area * perimeter;
    }

    println!("{}", acc);

}