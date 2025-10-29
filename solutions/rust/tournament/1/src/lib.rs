#[derive(Debug)]
struct Team {
    name: String,
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

impl Team {
    fn name(&self) -> String {
        self.name.clone()
    }
}

pub fn tally(match_results: &str) -> String {
    use std::collections::HashMap;

    let mut teams = HashMap::new();

    for line in match_results.lines() {
        let m: Vec<&str> = line.split(";").collect();

        if m[2] == "win" {
            let mut team1 = teams.entry(m[0]).or_insert(Team { name: m[0].to_string(), mp: 0, w: 0, d: 0, l: 0, p: 0 });
            team1.mp += 1;
            team1.w += 1;
            team1.p = (team1.w * 3) + team1.d;

            let mut team2 = teams.entry(m[1]).or_insert(Team { name: m[1].to_string(), mp: 0, w: 0, d: 0, l: 0, p: 0 });
            team2.mp += 1;
            team2.l += 1;
            team2.p = (team2.w * 3) + team2.d;
        } else if m[2] == "loss" {
            let mut team1 = teams.entry(m[0]).or_insert(Team { name: m[0].to_string(), mp: 0, w: 0, d: 0, l: 0, p: 0 });
            team1.mp += 1;
            team1.l += 1;
            team1.p = (team1.w * 3) + team1.d;

            let mut team2 = teams.entry(m[1]).or_insert(Team { name: m[1].to_string(), mp: 0, w: 0, d: 0, l: 0, p: 0 });
            team2.mp += 1;
            team2.w += 1;
            team2.p = (team2.w * 3) + team2.d;
        } else if m[2] == "draw" {
            let mut team1 = teams.entry(m[0]).or_insert(Team { name: m[0].to_string(), mp: 0, w: 0, d: 0, l: 0, p: 0 });
            team1.mp += 1;
            team1.d += 1;
            team1.p = (team1.w * 3) + team1.d;

            let mut team2 = teams.entry(m[1]).or_insert(Team { name: m[1].to_string(), mp: 0, w: 0, d: 0, l: 0, p: 0 });
            team2.mp += 1;
            team2.d += 1;
            team2.p = (team2.w * 3) + team2.d;
        }
    }

    let mut teams: Vec<Team> = teams.into_values().collect();
    teams.sort_by_key(|e| e.name());
    teams.sort_by(|x, y| y.p.cmp(&x.p));

    let mut result = String::new();

    result.push_str(&format!("Team                           | MP |  W |  D |  L |  P")[..]);

    for team in teams {
        result.push_str(&format!("\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", team.name, team.mp, team.w, team.d, team.l, team.p)[..]);
    }

    result
}
