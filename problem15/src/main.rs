use std::collections::HashMap;

fn get_rl_paths(pnt: (u32,u32), cache: &mut HashMap<(u32,u32), u64>) -> u64 {
    match cache.get(&pnt) {
        Some(p) => return *p,
        None => ()
    }
    if pnt.0 == 20 || pnt.1 == 20 {
        return 1;
    }
    let mut paths: u64 = 0;
    if pnt.0 < 20 { paths = paths + get_rl_paths((pnt.0+1, pnt.1), cache) }
    if pnt.1 < 20 { paths = paths + get_rl_paths((pnt.0, pnt.1+1), cache)}
    cache.insert((pnt.0, pnt.1), paths);
    return paths;
}

fn main() {
    let mut cache: HashMap<(u32,u32), u64> = HashMap::new();
    let rl_paths = get_rl_paths((0,0), &mut cache);
    println!("{}", rl_paths);
}