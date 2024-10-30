struct Depth {
    action: Action,
    level: u32,
}

#[derive(Debug, PartialEq)]
enum Action {
    Bid,
    Ask,
}

fn main() {
    let depth = vec![
        Depth {
            action: Action::Bid,
            level: 1,
        },
        Depth {
            action: Action::Bid,
            level: 2,
        },
        Depth {
            action: Action::Bid,
            level: 3,
        },
        Depth {
            action: Action::Bid,
            level: 3,
        },
    ];

    println!(
        "{:?}",
        get_new_depth_idx(
            &depth,
            Depth {
                action: Action::Ask,
                level: 1,
            }
        )
    );
}

fn get_new_depth_idx(depth: &Vec<Depth>, new_depth: Depth) -> u32 {
    let mut pos = 0;
    let mut started = false;

    for d in depth {
        if new_depth.action == d.action {
            started = true;
        }

        if started && d.action != new_depth.action {
            break;
        }

        if d.level >= new_depth.level && started {
            break;
        }
        pos += 1;
    }

    pos
}
