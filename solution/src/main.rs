use std::io;
use std::io::BufRead;

fn main() {
    let mut final_map: Vec<String> = Vec::new();
    let mut final_piece: Vec<String> = Vec::new();
    let mut my_turn = false;
    let mut player = (' ', ' ');
    loop {
        let mut y_map = 0;
        let mut y_piece = 0;
        let mut state_map = false;
        let mut pos_start_map = 0;
        let mut pos_start_piece = 0;
        let mut state_piece = false;
        let mut map: Vec<String> = Vec::new();
        let mut piece: Vec<String> = Vec::new();
        let mut c = 0;
        let mut j = 0;

        // Lecture de la ligne d'entrée
        let buffer = io::stdin().lock().lines();

        for (i, line) in buffer.enumerate() {
            // Détection de la carte
            if let Ok(v) = line {
                if v.contains("exec") {
                    if v.contains("p1") {
                        player = ('@', 'a');
                    } else {
                        player = ('$', 's');
                    }
                }

                if v.contains("Anfield") && !state_map {
                    let coordonnee: Vec<&str> = v.split_whitespace().collect();
                    y_map = coordonnee[2][0..coordonnee[2].len() - 1]
                        .to_string()
                        .parse::<usize>()
                        .unwrap();
                    state_map = true;
                    pos_start_map = i + 1;
                }

                // Détection de la pièce
                if v.contains("Piece") {
                    let coordonnee: Vec<&str> = v.split_whitespace().collect();
                    y_piece = coordonnee[2][0..coordonnee[2].len() - 1]
                        .to_string()
                        .parse::<usize>()
                        .unwrap();
                    state_piece = true;
                    pos_start_piece = i;
                }

                // Stockage des lignes de la carte
                if state_map && i > pos_start_map {
                    if c < y_map {
                        let dot: Vec<&str> = v.split_whitespace().collect();
                        if !dot.is_empty() {
                            map.push(dot[1].to_string());
                        }
                        c += 1;
                    }
                }

                // Stockage des lignes de la pièce
                if state_piece && i > pos_start_piece {
                    if j < y_piece {
                        piece.push(v.to_string());
                        j += 1;
                    }
                }

                // Gestion des données complètes de la carte
                if map.len() == y_map && !map.is_empty() {
                    final_map = map.clone();
                    map.clear();
                    c = 0;
                    state_map = false;
                    pos_start_map = 0;
                }

                // Gestion des données complètes de la pièce
                if piece.len() == y_piece && !piece.is_empty() {
                    final_piece = piece.clone();
                    my_turn = true;
                    piece.clear();
                    // j = 0;
                    // state_piece = false;
                    // pos_start_piece = 0;
                    break;
                }
            } else {
                break;
            }
        }
        // Calcul de la meilleure position pour la pièce
        if my_turn {
            best_position(&final_map, &final_piece, player);
            my_turn = false;
        }
    }
}

fn best_position(map: &Vec<String>, piece: &Vec<String>, player: (char, char)) {
    let mut best_score = std::i32::MAX;
    let mut best_position = (0, 0);
    let opponent = if player == ('@', 'a') { ('$', 's') } else { ('@', 'a') };

    let opponent_positions = get_opponent_positions(map, opponent);

    for i in 0..=map.len() {
        for j in 0..=map[0].chars().count() {
            if let Some(pos) = peut_placer_piece(map, piece, i, j, player) {
                let score = evaluate_distance(&pos, &opponent_positions);
                if score < best_score {
                    best_score = score;
                    best_position = pos;
                }
            }
        }
    }

    println!("{} {}", best_position.1, best_position.0);
}

fn get_opponent_positions(map: &Vec<String>, opponent: (char, char)) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for (i, row) in map.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            if ch == opponent.0 || ch == opponent.1 {
                positions.push((i, j));
            }
        }
    }

    positions
}

fn evaluate_distance(pos: &(usize, usize), opponent_positions: &Vec<(usize, usize)>) -> i32 {
    let mut min_distance = std::i32::MAX;

    for &(ox, oy) in opponent_positions {
        let distance = ((ox as i32) - (pos.0 as i32)).abs() + ((oy as i32) - (pos.1 as i32)).abs();
        if distance < min_distance {
            min_distance = distance;
        }
    } 
    min_distance
}

fn peut_placer_piece(
    map: &Vec<String>,
    piece: &Vec<String>,
    x: usize,
    y: usize,
    player: (char, char)
) -> Option<(usize, usize)> {
    let mut touche_at_ou_a = false;
    let mut position_premier_char: Option<(usize, usize)> = None;

    // Vérifier que la pièce ne déborde pas de la carte
    if x + piece.len() > map.len() || y + piece[0].len() > map[0].len() {
        return None;
    }

    for i in 0..piece.len() {
        for j in 0..piece[i].chars().count() {
            let current_char = piece[i].chars().nth(j).unwrap();
            let map_char = map[x + i]
                .chars()
                .nth(y + j)
                .unwrap();

            match current_char {
                'O' => {
                    if map_char == '.' {
                        if position_premier_char.is_none() {
                            position_premier_char = Some((x, y));
                        }
                    } else if map_char == player.0 || map_char == player.1 {
                        if !touche_at_ou_a {
                            touche_at_ou_a = true;
                            if position_premier_char.is_none() {
                                position_premier_char = Some((x, y));
                            }
                        } else {
                            return None; // Plus d'un 'O' touche un '@' ou 'a'
                        }
                    } else {
                        return None; // La pièce touche un autre caractère non valide
                    }
                }
                '.' => {
                    if position_premier_char.is_none() {
                        position_premier_char = Some((x, y));
                    }
                    // Les points peuvent se poser n'importe où
                }
                _ => {}
            }
        }
    }

    if touche_at_ou_a {
        position_premier_char
    } else {
        None
    }
}
