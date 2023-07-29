

Climate Types
Plains - Winds, Tornado
Forest - increase chance of wild life
Dessert - bigger fluxuation in temperature
Mountains - Cold
Hills - warmer then mountains colder then plains

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

## Camp Activites Part One

people can do tasks to help the expedition

* Raise Morale ( actions that benefit morale )
  * Music ( Singing, Dancing, Instrumentals )
  * Stories

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