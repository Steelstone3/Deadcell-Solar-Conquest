# Todo

## Steps

- Refactor single player to be independent
- Move sprite enums into client
- Move types into shared
- Move client components into client
- Keep server components in shared?

## Goals

- 1 Client/ Server
  - Dedicated headless state server
  - Client that connect to server
  - Some means of determining the host
  - State sync of host client state with server
- 2 Lobby
  - Menu system
  - Lobby that allows up to 4 players to join and pick a faction (flexibility to expand number of players to like 8 in the future)
- 3 Combat
  - Square projectile design
    - A laser peeeeeeeeeeeeew weapon
    - A pew pew weapon (with leading shots that can miss)
    - A guided pew weapon (that can miss if it overshoots target)
    - Entity deleted after a certain time has passed
    - Some sort of range calculation
  - 3 Attack patterns
    - Artillery
    - Strafe
    - Swarm
    - Moves into range before attacking
    - Enemy factions count as targets to shoot on right click
    - 2 Players of the same faction control the same ships with a combined economy and 2 "starbases"
  - Regenerative shield system
    - Visual indicator?
  - Armour/ Hull system
    - Visual indicator?
  - Deleted from play when depleted to 0 on both takes damage when shield is down
  - Formations so multi-selected ships don't converge on a point
- 4 Economy
  - Very basic energy credit economy
  - 10 per second up to a maximum of 10,000 or something
