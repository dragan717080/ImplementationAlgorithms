use std::io;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut numbers = line.split_whitespace().map(|c| c.parse::<i32>().unwrap());
    let [n, _] = [numbers.next().unwrap(), numbers.next().unwrap()];
    
    let mut grid: Vec<String> = Vec::new();

    for _ in 0..n {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        
        grid.push(line.trim().to_owned());
    }
    println!("{}", two_pluses(grid));
}

fn two_pluses(grid: Vec<String>) -> i32 {
    let mut areas: Vec<i32> = Vec::new();
    let mut overlap_grid: Vec<Vec<HashSet<i32>>> = vec![vec![HashSet::new(); grid[0].len()]; grid.len()];
    let mut counter = -1;
    let mut collisions: HashMap<i32, HashSet<i32>> = HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut valid_moves = grid[i].chars().nth(j).unwrap() == 'G';
            let mut idx = 0;
            let mut max_area = 0;

            while valid_moves {
                let mut to_detect_i = vec![i];
                let mut to_detect_j = vec![j];

                for k in 0..2 {
                    if k == 0 {
                        if i as i32 - idx < 0 || j as i32 - idx < 0 {
                            valid_moves = false;
                            break;
                        }
                    } 
                    else {
                        if i + idx as usize >= grid.len() || j + idx as usize >= grid[i].len() {
                            valid_moves = false;
                            break;
                        }
                    }
                }

                if valid_moves {
                    if grid[i + idx as usize].chars().nth(j).unwrap() == 'G'
                        && grid[i - idx as usize].chars().nth(j).unwrap() == 'G'
                        && grid[i].chars().nth(j + idx as usize).unwrap() == 'G'
                        && grid[i].chars().nth(j - idx as usize).unwrap() == 'G'
                    {
                        max_area += if idx == 0 { 1 } else { 4 };

                        for r in 0..2 {
                            for p in 1..idx + 1 {
                                for c in 0..2 {
                                    if c == 0 {
                                        if r == 0 {
                                            to_detect_i.push(i + p as usize);
                                            to_detect_j.push(j);
                                        } 
                                        else {
                                            to_detect_j.push(j + p as usize);
                                            to_detect_i.push(i);
                                        }
                                    } 
                                    else {
                                        if r == 0 {
                                            to_detect_i.push(i - p as usize);
                                            to_detect_j.push(j);
                                        } 
                                        else {
                                            to_detect_j.push(j - p as usize);
                                            to_detect_i.push(i);
                                        }
                                    }
                                }
                            }
                        }

                        idx += 1;
                        counter += 1;
                        collisions.insert(counter, HashSet::new());

                        for k in 0..to_detect_i.len() {
                            let l = &mut overlap_grid[to_detect_i[k]][to_detect_j[k]];
                            for element in l.iter() {
                                collisions.get_mut(element).unwrap().insert(counter);
                            }
                            collisions.get_mut(&counter).unwrap().extend(l.iter().cloned());
                            l.insert(counter);
                        }

                        areas.push(max_area);
                    }
                    else {
                        valid_moves = false;
                    }
                }
            }
        }
    }

    if areas.is_empty() {
        return 0;
    } 
    else {
        let mut max_vals: HashSet<i32> = HashSet::new();
        max_vals.insert(0);

        for i in 0..areas.len() {
            for j in (i + 1)..areas.len() {
                if !collisions.contains_key(&(i as i32))
                    || !collisions[&(i as i32)].contains(&(j as i32))
                {
                    max_vals.insert(areas[i] * areas[j]);
                }
            }
        }

        *max_vals.iter().max().unwrap()
    }
}
