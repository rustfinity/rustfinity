Enums in Rust can have associated functions and methods, just like structs. These methods make it easier to encapsulate behavior directly within the enum.

In this challenge, you'll model the statuses of different vehicles and implement methods to describe their behavior.

## Your Task

Create an enum `VehicleStatus` with the following variants:

1. `Parked` — a unit variant representing a parked vehicle.
2. `Driving { speed: u32 }` — a named field variant representing a vehicle driving at a certain speed.
3. `BrokenDown(String)` — a tuple variant with a `String` describing the reason for the breakdown.

Implement the following methods for `VehicleStatus`:

- `is_operational(&self) -> bool`:
  - Returns `true` if the vehicle is either `Parked` or `Driving`.
  - Returns `false` if the vehicle is `BrokenDown`.

- `description(&self) -> String`:
  - Returns `"The vehicle is parked."` for `Parked`.
  - Returns `"The vehicle is driving at {speed} km/h."` for `Driving { speed }`.
  - Returns `"The vehicle is broken down: {reason}."` for `BrokenDown(reason)`.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- Use a `match` expression inside the methods to handle each variant.
- Remember to use `&self` for the methods since they should not consume the enum.
- Use `format!` to construct strings with dynamic values, such as `speed` and `reason`.

</details>
