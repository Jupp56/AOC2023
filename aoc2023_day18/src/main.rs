use std::{cmp::Ordering, collections::HashSet};

#[derive(Clone, Copy, Debug, Eq)]
struct Vert {
    lower: isize,
    upper: isize,
    col: isize,
}

impl PartialEq<Vert> for Vert {
    fn eq(&self, other: &Vert) -> bool {
        self.col == other.col
    }
}

impl PartialOrd for Vert {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.col.cmp(&other.col))
    }
}

impl Ord for Vert {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let lines = std::fs::read_to_string("../input/day18/test-input").unwrap();

    let mut sum = 0;
    let mut verts = Vec::new();
    let mut current_line = 0;
    let mut current_col = 0;

    for line in lines.lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let len: isize = str::parse(split.next().unwrap()).unwrap();
        match direction {
            "R" => {
                current_col += len;
            }
            "L" => {
                current_col -= len;
            }
            "D" => {
                let prev = current_line;
                current_line += len;
                verts.push(Vert {
                    upper: current_line,
                    lower: prev,
                    col: current_col,
                });
            }

            "U" => {
                let prev = current_line;
                current_line -= len;
                verts.push(Vert {
                    upper: prev,
                    lower: current_line,
                    col: current_col,
                });
            }
            _ => panic!("Unknown direction"),
        }
    }

    // vertical corner pairs

    println!("{:?}", verts);

    //
    verts.sort_by(|a, b| {
        let cmp = a.col.cmp(&b.col);
        if cmp == Ordering::Equal {
            return a.upper.cmp(&b.upper);
        }
        cmp
    });

    println!("{:?}", verts);

    // zur Ermittlung:
    // schauen, ob es genau ein passendes gibt mit line_end_a = line_start_b

    // sonst weiter suchen, bei Abständen nie die zweite Reihe nehmen, Lücke lassen.
    // liste ist von oben nach unten sortiert
    // nimm den nächsten nicht überlappenden
    // such dann durch die nächsten, bis einer nicht mehr von einem weiter vorderen überlappt wird

    let mut firsts = get_firsts(&verts);

    firsts.sort_by(|a, b| a.lower.cmp(&b.lower));

    verts.retain(|x| !firsts.contains(x));

    verts.sort();

    // Für jedes Invervall der ersten Reihe:
    // aus den restlichen Intervallen der zweiten Liste das erste suchen, das irgendwie überlappt.
    for current_first in firsts {
        let (index, corresponding_element) = verts
            .iter()
            .enumerate()
            .find(|(_, other)| {
                (other.upper == current_first.upper && other.lower == current_first.lower)
                    || (other.upper >= current_first.lower && other.upper <= current_first.upper)
                    || (other.lower >= current_first.lower && other.lower <= current_first.upper)
            })
            // make it owned
            .map(|(i, v)| (i, *v))
            .unwrap();

        // beide genau gleich - einfach berechnen
        if corresponding_element.upper == current_first.upper
            && corresponding_element.lower == current_first.lower
        {
            verts.remove(index);
        }

        // zweites steht unten und oben über: zwei neue Einträge, neu sortieren
        if corresponding_element.upper < current_first.upper
            && corresponding_element.lower > current_first.lower
        {
            verts.remove(index);
            verts.push(Vert {
                upper: corresponding_element.upper,
                lower: current_first.upper - 1,
                col: corresponding_element.col,
            });

            verts.push(Vert {
                upper: current_first.lower + 1,
                lower: corresponding_element.lower,
                col: corresponding_element.col,
            });

            // Berechnen
            // kein Rest vom First übrig
            verts.sort();
        } else if corresponding_element.lower < current_first.lower {
            verts[index].upper = current_first.lower - 1;
            // berechnen

            verts.sort();
            // sortieren
        } else if corresponding_element.upper > current_first.upper {
            verts[index].lower = current_first.upper + 1;
            // berechnen

            verts.sort();
        }

        // DAs linke element wurde nicht vollständig verarbeitet.
        if corresponding_element.lower > current_first.lower {
            verts.push(Vert {
                upper: corresponding_element.lower - 1,
                lower: current_first.lower,
                col: current_first.col,
            });
            verts.sort();
        } else if corresponding_element.upper < current_first.upper {
            verts.push(Vert {
                upper: current_first.upper,
                lower: corresponding_element.upper + 1,
                col: current_first.col,
            });
            verts.sort();
        }

        let upper = current_first.upper.min(corresponding_element.upper);
        let lower = current_first.lower.max(corresponding_element.lower);
        let height = (upper - lower) + 1;
        let width = (corresponding_element.col - current_first.col) + 1;

        sum += height * width;
    }

    // go to "Erste Reihe aus intervallen bilden", wenn noch Intervalle zu verwursten sind.
    println!("{sum}");
}

/// the first row of vertical lines
fn get_firsts(verts: &[Vert]) -> Vec<Vert> {
    let mut firsts = verts.to_owned();

    for index in (0..firsts.len()).rev() {
        let element = firsts[index];

        if firsts.iter().any(|x| {
            x.col < element.col
                && (((x.upper < element.upper && x.upper > element.lower)
                    || x.lower < element.upper && x.lower > element.lower)
                    || x.upper == element.upper && x.lower == element.lower)
        }) {
            firsts.remove(index);
        }
    }
    firsts
}
fn part_1() {
    let lines = std::fs::read_to_string("../input/day18/test-input").unwrap();

    let mut sum = 0;
    let mut entries = HashSet::new();
    let mut current_line = 0;
    let mut current_col = 0;

    for line in lines.lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let len: isize = str::parse(split.next().unwrap()).unwrap();

        match direction {
            "R" => {
                for i in current_col + 1..=current_col + len {
                    entries.insert((current_line, i));
                }

                current_col += len;
            }
            "L" => {
                for i in (current_col - len..current_col).rev() {
                    entries.insert((current_line, i));
                }
                current_col -= len;
            }
            "D" => {
                for i in current_line + 1..=current_line + len {
                    entries.insert((i, current_col));
                }
                current_line += len;
            }

            "U" => {
                for i in (current_line - len..current_line).rev() {
                    entries.insert((i, current_col));
                }
                current_line -= len;
            }
            _ => panic!("Unknown direction"),
        }
    }

    let farthest_left = entries.iter().map(|x| x.1).min().unwrap();
    let farthest_right = entries.iter().map(|x| x.1).max().unwrap();
    let farthest_up = entries.iter().map(|x| x.0).min().unwrap();
    let farthest_down = entries.iter().map(|x| x.0).max().unwrap();

    for line in farthest_up..=farthest_down {
        //print!("{:03}", line);
        for col in farthest_left..=farthest_right {
            if entries.contains(&(line, col)) {
                print!("#");
                sum += 1;
                continue;
            }
            let mut walls = 0;

            let mut wall_before = false;
            let mut wall_above = false;
            let mut wall_below = false;
            for i in (farthest_left..col).rev() {
                if entries.contains(&(line, i)) {
                    if !wall_before {
                        if entries.contains(&(line - 1, i)) {
                            wall_above = true;
                        }

                        if entries.contains(&(line + 1, i)) {
                            wall_below = true;
                        }
                        walls += 1;
                        wall_before = true;
                    }
                } else if wall_before {
                    if wall_above && entries.contains(&(line - 1, i + 1)) {
                        walls += 1;
                    }
                    if wall_below && entries.contains(&(line + 1, i + 1)) {
                        walls += 1;
                    }
                    wall_before = false;
                    wall_above = false;
                    wall_below = false;
                }
            }

            if wall_before {
                if line != 0 && wall_above && entries.contains(&(line - 1, farthest_left)) {
                    walls += 1;
                }
                if wall_below && entries.contains(&(line + 1, farthest_left)) {
                    walls += 1;
                }
            }

            if walls % 2 == 1 {
                //println!("{line}, {col} yes");
                sum += 1;
                print!("i");
            } else {
                print!(".");
            }
        }
        println!()
    }

    println!("{sum}");
}

// vertikale Eckpunktpaare suchen
// sortieren links nach rechts
// erste Reihe finden - bei Überlappung in Teilbereiche aufgliedern
// die verarbeiten und dazu die zweite Reihe finden - ggfs mehrere, bei Überlappung aufgliedern.
// alles immer als verarbeitet markieren/löschen, sowohl erste als auch zweite Reihe
// immer Sortierung beachten
// dann wieder die ersten nehmen (das ist die dritte Reihe) und dazu die zweite Reihe finden.
// usw, bis alles verarbeitet/leer

// vertikale Eckpunktpaare ermitteln
// sortieren links nach rechts.
// erste Reihe aus Intervallen bilden, und die restlichen Intervalle in eine andere Liste packen
// zur Ermittlung:
// schauen, ob es genau ein passendes gibt mit line_end_a = line_start_b

// sonst weiter suchen, bei Abständen nie die zweite Reihe nehmen, Lücke lassen.
// liste ist von oben nach unten sortiert
// nimm den nächsten nicht überlappenden
// such dann durch die nächsten, bis einer nicht mehr von einem weiter vorderen überlappt wird
// Für jedes Invervall der ersten Reihe:
// aus den restlichen Intervallen der zweiten Listse das erste suchen, das irgendwie überlappt.
// Wenn das intervall rechts oben/unten größer ist, neues Interval rechts erzeugen und wieder in den Pool der restlichen Intervalle, das alte löschen
// wenn das Intervall rechts oben/unten kleiner ist, verarbeiten und das nächste suchen - für ein neues linkes Intervall ohne den verarbeiteten Teil
// go to "Erste Reihe aus intervallen bilden", wenn noch Intervalle zu verwursten sind.
