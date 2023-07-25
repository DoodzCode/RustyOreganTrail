A game in which a player is presented with a situation and several possible actions to choose from.
A player chooses an action to respond to the situation, and the game informs the player of the outcome.

WIN: Reach end of trail with at least one member
LOSS: Everyone dies

Situation: {
    description: "whats going on",
    actions: [action1, action2, action3],
    outcome? aftermath?
}

action {
    cause - 
    effect -
}

Player {
    name: String,
}

Caravan {
    members: <CaravanMember>,
    morale: u32,
    starvation_level?
    dehydration_level?
    cold_level? clothing_level?
    supplies: {
        wood
        food
        tools
        clothing
        water
    }
    wagaon_durability
    disease/sickness chance? // here or on Member
}

/// a single camp activity task performed by a single member
or
/// a single camp activity task performed by multiple members
Task {

}

impl Task for Hunting;

let t = Hunting::new( members:[CaravanMembers] )

> perform_task hunting([steve, oswald, percival])



CaravanMemeber {
    food_consumption - units of food consumed per day
    water_consumption - unites of water consumed per day
    skill: 
        physically_demanding
            hunting - bonus to food produced through hunting
            repairs - 
        not_physically_demanding
            foraging - obtain food from surrounding area
            cooking - prepare more effective food?
            sewing - repair clothing; tents/wagon covers





    sickness_chance: 15%,
}


Situation: 
You are attacked and held up for robbery.
actions:
fight back
let them take 

Difficulty Scale
total, good, nothing   bad
100%, 35%,    35%      30%

poossible sitations: [person gets sick, get robbed, you get sick, wagon breaks, some fucked up shit]


## threats:
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


## good things:
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

**people can do tasks to help the expedition**

* Raise Morale ( actions that benefit morale )
  * Music ( Singing, Dancing, Instrumentals )
  * Stories ( social activity )
* Gather Food
  * Hunt ( req tool )
  * Fish ( req tool )
  * Forage
* Gather Wood
  * Collect Kindle
  * Chop Trees ( req tool )
* Acquire Water*
* Repairs
  * Minor to Moderate Wagon Repair

## Town Activites

* Repairs
  * Major Wagon Repair
* Shop
  * Purchase Supplies
  * Purchase Wagons
  * Purchase Livestock


## Towns

* Economy
  * Desired goods ( wants to trade/buy ), surplus goods ( wants to trade/sell )
  * Quality of Commerce? - struggling, self sustaining, thriving?

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

* Missouri
* Kansas
* Nebraska
* Wyoming
* Idaho
* Oregon

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
  let current_zone = loadZone("Missouri);

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

Sickness has a chance to spread to other members

    a sick member is locked from performing tasks; a second member is locked from tasks to 
    administer aid to the sick member(s). Only 1 member is locked for any number of sick members.

    struct Memeber {
        locked: bool,
    }

    memeber.lock() -> true
    member.unlock() -> false

    get all members where unlock == false;
