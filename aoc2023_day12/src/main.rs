use std::{collections::HashMap, usize};

fn main() {
    let input = std::fs::read_to_string("../input/day12/input").unwrap();

    let mut counter = 0;

    let mut versions = 0;

    let mut cache = HashMap::new();
    //let mut variants = Vec::new();

    for line in input.lines() {
        let mut split = line.split(' ');
        let mut spring_conditions: Vec<char> = split.next().unwrap().chars().collect();
        let mut contiguous_groups: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        let original_conditions = spring_conditions.clone();
        let original_groups = contiguous_groups.clone();
        for _ in 0..4 {
            spring_conditions.push('?');
            spring_conditions.append(&mut original_conditions.clone());

            contiguous_groups.append(&mut original_groups.clone())
        }

        //println!("{:?}, {:?}", spring_conditions, contiguous_groups);

        println!("Line unfolded");

        let mut doubles = Vec::new();

        //doppelte entfernen
        for i in 0..spring_conditions.len() - 1 {
            if spring_conditions[i] == '.' && spring_conditions[i + 1] == '.' {
                doubles.push(i);
            }
        }

        for double in doubles.iter().rev() {
            spring_conditions.remove(*double);
        }

        if spring_conditions[0] != '.' {
            spring_conditions.insert(0, '.');
        }

        // PUnkt am Anfang und ende einfügen, falls nicht passiert
        if !spring_conditions.ends_with(&['.']) {
            spring_conditions.push('.');
        }

        counter += create_and_test_2(&spring_conditions, &contiguous_groups, &mut cache);

        //counter += create_and_test(&spring_conditions, &mut Vec::new(), &contiguous_groups);
        //println!("{:?}, {:?}", spring_conditions, contiguous_groups);
        /*
        for spring_condition in spring_conditions {
            if spring_condition == '?' {
                if variants.is_empty() {
                    variants.push(vec!['.']);
                    variants.push(vec!['#']);
                } else {
                    for variant in 0..variants.len() {
                        let mut clone = variants[variant].clone();
                        variants[variant].push('.');
                        clone.push('#');
                        variants.push(clone);
                    }
                }
            } else {
                if variants.is_empty() {
                    variants.push(vec![spring_condition]);
                } else {
                    for variant in &mut variants {
                        variant.push(spring_condition);
                    }
                }
            }
        }

        versions += variants.len();

        //println!("{:?}", variants);

        for variant in &variants {
            if check_variant(variant, &contiguous_groups) {
                counter += 1;
            }
        }
        //println!("LLine");

        variants.clear();*/
    }

    println!("{counter}");
}

fn check_variant(variant: &[char], contiguous_groups: &[usize]) -> bool {
    let variant_iter = variant.iter();
    let mut variant_iter = variant_iter.peekable();

    for contiguous_group in contiguous_groups {
        loop {
            let var = variant_iter.next();
            let next = match var {
                Some(v) => v,
                None => return false,
            };

            if *next == '#' {
                break;
            }
        }

        if *contiguous_group == 1 {
            if variant_iter.peek().is_some() && **variant_iter.peek().unwrap() != '.' {
                return false;
            }
            continue;
        }

        for _ in 1..*contiguous_group {
            match variant_iter.next() {
                Some(x) => {
                    if *x != '#' {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }

        if variant_iter.peek().is_some() && **variant_iter.peek().unwrap() != '.' {
            let variant_iter = variant.iter();
            let mut variant_iter = variant_iter.peekable();

            for contiguous_group in contiguous_groups {
                loop {
                    let var = variant_iter.next();
                    let next = match var {
                        Some(v) => v,
                        None => return false,
                    };

                    if *next == '#' {
                        break;
                    }
                }

                if *contiguous_group == 1 {
                    if variant_iter.peek().is_some() && **variant_iter.peek().unwrap() != '.' {
                        return false;
                    }
                    continue;
                }

                for _ in 1..*contiguous_group {
                    match variant_iter.next() {
                        Some(x) => {
                            if *x != '#' {
                                return false;
                            }
                        }
                        None => {
                            return false;
                        }
                    }
                }

                if variant_iter.peek().is_some() && **variant_iter.peek().unwrap() != '.' {
                    return false;
                }
            }

            if variant_iter.peek().is_none() || variant_iter.all(|x| *x == '.') {
                return true;
            };
        }
    }

    if variant_iter.peek().is_none() || variant_iter.all(|x| *x == '.') {
        return true;
    } else {
        return false;
    }
}

fn create_and_test(input: &[char], acc: &mut Vec<char>, contiguous_groups: &[usize]) -> usize {
    if input.is_empty() {
        println!("{:?}", acc);
        if check_variant(&acc, contiguous_groups) {
            return 1;
        } else {
            return 0;
        }
    }

    let first = input[0];
    if first == '?' {
        acc.push('#');
        let r1 = create_and_test(&input[1..], acc, contiguous_groups);
        acc.pop();
        acc.push('.');
        let r2 = create_and_test(&input[1..], acc, contiguous_groups);
        acc.pop();
        r1 + r2
    } else {
        acc.push(first);
        let res = create_and_test(&input[1..], acc, contiguous_groups);
        acc.pop();
        res
    }
}

fn create_and_test_2(
    input: &[char],
    groups_left: &[usize],
    cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
) -> usize {
    // If groups are empty and no # left, we can assume all ? are .
    if groups_left.is_empty() {
        if !input.contains(&'#') {
            return 1;
        } else {
            return 0;
        }
    }

    // if only . are left but group (see above) is not empty, this didnt work ouz
    if !input.contains(&'#') && !input.contains(&'?') {
        return 0;
    }

    let mut count = 0;

    // get first index that needs to be a spring
    let first_guaranteed_spring = input
        .iter()
        .position(|x| x == &'#')
        .unwrap_or_else(|| input.len() - 1);
    let first_guaranteed_spring =
        first_guaranteed_spring.min(input.len() - groups_left.iter().sum::<usize>());

    for i in 1..=first_guaranteed_spring {
        let part_row = &input[i..];

        if part_row.is_empty() || part_row[..groups_left[0]].contains(&'.') {
            continue;
        }

        let new_row = &part_row[groups_left[0]..];

        // cache here
        let cached = cache.get(&(new_row.to_owned(), groups_left[1..].to_owned()));
        count += match cached {
            Some(cached_result) => *cached_result,
            None => {
                let result = create_and_test_2(new_row, &groups_left[1..], cache);
                cache.insert((new_row.to_owned(), groups_left[1..].to_owned()), result);
                result
            }
        };
    }

    count
    /*
    let mut advanced_input = input;
    // wenn aktuelles Zeichen ., gehe so lange vor bis # oder ? erreicht.
    for i in 0..input.len() {
        if input[i] == '.' {
            advanced_input = &input[1..];
        }
    }

    // wenn Ende von input erreicht, return 0
    if advanced_input.is_empty() {
        return 0;
    }

    // neue Gruppe anfangen
    // zunächst schauen, ob es mit der Gruppe hier überhaupt geht
    // für jedes Element der Gruppe schauen, ob ein # oder ? existiert
    if advanced_input.len() < groups_left[0] {
        return 0;
    }
    // wenn input ende, return 0
    for i in 0..groups_left[0] {
        // wenn nein, return 0
        if advanced_input[i] == '.' {
            return 0;
        }
    }

    // für jedes Element der Gruppe schauen, was das Element dazu aus input ist

    // wenn alle #

    if advanced_input[0..groups_left[0]].iter().all(|x| *x == '#') {
        // und danach . oder Ende: Rekursion mit input[grupps_left[0]..] groups_left[1..], current 0
        if advanced_input.get(groups_left[0]).is_none()
            || *advanced_input.get(groups_left[0]).unwrap() == '.'
        {
            return create_and_test_2(&input[groups_left[0]..], &groups_left[1..], 0);
        }
        // wenn alle # und danach ?: Das ? muss . sein, also danach weiter mit input[groups_left[0] + 1..] groups_left[1..] current 0
        else if *advanced_input.get(groups_left[0]).unwrap() == '?' {
            return create_and_test_2(&input[groups_left[0] + 1..], &groups_left[1..], 0);
        }
        // wenn alle # und danach #: return 0, weil Gruppen getrennt sein müssen
        else if *advanced_input.get(groups_left[0]).unwrap() == '#' {
            return 0;
        }
    }
    // wenn alle ? und danach . oder Ende: Rekursion mit input[goups_left[0]..] groups_left[1..] current 0
    // und input[groups_left[0]..] groups_Left[..] current 0
    else if advanced_input[0..groups_left[0]].iter().all(|x| *x == '?') {
            if advanced_input.get(groups_left[0]).is_none() || advanced_input[groups_left[0]] == '.' {
                return create_and_test_2(&advanced_input[groups_left[0]..], &groups_left[1..], 0)
                 + create_and_test_2(&advanced_input[groups_left[0]..], groups_left, 0);
            }
    }
      // wenn genau richtig viele # und ? und danach . oder Ende: Rekursion mit input[groups_left[0]..] und groups_left[1..] current 0
      else if ()

    todo!()

    // sonst schauen, was kommt:
    // genau richtig viele # und weitere ? dazwischen: Rekursion input[anzahl_zeichen..] groups_left[1..]
    // zu viele # mit ? dazwischen: könnte passen, je nach Lage der ?
    // zu wenige # mit zu vielen ? dazwischen:
    // um alle # am Anfang vorrücken, Aufruf mit input[vorgerückte_zahl..], groups_left[0..], current_entry_left = groups_left[0] - vorgerückte Zahl
    */
}

// tests:
// current entry left testen
