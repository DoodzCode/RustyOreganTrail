
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


## Sick Caravan Members

    a sick member is locked from performing tasks; a second member is locked from tasks to
    administer aid to the sick member(s). Only 1 member is locked for any number of sick members.

    struct Memeber {
        locked: bool,
    }

    member.lock() -> true
    member.unlock() -> false

    get all members where unlock == false;

