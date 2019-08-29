struct FootballTeam {
    members: Vec<String>,
}

impl FootballTeam {
    fn new(members: Vec<String>) -> FootballTeam {
        FootballTeam { members: members }
    }
}

impl IntoIterator for FootballTeam {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}

fn main() {
    let mut members: Vec<String> = Vec::new();

    for i in 1..23 {
        members.push(String::from(format!("player{}", i)));
    }

    members.push("coach1".to_string());
    members.push("coach2".to_string());
    members.push("coach3".to_string());
    
    let team = FootballTeam::new(members);
    let mut team_it = team.into_iter();

    while true {
        println!("{}", team_it.next().unwrap());
    }
}
