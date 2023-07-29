A game in which a player is presented with a situation and several possible actions to choose from.
A player chooses an action to respond to the situation, and the game informs the player of the outcome.

[Flow Chart](https://lucid.app/lucidspark/ff00639e-d6b6-499f-95b2-32c1512139ca/edit?beaconFlowId=3DB1E64FB4E29A4B&invitationId=inv_a1fb25c1-2e84-4a07-87d9-cb830a418dfb&page=0_0#)

1. figure what we need to do and how
   1. how to persist data: Files, Database, DataStore, Spreadsheet
   2. how we structure story data
   3.
   4.
2. figure out what tools to use ( crates, external tools )

1. Caravans leave in mid May and must reach the end by Sep 30.
2. this gives approx 6 months to complete the journey

<hr>

backup_saves





```rust
struct SaveData {
  player_name
  current_region
  caravan
  miles_travelled
  miles_remaining
}

// history of situations and decisions
StoryLog {

}

struct Situation: {
    description: "whats going on",
    actions: [action1, action2, action3],
    outcome? aftermath?
}

struct Action {}


Situation:
You are attacked and held up for robbery.
actions:
fight back
let them take

Difficulty Scale
total, good, nothing   bad
100%, 35%,    35%      30%

poossible sitations: [person gets sick, get robbed, you get sick, wagon breaks, some fucked up shit]

## threats

* starvation - food spoilage, wolves/bears raid food stores
* dehydration - damage to water containers causes loss
* disease - actions could lead to disease; drinking dirty water; not cleaning a wound(infection)
* temperature (freezing to death)
* violent death - attacked by wolves/bears, attack by people
* crossing rivers - people drown
* landslides
* time ( get to oregon before winter )
* supply loss to robbery/theft
* wagon degredation ( it breaks over time; replace wagon wheels)
* in-fighting; inner caravan conflict; domestic politics
* low morale - get voted off the island
* getting lost - lowers morale; extends time between towns for restocking supplies
* people - tied to location/region

## good things

* another caravan joins +people
* find grove of berry bushes: +food
* potable water stream: +water
* town +people? +food? +water? +clothing?

=|=|=|=|=|=|=|=|~|=|=|=
          0

situations ( events presented to the player )
actions ( player's response to situation )
outcome ( result of player's decision )

resources: wood 50 people 10
forage: 5
gather wood: 2
setup camp: 2
cook food: 1
gather_water:


Variations in Events

Zone Difference
climate
environmental hazzards (cold, heat, wind)
  - types of storms: Tornado, Lightning Storms, Blizzards

terrain
resource availability

dangers/threats
danger level: Kansas 4/5
              missouri 1/5

human factions
Americans
Native Americans

Time Differnce
 Weather gets more dangerous as time progresses