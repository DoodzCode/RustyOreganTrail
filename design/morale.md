* Morale will be an i8 with ranges from -100 - 100
    - -100 == game over
    - -50 == low
    - 0 == normal
    - 50 == high
    - 100 == max
* Morale effects
    - population
    - production
* Morale effected by
    - chances of success
    - weather
    - entertainment
    - resources:
        - food 
        - water
        - wood
    - proximity to towns
    - desperation
    - sickness
* Events Triggered by Morale
    - 
* genarosity
* desperation  

```rust
struct Morale {
    level: MoraleLevel,
    
}

enum MoraleLevel {
    Min,
    Normal,
    Max,
}

```