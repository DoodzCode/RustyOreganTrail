

// struct PopulationEffect {
//     target: String,
//     modifier: i32,
// }

// #[derive(Debug)]
// struct Stat (String, u32);



// #[derive(Debug)]
// struct Population {
//     total: Stat,
//     status_effects: Vec<PopulationEffect>,
// }

// impl Population {
//     fn get_stat(&self, target: &str) -> u8 {
//         match target {
//             "total" => self.total,
//             "sick" => self.sick,
//             "injured" => self.injured,
//             &_ => 0,
//         }

//     }

//     // fn add_status_effect(&mut self, status_effect: StatusEffect) {

//     // }


// }

// // #[cfg(test)]

// // use super::*;

// // #[test]
// // fn test_stats() {
// //     let mut pop = Population{ total: 20, sick: 0, injured: 0, status_effects: Vec::new()};
// //     // let modifier = Modifier{ name: "Minor Injury", target: &pop.injured, modify: 1};
// //     dbg!(&pop);
// //     pop.status_effects.push(StatusEffect::MinorInjury);
// //     dbg!(&pop);
// // }