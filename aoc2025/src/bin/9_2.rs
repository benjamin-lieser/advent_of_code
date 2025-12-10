use advent_of_code::*;
use itertools::iproduct;

fn dist(a: &[int; 2], b: &[int; 2]) -> int {
    ((a[0] - b[0]).abs() + 1) * ((a[1] - b[1]).abs() + 1)
}

fn is_intersection(a: &[int; 2], b: &[int; 2], c: &[int; 2], d: &[int; 2]) -> bool {
    let ab_horizontal = a[1] == b[1];
    let cd_horizontal = c[1] == d[1];
    if ab_horizontal == cd_horizontal {
        return false;
    }
    let (h1, h2, v1, v2) = if ab_horizontal {
        (a, b, c, d)
    } else {
        (c, d, a, b)
    };
    let h_min_x = h1[0].min(h2[0]);
    let h_max_x = h1[0].max(h2[0]);
    let v_min_y = v1[1].min(v2[1]);
    let v_max_y = v1[1].max(v2[1]);
    (v1[0] > h_min_x) && (v1[0] < h_max_x) && (h1[1] > v_min_y) && (h1[1] < v_max_y)
}

fn rect_intersect(a: &[int; 2], b: &[int; 2], poly: &[[int;2]]) -> bool {
    let rect = [
        [a[0], a[1]],
        [b[0], a[1]],
        [b[0], b[1]],
        [a[0], b[1]],
        [a[0], a[1]],
    ];
    for i in 0..4 {
        for j in 0..poly.len() {
            if is_intersection(&rect[i], &rect[i + 1], &poly[j], &poly[(j + 1) % poly.len()]) {
                return true;
            }
        }
    }
    false
}

fn point_in_rect(points: &[[int; 2]], rect_a: &[int; 2], rect_b: &[int; 2]) -> bool {
    let min_x = rect_a[0].min(rect_b[0]);
    let max_x = rect_a[0].max(rect_b[0]);
    let min_y = rect_a[1].min(rect_b[1]);
    let max_y = rect_a[1].max(rect_b[1]);
    for p in points {
        if p[0] > min_x && p[0] < max_x && p[1] > min_y && p[1] < max_y {
            return true;
        }
    }
    false
}

fn main() {
    let input = get_input(2025, 9);

    let pos: Vec<[int; 2]> = input.lines().map(|l| split(l, ",")).collect();

    // This works only because the input is quite simple, there are definitly advisarial inputs that would break this

    let pairs = iproduct!(0..pos.len(), 0..pos.len())
        .filter(|(a, b)| a < b && !rect_intersect(&pos[*a], &pos[*b], &pos) && !point_in_rect(&pos, &pos[*a], &pos[*b]))
        .map(|(a, b)| (dist(&pos[a], &pos[b]), a, b))
        .sorted()
        .collect::<Vec<_>>();

    println!("{}", pairs[pairs.len() - 1].0);

}
