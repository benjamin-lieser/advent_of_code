use advent_of_code::*;

fn main() {
    let input = get_input(2024, 12);

    let mut acc = 0;

    let grid = Grid::from_str(&input);

    let (clusters, cluster_num) = clusters(&grid);

    for i in 0..cluster_num {
        let area = clusters.count(i);

        let mut sides = 0;

        let is_perimeter_left = |pos| {
            let l = clusters.get(pos + Dir::Left);
            let r = clusters.get(pos);
            if (r == Some(i) || l == Some(i)) && r != l {
                return true;
            }
            false
        };

        let is_perimeter_up = |pos| {
            let l = clusters.get(pos + Dir::Up);
            let r = clusters.get(pos);
            if (r == Some(i) || l == Some(i)) && r != l {
                return true;
            }
            false
        };

        for line in 0..=grid.c() {
            let mut peri = false;
            for row in 0..grid.r() {
                if is_perimeter_left((line, row)) {
                    if !peri {
                        sides += 1;
                        peri = true;
                    }
                } else {
                    peri = false;
                }
            }
        }

        for line in 0..=grid.r() {
            let mut peri = false;
            for col in 0..grid.c() {
                if is_perimeter_up((col, line)) {
                    if !peri {
                        sides += 1;
                        peri = true;
                    }
                } else {
                    peri = false;
                }
            }
        }
        //dbg!(sides, area);
        acc += area * sides;
    }

    println!("{}", acc);
}
