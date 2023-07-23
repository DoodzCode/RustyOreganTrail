A game in which a player is presented with a situation and several possible actions to choose from.
A player chooses an action to respond to the situation, and the game informs the player of the outcome.

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

Terrain Types

Plains
Desert
Mountains
Hills
Forest

River Crossings
  - calm
  - rapids

Trail Blocks



Climate Types
Plains - Winds, Tornado
Forest - increase chance of wild life
Dessert - bigger fluxuation in temperature
Mountains - Cold
Hills - warmer then mountains colder then plains



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

struct Player {
    name: String,
}

struct Caravan {
  // members: <CaravanMember>,
  specialists: { // bonus to production
    lumberjack: 3
    medical: 2
    hunting: 5
  }
  non_specialist: 10
  total_roster: 20
  morale: u32,
  food: CaravanResource {
    stock
    consumption_rate
    starvation_level?
  }
  water: CaravanResource {
    stock
    consumption_rate
    dehydration_level?
  }
  cold_level? clothing_level?,
  supplies: {
      wood
      tools
      clothing
  }
  weight: {
    current
    max
  }
  wagaon_durability
}

struct CaravanResource {}

struct CaravanMemeber {
  specialty: None | Hunter | Doctor | Lumberjack
}
```

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

## Camp Activites Part One

people can do tasks to help the expedition

* Raise Morale ( actions that benefit morale )
  * Music ( Singing, Dancing, Instrumentals )
  * Stories

## Resources

* time
* food
* water
* people  can do tasks to help the expedition; people can get sick;
* wood  for fire and repairs
* tools  for repairs
* livestock  pack animals; food animals
* weapons  for defense
* luxury_resource - instruments, finer clothing, things that benefit morale

## states travelled through: Zones ( Rename to match names of the story setting )

### Approx trail length per zone

* Missouri  - 80mi  - 4 days -    Trail points 1 - 40
* Kansas    - 140mi - 7 days -    Trail points 41 - 111
* Nebraska  - 430mi - 22 days -   Trail Points 112 - 327,
* Wyoming   - 500mi - 25 days     Trail Points 328 - 578,
* Idaho     - 500mi - 25 days     579 - 829
* Oregon    - 525mi - 26 days     830 - 1092

1,092 situations per game - on the trail

1,092 travel points on trail

## Tracking Progress

let trail = [PointData, 1085];

trail[4]

- missouri,
- towns

progress Vec<TravelPoint>
{
  point: 1,
  what zone
  mwpofmwePOFW
  trail_point: 1

},
{
  point: 2,
  zone: Missouri,
  on_trail: true,
  qwpodqwopdq
  miles_travelled: 2
  trail_point: 2
},
{
  point: 3,
  zone: missouri,
  on_trail: false,
  miles_traveled: 4,
  trail_point: None,
  off_trail_distance: 2,
},
{
  point: 4,
  zone: missouri
  on_trail: false,
  miles_travelled: 8,
  trail_point: None,
  off_trail_distance: 4,
},
{
  point: 5,
  zone: missouri
  miles_travelled: 10,
  on_trail: false
  trail_point: None
  off_trail_distance: 2,
},
{
  point: 6,
  zone: missouri
  miles_travelled: 12,
  on_trail: true
  trail_point: 3
  off_trail_distance: 0,
}
{
  point: 7,
  zone: missouri
  miles_travelled: 14,
  on_trail: true
  trail_point: 4
  off_trail_distance: 0,
}

{
  trail_point: 1084,
  last point before end of trail

  trail_point: 1085
  end of the trail
}

{

}


```rust
Zone {
    name: "Missouri"
    towns: [],
    peoples: {
        good []
        bad []
    },
    situations: [],
}
let current_zone = loadZone("Missouri");

current_zone.get_situation()

update_current_zone("Kansas")

current_zone.get_situation()
```

## Traveling

trail length: 2170 miles
average distance travelled per day: 20 miles
days to complete trail: ~110

day has 2 parts: mid 10miles, end 10miles
only camp at mid or end day

### morning to mid 10 miles

- if player camps, then 2 rounds of activities/tasks is performed

### mid to end 10 miles

- at camp, 1 round of activities/tasks is performed

1 situation per 2 miles = 5 per day part

1 situation per mile = 3 situations per day
at the end of each day -> special decision/action/situation (camp activity)

## On Trail Activities

* hunting - req mats
* foraging
* fishing - req mats
* repairs
* raise morale
* collect water ( cleaning? purifying? boiling? )

CampActivity {
    name

}

## Sick Caravan Members

    a sick member is locked from performing tasks; a second member is locked from tasks to
    administer aid to the sick member(s). Only 1 member is locked for any number of sick members.

    struct Memeber {
        locked: bool,
    }

    member.lock() -> true
    member.unlock() -> false

    get all members where unlock == false;


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